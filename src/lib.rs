//! `supabase-js-rs` is a Rust bindings for Supabase JavaScript library via WebAssembly.

use wasm_bindgen::prelude::*;

/// Sign in with email and password credentials
#[wasm_bindgen(getter_with_clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

/*
#[wasm_bindgen(getter_with_clone)]
pub struct MFAChallengeParams {
    pub factor_id: String,
}
*/

/*
#[wasm_bindgen(getter_with_clone)]
pub struct MFAVerifyParams {
    pub factor_id: String,
    pub challenge_id: String,
    pub code: String,
}
*/

#[wasm_bindgen]
extern "C" {

    /// Create client
    #[wasm_bindgen(js_namespace = ["supabase"], js_name = createClient)]
    pub fn create_client(supabase_url: &str, supabase_key: &str) -> SupabaseClient;

    #[derive(Debug, Clone, PartialEq)]
    pub type SupabaseClient;

    /// Auth methods
    #[wasm_bindgen(method, getter = auth)]
    pub fn auth(this: &SupabaseClient) -> Auth;

    pub type Auth;

    /// Sign in a user
    #[wasm_bindgen(method, catch, js_name = getSession)]
    pub async fn get_session(this: &Auth) -> Result<JsValue, JsValue>;

    /// Create a new user
    #[wasm_bindgen(method, catch, js_name = signUp)]
    pub async fn sign_up(this: &Auth, credentials: Credentials) -> Result<JsValue, JsValue>;

    /// Sign in a user
    #[wasm_bindgen(method, catch, js_name = signInWithPassword)]
    pub async fn sign_in_with_password(
        this: &Auth,
        credentials: Credentials,
    ) -> Result<JsValue, JsValue>;

    /// Sign out a user
    #[wasm_bindgen(method, catch, js_name = signOut)]
    pub async fn sign_out(this: &Auth) -> Result<JsValue, JsValue>;

    /// Listen to auth events
    ///
    /// # Example
    ///
    /// ```ignore
    /// let client = supabase_js_rs::create_client("SUPABASE_URL", "SUPABASE_ANON_KEY");
    /// let auth_event_callback: Closure<dyn FnMut(JsValue, JsValue)> = Closure::new(move |event: JsValue, session: JsValue| {
    ///     
    /// });
    /// client.auth().on_auth_state_change(&auth_event_callback);
    /// auth_event_callback.forget();
    /// ```
    #[wasm_bindgen(method, js_name = onAuthStateChange)]
    pub fn on_auth_state_change(this: &Auth, callback: &Closure<dyn FnMut(JsValue, JsValue)>);

    /*
    pub type Mfa;

    #[wasm_bindgen(method, getter = mfa)]
    pub fn mfa(this: &Auth) -> Mfa;

    /// Create a challenge
    #[wasm_bindgen(method, catch, js_name = challenge)]
    pub fn challenge(this: &Mfa, params: MFAChallengeParams) -> Result<JsValue, JsValue>;

    /// Verify a challenge
    #[wasm_bindgen(method, catch, js_name = verify)]
    pub fn verify(this: &Mfa, params: MFAVerifyParams) -> Result<JsValue, JsValue>;
    */

}
