use actix_web::{
    web::{resource, Data, Path},
    App, HttpServer,
};
use web::event::Sender;

fn zmq_server() {
    let ctx = zmq::Context::new();
    let mut receiver = web::event::Receiver::new(&ctx, "ipc://@queue");
    let mut counter: u64 = 0;
    loop {
        match receiver.receive() {
            Some(_ev) => {
                if counter == 0 {
                    receiver.set_timeout(1_000);
                }
                counter += 1;
            }
            None => {
                println!("Processed {} entries", counter);
                counter = 0;
                receiver.set_timeout(-1);
            }
        }
    }
}

fn run_actix() -> std::io::Result<()> {
    let ctx = zmq::Context::new();
    HttpServer::new(move || {
        App::new()
            .data(Sender::new(&ctx, "ipc://@queue"))
            .service(
                resource("/{path:.*}").to(|path: Path<String>, sender: Data<Sender>| {
                    let args: Vec<&str> = path.split('/').collect();
                    web::run(&sender, &args).map_err(|reason| -> std::io::Error { panic!(reason) })
                }),
            )
    })
    .bind("127.0.0.1:4321")?
    .run()
}

fn main() {
    match std::env::args().nth(1).as_ref().map(AsRef::as_ref) {
        Some("actix") => run_actix().unwrap(),
        Some("zmq-server") => zmq_server(),
        _ => panic!("Dont understand!"),
    }
}
