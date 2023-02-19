use leptos_reactive::{create_rw_signal, provide_context, ReadSignal, RwSignal, Scope};
use tui::layout::Rect;

#[derive(Clone, Copy)]
pub struct Region(pub ReadSignal<Rect>);

impl Region {
    pub fn attach(cx: Scope, v: Rect) -> RwSignal<Rect> {
        let base = create_rw_signal(cx, v);
        let v = Self(base.read_only());

        provide_context(cx, v);

        base
    }
}
