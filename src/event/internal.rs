use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::borrow::Cow;
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Event<'a> {
    pub timestamp: Option<i32>,
    pub subject: mod_Event::OneOfsubject<'a>,
}

impl<'a> MessageRead<'a> for Event<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.timestamp = Some(r.read_sfixed32(bytes)?),
                Ok(26) => {
                    msg.subject = mod_Event::OneOfsubject::visit(r.read_message::<Visit>(bytes)?)
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Event<'a> {
    fn get_size(&self) -> usize {
        0 + self.timestamp.as_ref().map_or(0, |_| 1 + 4)
            + match self.subject {
                mod_Event::OneOfsubject::visit(ref m) => 1 + sizeof_len((m).get_size()),
                mod_Event::OneOfsubject::None => 0,
            }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.timestamp {
            w.write_with_tag(13, |w| w.write_sfixed32(*s))?;
        }
        match self.subject {
            mod_Event::OneOfsubject::visit(ref m) => {
                w.write_with_tag(26, |w| w.write_message(m))?
            }
            mod_Event::OneOfsubject::None => {}
        }
        Ok(())
    }
}

pub mod mod_Event {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfsubject<'a> {
        visit(Visit<'a>),
        None,
    }

    impl<'a> Default for OneOfsubject<'a> {
        fn default() -> Self {
            OneOfsubject::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Visit<'a> {
    pub url: Option<Cow<'a, str>>,
    pub param: Option<i32>,
}

impl<'a> MessageRead<'a> for Visit<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.param = Some(r.read_int32(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Visit<'a> {
    fn get_size(&self) -> usize {
        0 + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .param
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.url {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.param {
            w.write_with_tag(16, |w| w.write_int32(*s))?;
        }
        Ok(())
    }
}
