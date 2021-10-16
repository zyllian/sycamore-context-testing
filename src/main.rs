use sycamore::{
    context::*,
    prelude::{cloned, create_memo, template, Signal, StateHandle},
    reactive::create_context_scope,
};
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Debug, Route)]
enum Routes {
    #[to("/")]
    Home,
    #[not_found]
    NotFound,
    #[to("/test")]
    Test,
}

// Not using ContextProvider
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let value = Signal::new(0);
    sycamore::render(move || {
        template! {
            Router(RouterProps::new(HistoryIntegration::new(), move |route: StateHandle<Routes>| {
                let t = create_memo(cloned!((route, value) => move || {
                    tracing::info!("{:?}", route.get());
                    create_context_scope(value.clone(), cloned!((route) => move || {
                        match route.get().as_ref() {
                            Routes::Home => {
                                tracing::info!("home route");
                                use_context::<Signal<i32>>();
                                template! {
                                    a(href="/nf") { "to NotFound" }
                                }
                            }
                            Routes::NotFound => {
                                tracing::info!("not found route");
                                use_context::<Signal<i32>>();
                                template! {
                                    a(href="/test") { "to Test" }
                                }
                            }
                            Routes::Test => {
                                tracing::info!("test route");
                                use_context::<Signal<i32>>();
                                template! {
                                    a(href="/") { "to Home" }
                                }
                            }
                        }
                    }))
                }));

                template! {
                    div() {
                        (t.get().as_ref().clone())
                    }
                }
            }))
        }
    })
}

// Root ContextProvider containing Router
// fn main() {
//     console_error_panic_hook::set_once();
//     tracing_wasm::set_as_global_default();
//     let value = Signal::new(0);
//     sycamore::render(move || {
//         template! {
//             ContextProvider(ContextProviderProps {
//                 value,
//                 children: move || template! {
//                     Router(RouterProps::new(HistoryIntegration::new(), |route: StateHandle<Routes>| {
//                         let t = create_memo(move || {
//                             // on_cleanup(|| tracing::info!("this happens"));
//                             match route.get().as_ref() {
//                                 Routes::Home => {
//                                     tracing::info!("home route");
//                                     use_context::<Signal<i32>>();
//                                     template! { a(href="/nf") { "to NotFound" } }
//                                 }
//                                 Routes::NotFound => {
//                                     tracing::info!("not found route");
//                                     use_context::<Signal<i32>>();
//                                     template! { a(href="/test") { "to Test" }}
//                                 }
//                                 Routes::Test => {
//                                     tracing::info!("test route");
//                                     use_context::<Signal<i32>>();
//                                     template! { a(href="/") { "to Home" } }
//                                 }
//                             }
//                         });

//                         template! {
//                             div() {
//                                 (t.get().as_ref().clone())
//                             }
//                         }
//                     }))
//                 }
//             })
//         }
//     })
// }

// Callback inside ContextProvider inside Router
// fn main() {
//     console_error_panic_hook::set_once();
//     tracing_wasm::set_as_global_default();
//     let value = Signal::new(0);
//     sycamore::render(move || {
//         template! {
//             Router(RouterProps::new(HistoryIntegration::new(), move |route: StateHandle<Routes>| {
//                 create_effect(cloned!((route) => move || {
//                     tracing::info!("{:?} testing", route.get());
//                 }));

//                 let t = create_memo(cloned!((route, value) => move || {
//                     on_cleanup(|| tracing::info!("this happens"));
//                     tracing::info!("{:?}", route.get());
//                     value.get();
//                     let route = route.clone();
//                     let value = value.clone();
//                     template! {
//                         ContextProvider(ContextProviderProps {
//                             value: value.clone(),
//                             children: cloned!((route) => move|| {
//                                 route.get();
//                                 template! {
//                                     ((match route.get().as_ref() {
//                                         Routes::Home => {
//                                             tracing::info!("home route");
//                                             let _ = use_context::<Signal<i32>>();
//                                             template! {
//                                                 a(href="/nf") { "to NotFound" }
//                                             }
//                                         }
//                                         Routes::NotFound => {
//                                             tracing::info!("not found route");
//                                             let _ = use_context::<Signal<i32>>();
//                                             template! {
//                                                 a(href="/test") { "to Test" }
//                                             }
//                                         }
//                                         Routes::Test => {
//                                             tracing::info!("test route");
//                                             let _ = use_context::<Signal<i32>>();
//                                             template! {
//                                                 a(href="/") { "to Home" }
//                                             }
//                                         }
//                                     }))
//                                 }
//                             })
//                         })
//                     }
//                 }));
//                 template! {
//                     div() {
//                         (t.get().as_ref())
//                     }
//                 }
//             }))
//         }
//     })
// }

// Non-routing ContextProvider
// fn main() {
//     console_error_panic_hook::set_once();
//     tracing_wasm::set_as_global_default();
//     let value = Signal::new(0);
//     sycamore::render(move || {
//         template! {
//             div() {
//                 ContextProvider(ContextProviderProps {
//                     value: Signal::new(0),
//                     children: move || {
//                         let t = create_memo(move || {
//                             on_cleanup(|| panic!("this never happens"));
//                             move || {
//                                 let ctx = use_context::<Signal<i32>>();
//                                 let ctxc = ctx.clone();
//                                 let c = move |_| {
//                                     ctxc.set(ctxc.get().as_ref() + 1);
//                                 };

//                                 template! {
//                                     button(on:click=c) { (ctx.get()) }
//                                 }
//                             }
//                         });

//                         template! {
//                             ContextProvider(ContextProviderProps {
//                                 value,
//                                 children: || template! {
//                                     (t.get().as_ref()())
//                                 }
//                             })
//                         }
//                     }
//                 })
//             }
//         }
//     })
// }
