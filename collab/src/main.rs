use serde::Serialize;
use std::collections::HashMap;
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
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::filters::cors::CorsForbidden;
use warp::{http::Method, http::StatusCode, reject::Reject, Filter, Rejection, Reply};

struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    fn add_question(mut self, question: &Question) -> Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }

    fn init(mut self) -> Self{
        let question = Question::new(
            QuestionId("1".to_string()),
            "How?".to_string(),
            "Please help!".to_string(),
            Some(vec!["general".to_string()]),
        );
        self.add_question(&question)
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
struct QuestionId(String);

#[derive(Debug, Serialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);

    match question.id.0.is_empty() {
        true => Err(warp::reject::custom(InvalidId)),
        false => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
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

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
