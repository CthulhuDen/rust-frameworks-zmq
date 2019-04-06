mod context;
mod queue;

pub use context::Context;
pub use context::ZmqWorker;

pub fn run(worker: &ZmqWorker, path: &[&str]) -> Result<String, String> {
    match path {
        ["queue", name] => queue::queue(worker, name),
        _ => Err(String::from("invalid request")),
    }
}
