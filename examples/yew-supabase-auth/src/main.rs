use js_sys::{JsString, Object, Reflect};
use std::rc::Rc;
use supabase_js_rs::{self, Credentials, SupabaseClient};
use wasm_bindgen::{self, prelude::Closure, JsCast, JsValue, __rt::IntoJsResult};
use web_sys::{EventTarget, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub client: SupabaseClient,
    pub data: JsValue,
}

impl Default for Session {
    fn default() -> Self {
        let client =
            supabase_js_rs::create_client("https://xyzcompany.supabase.co", "public-anon-key");
        Self {
            client,
            data: JsValue::NULL,
        }
    }
}

impl Session {
    pub async fn sign_in(&self, email: &str, password: &str) -> Result<JsValue, JsValue> {
        let res = self
            .client
            .auth()
            .sign_in_with_password(Credentials {
                email: email.to_string(),
                password: password.to_string(),
            })
            .await;
        Ok(res.unwrap())
    }
    pub async fn sign_out(&self) -> JsValue {
        self.client.auth().sign_out().await.unwrap()
    }
}

impl Reducible for Session {
    type Action = JsValue;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Session {
            client: self.client.clone(),
            data: action,
        }
        .into()
    }
}

pub type SessionContext = UseReducerHandle<Session>;

#[derive(Debug, Properties, PartialEq)]
pub struct SessionProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn SessionProvider(props: &SessionProviderProps) -> Html {
    let context: UseReducerHandle<Session> = use_reducer(|| Session::default());
    let context_clone = context.clone();
    let callback: Closure<dyn FnMut(JsValue, JsValue)> = Closure::new(move |_event, session| {
        context_clone.dispatch(session);
    });
    context.client.auth().on_auth_state_change(&callback);
    callback.forget();
    html! {
        <ContextProvider<SessionContext> {context}>
            {props.children.clone()}
        </ContextProvider<SessionContext>>
    }
}

#[function_component]
fn Index() -> Html {
    let loading_state = use_state(|| false);
    let is_loading = loading_state.clone();

    let error = use_state(|| JsValue::NULL);
    let error_value = error.clone();

    let session: UseReducerHandle<Session> = use_context::<SessionContext>().unwrap();
    let data: JsValue = session.data.to_owned();

    let email_input_ref = use_node_ref();
    let email_input_handle = use_state(|| String::default());
    let email_input_value = (*email_input_handle).clone();
    let email = email_input_value.clone();

    let onchange = {
        let email_input_ref = email_input_ref.clone();
        Callback::from(move |_| {
            let input = email_input_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                email_input_handle.set(input.value());
            }
        })
    };

    let password_input_value_handle = use_state(|| String::default());
    let password_input_value = (*password_input_value_handle).clone();
    let password = password_input_value.clone();

    let on_password_input_change: Callback<InputEvent> = {
        let password_input_value_handle = password_input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input: Option<HtmlInputElement> =
                target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                password_input_value_handle.set(input.value());
            }
        })
    };

    let use_session = session.clone();
    let sign_out = {
        move |_| {
            let use_session = use_session.clone();
            spawn_local(async move {
                use_session.sign_out().await;
            });
        }
    };

    let use_session = session.clone();

    let sign_in = {
        move |_| {
            let use_session = use_session.clone();
            let email = email.clone();
            let password = password.clone();
            let error_value = error_value.clone();
            loading_state.set(true);
            let loading_clone = loading_state.clone();
            spawn_local(async move {
                let sign_in_result = use_session.sign_in(&email, &password).await;
                let message = Reflect::get(
                    &Reflect::get(
                        &Object::from(sign_in_result.unwrap()),
                        &"error".into_js_result().unwrap(),
                    )
                    .unwrap(),
                    &"message".into_js_result().unwrap(),
                );
                error_value.set(message.unwrap());
                loading_clone.clone().set(false);
            });
        }
    };

    let loading = is_loading.clone();

    html! {
        <div>
            <h1>{"Yew with Supabase"}</h1>

            if data == JsValue::NULL {

                <input
                    type="email"
                    placeholder="Email"
                    ref={email_input_ref}
                    oninput={onchange}
                    disabled={*loading.clone()} />

                <input
                    type="password"
                    placeholder="Password"
                    oninput={on_password_input_change}
                    value={password_input_value.clone()}
                    disabled={*loading.clone()} />

                <button
                    onclick={sign_in}
                    disabled={*loading.clone()}>{"Sign In"}</button>

                if *error != JsValue::NULL {
                    <p>{error.as_string()}</p>
                }

            } else {
                <h1>{"Logged in"}</h1>
                <code>{format!("{:#?}", data)}</code>
                <button
                    onclick={sign_out}>{"Sign Out"}</button>
            }
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <SessionProvider>
            <Index />
        </SessionProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
