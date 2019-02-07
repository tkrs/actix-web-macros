#[macro_use]
extern crate serde_derive;

use actix_web::{http::Method, middleware::Logger, server, App, Json, Path, Responder};

#[macro_use]
mod macros {
    macro_rules! app_new {
        ($( $path:tt => $rest:tt )*) => {
            app_new!(App::new(); ; $( $path => $rest )*)
        };

        (
            $( $middleware:expr ),*;
            $( $p:tt => $rest:tt )*
        ) => {
            app_new!(App::new(); $( $middleware )*; $( $p => $rest )*)
        };

        (
            $app:expr;
            $( $middleware:expr ),*;
            $( $path:tt => [
                $( $func:ident ($method:path, $handler:expr) )*
            ] )*
        ) => {
            $app
            $(.middleware($middleware))*
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
    Json("captured")
}

fn greet(name: Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    env_logger::init();
    let app = || {
        app_new!(
            Logger::default();
            "/event" => [
                with(Method::POST, capture_event)
            ]
            "/greet/{name}" => [
                with(Method::GET, greet)
            ]
        )
    };

    let srv = server::new(app);

    srv.bind("127.0.0.1:3000")
        .expect("Unable to start server")
        .run();
}
