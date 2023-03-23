use leptos_reactive::*;

#[derive(Clone, Copy)]
pub struct Quit(RwSignal<Option<String>>);

impl Quit {
    pub fn attach(cx: Scope) -> Self {
        let v = Self(create_rw_signal(cx, None));
        provide_context(cx, v);
        v
    }

    pub fn quit_with_message(&self, msg: impl Into<String>) {
        self.0.set(Some(msg.into()))
    }
    pub fn quit(&self) {
        self.0.set(Some(String::new()));
    }
    pub fn get_msg(&self) -> String {
        self.0.get().unwrap()
    }
    pub fn should_quit(&self) -> bool {
        self.0.get_untracked().is_some()
    }
}
