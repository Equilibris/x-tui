pub mod init;
mod prefix_sum_2d;
pub mod shared_ctx;
mod test;
pub use test::*;
mod app;
pub use app::*;

use leptos_reactive::{create_runtime, create_scope, Scope};
use std::{cell::OnceCell, error::Error, sync::Arc};
use tui::backend::Backend;

use init::{Init, Term};

pub fn bootstrap<T: FnOnce(Scope) + 'static, B: Backend + 'static, I: Init<B> + 'static>(
    terminal: Term<B>,
    boot: T,
    init: I,
) -> Result<I, Box<dyn Error>> {
    let out = Arc::new(OnceCell::new());
    let inner_out = Arc::clone(&out);

    let dispose = create_scope(create_runtime(), move |cx| {
        inner_out
            .set(init.init(cx, terminal, boot))
            .unwrap_or_else(|_| unreachable!());
    });

    dispose.dispose();

    Arc::try_unwrap(out)
        .unwrap_or_else(|_| unreachable!())
        .into_inner()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use leptos_reactive::{create_effect, use_context};

    use super::{
        assert_rb,
        shared_ctx::{Quit, RenderBaseAuto},
        test_bootstrap,
    };

    #[test]
    fn basic_init_then_quit() {
        test_bootstrap(
            |cx| {
                let quit = use_context::<Quit>(cx).unwrap();

                let rb: RenderBaseAuto = use_context(cx).unwrap();

                create_effect(cx, move |_| {
                    assert_rb(&rb, "base");
                })
            },
            10,
            10,
            Some(0),
        )
        .unwrap();
    }
}
