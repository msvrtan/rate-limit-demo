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
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::{thread, time};

type MyCache = Arc<RwLock<Cache>>;
type MyProcessing = Arc<RwLock<Processing>>;
#[get("/user/<id>")]
fn index(cache: State<MyCache>, grab: State<MyProcessing>, id: i64) -> Json<ProxyResult> {
    try_cached_response_or_fetch_it(cache, grab, id)
}

fn try_cached_response_or_fetch_it(
    cache: State<MyCache>,
    grab: State<MyProcessing>,
    id: i64,
) -> Json<ProxyResult> {
    let lookedup = cache.clone().read().unwrap().lookup(&id);

    //

    match lookedup {
        Some(data) => Json(ProxyResult::Cached(data.clone())),
        None => {
            let being_processed = grab.clone().read().unwrap().in_processing(&id);

            println!("In processing:{}", being_processed);

            match being_processed {
                true => {
                    println!("Sleeping:{}", &id);
                    thread::sleep(time::Duration::from_millis(50));

                    try_cached_response_or_fetch_it(cache, grab, id)
                }
                false => fetch_it(cache, grab, id),
            }
        }
    }
}
fn fetch_it(cache: State<MyCache>, grab: State<MyProcessing>, id: i64) -> Json<ProxyResult> {
    println!("Fetching data:{}", &id);

    println!("Locking grab:{}", &id);
    grab.write().unwrap().lock(&id);
    println!("Locked grab:{}", &id);

    let result = ask(id);
    match result {
        Ok(data) => {
            let api_result: ApiResult = serde_json::from_str(data.as_str()).unwrap();

            cache.write().unwrap().store(&api_result);

            println!("Unlocking grab:{}", &id);
            grab.write().unwrap().unlock(&id);
            println!("Unlocked grab:{}", &id);

            Json(ProxyResult::Ok(api_result))
        }
        Err(_) => Json(ProxyResult::Err()),
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
        .manage(Arc::new(RwLock::new(Cache::new())))
        .manage(Arc::new(RwLock::new(Processing::new())))
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
    Cached(ApiResult),
    Ok(ApiResult),
    Err(),
}

pub struct Cache {
    data: HashMap<i64, ApiResult>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn store(&mut self, item: &ApiResult) -> bool {
        let id = item.id;
        self.data.insert(id, item.clone());
        true
    }

    pub fn lookup(&self, id: &i64) -> Option<ApiResult> {
        match self.data.get(id) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}

pub struct Processing {
    data: HashSet<i64>,
}

impl Processing {
    pub fn new() -> Processing {
        Processing {
            data: HashSet::new(),
        }
    }

    pub fn lock(&mut self, id: &i64) -> bool {
        self.data.insert(id.clone())
    }

    pub fn unlock(&mut self, id: &i64) -> bool {
        self.data.remove(id)
    }

    pub fn in_processing(&self, id: &i64) -> bool {
        match self.data.get(id) {
            Some(_) => true,
            None => false,
        }
    }
}
