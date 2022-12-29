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

    #[wasm_bindgen(method, js_name = from)]
    pub fn from(this: &SupabaseClient, table: &str) -> Database;

    pub type Database;

    #[wasm_bindgen(method, catch, js_name = select)]
    pub async fn select(this: &Database, columns: Option<&str>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = select)]
    pub fn select_chain(this: &Database, columns: Option<&str>) -> Database;

    /// Column is equal to a value
    /// Match only rows where column is equal to value.
    #[wasm_bindgen(method, catch, js_name = eq)]
    pub async fn eq(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = eq)]
    pub fn eq_chain(this: &Database, column: &str, value: &JsValue) -> Database;

    /// Column is not equal to a value
    /// Match only rows where column is not equal to value.
    #[wasm_bindgen(method, catch, js_name = neq)]
    pub async fn neq(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = neq)]
    pub fn neq_chain(this: &Database, column: &str, value: &JsValue) -> Database;

    #[wasm_bindgen(method, catch, js_name = update)]
    pub async fn update(this: &Database, values: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = insert)]
    pub async fn insert(this: &Database, values: JsValue) -> Result<JsValue, JsValue>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn creates_new_client() {
        let client = create_client("supabase_url", "supabase_key");
    }
    #[test]
    fn get_auth() {
        let clinet = create_client("supabase_url", "supabase_key");
        clinet.auth();
    }
}
