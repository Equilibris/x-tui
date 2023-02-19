use leptos_reactive::{create_rw_signal, provide_context, RwSignal, Scope, Signal};
use tui::layout::Rect;

#[derive(Clone, Copy)]
pub struct Region(pub Signal<Rect>);

impl std::ops::Deref for Region {
    type Target = Signal<Rect>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Region {
    pub fn derive(cx: Scope, base: Signal<Rect>) {
        let v = Self(base);

        provide_context(cx, v)
    }
    pub fn attach(cx: Scope, v: Rect) -> RwSignal<Rect> {
        let base = create_rw_signal(cx, v);
        let v = Self(base.read_only().into());

        provide_context(cx, v);

        base
    }
}
