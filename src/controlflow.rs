use leptos_reactive::{create_effect, Scope, ScopeDisposer};

pub fn show(
    cx: Scope,
    when: impl Fn() -> bool + 'static,
    valid: impl Fn(Scope) + Clone + 'static,
    fallback: impl Fn(Scope) + Clone + 'static,
) {
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

// pub struct EachContainer<K> {
//     current_iteration: bool,

//     map: HashMap<K, (bool, RwSignal<Rect>, ScopeDisposer)>,
// }

// impl<K> EachContainer<K> {
//     fn new() -> Self {
//         Self {
//             current_iteration: false,
//             map: HashMap::new(),
//         }
//     }

//     fn update(mut self) -> Self {
//         // let (cv, mut keymap) = v.unwrap_or((false, HashMap::new()));
//         // let cvg = !cv;

//         // let vals = it();

//         // let mut last_rect = None;
//         // for (idx, val) in vals.enumerate() {
//         //     let k = key(&val, idx);
//         //     let rect = area_aggregator(last_rect, &val, idx);

//         //     if let Some((cv, v, _)) = keymap.get_mut(&k) {
//         //         *cv = cvg;
//         //         v.set(rect);
//         //     } else {
//         //         let rect = create_rw_signal(cx, rect);
//         //         let rect_read = rect.read_only();
//         //         keymap.insert(
//         //             k,
//         //             (
//         //                 cvg,
//         //                 rect,
//         //                 cx.child_scope(|cx| render(cx, rect_read, idx, &val)),
//         //             ),
//         //         );
//         //     }
//         //     last_rect = Some(rect);
//         // }

//         // keymap.retain(|_, v| v.0 == cvg);

//         // self
//     }
// }

// impl<K> PartialEq<Self> for EachContainer<K> {
//     fn eq(&self, other: &Self) -> bool {
//         self.current_iteration == other.current_iteration
//     }
// }

// pub fn each<T: 'static, K: Eq + Hash + 'static>(
//     cx: Scope,
//     it: &MaybeSignal<&impl IntoIterator<Item = T>>,
//     render: impl FnMut(Scope, ReadSignal<Rect>, usize, &T),
//     area_aggregator: impl FnMut(Option<Rect>, &T, usize) -> Rect,
//     key: impl FnMut(&T, usize) -> K,
//     // TODO:
//     // fallback
// ) -> Memo<RefCell<Option<EachContainer<K>>>> {
//     create_memo(cx, |last| {
//         let current = std::mem::take(last.borrow_mut());

//         todo!()
//     })
// }
