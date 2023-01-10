use serde::{Deserialize, Serialize};
use supabase_js_rs::create_client;
use sycamore::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::console::{log_1, log_2};

#[derive(Serialize, Deserialize)]
pub struct RealtimePostgresChangesFilter {
    pub event: String,
    pub schema: String,
}

fn main() {
    let supabase_url = std::env!("SUPABASE_URL");
    let supabase_key = std::env!("SUPABASE_KEY");

    sycamore::render(|cx| {
        let client = create_rc_signal(create_client(supabase_url, supabase_key));

        let payload = Closure::new(move |event: JsValue| {
            log_1(&event);
        });

        let subscribition_callback: Closure<dyn FnMut(JsValue, JsValue)> =
            Closure::new(move |status: JsValue, error: JsValue| {
                log_2(&status, &error);
            });

        let filter = RealtimePostgresChangesFilter {
            event: "*".to_string(),
            schema: "*".to_string(),
        };

        client
            .get()
            .channel("*")
            .on(
                "postgres_changes",
                &serde_wasm_bindgen::to_value(&filter).unwrap(),
                &payload,
            )
            .subscribe(Some(&subscribition_callback));

        payload.forget();
        subscribition_callback.forget();

        view! {
            cx,
            div {"Real time chat"}
        }
    });
}
