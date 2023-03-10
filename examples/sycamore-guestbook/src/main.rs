use js_sys::{Array, Object, Reflect};
use serde::{Deserialize, Serialize};
use supabase_js_rs::{create_client, SupabaseClient};
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense, web::html::tr};
use wasm_bindgen::{JsValue, __rt::IntoJsResult};
use web_sys::console::log_1;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrderOptions {
    ascending: bool,
}

#[component]
async fn Index<G: Html>(cx: Scope<'_>) -> View<G> {
    let client: &RcSignal<SupabaseClient> = use_context::<RcSignal<SupabaseClient>>(cx);
    let res: Result<JsValue, JsValue> = client
        .get()
        .from("messages")
        .select_(Some("*"))
        .order_(
            "id",
            serde_wasm_bindgen::to_value(&OrderOptions { ascending: false }).unwrap(),
        )
        .limit(1)
        .await;

    let data: Array = Array::from(&Object::from(
        Reflect::get(&res.unwrap(), &"data".into_js_result().unwrap()).unwrap(),
    ));

    let messages: &Signal<Vec<JsValue>> = create_signal(cx, data.to_vec());

    let message: &Signal<String> = create_signal(cx, String::new());
    let name: &Signal<String> = create_signal(cx, String::new());

    view! {
        cx,
        ul {
            Indexed(
                iterable=messages,
                view=|cx, message| view! {
                    cx,
                    li  { (Reflect::get(&message, &"message".into_js_result().unwrap()).unwrap().as_string().unwrap()) }
                },
            )
        }

        p { "Message" }
        textarea(bind:value=message)

        p { "Name" }
        input(bind:value=name)

        button(on:click=move |_| {
            spawn_local_scoped(cx, async move {
                let post = Post {
                    message: message.get().to_string(),
                    name: name.get().to_string(),
                };
                let res = client.get().from("messages").insert_(serde_wasm_bindgen::to_value(&post).unwrap()).select(Some("*")).await;
                let inserted = Array::from(&Object::from(Reflect::get(&res.unwrap(), &"data".into_js_result().unwrap()).unwrap()));
                messages.modify().push(inserted.get(0));
                message.set("".to_string());
                name.set("".to_string());
            });
        }) { "Submit" }
    }
}

#[derive(Serialize, Deserialize)]
struct Post {
    message: String,
    name: String,
}

fn main() {
    let url = std::env!("SUPABASE_URL");
    let key = std::env!("SUPABASE_KEY");

    sycamore::render(|cx| {
        let client: RcSignal<SupabaseClient> = create_rc_signal(create_client(url, key));
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
