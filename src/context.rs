pub struct Context {
    pub zmq: zmq::Context,
    pub sock: zmq::Socket,
}

impl Context {
    pub fn new() -> Context {
        let ctx = zmq::Context::new();
        let sock = ctx.socket(zmq::SocketType::PUSH).unwrap();
        sock.connect("ipc://@queue").unwrap();
        Context {
            zmq: ctx,
            sock: sock,
        }
    }
}

enum WorkerMsg {
    MsgSend(Box<[u8]>),
    MsgStop,
}

pub struct ZmqWorker {
    sender: std::sync::mpsc::SyncSender<WorkerMsg>,
}

impl ZmqWorker {
    pub fn new(ctx: Context) -> ZmqWorker {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);
        std::thread::spawn(move || loop {
            match receiver.recv().unwrap() {
                WorkerMsg::MsgStop => break,
                WorkerMsg::MsgSend(msg) => {
                    let msg: &[u8] = &msg;
                    ctx.sock.send(&msg, 0).unwrap();
                }
            }
        });
        ZmqWorker { sender: sender }
    }
    pub fn send(&self, msg: Box<[u8]>) {
        self.sender.send(WorkerMsg::MsgSend(msg)).unwrap();
    }
}

impl Drop for ZmqWorker {
    fn drop(&mut self) {
        self.sender.send(WorkerMsg::MsgStop).unwrap();
    }
}
