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

use chrono::{DateTime, NaiveDateTime, Utc};

use itertools::Itertools;

static DB: Surreal<Client> = Surreal::init();
const DATABASE_URL: &str = "localhost:8000";

#[derive(Serialize, Deserialize, Debug)]
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

#[get("/total_clicks")]
async fn total_clicks() -> Result<Value, NotFound<String>> {
    let result = DB
        .query("SELECT * FROM count((SELECT * FROM clickevent))")
        .await;

    match result {
        Ok(mut n) => {
            let count: Option<u64> = n.take(0).unwrap_or(None);
            match count {
                Some(c) => Ok(json!(c)),
                None => Err(NotFound(String::from("No such value"))),
            }
        }
        Err(_) => Err(NotFound(String::from("Error querying DB"))),
    }
}

#[get("/high_score")]
async fn high_score() -> Result<Value, NotFound<String>> {
    let mut result: Vec<ClickEvent> = match DB.select("clickevent").await {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Unable to unpack base DB query: {}", e.to_string());
            return Err(NotFound(e.to_string()));
        }
    };

    result.sort_by(|a: &ClickEvent, b: &ClickEvent| -> Ordering { b.time.cmp(&a.time) });

    match result
        .iter()
        .map(|c| {
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_micros(c.time.timestamp_micros()).unwrap(),
                Utc,
            )
        })
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| a - b)
        .max()
    {
        Some(t) => Ok(json!(t.num_seconds())),
        None => return Err(NotFound("Unable to find max window".to_string())),
    }
}

#[get("/prev_time")]
async fn prev_time() -> Result<Value, NotFound<String>> {
    let result = DB
        .query("SELECT * FROM clickevent ORDER BY time DESC LIMIT 2")
        .await;

    println!("Result: {:?}", result);

    let times: Vec<Datetime> = match result {
        Ok(mut n) => {
            n.take(0).unwrap()
        },
        Err(_) => {
            return Err(NotFound(String::from("Error querying DB")));
        },
    };

    println!("Times: {:?}", times);

    let times: Vec<DateTime<Utc>> = times
        .iter()
        .map(|c| {
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_micros(c.timestamp_micros()).unwrap(),
                Utc,
            )
        })
        .collect();

    println!("Times: {:?}", times);

    Ok(json!(0))
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
        .mount("/api/data", routes![total_clicks, high_score, prev_time])
        .attach(CORS)
}
