use leptos_reactive::{
    create_rw_signal, provide_context, RwSignal, Scope, UntrackedGettableSignal,
};

#[derive(Clone, Copy)]
pub struct Quit(RwSignal<bool>);

impl Quit {
    pub fn attach(cx: Scope) -> Self {
        let v = Self(create_rw_signal(cx, false));
        provide_context(cx, v);
        v
    }

    pub fn quit(&self) {
        self.0.set(true);
    }
    pub fn should_quit(&self) -> bool {
        self.0.get_untracked()
    }
}
