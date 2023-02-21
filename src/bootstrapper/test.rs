use leptos_reactive::Scope;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tui::{backend::TestBackend, Terminal};

use super::{bootstrap, init::TestInit, prefix_sum_2d::PrefixSum2d};

pub fn test_bootstrap(
    boot: impl FnOnce(Scope) + 'static,
    w: u16,
    h: u16,
    once: bool,
) -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(w, h);
    let terminal = Terminal::new(backend)?;
    let ps = PrefixSum2d::new(terminal.size()?);

    let terminal = Arc::new(Mutex::new((terminal, ps)));

    let init = TestInit::new(once);

    bootstrap(terminal, boot, init)?;
    Ok(())
}

#[cfg(test)]
use super::shared_ctx::RenderBase;
#[cfg(test)]
pub fn assert_rb(rb: &RenderBase<TestBackend>, snap_name: &'_ str) {
    let term = rb.access();
    let mut term = term.try_lock().unwrap();

    let rect = term.0.size().unwrap();

    let mut symbols = term
        .0
        .current_buffer_mut()
        .content
        .iter()
        .map(|v| v.symbol.as_str());
    let out = (0..rect.height)
        .map(|_| {
            (0..rect.width)
                .map(|_| symbols.next().unwrap())
                .collect::<String>()
        })
        .intersperse("$\n".into())
        .chain(std::iter::once("$".into()))
        .collect::<String>();

    insta::assert_snapshot!(snap_name, out.as_str());
}
