mod prefix_sum_2d;
pub mod shared_ctx;

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

use self::shared_ctx::*;

pub fn bootstrap<T: FnOnce(Scope) + 'static, B: Backend + 'static>(
    terminal: Arc<Mutex<(Terminal<B>, PrefixSum2d)>>,
    boot: T,
) -> Result<(), Box<dyn Error>> {
    let out = Arc::new(OnceCell::new());
    let inner_out = Arc::clone(&out);

    let dispose = create_scope(create_runtime(), move |cx| {
        let out = inner_out;

        let quit = Quit::attach(cx);
        let eq = EventQueue::attach(cx);
        let region = Region::attach(
            cx,
            match terminal.try_lock().unwrap().0.size() {
                Ok(t) => t,
                Err(e) => {
                    out.set(Err(e.into())).unwrap();
                    return;
                }
            },
        );
        RenderBase::attach(cx, terminal);

        boot(cx);

        while !quit.should_quit() {
            match use_context::<RenderBase<B>>(cx).unwrap().do_frame() {
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
