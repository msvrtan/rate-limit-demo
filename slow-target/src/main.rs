#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;

use chrono::{DateTime, SecondsFormat, Utc};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::{thread, time};

#[get("/user/<id>")]
fn index(id: i64) -> Json<ApiResult> {
    let requested_at = current_time();

    let sleep_for = time::Duration::from_secs(10);
    thread::sleep(sleep_for);

    let responded_at = current_time();

    let result = ApiResult {
        id: id,
        requested_at: requested_at,
        responded_at: responded_at,
    };

    Json(result)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

fn current_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[derive(Debug, Serialize)]
struct ApiResult {
    id: i64,
    requested_at: String,
    responded_at: String,
}
