use js_sys::{JsString, Object, JSON};
use supabase_js_rs::*;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::console::log_1;

async fn sign_in(auth: Auth, credentials: Credentials) -> Result<JsValue, JsValue> {
    let res = auth.sign_in_with_password(credentials).await;
    match &res {
        Ok(data) => log_1(&data),
        Err(err) => log_1(&err),
    };
    res
}

#[component]
async fn Auth<G: Html>(cx: Scope<'_>) -> View<G> {
    let state = use_context::<RcSignal<Session>>(cx);
    let client = use_context::<RcSignal<SupabaseClient>>(cx);

    let email = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());

    view! {
        cx,
        div(class="form") {

            h2 {"Signin or create account"}

            input(type="email", placeholder="Email", bind:value=email) {}
            input(type="password", placeholder="Password", bind:value=password) {}

            button(class="sign-in-button", disabled=email.get().is_empty() || password.get().is_empty(), on:click=move |_| {

                let email = email.get().to_string();
                let password = password.get().to_string();

                spawn_local_scoped(cx, async move {
                    let auth = client.get().auth();
                    let cred = Credentials {
                            email,
                            password
                        };
                    sign_in(auth, cred).await;
                });
            }) { "Sign In" }

            button(disabled=email.get().is_empty() || password.get().is_empty(), on:click=move |_| {
                let email = email.get().to_string();
                let password = password.get().to_string();

                spawn_local_scoped(cx, async move {
                    let res: Result<JsValue, JsValue> = client.get().auth().sign_in_with_password(Credentials {
                        email,
                        password
                    }).await;
                });
            }) { "Create account" }

            // p(class="error") {"Auth error:" (format!(" {:#?}", state.get().error.get()))}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Loading(bool);

impl Loading {
    fn is_loading(self) -> bool {
        self.0
    }
}

pub struct Session {
    pub access_key: RcSignal<String>,
    pub data: RcSignal<JsValue>,
    pub error: RcSignal<JsValue>,
}

impl Session {
    pub fn set_access_key(&self, access_key: &str) {
        self.access_key.set(access_key.to_string());
    }
    pub fn get_access_key(&self) -> String {
        self.access_key.get().to_string()
    }
    pub fn set_data(&self, data: JsValue) {
        self.data.set(data);
    }
    pub fn stringify_data(&self) -> JsString {
        JSON::stringify(&self.data.get()).unwrap()
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let client = supabase_js_rs::create_client(
        &std::env!("SUPABASE_URL").to_string(),
        &std::env!("SUPABASE_KEY").to_string(),
    );

    let loading = create_rc_signal(Loading(false));
    provide_context(cx, loading);
    let use_loading_context = use_context::<RcSignal<Loading>>(cx);
    let loading = use_loading_context.clone();

    let create_session = create_rc_signal(Session {
        access_key: create_rc_signal(String::from("")),
        data: create_rc_signal(JsValue::NULL),
        error: create_rc_signal(JsValue::NULL),
    });
    provide_context(cx, create_session);
    let use_create_session_context = use_context::<RcSignal<Session>>(cx);
    let session_clone = use_create_session_context.clone();

    let auth_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |_: JsValue, session: JsValue| {
            session_clone.get().set_data(session);
            loading.set(Loading(false));
        });
    client.auth().on_auth_state_change(&auth_callback);
    auth_callback.forget();

    let client = create_rc_signal(client);
    provide_context(cx, client);

    view! {
        cx,
        div(class="app") {
            h1 { span(class="sycamore") {"Sycamore"} " with " span(class="supabase") {"Supabase"} " Auth" }
            (
                if use_context::<RcSignal<Loading>>(cx).get().is_loading() {
                    view! {
                        cx,
                        h2 {"Loading..."}
                    }
                }
                else {
                    let session_context = use_context::<RcSignal<Session>>(cx);
                    if Object::is(&session_context.get().stringify_data(), &JSON::stringify(&JsValue::NULL).unwrap()) {
                        Auth(cx)
                    }
                    else {
                        view! {
                            cx,
                            code {(format!(" {}", session_context.get().stringify_data()))}
                            button(on:click=move |_| {
                                let loading = use_context::<RcSignal<Loading>>(cx);
                                let loading_clone = loading.clone();

                                let client = use_context::<RcSignal<SupabaseClient>>(cx);
                                let client_clone = client.clone();

                                spawn_local_scoped(cx, async move {
                                    use_loading_context.set(Loading(true));
                                    let _ = client.get().auth().sign_out().await;
                                    use_loading_context.set(Loading(false));
                                });
                            }) { "Sign Out" }
                        }
                    }
                }
            )
        }
    }
}

fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
