use std::collections::HashMap;

use leptos_reactive::*;

pub fn show<Valid: Fn(Scope) + Clone + 'static, Fallback: Fn(Scope) + Clone + 'static>(
    cx: Scope,
    when: impl Fn() -> bool + 'static,
    valid: Valid,
    fallback: Fallback,
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

pub fn each<
    K: std::hash::Hash + Eq + 'static,
    Com: Clone + 'static,
    T: std::fmt::Debug,
    It: Iterator<Item = T> + Clone,
>(
    cx: Scope,
    it: impl Fn() -> It + 'static,
    key: impl Fn(&T, usize) -> K + 'static + Clone,
    sizer: impl for<'a> Fn(Option<&'a Com>, T, usize) -> Option<Com> + Clone + 'static,

    render: impl Fn(Scope, ReadSignal<Com>) + 'static + Clone,
) {
    create_effect(cx, move |last: Option<EachContainer<K, Com>>| {
        let it = it();

        let v = last.unwrap_or_else(EachContainer::new);

        v.update(cx, it, key.clone(), sizer.clone(), render.clone())
    })
}

#[cfg(test)]
mod tests {
    use leptos_reactive::use_context;
    use tui::{layout::Rect, text::Span, widgets::Paragraph};

    use super::*;
    use crate::bootstrapper::{
        assert_rb,
        shared_ctx::{Quit, RenderBaseAuto, RenderCounter},
        test_bootstrap,
    };

    #[test]
    fn each_basics() {
        test_bootstrap(
            |cx| {
                let v = create_rw_signal(cx, vec![0, 1, 2, 3, 4]);
                let cycle: RenderCounter = use_context(cx).unwrap();

                let rb: RenderBaseAuto = use_context(cx).unwrap();
                let quit: Quit = use_context(cx).unwrap();

                each(
                    cx,
                    move || v.with(|v| v.clone().into_iter()),
                    |_, i| i,
                    |_, v, idx| Some((v, Rect::new(0, idx.try_into().unwrap(), 9, 1))),
                    |cx, v| {
                        let rb: RenderBaseAuto = use_context(cx).unwrap();
                        create_effect(cx, move |_| {
                            let v = v();
                            let p = Paragraph::new(v.0.to_string());

                            rb.render(p, v.1);
                        })
                    },
                );

                create_effect(cx, move |_| match cycle.0() {
                    0 => {
                        assert_rb(&rb, "each-basics-0");
                        v.update(|v| v.push(5));
                    }
                    1 => {
                        assert_rb(&rb, "each-basics-1");
                        v.update(|v| v.push(6));
                    }
                    2 => {
                        assert_rb(&rb, "each-basics-2");
                        v.update(|v| v.push(7));
                    }
                    _ => (),
                })
            },
            10,
            10,
            Some(3),
        )
        .unwrap();
    }
}
