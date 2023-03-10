use std::time::Instant;

use crossterm::event::KeyCode;
use leptos_reactive::{
    create_effect, create_memo, create_rw_signal, provide_context, use_context, RwSignal, Scope,
    Signal,
};
use tui::{
    layout::Rect,
    widgets::{Block, Borders, Clear},
};

use crate::controlflow::show;

use super::{EventQueue, Region, RenderBaseAuto};

#[derive(Clone, Copy)]
pub struct Terminal(RwSignal<Vec<(Instant, String)>>);

impl Terminal {
    pub fn attach(cx: Scope) -> Self {
        let v = Self(create_rw_signal(cx, vec![]));

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
                let rb: RenderBaseAuto = use_context(cx).unwrap();

                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(tui::style::Style::default().bg(tui::style::Color::Red));

                create_effect(cx, move |_| {
                    let block = block.clone();
                    let reg = my_area();

                    rb.render(Clear, reg);
                    rb.render(block, reg);
                });
            },
            |_cx| {},
        );
    }

    pub fn log(&self, msg: String) {
        self.0.update(move |v| v.push((Instant::now(), msg)))
    }
    pub fn clear(&self) {
        self.0.update(move |v| v.clear())
    }
}
