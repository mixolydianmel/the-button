#[macro_use]
extern crate rocket;

pub mod cors;

use cors::CORS;

use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

use rocket::{
    fs::{relative, NamedFile},
    http::Status,
    response::status::NotFound,
    serde::json::{serde_json::json, Value},
};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::http::{Client, Http},
    opt::auth::Root,
    sql::{Datetime, Thing},
    Surreal,
};

static DB: Surreal<Client> = Surreal::init();
const DATABASE_URL: &str = "localhost:8000";

#[derive(Serialize, Deserialize)]
struct ClickEvent {
    id: Thing,
    time: Datetime,
}

#[get("/<file..>")]
async fn get_file(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(relative!("../frontend/dist/")).join(file);
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_file("index.html".into()).await
}

#[get("/latest")]
async fn latest_click() -> Result<Value, NotFound<String>> {
    let mut result: Vec<ClickEvent> = match DB.select("clickevent").await {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Unable to unpack base DB query: {}", e.to_string());
            return Err(NotFound(e.to_string()));
        }
    };

    result.sort_by(|a: &ClickEvent, b: &ClickEvent| -> Ordering { b.time.cmp(&a.time) });

    match result.get(0) {
        Some(v) => Ok(json!(v.time)),
        None => {
            let estr = "No latest time in vector";
            eprintln!("{}", estr);
            Err(NotFound(estr.to_string()))
        }
    }
}

#[post("/click")]
async fn update_click() -> Status {
    let result = DB.query("CREATE clickevent SET time = time::now()").await;

    match result {
        Ok(_) => Status::Created,
        Err(_) => Status::NotModified,
    }
}

#[launch]
async fn rocket() -> _ {
    DB.connect::<Http>(DATABASE_URL).await.unwrap();
    DB.use_ns("button").use_db("clicks").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    rocket::build()
        .mount("/", routes![index, get_file])
        .mount("/api", routes![latest_click, update_click])
        .attach(CORS)
}
