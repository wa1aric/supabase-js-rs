use js_sys::{Array, Object, Reflect};
use serde::{Deserialize, Serialize};
use supabase_js_rs::{create_client, SupabaseClient};
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense};
use wasm_bindgen::{JsValue, __rt::IntoJsResult};
use web_sys::console::log_1;

#[component]
async fn Index<G: Html>(cx: Scope<'_>) -> View<G> {
    let client: &RcSignal<SupabaseClient> = use_context::<RcSignal<SupabaseClient>>(cx);
    let res: Result<JsValue, JsValue> = client.get().from("messages").select(Some("*")).await;
    let data: Array = Array::from(&Object::from(
        Reflect::get(&res.unwrap(), &"data".into_js_result().unwrap()).unwrap(),
    ));

    let messages: &Signal<Vec<JsValue>> = create_signal(cx, data.to_vec());

    view! {
        cx,
        "Index"
        ul {
            Indexed(
                iterable=messages,
                view=|cx, x| view! {
                    cx,
                    li  { (Reflect::get(&x, &"message".into_js_result().unwrap()).unwrap().as_string().unwrap()) }
                },
            )
        }
        Form
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub message: String,
    pub name: String,
}

#[component]
async fn Form<G: Html>(cx: Scope<'_>) -> View<G> {
    let client: &RcSignal<SupabaseClient> = use_context::<RcSignal<SupabaseClient>>(cx);

    let message: &Signal<String> = create_signal(cx, String::new());
    let name: &Signal<String> = create_signal(cx, String::new());

    view! {
        cx,

        p { "Message" }
        textarea(bind:value=message)

        p { "Name" }
        input(bind:value=name)

        button(on:click=move |_| {
            spawn_local_scoped(cx, async move {
                let message = message.get().to_string();
                let name = name.get().to_string();
                let post = Post {
                    message,
                    name
                };
                let res = client.get().from("messages").insert_(serde_wasm_bindgen::to_value(&post).unwrap()).select(Some("*")).await;
                log_1(&res.unwrap());

            });
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
