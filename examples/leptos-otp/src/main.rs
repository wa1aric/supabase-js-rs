use leptos::{
    wasm_bindgen::{prelude::Closure, JsValue},
    web_sys::console::log_1,
    *,
};
use serde::{Deserialize, Serialize};
use supabase_js_rs::create_client;

#[derive(Serialize, Deserialize)]
struct SignInWithPasswordlessCredentials<'a> {
    pub email: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub options: Option<Options<'a>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Options<'a> {
    pub email_redirect_to: &'a str,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (client, _) = create_signal(
        cx,
        create_client(std::env!("SUPABASE_URL"), std::env!("SUPABASE_KEY")),
    );
    let (session, set_session) = create_signal(cx, wasm_bindgen::JsValue::NULL);

    let auth_callback = Closure::new(move |event: JsValue, session: JsValue| {
        log_1(&event);
    });
    client.get().auth().on_auth_state_change(&auth_callback);
    auth_callback.forget();

    let (email, set_email) = create_signal(cx, String::from(""));

    let sign_in = move |_| {
        spawn_local(async move {
            let email = email.get();
            let credentials = SignInWithPasswordlessCredentials {
                email: Some(email.as_str()),
                phone: None,
                options: Some(Options {
                    email_redirect_to: "http://127.0.0.1:8080",
                }),
            };
            let res = client
                .get()
                .auth()
                .sign_in_with_otp(serde_wasm_bindgen::to_value(&credentials).unwrap())
                .await;
            log_1(&res.unwrap());
        });
    };
    view! {
        cx,
        <input
            type="email"
            prop:value={move || email.get()}
            on:input=move |event| set_email.set(event_target_value(&event)) />
        <button on:click=sign_in>"Send magic link"</button>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <App />
        }
    });
}
