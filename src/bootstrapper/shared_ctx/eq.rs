use std::{cell::OnceCell, sync::Arc};

use crossterm::event::{self, Event};
use leptos_reactive::{create_rw_signal, provide_context, ReadSignal, RwSignal, Scope};

#[derive(Clone, Copy)]
pub struct EventQueue(pub ReadSignal<Event>);

impl EventQueue {
    pub fn attach(cx: Scope) -> RwSignal<Event> {
        let base = create_rw_signal(cx, Event::FocusGained);
        let v = Self(base.read_only());

        provide_context(cx, v);

        base
    }
    pub fn poll(eq: &RwSignal<Event>, out: &Arc<OnceCell<Result<(), Box<dyn std::error::Error>>>>) {
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            match event::read() {
                Ok(Event::Resize(h, w)) => {
                    todo!()
                }
                Ok(e) => {
                    eq.set(e);
                }
                Err(e) => {
                    out.set(Err(e.into())).unwrap();
                    return;
                }
            }
        }
    }
}
