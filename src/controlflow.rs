use std::collections::HashMap;

use leptos_reactive::{
    create_effect, create_memo, create_rw_signal, ReadSignal, RwSignal, Scope, ScopeDisposer,
};

pub fn show(
    cx: Scope,
    when: impl Fn() -> bool + 'static,
    valid: impl Fn(Scope) + Clone + 'static,
    fallback: impl Fn(Scope) + Clone + 'static,
) {
    let when = create_memo(cx, move |_| when());

    create_effect(cx.clone(), move |last: Option<ScopeDisposer>| {
        if let Some(last) = last {
            last.dispose();
        }
        if when() {
            cx.child_scope(valid.clone())
        } else {
            cx.child_scope(fallback.clone())
        }
    })
}

struct EachContainer<K, Com: 'static> {
    current_iteration: bool,

    map: HashMap<K, (bool, RwSignal<Com>, ScopeDisposer)>,
}

impl<K: std::hash::Hash + Eq, Com: Clone + 'static> EachContainer<K, Com> {
    fn new() -> Self {
        Self {
            current_iteration: false,
            map: HashMap::new(),
        }
    }

    fn update<T, It: Iterator<Item = T>>(
        mut self,
        cx: Scope,
        it: It,
        key: impl Fn(&T, usize) -> K,
        sizer: impl for<'a> Fn(Option<&'a Com>, T, usize) -> Option<Com>,

        render: impl Fn(Scope, ReadSignal<Com>),
    ) -> Self {
        let cvg = !self.current_iteration;

        let mut last_com = None;

        let mut it = it.enumerate();

        loop {
            let next = it.next();

            let key = next.as_ref().map(|(idx, v)| key(v, idx.to_owned()));
            let next_com =
                next.map(|(idx, v)| sizer(last_com.as_ref().map(|(_, c)| c), v, idx.to_owned()));

            if let Some((key, com)) = last_com {
                if let Some((cv, v, _)) = self.map.get_mut(&key) {
                    *cv = cvg;
                    v.set(com.clone());
                } else {
                    let com = create_rw_signal(cx, com.clone());
                    let com_read = com.read_only();
                    self.map
                        .insert(key, (cvg, com, cx.child_scope(|cx| render(cx, com_read))));
                }
            }

            last_com = Some(match (key, next_com) {
                (Some(k), Some(Some(com))) => (k, com),
                _ => break,
            })
        }

        self.map.retain(|_, v| v.0 == cvg);

        self
    }
}

impl<K: std::hash::Hash + Eq, Com: Clone + 'static> Default for EachContainer<K, Com> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn each<K: std::hash::Hash + Eq + 'static, Com: Clone + 'static, T, It: Iterator<Item = T>>(
    cx: Scope,
    it: impl Fn() -> It + 'static,
    key: impl Fn(&T, usize) -> K + 'static + Clone,
    sizer: impl for<'a> Fn(Option<&'a Com>, T, usize) -> Option<Com> + Clone + 'static,

    render: impl Fn(Scope, ReadSignal<Com>) + 'static + Clone,
) {
    create_effect(cx, move |last: Option<EachContainer<K, Com>>| {
        let v = last.unwrap_or_else(Default::default);

        v.update(cx, it(), key.clone(), sizer.clone(), render.clone())
    })
}

#[cfg(test)]
mod tests {
    use leptos_reactive::use_context;
    use tui::{layout::Rect, text::Span, widgets::Paragraph};

    use super::*;
    use crate::bootstrapper::{assert_rb, shared_ctx::RenderBaseAuto, test_bootstrap};

    #[test]
    fn each_basics() {
        test_bootstrap(
            |cx| {
                let v = create_rw_signal(cx, [0, 1, 2]);

                let rb: RenderBaseAuto = use_context(cx).unwrap();

                each(
                    cx,
                    move || v().into_iter(),
                    |_, i| i,
                    |_, v, idx| Some((v, Rect::new(idx.try_into().unwrap(), 0, 9, 1))),
                    |cx, v| {
                        let rb: RenderBaseAuto = use_context(cx).unwrap();
                        create_effect(cx, move |_| {
                            let v = v();
                            let p = Paragraph::new(v.0.to_string());

                            rb.render(p, v.1);
                        })
                    },
                );

                create_effect(cx, move |_| {
                    assert_rb(&rb, "each-basics");
                })
            },
            10,
            10,
            true,
        )
        .unwrap();
    }
}
