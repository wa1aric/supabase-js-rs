use js_sys::{Array, Object, Reflect};
use supabase_js_rs::{create_client, SupabaseClient};
use sycamore::{prelude::*, suspense::Suspense};
use wasm_bindgen::__rt::IntoJsResult;
use web_sys::console::log_1;

#[component]
async fn Index<G: Html>(cx: Scope<'_>) -> View<G> {
    let client: &RcSignal<SupabaseClient> = use_context::<RcSignal<SupabaseClient>>(cx);
    let res = client.get().from("messages").select(Some("*")).await;
    let data = Array::from(&Object::from(
        Reflect::get(&res.unwrap(), &"data".into_js_result().unwrap()).unwrap(),
    ));
    log_1(&data);

    let vector = create_signal(cx, data.to_vec());

    view! {
        cx,
        "Index"
        ul {
            Indexed(
                iterable=vector,
                view=|cx, x| view! {
                    cx,
                    li  { (Reflect::get(&x, &"message".into_js_result().unwrap()).unwrap().as_string().unwrap()) }
                },
            )
        }
        Form
    }
}

#[component]
async fn Form<G: Html>(cx: Scope<'_>) -> View<G> {
    view! {
        cx,
        p { "Message" }
        textarea()
        p { "Name" }
        input()
        button(on:click= move |_| {

        }) { "Submit" }
    }
}

fn main() {
    sycamore::render(|cx| {
        let client: RcSignal<SupabaseClient> = create_rc_signal(create_client("", ""));
        provide_context(cx, client);

        view! {
            cx,
            h1 { "Guestbook" }
            Suspense(fallback=view! {
                cx,
                "Loading..."
            }) {
                Index {}
            }
        }
    });
}
