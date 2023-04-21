use crate::bootstrapper::shared_ctx::RenderBase;
use leptos_reactive::*;
use tui::{
    backend::Backend,
    layout::Alignment,
    layout::Rect,
    style::Style,
    text::Spans,
    widgets::{self, BorderType, Borders, Clear},
};

pub trait Render<B: Backend> {
    fn render(self, cx: Scope, area: Rect, base: &RenderBase<B>);
}
pub trait SelfSized<B: Backend>: Render<B> {
    fn size(&self, max: Rect);
}

#[derive(Default, Clone, Copy)]
pub struct Clearing;

impl<B: Backend> Render<B> for Clearing {
    fn render(self, cx: Scope, area: Rect, base: &RenderBase<B>) {
        base.render(Clear, area)
    }
}

#[derive(Clone)]
pub struct Block<Child: 'static = Clearing, Title: Clone + 'static = ()> {
    title: MaybeSignal<Title>,
    title_alignment: MaybeSignal<Alignment>,

    borders: MaybeSignal<Borders>,
    border_style: MaybeSignal<Style>,
    border_type: MaybeSignal<BorderType>,
    style: MaybeSignal<Style>,

    child: MaybeSignal<Child>,
}

impl Default for Block<Clearing, ()> {
    fn default() -> Self {
        Self {
            title: Default::default(),
            title_alignment: Alignment::Left.into(),
            borders: Borders::ALL.into(),
            border_style: Default::default(),
            border_type: BorderType::Plain.into(),
            style: Default::default(),
            child: Default::default(),
        }
    }
}

impl<B: Backend, Child: Render<B>, Title: Clone + for<'a> Into<Spans<'a>>> Render<B>
    for Block<Child, Title>
{
    fn render(self, cx: Scope, area: Rect, base: &RenderBase<B>) {
        let block = MaybeSignal::derive(cx, move || {
            widgets::Block::default()
                .title_alignment(self.title_alignment.get())
                .borders(self.borders.get())
                .border_style(self.border_style.get())
                .border_type(self.border_type.get())
                .style(self.style.get())
                .title(self.title.get())
        });

        let inner = MaybeSignal::derive(cx, move || block.with(|v| v.inner(area)));

        create_effect(cx, move |_| base.render(block(), area));
    }
}

impl<Child: 'static, Title: 'static + Clone> Block<Child, Title> {
    pub fn title<NewTitle: Clone + 'static + for<'a> Into<Spans<'a>>>(
        self,
        title: MaybeSignal<NewTitle>,
    ) -> Block<Child, NewTitle> {
        Block {
            title: title.into(),
            title_alignment: self.title_alignment,
            borders: self.borders,
            border_style: self.border_style,
            border_type: self.border_type,
            style: self.style,
            child: self.child,
        }
    }

    pub fn title_alignment(mut self, alignment: impl Into<MaybeSignal<Alignment>>) -> Self {
        self.title_alignment = alignment.into();
        self
    }

    pub fn border_style(mut self, style: impl Into<MaybeSignal<Style>>) -> Self {
        self.border_style = style.into();
        self
    }

    pub fn style(mut self, style: impl Into<MaybeSignal<Style>>) -> Self {
        self.style = style.into();
        self
    }

    pub fn borders(mut self, flag: impl Into<MaybeSignal<Borders>>) -> Self {
        self.borders = flag.into();
        self
    }

    pub fn border_type(mut self, border_type: impl Into<MaybeSignal<BorderType>>) -> Self {
        self.border_type = border_type.into();
        self
    }
}
