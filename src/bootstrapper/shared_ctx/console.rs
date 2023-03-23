use std::mem::MaybeUninit;

use chrono::{DateTime, Local, Timelike};
use crossterm::event::KeyCode;
use leptos_reactive::*;
use tui::{
    layout::Rect,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    controlflow::{each, show},
    split_word_wrap::split_word_wrap,
};

use super::{EventQueue, Region, RenderBaseAuto};

#[derive(Clone, Copy)]
pub struct Console(RwSignal<Vec<(DateTime<Local>, String)>>);

impl std::ops::Deref for Console {
    type Target = RwSignal<Vec<(DateTime<Local>, String)>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Console {
    pub fn attach(cx: Scope) -> Self {
        let v = Self(create_rw_signal(
            cx,
            vec![
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
                (
                    Local::now(),
                    "Terminal attached; program is bootstrapped".into(),
                ),
            ],
        ));

        provide_context(cx, v);

        v
    }

    pub fn render_encapsulate(self, cx: Scope, child: impl FnOnce(Scope)) {
        let region: Region = use_context(cx).unwrap();
        let eq: EventQueue = use_context(cx).unwrap();

        let when = create_memo(cx, move |v| {
            let v = v.cloned().unwrap_or(false);
            eq.0.with(move |e| match e {
                crossterm::event::Event::Key(k) if k.code == KeyCode::F(12) => !v,
                _ => v,
            })
        });

        let child_area = Signal::derive(cx, move || {
            let r = region();
            if when() {
                Rect::new(r.x, r.y, r.width, r.height / 2)
            } else {
                r
            }
        });
        let my_area = Signal::derive(cx, move || {
            let r = region();
            let c = child_area();

            Rect::new(0, c.height, r.width, r.height - c.height)
        });

        cx.child_scope(|cx| {
            Region::derive(cx, child_area);

            child(cx);
        });

        show(
            cx,
            when,
            move |cx| {
                let block = store_value(
                    cx,
                    Block::default()
                        .borders(Borders::ALL)
                        .style(tui::style::Style::default().bg(tui::style::Color::Red)),
                );

                let inner_area =
                    Signal::derive(cx, move || block.with(|block| block.inner(my_area())));

                let rb: RenderBaseAuto = use_context(cx).unwrap();

                create_effect(cx, move |_| {
                    let reg = my_area();

                    rb.render(Clear, reg);
                    rb.render(block(), reg);
                });

                each(
                    cx,
                    move || {
                        self.with(move |data| {
                            data.iter()
                                .rev()
                                .take(inner_area().height.into())
                                .cloned()
                                .collect::<Vec<_>>()
                                .into_iter()
                        })
                    },
                    move |_, idx| idx,
                    move |last: Option<&(u16, u16, Paragraph)>, (inst, data), _| {
                        let inner_area = inner_area();

                        let offset = last.map(move |v| v.0 as u16).unwrap_or_default();
                        let available = inner_area.height - offset;

                        if available == 0 {
                            return None;
                        }

                        let data_blocks = split_word_wrap(&data, inner_area.width as usize - 8);

                        let mut lines = vec![];

                        let used_height = std::cmp::min(available.into(), data_blocks.len());

                        let mut data_blocks = data_blocks.into_iter().rev().take(used_height).rev();

                        lines.push(Spans::from(format!(
                            "[{:02}:{:02}] {}",
                            inst.minute(),
                            inst.second(),
                            &data[data_blocks.next().unwrap()]
                        )));

                        for i in data_blocks {
                            lines.push(Spans::from(format!("    >>> {}", &data[i])))
                        }

                        let used_height = used_height as u16;

                        Some((
                            offset + used_height,
                            used_height,
                            Paragraph::new(Text { lines }),
                        ))
                    },
                    move |cx, data| {
                        let rb: RenderBaseAuto = use_context(cx).unwrap();
                        data.with(move |(off, h, p)| {
                            let inner_area = inner_area();

                            let rect = Rect::new(
                                1,
                                inner_area.y + inner_area.height - off,
                                inner_area.width,
                                *h,
                            );

                            rb.render(p.clone(), rect);
                        })
                    },
                );
            },
            |_cx| {},
        );
    }

    pub fn log(&self, msg: impl Into<String>) {
        self.0.update(move |v| v.push((Local::now(), msg.into())))
    }
    pub fn clear(&self) {
        self.0.update(move |v| v.clear())
    }
}
