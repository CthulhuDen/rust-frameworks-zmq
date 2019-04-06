use crate::context::ZmqWorker;

pub trait Queue {
    fn push(&self, msg: &[u8]);
}

impl Queue for ZmqWorker {
    fn push(&self, msg: &[u8]) {
        self.send(msg.to_owned().into_boxed_slice());
    }
}

pub fn queue(worker: &ZmqWorker, name: &str) -> Result<String, String> {
    let queue: &Queue = match name {
        "zmq" => worker,
        _ => return Err(String::from("unknown queue type")),
    };
    queue.push("visit".as_bytes());
    Ok(String::from(""))
}
