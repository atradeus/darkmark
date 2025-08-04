// use leptos::ev;
// use leptos::prelude::*;
// 
// #[component]
// pub fn credentials_form(
//     title: Option<&'static str>,
//     action_label: &'static str,
//     action: Action<(String, String), ()>,
//     error: Signal<Option<String>>,
//     disabled: Signal<bool>,
// ) -> impl IntoView {
//     let (password, set_password) = signal(String::new());
//     let (email, set_email) = signal(String::new());
// 
//     let dispatch_action = move || action.dispatch((&email.get(), &password.get()));
// 
//     let button_is_disabled = Signal::derive(move || {
//         disabled.get() || password.get().is_empty() || email.get().is_empty()
//     });
// 
//     view! {
//         <form on:submit=|ev| ev.prevent_default()>
//             {move || {
//                 title.map(|t| view! {<p>{t}</p>})
//             }}
//             {move || {
//                 error
//                     .get()
//                     .map(|err| {
//                         view! { <p class="text-center text-red-600 pb-2">{err}</p> }
//                     })
//             }}
//             <div class="mb-4 text-lg">
//                 <input
//                     class="rounded-2xl border-none bg-blue-400 bg-opacity-50 px-6 py-2 text-center text-inherit placeholder-slate-200 shadow-lg outline-none backdrop-blur-md"
//                     type="email"
//                     required
//                     placeholder="Email"
//                     prop:disabled=move || disabled.get()
//                     on:keyup=move |ev: ev::KeyboardEvent| {
//                         let val = event_target_value(&ev);
//                         set_email.update(|v| *v = val);
//                     }
//                     on:change=move |ev| {
//                         let val = event_target_value(&ev);
//                         set_email.update(|v| *v = val);
//                     }
//                 />
//             </div>
//             <div class="mb-4 text-lg">
//                 <input
//                     class="rounded-2xl border-none bg-blue-400 bg-opacity-50 px-6 py-2 text-center text-inherit placeholder-slate-200 shadow-lg outline-none backdrop-blur-md"
//                     type="password"
//                     required
//                     placeholder="Password"
//                     prop:disabled=move || disabled.get()
//                     on:keyup=move |ev: ev::KeyboardEvent| {
//                         match &*ev.key() {
//                             "Enter" => {
//                                 dispatch_action();
//                             }
//                             _ => {
//                                 let val = event_target_value(&ev);
//                                 set_password.update(|p| *p = val);
//                             }
//                         }
//                     }
//                     on:change=move |ev| {
//                         let val = event_target_value(&ev);
//                         set_password.update(|p| *p = val);
//                     }
//                 />
//             </div>
//             <div class="mt-8 flex justify-center text-lg text-black">
//                 <button
//                     class="rounded-2xl bg-yellow-400 bg-opacity-50 px-10 py-2 text-white shadow-xl backdrop-blur-md transition-colors duration-300 hover:bg-yellow-600"
//                     prop:disabled=move || button_is_disabled.get()
//                     on:click=move |_| {dispatch_action();}
//                 >
//                     {action_label}
//                 </button>
//             </div>
//         </form>
//     }
// }
