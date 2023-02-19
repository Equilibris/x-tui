use std::sync::{Arc, Mutex, Weak};

use leptos_reactive::{provide_context, Scope};
use tui::{backend::Backend, layout::Rect, Terminal};

use crate::bootstrapper::prefix_sum_2d::PrefixSum2d;

// Flushing could be scoped but this is an optimization that has to be evaluated
pub struct RenderBase<B: Backend + 'static>(pub(super) Weak<Mutex<(Terminal<B>, PrefixSum2d)>>);

pub type RenderBaseCT = RenderBase<tui::backend::CrosstermBackend<std::io::Stdout>>;

impl<B: Backend + 'static> RenderBase<B> {
    pub fn attach(cx: Scope, v: Arc<Mutex<(Terminal<B>, PrefixSum2d)>>) {
        let v = Self(Arc::downgrade(&v));
        provide_context(cx, v)
    }

    pub fn access(&self) -> Arc<Mutex<(Terminal<B>, PrefixSum2d)>> {
        self.0.upgrade().unwrap()
    }
    pub fn render<T: tui::widgets::Widget>(&self, widget: T, area: Rect) {
        let upgrade = self.access();
        let mut term = upgrade.try_lock().unwrap();

        term.0.get_frame().render_widget(widget, area);
        term.1.insert(area);
    }

    pub fn do_frame(&self) -> Result<(), std::io::Error> {
        let term = self.access();
        let mut term = &mut term.try_lock().unwrap();

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

        Ok(())
    }
}

impl<B: Backend> Clone for RenderBase<B> {
    fn clone(&self) -> Self {
        Self(Weak::clone(&self.0))
    }
}
