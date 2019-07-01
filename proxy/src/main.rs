#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate reqwest;

use rocket::State;
use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;
use std::clone::Clone;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[get("/user/<id>")]
fn index(repo: State<Arc<RwLock<Repository>>>, id: i64) -> Json<ProxyResult> {
    let lookedup = repo.clone().read().unwrap().lookup(&id);

    //

    match lookedup {
        Some(data) => {
            let response = ProxyResult::Ok(data.clone(), true);
            Json(response)
        }
        None => {
            let result = ask(id);
            match result {
                Ok(data) => {
                    let mut lock = repo.write().unwrap();
                    let api_result: ApiResult = serde_json::from_str(data.as_str()).unwrap();

                    lock.store(&api_result);

                    let response = ProxyResult::Ok(api_result, false);
                    Json(response)
                }
                Err(_) => {
                    let response = ProxyResult::Err();
                    Json(response)
                }
            }
        }
    }
}

#[get("/")]
fn homepage() -> String {
    format!("Hello, 123!")
}

fn ask(id: i64) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:8069/user/{}", id);
    let response = reqwest::get(url.as_str())
        .expect("Couldnt make request")
        .text()
        .expect("Couldn't read response ");

    Ok(response)
}

fn main() {
    rocket::ignite()
        .manage(Arc::new(RwLock::new(Repository::new())))
        .mount("/", routes![index, homepage])
        .launch();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResult {
    id: i64,
    requested_at: String,
    responded_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProxyResult {
    Ok(ApiResult, bool),
    Err(),
}

pub struct Repository {
    data: HashMap<i64, ApiResult>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            data: HashMap::new(),
        }
    }

    pub fn store(&mut self, item: &ApiResult) -> String {
        let id = item.id;
        self.data.insert(id, item.clone());
        "OK".to_string()
    }

    pub fn lookup(&self, id: &i64) -> Option<ApiResult> {
        match self.data.get(id) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}
