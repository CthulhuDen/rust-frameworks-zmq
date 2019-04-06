#[macro_use]
extern crate tower_web;

use warp::{self, path, Filter};

fn run_console() {
    let ctx = web::Context::new();
    let args = std::env::args().skip(2).collect::<Vec<String>>();
    let args = args.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
    let worker = web::ZmqWorker::new(ctx);
    match web::run(&worker, &args) {
        Ok(string) => println!("{}", string),
        Err(string) => eprintln!("{}", string),
    }
}

fn zmq_server() {
    let ctx = zmq::Context::new();
    let sock = ctx.socket(zmq::SocketType::PULL).unwrap();
    sock.set_rcvtimeo(1000).unwrap();
    sock.bind("ipc://@queue").unwrap();

    let mut counter = 0;
    loop {
        match sock.recv_bytes(0) {
            Ok(_) => counter += 1,
            Err(_) => {
                if counter > 0 {
                    println!("Processed {} entries", counter);
                    counter = 0;
                }
            }
        }
    }
}

struct App {
    worker: web::ZmqWorker,
}

tower_web::impl_web! {
    impl App {
        #[get("/queue/:name")]
        fn index(&self, name: String) -> Result<String, String> {
            let args: Vec<String> = vec![String::from("queue"), name];
            let args: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
            web::run(&self.worker, &args)
        }
    }
}

fn run_tower() {
    let ctx = web::Context::new();
    let worker = web::ZmqWorker::new(ctx);
    let addr = "127.0.0.1:4321".parse().unwrap();
    println!("Listening on http://{}", addr);

    tower_web::ServiceBuilder::new()
        .resource(App { worker: worker })
        .run(&addr)
        .unwrap();
}

fn run_warp() {
    let ctx = web::Context::new();
    let worker = std::sync::Arc::new(web::ZmqWorker::new(ctx));

    let index = warp::path!("queue" / String).map(move |name| {
        let args: Vec<String> = vec![String::from("queue"), name];
        let args: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
        match web::run(&worker, &args) {
            Ok(reply) => reply,
            Err(reply) => reply,
        }
    });

    warp::serve(index).run(([127, 0, 0, 1], 4321));
}

fn actix_index(
    req: &actix_web::HttpRequest<std::sync::Arc<web::ZmqWorker>>,
) -> impl actix_web::Responder {
    let args: Vec<&str> = req.path().split('/').skip(1).collect();
    match web::run(&req.state(), &args) {
        Ok(reply) => reply,
        Err(reply) => reply,
    }
}

fn run_actix() {
    let ctx = web::Context::new();
    let worker = std::sync::Arc::new(web::ZmqWorker::new(ctx));
    actix_web::server::new(move || {
        actix_web::App::with_state(worker.clone())
            .resource("/queue/{name}", |r| r.f(actix_index))
            .finish()
    })
    .bind("127.0.0.1:4321")
    .unwrap()
    .run();
}

fn main() {
    match std::env::args().nth(1).as_ref().map(AsRef::as_ref) {
        Some("tower") => run_tower(),
        Some("warp") => run_warp(),
        Some("actix") => run_actix(),
        Some("zmq-server") => zmq_server(),
        Some("console") => run_console(),
        _ => panic!("Dont understand!"),
    }
}
