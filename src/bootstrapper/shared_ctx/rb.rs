use std::{
    convert::Infallible,
    sync::{Arc, Mutex, Weak},
};

use leptos_reactive::{provide_context, Scope};
use tui::{backend::Backend, layout::Rect, Terminal};

use crate::bootstrapper::prefix_sum_2d::PrefixSum2d;

// Flushing could be scoped but this is an optimization that has to be evaluated
pub struct RenderBase<B: Backend + 'static>(
    pub(in super::super) Weak<Mutex<(Terminal<B>, PrefixSum2d)>>,
);

#[cfg(not(test))]
#[allow(dead_code)]
pub type RenderBaseAuto = RenderBase<tui::backend::CrosstermBackend<std::io::Stdout>>;
#[cfg(test)]
pub type RenderBaseAuto = RenderBase<tui::backend::TestBackend>;

pub enum RBOp<T: tui::widgets::Widget = tui::widgets::Clear> {
    Component(T, Rect),
    Add(Rect),
    Sub(Rect),
}

impl<B: Backend + 'static> RenderBase<B> {
    pub fn attach(cx: Scope, v: Arc<Mutex<(Terminal<B>, PrefixSum2d)>>) {
        let v = Self(Arc::downgrade(&v));
        provide_context(cx, v)
    }

    pub fn access(&self) -> Arc<Mutex<(Terminal<B>, PrefixSum2d)>> {
        self.0.upgrade().unwrap()
    }

    #[inline]
    pub fn render<T: tui::widgets::Widget>(&self, widget: T, area: Rect) {
        self.batch_render([RBOp::Component(widget, area), RBOp::Add(area)])
    }

    pub fn batch_render<'a, T: tui::widgets::Widget>(
        &self,
        ops: impl IntoIterator<Item = RBOp<T>>,
    ) {
        let upgrade = self.access();
        let mut term = upgrade.try_lock().unwrap();

        for op in ops {
            match op {
                RBOp::Component(w, a) => term.0.get_frame().render_widget(w, a),
                RBOp::Add(a) => term.1.insert(a),
                RBOp::Sub(a) => term.1.insert_mul(a, -1),
            }
        }
    }

    pub fn do_frame(&self) -> Result<(), std::io::Error> {
        let term = self.access();
        let term = &mut term.try_lock().unwrap();

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
