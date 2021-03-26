
use serde::{Deserialize, Serialize};

pub mod heartbeat;
pub use heartbeat::Heartbeat;
pub mod logon;
pub use logon::Logon;
pub mod logout;
pub use logout::Logout;
pub mod reject;
pub use reject::Reject;
pub mod resend_request;
pub use resend_request::ResendRequest;
pub mod sequence_reset;
pub use sequence_reset::SequenceReset;
pub mod test_request;
pub use test_request::TestRequest;

use super::header::{Header, HasHeader};
/*
use super::{header::{Header, MsgType}, Trailer};
use crate::entities::ApplVerID;

#[derive(Serialize, Debug)]
pub struct Message {
    #[serde(flatten)]
    pub header: Header,
    #[serde(flatten)]
    pub body: Body,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str as Deserialize>::deserialize(deserializer)?;
        println!("{}", s);
        Ok(Message {
            header: crate::from_str(s).map_err(serde::de::Error::custom)?,
            body: crate::from_str(s).map_err(serde::de::Error::custom)?,
            trailer: crate::from_str(s).map_err(serde::de::Error::custom)?,
        })
    }
}

impl Message {
    /// approach due to https://github.com/serde-rs/serde/issues/1183
    pub fn from_str(s: &str) -> Result<Message, serde::de::value::Error> {
        let header: Header = crate::from_str(s)?;
        let body = Body::from_str(&header, s)?;
        Ok(Message {
            header,
            body,
            trailer: crate::from_str(s)?,
        })
    }
}
*/
#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
    #[serde(rename = "0")]
    Heartbeat(Heartbeat),
    #[serde(rename = "A")]
    Logon(Logon),
    #[serde(rename = "5")]
    Logout(Logout),
    #[serde(rename = "3")]
    Reject(Reject),
    #[serde(rename = "2")]
    ResendRequest(ResendRequest),
    #[serde(rename = "4")]
    SequenceReset(SequenceReset),
    #[serde(rename = "1")]
    TestRequest(TestRequest),
    // FIX50SP2(crate::entities::fix50sp2::messages::Message),
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Message::Heartbeat(m) => m.serialize(serializer),
            Message::Logon(m) => m.serialize(serializer),
            Message::Logout(m) => m.serialize(serializer),
            Message::Reject(m) => m.serialize(serializer),
            Message::ResendRequest(m) => m.serialize(serializer),
            Message::SequenceReset(m) => m.serialize(serializer),
            Message::TestRequest(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Message {
    fn get_header(&self) -> &Header {
        match self {
            Message::Heartbeat(m) => m.get_header(),
            Message::Logon(m) => m.get_header(),
            Message::Logout(m) => m.get_header(),
            Message::Reject(m) => m.get_header(),
            Message::ResendRequest(m) => m.get_header(),
            Message::SequenceReset(m) => m.get_header(),
            Message::TestRequest(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Message::Heartbeat(m) => m.get_header_mut(),
            Message::Logon(m) => m.get_header_mut(),
            Message::Logout(m) => m.get_header_mut(),
            Message::Reject(m) => m.get_header_mut(),
            Message::ResendRequest(m) => m.get_header_mut(),
            Message::SequenceReset(m) => m.get_header_mut(),
            Message::TestRequest(m) => m.get_header_mut(),
        }
    }
}
/*
impl Body {
    /// approach due to https://github.com/serde-rs/serde/issues/1183
    pub fn from_str(h: &Header, s: &str) -> Result<Body, serde::de::value::Error> {
        Ok(match h.msg_type {
            MsgType::Heartbeat => Body::Heartbeat(crate::from_str(s)?),
            MsgType::Logon => Body::Logon(crate::from_str(s)?),
            MsgType::Logout => Body::Logout(crate::from_str(s)?),
            MsgType::Reject => Body::Reject(crate::from_str(s)?),
            MsgType::ResendRequest => Body::ResendRequest(crate::from_str(s)?),
            MsgType::SequenceReset => Body::SequenceReset(crate::from_str(s)?),
            MsgType::TestRequest => Body::TestRequest(crate::from_str(s)?),
            _ => {
                match h.appl_ver_id {
                    Some(ApplVerID::FIX50SP2) => Body::FIX50SP2(crate::entities::fix50sp2::messages::Body::from_str(&h.msg_type, s)?),
                    _ => unimplemented!(),
                }
            },
        })
    }
}
*/
#[cfg(test)]
mod test {
    use super::Message;
    use crate::entities::fixt11::header::MsgType;

    #[test]
    fn logon() {
        let msg = "8=FIXT.1.1\u{1}9=78\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}789=1\u{1}1137=0\u{1}10=037\u{1}";
        let mut obj = dbg!(crate::from_str::<Message>(msg)).unwrap();
        match obj {
            Message::Logon(ref mut logon) => {
                logon.header.msg_type = Some(MsgType::Logon);
            },
            _ => panic!("Message isn't of type Logon (A)"),
        }
        assert_eq!(crate::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
