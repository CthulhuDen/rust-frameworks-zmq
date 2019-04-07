pub mod event;
mod queue;

pub fn run(sender: &event::Sender, path: &[&str]) -> Result<(), String> {
    match path {
        ["queue", name] => queue::queue(sender, name),
        _ => Err(String::from("invalid request")),
    }
}
