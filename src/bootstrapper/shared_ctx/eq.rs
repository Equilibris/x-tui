use std::{
    error::Error,
    sync::mpsc::{self},
    time::Duration,
};

use crossterm::event::{self, Event};
use leptos_reactive::*;
use tui::{backend::Backend, layout::Rect};

use super::RenderBase;

#[derive(Clone)]
pub struct EventDispatcher(mpsc::Sender<Event>);

impl EventDispatcher {
    pub fn dispatch(&self, e: Event) -> Result<(), mpsc::SendError<Event>> {
        self.0.send(e)
    }
    pub fn attach(cx: Scope, tx: mpsc::Sender<Event>) {
        provide_context(cx, Self(tx))
    }
}

#[derive(Clone, Copy)]
pub struct EventQueue(pub ReadSignal<Event>);

impl std::ops::Deref for EventQueue {
    type Target = ReadSignal<Event>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EventQueue {
    pub fn attach(cx: Scope) -> RwSignal<Event> {
        let base = create_rw_signal(cx, Event::FocusGained);
        let v = Self(base.read_only());

        provide_context(cx, v);

        base
    }
    fn dispatch<B: Backend>(
        eq: &RwSignal<Event>,
        e: Event,
        region: &RwSignal<Rect>,
        rb: RenderBase<B>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(match e {
            Event::Resize(h, w) => {
                let rect = tui::layout::Rect::new(0, 0, w, h);
                {
                    let rb = rb.access();
                    let mut rb = rb.lock().unwrap();
                    rb.1.resize(rect);
                    rb.0.resize(rect)?;
                }

                region.set(rect);
                eq.set(Event::Resize(h, w));
            }
            e => eq.set(e),
        })
    }
    pub fn test_poll<B: Backend>(
        eq: &RwSignal<Event>,
        rec: &mpsc::Receiver<Event>,
        region: &RwSignal<Rect>,
        rb: RenderBase<B>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(match rec.recv_timeout(Duration::from_millis(1)) {
            Ok(e) => Self::dispatch(eq, e, region, rb)?,
            Err(e) => match e {
                mpsc::RecvTimeoutError::Timeout => (),
                mpsc::RecvTimeoutError::Disconnected => Err(e)?,
            },
        })
    }
    pub fn poll<B: Backend>(
        eq: &RwSignal<Event>,
        region: &RwSignal<Rect>,
        rb: RenderBase<B>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(if event::poll(Duration::from_millis(10))? {
            Self::dispatch(eq, event::read()?, region, rb)?
        })
    }
}
