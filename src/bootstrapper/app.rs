use super::{bootstrap, init::AppInit};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use leptos_reactive::Scope;
use std::{
    borrow::BorrowMut,
    error::Error,
    io, panic,
    sync::{Arc, Mutex},
};
use tui::{backend::CrosstermBackend, Terminal};

use super::prefix_sum_2d::PrefixSum2d;

pub fn app_bootstrap(boot: impl FnOnce(Scope) + 'static) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let ps = PrefixSum2d::new(terminal.size()?);

    terminal.hide_cursor()?;
    terminal.clear()?;

    let terminal = Arc::new(Mutex::new((terminal, ps)));

    panic::set_hook(Box::new(|c| {
        disable_raw_mode().unwrap();

        println!("{}", c);
    }));

    let res = bootstrap(terminal.clone(), boot, AppInit::default());

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

    match res?.0 {
        Some(v) if v.len() > 0 => println!("{}", v),
        _ => (),
    }
    Ok(())
}
