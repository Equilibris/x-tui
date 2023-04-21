#![feature(once_cell, try_blocks, iter_intersperse)]
mod bootstrapper;
mod controlflow;
mod split_word_wrap;
mod tdom;

use bootstrapper::{app_bootstrap, shared_ctx::*};
use crossterm::event::{Event, KeyCode};
use leptos_reactive::{create_effect, create_rw_signal, prelude::*, use_context};
use std::error::Error;
use tdom::*;
use tui::{
    text::Span,
    widgets::{BorderType, Borders, Clear, Paragraph},
};

fn main() -> Result<(), Box<dyn Error>> {
    app_bootstrap(|cx| {
        let quit: Quit = use_context(cx).unwrap();
        let event: EventQueue = use_context(cx).unwrap();
        let sz: Region = use_context(cx).unwrap();
        let term: RenderBaseAuto = use_context(cx).unwrap();
        let cons: Console = use_context(cx).unwrap();
        let term0 = term.clone();
        let str = create_rw_signal(cx, "    ".to_string());

        tdom::Block::default()
            .borders(Borders::ALL)
            .title(MaybeSignal::derive(cx, move || {
                str()
                // str.with(|v| Span::raw(v.as_str()))
            }))
            .title_alignment(tui::layout::Alignment::Center)
            .border_type(BorderType::Rounded)
            .render(cx, sz.get_untracked(), &term);
        // create_effect(cx, move |_| {
        //     str.with(|v| {
        //         let block = tui::widgets::Block::default().borders(Borders::ALL);
        //         let p =
        //             Paragraph::new(Span::from(v.as_str())).wrap(tui::widgets::Wrap { trim: true });

        //         term.render(Clear, sz());
        //         term.render(p, block.inner(sz()));
        //         term.render(block, sz());
        //     })
        // });

        create_effect(cx, move |_| match event() {
            Event::Key(e) => match e.code {
                KeyCode::Char(c) => str.update(|v| v.push(c)),
                KeyCode::Enter => quit.quit_with_message("Hello World"),
                KeyCode::Up => term0.render(Clear, sz()),
                KeyCode::Backspace => str.update(|v| {
                    v.pop();
                }),
                KeyCode::Down => cons.log("Down pressed\n\n\nhi\nhi"),
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
