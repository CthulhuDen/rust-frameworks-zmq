mod internal;

use internal::mod_Event::OneOfsubject as OneOfSubject;
use std::borrow::Cow;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Visit<'a> {
    pub url: std::borrow::Cow<'a, str>,
    pub param: i32,
}

#[derive(Debug)]
pub enum Subject<'a> {
    Visit(Visit<'a>),
}

#[derive(Debug)]
pub struct Event<'a> {
    pub timestamp: i32,
    pub subject: Subject<'a>,
}

pub struct Sender {
    sock: zmq::Socket,
    buff: RefCell<Vec<u8>>,
}

impl Sender {
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {
        let sock = ctx.socket(zmq::SocketType::PUSH).unwrap();
        sock.set_sndhwm(1_000_000).unwrap();
        sock.connect(endpoint).unwrap();

        Self {
            sock: sock,
            buff: RefCell::new(vec![]),
        }
    }

    pub fn send(&self, subject0: Subject) {
        let subject = match &subject0 {
            Subject::Visit(visit) => OneOfSubject::visit(internal::Visit {
                url: Some(Cow::from(&*visit.url)),
                param: Some(visit.param),
            }),
        };
        let event = internal::Event {
            timestamp: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i32,
            ),
            subject: subject,
        };

        let mut buff = self.buff.borrow_mut();

        buff.truncate(0);
        quick_protobuf::Writer::new(&mut *buff)
            .write_message(&event)
            .expect("Could not write Event into protobuf");

        self.sock.send(&*buff, 0).unwrap();
    }
}

pub struct Receiver {
    sock: zmq::Socket,
    msg: zmq::Message,
}

impl Receiver {
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {
        let sock = ctx.socket(zmq::SocketType::PULL).unwrap();
        sock.set_sndhwm(1_000_000).unwrap();
        sock.bind(endpoint).unwrap();
        Receiver {
            sock: sock,
            msg: zmq::Message::new(),
        }
    }

    pub fn set_timeout(&self, timeout: i32) {
        self.sock.set_rcvtimeo(timeout).unwrap();
    }

    pub fn receive(&mut self) -> Option<Event> {
        if self.sock.recv(&mut self.msg, 0).is_err() {
            return None;
        }

        let event: internal::Event = quick_protobuf::BytesReader::from_bytes(&*self.msg)
            .read_message(&*self.msg)
            .expect("Could not read Event from protobuf");

        Some(Event {
            timestamp: event.timestamp.unwrap(),
            subject: match event.subject {
                OneOfSubject::None => panic!("OneOfSubject::None"),
                OneOfSubject::visit(visit) => Subject::Visit(Visit {
                    url: visit.url.expect("Did not have url"),
                    param: visit.param.expect("Did not have param"),
                }),
            },
        })
    }
}
