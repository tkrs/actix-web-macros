#[macro_use]
extern crate serde_derive;

use actix_web::{
    http::Method, middleware::DefaultHeaders, middleware::Logger, server, App, Json, Path,
    Responder, State,
};

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
            app_new!(App::new(); $( $middleware ),*; $( $p => $rest )*)
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

    macro_rules! app_with_state {
        (
            $state:expr;
            $( $path:tt => $rest:tt )*
        ) => {
            app_with_state!($state; ; $($path => $rest)*);
        };

        (
            $state:expr;
            $( $middleware:expr ),*;
            $( $path:tt => $rest:tt )*
        ) => {
            app_new!(App::with_state($state);$($middleware),*;$($path => $rest)*)
        };
    }
}

/// Application state
struct MyApp {
    msg: &'static str,
}

#[derive(Deserialize, Debug)]
struct Event {
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

fn capture_event(state: State<MyApp>, evt: Json<Event>) -> impl Responder {
    println!("{}: {:?}", state.msg, evt);
    Json("captured")
}

fn greet(state: State<MyApp>, name: Path<String>) -> impl Responder {
    println!("{}: {}", state.msg, name);
    format!("Hello {}!", name)
}

fn main() {
    env_logger::init();

    let app = || {
        app_with_state!(
            MyApp { msg: "Welcome" };

            Logger::default(), DefaultHeaders::new().header("X-Version", "0.1");

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
