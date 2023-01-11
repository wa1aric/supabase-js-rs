use leptos::*;
use serde::{Deserialize, Serialize};
use supabase_js_rs::{create_client, SignInWithOAuthCredentials};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::console::log_1;

#[derive(Serialize, Deserialize)]
pub struct Options {
    #[serde(rename = "camelCase")]
    pub redirect_to: String,
}

pub fn main() {
    mount_to_body(|cx| {
        let (session, set_session) = create_signal(cx, JsValue::NULL);
        let (client, _) = create_signal(
            cx,
            create_client(std::env!("SUPABASE_URL"), std::env!("SUPABASE_KEY")),
        );

        let callback = Closure::new(move |event: JsValue, session: JsValue| {
            log_1(&event);
            log_1(&session);
            set_session.set(session);
        });
        client.get().auth().on_auth_state_change(&callback);
        callback.forget();

        let sign_in = move |_| {
            spawn_local(async move {
                let _result = client
                    .get()
                    .auth()
                    .sign_in_with_oauth(SignInWithOAuthCredentials {
                        provider: "github".to_string(),
                        options: serde_wasm_bindgen::to_value(&Options {
                            redirect_to: "http://127.0.0.1:8080/".to_string(),
                        })
                        .unwrap(),
                    })
                    .await;
                match _result {
                    Ok(data) => log_1(&data),
                    Err(error) => log_1(&error),
                }
            });
        };

        let sign_out = move |_| {
            let _ = spawn_local(async move {
                client.get().auth().sign_out().await;
            });
        };

        view! {
            cx,
            <div class="container">
                "Leptos OAuth with Supabase"
                <p>"" {move || format!("{:#?}", session.get())} ""</p>
                <button on:click=sign_in>"Sign In"</button>
                <button on:click=sign_out>"Sign Out"</button>
            </div>
        }
    })
}
