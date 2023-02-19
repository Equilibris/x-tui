use crossterm::event::Event;
use leptos_reactive::{use_context, Scope};
use std::{
    error::Error,
    sync::{
        mpsc::{self, channel},
        Arc, Mutex,
    },
};
use tui::{backend::Backend, Terminal};

use super::{prefix_sum_2d::PrefixSum2d, shared_ctx};

use super::shared_ctx::*;

pub type Term<B> = Arc<Mutex<(Terminal<B>, PrefixSum2d)>>;

pub trait Init<B: Backend>: Sized {
    fn init(
        self,
        cx: Scope,
        terminal: Term<B>,
        boot: impl FnOnce(Scope) + 'static,
    ) -> Result<Self, Box<dyn Error>>;
}

pub struct AppInit;

impl<B: Backend + 'static> Init<B> for AppInit {
    fn init(
        self,
        cx: Scope,
        terminal: Term<B>,
        boot: impl FnOnce(Scope) + 'static,
    ) -> Result<Self, Box<dyn Error>> {
        let quit = Quit::attach(cx);
        let eq = EventQueue::attach(cx);
        let region = Region::attach(cx, terminal.try_lock().unwrap().0.size()?);
        RenderBase::attach(cx, terminal);

        shared_ctx::Terminal::attach(cx).render_encapsulate(cx, |cx| boot(cx));

        while !quit.should_quit() {
            let rb = use_context::<RenderBase<B>>(cx).unwrap();
            rb.do_frame()?;

            EventQueue::poll(&eq, &region, rb)?;
        }

        Ok(Self)
    }
}

pub struct TestInit(mpsc::Sender<Event>, mpsc::Receiver<Event>);

impl TestInit {
    pub fn new() -> Self {
        let (tc, tx) = channel();

        Self(tc, tx)
    }
}

impl<B: Backend + 'static> Init<B> for TestInit {
    fn init(
        self,
        cx: Scope,
        terminal: Term<B>,
        boot: impl FnOnce(Scope) + 'static,
    ) -> Result<Self, Box<dyn Error>> {
        let quit = Quit::attach(cx);
        let eq = EventQueue::attach(cx);
        let region = Region::attach(cx, terminal.try_lock().unwrap().0.size()?);
        EventDispatcher::attach(cx, self.0.clone());
        RenderBase::attach(cx, Arc::clone(&terminal));

        boot(cx);

        while !quit.should_quit() {
            let rb = use_context::<RenderBase<B>>(cx).unwrap();
            rb.do_frame()?;

            EventQueue::test_poll(&eq, &self.1, &region, rb)?;
        }

        Ok(self)
    }
}
