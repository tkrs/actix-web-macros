#[macro_use]
extern crate serde_derive;

use actix_web::{server, App, middleware::Logger, Responder, http::Method, Json, Path};

#[macro_use]
mod macros {
    macro_rules! app_new {
        (
            $( $path:tt => [
                $( $func:ident ($method:path, $handler:expr) )*
            ] )*
        ) => {
            App::new().middleware(Logger::default())
            $(.resource($path, |r| {
                $(r.method($method).$func($handler);)*
            }))*
        };
    }
}

#[derive(Deserialize, Debug)]
struct Event {
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

fn capture_event(evt: Json<Event>) -> impl Responder {
    println!("{:?}", evt);
    "captured"
}

fn greet(name: Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    env_logger::init();
    let app = || app_new!(
        "/event" => [
            with(Method::POST, capture_event)
        ]
        "/greet/{name}" => [
            with(Method::GET, greet)
        ]
    );

    let srv = server::new(app);

    srv.bind("127.0.0.1:3000").expect("Unable to start server").run();
}
