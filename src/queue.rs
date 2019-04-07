use crate::event::{Sender, Subject, Visit};

pub fn queue(sender: &crate::event::Sender, name: &str) -> Result<(), String> {
    if name != "zmq" {
        return Err(String::from("unknown queue type"));
    }
    for i in 0..100 {
        sender.send(Subject::Visit(Visit {
            url: std::borrow::Cow::from("unknown"),
            param: i,
        }));
    }
    Ok(())
}
