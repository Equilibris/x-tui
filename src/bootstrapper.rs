mod prefix_sum_2d;

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use leptos_reactive::{
    create_runtime, create_rw_signal, create_scope, provide_context, use_context, ReadSignal,
    RwSignal, Scope,
};
use std::{
    cell::OnceCell,
    error::Error,
    io::{self, Stdout},
    sync::{atomic::AtomicBool, Arc, Mutex, Weak},
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::Rect,
    Terminal, TerminalOptions,
};

use self::prefix_sum_2d::PrefixSum2d;

#[derive(Clone, Copy)]
pub struct Quit(RwSignal<bool>);

impl Quit {
    pub fn quit(&self) {
        self.0.set(true);
    }
}

#[derive(Clone, Copy)]
pub struct EventQueue(pub ReadSignal<Event>);

#[derive(Clone, Copy)]
pub struct Region(pub ReadSignal<Rect>);

// Flushing could be scoped but this is an optimization that has to be evaluated
pub struct RenderBase<B: Backend + 'static>(Weak<Mutex<(Terminal<B>, PrefixSum2d)>>);
pub type RenderBaseCT = RenderBase<CrosstermBackend<Stdout>>;

impl<B: Backend + 'static> RenderBase<B> {
    pub fn access(&self) -> Arc<Mutex<(Terminal<B>, PrefixSum2d)>> {
        self.0.upgrade().unwrap()
    }
    pub fn render<T: tui::widgets::Widget>(&self, widget: T, area: Rect) {
        let upgrade = self.access();
        let mut term = upgrade.try_lock().unwrap();

        term.0.get_frame().render_widget(widget, area);
        term.1.insert(area);
    }
}

impl<B: Backend> Clone for RenderBase<B> {
    fn clone(&self) -> Self {
        Self(Weak::clone(&self.0))
    }
}

pub fn bootstrap<T: FnOnce(Scope) + 'static, B: Backend + 'static>(
    terminal: Arc<Mutex<(Terminal<B>, PrefixSum2d)>>,
    boot: T,
) -> Result<(), Box<dyn Error>> {
    let out = Arc::new(OnceCell::new());
    let inner_out = Arc::clone(&out);

    let dispose = create_scope(create_runtime(), move |cx| {
        let out = inner_out;
        let quit = create_rw_signal(cx, false);
        let quit = Quit(quit);

        let eq = create_rw_signal(cx, Event::FocusGained);
        let eqr = EventQueue(eq.read_only());

        let region = create_rw_signal(
            cx,
            match terminal.try_lock().unwrap().0.size() {
                Ok(t) => t,
                Err(e) => {
                    out.set(Err(e.into())).unwrap();
                    return;
                }
            },
        );
        let regionr = Region(region.read_only());

        let rb = RenderBase(Arc::downgrade(&terminal));

        provide_context(cx, quit);
        provide_context(cx, eqr);
        provide_context(cx, regionr);
        provide_context(cx, rb);

        boot(cx);

        while !quit.0() {
            match {
                let term = use_context::<RenderBase<B>>(cx).unwrap().access();
                let mut term = &mut term.try_lock().unwrap();
                let o: Result<_, io::Error> = try {
                    let sz = term.0.size()?;
                    let data = term.0.current_buffer_mut().clone();
                    let base_data = term.1.clone();

                    let data = data
                        .content
                        .iter()
                        .zip(base_data.iter())
                        .filter_map(|(cell, (x, y, v))| (v > 0).then_some((x, y, cell)));

                    term.0.backend_mut().draw(data)?;
                    term.0.backend_mut().flush()?;
                    term.1.clear();
                };

                o
            } {
                Ok(_) => (),
                Err(e) => {
                    out.set(Err(e.into())).unwrap();
                    return;
                }
            }

            if poll(Duration::from_millis(10)).unwrap() {
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
    });

    dispose.dispose();

    Arc::try_unwrap(out).unwrap().into_inner().unwrap_or(Ok(()))
}

pub fn universal_bootstrap<F: FnOnce(Scope) + 'static>(boot: F) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let ps = PrefixSum2d::new(terminal.size()?);
    let mut terminal = Arc::new(Mutex::new((terminal, ps)));

    terminal.try_lock().unwrap().0.hide_cursor().unwrap();
    terminal.try_lock().unwrap().0.clear().unwrap();
    // .draw(|f| f.render_widget(Clear, f.size()))?;
    // create app and run it
    let res = bootstrap(terminal.clone(), boot);

    let mut terminal = Arc::try_unwrap(terminal)
        .unwrap_or_else(|_| panic!("Terminal leaked in main-loop"))
        .into_inner()?
        .0;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
