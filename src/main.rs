#![feature(once_cell, try_blocks)]
mod bootstrapper;
mod controlflow;
mod run;

use bootstrapper::{
    shared_ctx::{EventQueue, Quit, Region, RenderBase, RenderBaseCT},
    universal_bootstrap,
};
use crossterm::event::{Event, KeyCode, KeyEvent};
use leptos_reactive::{create_effect, create_rw_signal, use_context};
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Terminal,
};

use run::run_app;

fn main() -> Result<(), Box<dyn Error>> {
    universal_bootstrap(|cx| {
        let quit = use_context::<Quit>(cx).unwrap();
        let event = use_context::<EventQueue>(cx).unwrap();
        let sz = use_context::<Region>(cx).unwrap();
        let term = use_context::<RenderBaseCT>(cx).unwrap();
        let term0 = term.clone();
        let str = create_rw_signal(cx, "    ".to_string());

        create_effect(cx, move |_| {
            str.with(|v| {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(Span::raw(
                        v.as_str(),
                        // Style::default().bg(tui::style::Color::Black),
                    ))
                    .title_alignment(tui::layout::Alignment::Center)
                    .border_type(BorderType::Rounded);
                let p =
                    Paragraph::new(Span::from(v.as_str())).wrap(tui::widgets::Wrap { trim: true });

                term.render(p, block.inner(sz.0()));
                term.render(block, sz.0());
            })
        });

        create_effect(cx, move |_| match event.0() {
            Event::Key(e) => match e.code {
                KeyCode::Char(c) => str.update(|v| v.push(c)),
                KeyCode::Enter => quit.quit(),
                KeyCode::Up => term0.render(Clear, sz.0()),
                _ => (),
            },
            _ => {}
        })
    })
}
// fn main() -> Result<(), Box<dyn Error>> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // create app and run it
//     let res = run_app(&mut terminal);

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{:?}", err)
//     }

//     Ok(())
// }
