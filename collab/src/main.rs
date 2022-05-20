/**
 * The first two steps for each framework are the same:
    • Start a server on a specific port
    • Provide route handler functions to an incoming HTTP request, which matches the
    path, HTTP method and parameter specified
 */

/**
 * Every time a HTTP request comes in, the framework is 
processing it in a few steps: 
    • Check the request path inside the HTTP request 
    • Check the HTTP request type (GET, PUT, POST etc.) 
    • Forward the request to a route handler which is responsible for the path and type 
    • Before forwarding it to the end route handler, the request can be passed through a 
    so-called middleware which checks things like authentication headers or adds further 
    information to the request object for the end route handler 
 */

use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);
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

use warp::Filter;

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);

    // create a path Filter
    let hi = warp::path("hello").map(|| {
        println!("request received");
        format!("Hello, World!")
    });

    warp::serve(hi).run(([127, 0, 0, 1], 3030)).await;
}
