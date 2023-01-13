//! `supabase-js-rs` is a Rust bindings for Supabase JavaScript library via WebAssembly.
//!

use wasm_bindgen::prelude::*;

/// Sign in with email and password credentials
#[wasm_bindgen(getter_with_clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct SignInWithOAuthCredentials {
    pub provider: String,
    pub options: JsValue,
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

    #[derive(Debug, Clone, PartialEq)]
    pub type SupabaseClient;

    /// # Create client
    ///
    #[wasm_bindgen(js_namespace = ["supabase"], js_name = createClient)]
    pub fn create_client(supabase_url: &str, supabase_key: &str) -> SupabaseClient;

    #[wasm_bindgen(method, js_name = from)]
    pub fn from(this: &SupabaseClient, table: &str) -> Database;

    pub type Database;

    #[wasm_bindgen(method, catch, js_name = select)]
    pub async fn select(this: &Database, columns: Option<&str>) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = select)]
    pub fn select_(this: &Database, columns: Option<&str>) -> Database;

    /// # Retrieve the query as a CSV string
    /// 
    /// Return data as a string in CSV format.
    /// 
    /// ```ignore
    /// let csv = client.get().from("countries").select_(Some("*")).csv().await.unwrap();
    /// ```
    /// 
    #[wasm_bindgen(method, catch, js_name = csv)]
    pub async fn csv(this: &Database) -> Result<JsValue, JsValue>;

    /// # Column is equal to a value
    ///
    /// Match only rows where column is equal to value.
    ///
    #[wasm_bindgen(method, catch, js_name = eq)]
    pub async fn eq(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = eq)]
    pub fn eq_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column is not equal to a value
    ///
    /// Match only rows where column is not equal to value.
    ///
    #[wasm_bindgen(method, catch, js_name = neq)]
    pub async fn neq(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = neq)]
    pub fn neq_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column is greater than a value
    ///
    /// Match only rows where column is greater than value.
    ///
    #[wasm_bindgen(method, catch, js_name = gt)]
    pub async fn gt(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = gt)]
    pub fn gt_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column is greater than or equal to a value
    ///
    /// Match only rows where column is greater than or equal to value.
    ///
    #[wasm_bindgen(method, catch, js_name = gte)]
    pub async fn gte(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = gte)]
    pub fn gte_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column is less than a value
    ///
    /// Match only rows where column is less than value.
    ///
    #[wasm_bindgen(method, catch, js_name = lt)]
    pub async fn lt(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = lt)]
    pub fn lt_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column is less than or equal to a value
    ///
    /// Match only rows where column is less than or equal to value.
    ///
    #[wasm_bindgen(method, catch, js_name = lte)]
    pub async fn lte(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = lte)]
    pub fn lte_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column matches a pattern
    ///
    /// Match only rows where column matches pattern case-sensitively.
    ///
    #[wasm_bindgen(method, catch, js_name = like)]
    pub async fn like(this: &Database, column: &str, pattern: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = like)]
    pub fn like_(this: &Database, column: &str, pattern: &str) -> Database;

    /// # Column matches a case-insensitive pattern
    ///
    /// Match only rows where column matches pattern case-insensitively.
    ///
    /// ```ignore
    /// client.from("countries").select(None).ilike(&"name", &"%alba%").await;
    /// ```
    ///
    #[wasm_bindgen(method, catch, js_name = ilike)]
    pub async fn ilike(this: &Database, column: &str, pattern: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = ilike)]
    pub fn ilike_(this: &Database, column: &str, pattern: &str) -> Database;

    /// # Column is a value
    ///
    /// Match only rows where column IS value.
    ///
    /// ```ignore
    /// // check for nullness
    /// client.from("countries").select(None).is("name", JsValue::NULL);
    /// // or check for true of false
    /// client.from("countries").select(None).is("name", JsValue::TRUE);
    /// ```
    ///
    #[wasm_bindgen(method, catch, js_name = is)]
    pub async fn is(this: &Database, column: &str, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = is)]
    pub fn is_(this: &Database, column: &str, value: &JsValue) -> Database;

    /// # Column is in an array
    ///
    /// Match only rows where column is included in the values array.
    ///
    #[wasm_bindgen(method, catch, js_name = in)]
    pub async fn r#in(
        this: &Database,
        column: &str,
        values: Vec<JsValue>,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = in)]
    pub fn r#in_(this: &Database, column: &str, values: Vec<JsValue>) -> Database;

    /// # Column contains every element in a value
    ///
    /// Only relevant for jsonb, array, and range columns. Match only rows where column contains every element appearing in value.
    ///
    #[wasm_bindgen(method, catch, js_name = contains)]
    pub async fn contains(
        this: &Database,
        column: &str,
        value: JsValue,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = contains)]
    pub fn contains_(this: &Database, column: &str, value: JsValue) -> Database;

    /// # Contained by value
    ///
    /// Only relevant for jsonb, array, and range columns. Match only rows where every element appearing in column is contained by value.
    ///
    #[wasm_bindgen(method, catch, js_name = containedBy)]
    pub async fn contained_by(
        this: &Database,
        column: &str,
        value: JsValue,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = containedBy)]
    pub fn contained_by_(this: &Database, column: &str, value: JsValue) -> Database;

    /// # Greater than a range
    ///
    /// Only relevant for range columns. Match only rows where every element in column is greater than any element in range.
    ///
    #[wasm_bindgen(method, catch, js_name = rangeGt)]
    pub async fn range_gt(this: &Database, column: &str, range: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = rangeGt)]
    pub fn range_gt_(this: &Database, column: &str, range: &str) -> Database;

    /// # Greater than or equal to a range
    ///
    /// Only relevant for range columns. Match only rows where every element in column is either contained in range or greater than any element in range.
    ///
    #[wasm_bindgen(method, catch, js_name = rangeGte)]
    pub async fn range_gte(this: &Database, column: &str, range: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = rangeGte)]
    pub fn range_gte_(this: &Database, column: &str, range: &str) -> Database;

    /// # Less than a range
    ///
    /// Only relevant for range columns. Match only rows where every element in column is less than any element in range.
    ///
    #[wasm_bindgen(method, catch, js_name = rangeLt)]
    pub async fn range_lt(this: &Database, column: &str, range: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = rangeLt)]
    pub fn range_lt_(this: &Database, column: &str, range: &str) -> Database;

    /// # Less than or equal to a range
    ///
    /// Only relevant for range columns. Match only rows where every element in column is either contained in range or less than any element in range.
    ///
    #[wasm_bindgen(method, catch, js_name = rangeLte)]
    pub async fn range_lte(this: &Database, column: &str, range: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = rangeLte)]
    pub fn range_lte_(this: &Database, column: &str, range: &str) -> Database;

    /// # Mutually exclusive to a range
    ///
    /// Only relevant for range columns. Match only rows where column is mutually exclusive to range and there can be no element between the two ranges.
    ///
    #[wasm_bindgen(method, catch, js_name = rangeAdjacent)]
    pub async fn range_adjacent(
        this: &Database,
        column: &str,
        range: &str,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = rangeAdjacent)]
    pub fn range_adjacent_(this: &Database, column: &str, range: &str) -> Database;

    /// # With a common element
    ///
    /// Only relevant for array and range columns. Match only rows where column and value have an element in common.
    ///
    #[wasm_bindgen(method, catch, js_name = overlaps)]
    pub async fn overlaps(
        this: &Database,
        column: &str,
        value: JsValue,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = overlaps)]
    pub fn overlaps_(this: &Database, column: &str, value: JsValue) -> Database;

    /// # Match a string
    ///
    /// Only relevant for text and tsvector columns. Match only rows where column matches the query string in query.
    ///
    #[wasm_bindgen(method, catch, js_name = textSearch)]
    pub async fn text_search(
        this: &Database,
        column: &str,
        query: &str,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = textSearch)]
    pub fn text_search_(this: &Database, column: &str, query: &str, options: JsValue) -> Database;

    /// # Update data
    ///
    /// Perform an UPDATE on the table or view.
    ///
    #[wasm_bindgen(method, catch, js_name = update)]
    pub async fn update(this: &Database, values: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = update)]
    pub fn update_(this: &Database, values: &JsValue) -> Database;

    /// # Upsert data
    ///
    /// Perform an UPSERT on the table or view.
    ///
    #[wasm_bindgen(method, js_name = upsert)]
    pub fn upsert(this: &Database, values: JsValue) -> Database;

    /// # Delete data
    ///
    /// Should always be combined with filters
    ///
    /// ```ignore
    /// let client = supabase_js_rs::create_client("https://xyzcompany.supabase.co", "public-anon-key");
    /// let res: Result<JsValue, JsValue> = client.from("countries").delete().eq("id", 1.into_js_result().unwrap()).await;
    /// ```
    ///
    #[wasm_bindgen(method, js_name = delete)]
    pub fn delete(this: &Database) -> Database;

    /// # Insert data
    ///
    /// Perform an INSERT into the table or view.
    ///
    #[wasm_bindgen(method, catch, js_name = insert)]
    pub async fn insert(this: &Database, values: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = insert)]
    pub fn insert_(this: &Database, values: JsValue) -> Database;

    /// Auth methods
    #[wasm_bindgen(method, getter = auth)]
    pub fn auth(this: &SupabaseClient) -> Auth;

    pub type Auth;

    /// # Create a new user
    ///
    #[wasm_bindgen(method, catch, js_name = signUp)]
    pub async fn sign_up(this: &Auth, credentials: Credentials) -> Result<JsValue, JsValue>;

    /// # Sign in a user
    ///
    #[wasm_bindgen(method, catch, js_name = signInWithPassword)]
    pub async fn sign_in_with_password(
        this: &Auth,
        credentials: Credentials,
    ) -> Result<JsValue, JsValue>;

    /// # Sign in a user through OAuth
    ///
    /// Log in an existing user via a third-party provider.
    ///
    #[wasm_bindgen(method, catch, js_name = signInWithOAuth)]
    pub async fn sign_in_with_oauth(
        this: &Auth,
        credentials: SignInWithOAuthCredentials,
    ) -> Result<JsValue, JsValue>;

    /// # Sign out a user
    ///
    #[wasm_bindgen(method, catch, js_name = signOut)]
    pub async fn sign_out(this: &Auth) -> Result<JsValue, JsValue>;

    /// # Retrieve a session
    ///
    /// Returns the session, refreshing it if necessary.
    #[wasm_bindgen(method, catch, js_name = getSession)]
    pub async fn get_session(this: &Auth) -> Result<JsValue, JsValue>;

    /// # Retrieve a new session
    ///
    /// Returns a new session, regardless of expiry status.
    #[wasm_bindgen(method, catch, js_name = refreshSession)]
    pub async fn refresh_session(this: &Auth) -> Result<JsValue, JsValue>;

    /// # Retrieve a user
    ///
    /// Takes in an optional access token jwt or get the jwt from the current session.
    #[wasm_bindgen(method, catch, js_name = getUser)]
    pub async fn get_user(this: &Auth, jwt: Option<&str>) -> Result<JsValue, JsValue>;

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

    #[wasm_bindgen(method, js_name = channel)]
    pub fn channel(this: &SupabaseClient, name: &str) -> RealtimeChannel;

    /// # Unsubscribe from all channels
    ///
    #[wasm_bindgen(method, js_name = removeAllChannels)]
    pub fn remove_all_channels(this: &SupabaseClient);

    /// # Retrieve all channels
    ///
    #[wasm_bindgen(method, js_name = getChannels)]
    pub fn get_channels(this: &SupabaseClient) -> JsValue;

    pub type RealtimeChannel;

    /// # Subscribe to database changes
    ///
    #[wasm_bindgen(method, js_name = on)]
    pub fn on(
        this: &RealtimeChannel,
        r#type: &str,
        filter: &JsValue,
        callback: &Closure<dyn Fn(JsValue)>,
    ) -> RealtimeChannel;

    #[wasm_bindgen(method, js_name = subscribe)]
    pub fn subscribe(
        this: &RealtimeChannel,
        callback: Option<&Closure<dyn FnMut(JsValue, JsValue)>>,
    ) -> RealtimeChannel;

}
