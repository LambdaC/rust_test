#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[head("/head")]
fn handle_head() -> &'static str {
    println!("handle_head");
    "handle_head"
}

// head请求没有对应的route会使用对应的get方法
#[get("/head")]
fn handle_head2() -> &'static str {
    println!("handle_head2");
    "handle_head2"
}

// 动态路径
// host:port/hello/adam -> hello, adam
// host:port/hello/bruce -> hello, bruce
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("hello, {}", name)
}

// host:port/hello/adam/18/true -> You're a cool 18 year old, adam!
// host:port/hello/adam/18/false -> adam, we need to talk about your coolness.
// host:port/hello/adam/aa/true -> 404
// host:port/hello/adam/18/1 -> 404
// type that implements FromParam trait can be used as params
#[get("/hello/<name>/<age>/<cool>")]
fn hello2(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, handle_head, handle_head2, hello, hello2],
    )
}

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build()
//         .mount("/", routes![index])
//         .launch()
//         .await?;

//     Ok(())
// }
