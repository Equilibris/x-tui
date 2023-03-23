use crossterm::event::Event;
use leptos_reactive::*;
use std::{
    error::Error,
    sync::{
        mpsc::{self, channel},
        Arc, Mutex,
    },
};
use tui::{backend::Backend, Terminal};

use super::{
    prefix_sum_2d::PrefixSum2d,
    shared_ctx::{self, Console},
};

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

#[derive(Default)]
pub struct AppInit(pub(super) Option<String>);

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
        let rc = RenderCounter::attach(cx);

        RenderBase::attach(cx, terminal);

        Console::attach(cx).render_encapsulate(cx, |cx| boot(cx));

        let mut count = 0;

        while !quit.should_quit() {
            let rb = use_context::<RenderBase<B>>(cx).unwrap();
            rb.do_frame()?;

            EventQueue::poll(&eq, &region, rb)?;
            count += 1;

            rc.set(count);
        }

        Ok(Self(Some(quit.get_msg())))
    }
}

pub struct TestInit(Option<usize>, mpsc::Sender<Event>, mpsc::Receiver<Event>);

impl TestInit {
    pub fn new(count: Option<usize>) -> Self {
        let (tc, tx) = channel();

        Self(count, tc, tx)
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
        let rc = RenderCounter::attach(cx);

        EventDispatcher::attach(cx, self.1.clone());
        RenderBase::attach(cx, Arc::clone(&terminal));

        let mut count = 0;

        boot(cx);

        while !quit.should_quit()
            && match self.0 {
                Some(v) => count < v,
                _ => true,
            }
        {
            let rb = use_context::<RenderBase<B>>(cx).unwrap();
            rb.do_frame()?;

            EventQueue::test_poll(&eq, &self.2, &region, rb)?;
            count += 1;

            rc.set(count);
        }

        Ok(self)
    }
}
