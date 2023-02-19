use leptos_reactive::{create_rw_signal, provide_context, RwSignal, Scope};

#[derive(Clone, Copy)]
pub struct Terminal(RwSignal<Vec<String>>);

impl Terminal {
    pub fn attach(cx: Scope) -> Self {
        let v = Self(create_rw_signal(cx, vec![]));

        provide_context(cx, v);

        v
    }
}
