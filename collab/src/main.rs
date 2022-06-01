/**
* The first two steps for each framework are the same:
   • Start a server on a specific port
   • Provide route handler functions to an incoming HTTP request, which matches the
   path, HTTP method and parameter specified
*/

/**
* Every time a HTTP request comes in, the framework is processing it in a few steps:
   • Check the request path inside the HTTP request
   • Check the HTTP request type (GET, PUT, POST etc.)
   • Forward the request to a route handler which is responsible for the path and type
   • Before forwarding it to the end route handler, the request can be passed through a
   so-called middleware which checks things like authentication headers or adds further
   information to the request object for the end route handler
*/

/**
* error handling
* We have to do three things to be able return a custom error in our recover handler:
   • Create our own custom error-type
   • Implement the Reject trait from warp on this type
   • Return the custom error in our route-handler
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::filters::cors::CorsForbidden;
use warp::{http::Method, http::StatusCode, Filter, Rejection, Reply};

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone, Deserialize)]
struct QuestionId(String);

#[derive(Debug, Serialize, Clone, Deserialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    // create a path Filter
    // let hi = warp::path("hello").map(|| {
    //     println!("request received");
    //     format!("Hello, World!")
    // });

    // only handle get request
    // let hi = warp::get().map(|| format!("Hello"));

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
