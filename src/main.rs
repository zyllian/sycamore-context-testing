use sycamore::{
    context::*,
    prelude::{create_memo, on_cleanup, template, Signal, StateHandle},
};
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Route)]
enum Routes {
    #[to("/")]
    Home,
    #[not_found]
    NotFound,
}

// Root ContextProvider containing Router
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let value = Signal::new(0);
    sycamore::render(move || {
        template! {
            ContextProvider(ContextProviderProps {
                value,
                children: move || template! {
                    Router(RouterProps::new(HistoryIntegration::new(), |route: StateHandle<Routes>| {
                        let t = create_memo(move || {
                            on_cleanup(|| tracing::info!("this happens"));
                            match route.get().as_ref() {
                                Routes::Home => {
                                    use_context::<Signal<i32>>();
                                    template! { a(href="/nf") { "to NotFound" } }
                                }
                                Routes::NotFound => {
                                    use_context::<Signal<i32>>();
                                    template! { a(href="/") { "to Home" }}
                                }
                            }
                        });

                        template! {
                            div() {
                                (t.get().as_ref().clone())
                            }
                        }
                    }))
                }
            })
        }
    })
}

// Callback inside ContextProvider inside Router
// fn main() {
//     console_error_panic_hook::set_once();
//     tracing_wasm::set_as_global_default();
//     let value = Signal::new(0);
//     sycamore::render(move || {
//         template! {
//             Router(RouterProps::new(HistoryIntegration::new(), |route: StateHandle<Routes>| {
//                 let t = create_memo(move || {
//                     on_cleanup(|| tracing::info!("this happens"));
//                     match route.get().as_ref() {
//                         Routes::Home => || {
//                             let _ = use_context::<Signal<i32>>();
//                             template! {
//                                 a(href="/nf") { "to NotFound" }
//                             }
//                         },
//                         Routes::NotFound => || {
//                             let _ = use_context::<Signal<i32>>();
//                             template! {
//                                 a(href="/") { "to Home" }
//                             }
//                         }
//                     }
//                 });
//                 template! {
//                     div() {
//                         ContextProvider(ContextProviderProps {
//                             value,
//                             children: || template! {
//                                 (t.get().as_ref()())
//                             }
//                         })
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
