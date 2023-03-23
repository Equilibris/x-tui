use leptos_reactive::{create_rw_signal, provide_context, ReadSignal, Scope, WriteSignal};

#[derive(Clone, Copy)]
pub struct RenderCounter(pub ReadSignal<usize>);

impl RenderCounter {
    pub fn attach(cx: Scope) -> WriteSignal<usize> {
        let (r, w) = create_rw_signal(cx, 0).split();

        let v = Self(r);

        provide_context(cx, v);

        w
    }
}
