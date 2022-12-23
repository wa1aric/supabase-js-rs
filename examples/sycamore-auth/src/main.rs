use js_sys::{JsString, Object, Reflect, JSON};
use supabase_js_rs::*;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen::{prelude::Closure, JsValue, __rt::IntoJsResult};

#[component]
async fn Auth<G: Html>(cx: Scope<'_>) -> View<G> {
    let loading = use_context::<RcSignal<Loading>>(cx);

    let state = use_context::<RcSignal<Session>>(cx);
    let state_clone = state.clone();

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
                    let res: Result<JsValue, JsValue> = client.get().auth().sign_in_with_password(Credentials {
                        email,
                        password,
                    }).await;
                    match res {
                        Ok(res) => {
                            let response = Object::from(res);
                            let error_oject = Object::from(Reflect::get(&response, &"error".into_js_result().unwrap()).unwrap());
                            let message = Reflect::get(&error_oject, &"message".into_js_result().unwrap());
                            match message {
                                Ok(msg) => state.get().error.set(msg),
                                Err(_) => (),
                            }
                        },
                        _ => (),
                    };
                    loading.set(Loading(false));
                });

            }) { "Sign In" }

            button(disabled=email.get().is_empty() || password.get().is_empty(), on:click=move |_| {

                let email = email.get().to_string();
                let password = password.get().to_string();

                spawn_local_scoped(cx, async move {
                    loading.set(Loading(true));
                    let res: Result<JsValue, JsValue> = client.get().auth().sign_up(Credentials {
                        email,
                        password
                    }).await;
                    match res {
                        Ok(res) => {
                            let response = Object::from(res);
                            let error_oject = Object::from(Reflect::get(&response, &"error".into_js_result().unwrap()).unwrap());
                            let message = Reflect::get(&error_oject, &"message".into_js_result().unwrap());
                            match message {
                                Ok(msg) => state.get().error.set(msg),
                                Err(_) => (),
                            }
                        },
                        _ => (),
                    };
                    loading.set(Loading(false));
                });

            }) { "Create account" }

            p(class="error") {"Auth error:" (format!(" {:#?}", state.get().error.get()))}
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
    let client =
        supabase_js_rs::create_client(std::env!("SUPABASE_URL"), std::env!("SUPABASE__KEY"));

    let loading = create_rc_signal(Loading(false));
    provide_context(cx, loading);
    let loading = use_context::<RcSignal<Loading>>(cx);

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
            // log_2(&event, &session);
            session_clone.get().set_data(session);
            loading.set(Loading(false));
        });
    client.auth().on_auth_state_change(&auth_callback);
    auth_callback.forget();

    let client: RcSignal<SupabaseClient> = create_rc_signal(client);
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

                                spawn_local_scoped(cx, async move {
                                    loading.set(Loading(true));
                                    let _ = client.get().auth().sign_out().await;
                                    loading.set(Loading(false));
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
