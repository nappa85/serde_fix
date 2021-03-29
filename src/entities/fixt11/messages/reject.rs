
use serde::{Serialize, Deserialize};

use crate::entities::{fixt11::{header::{Header, HasHeader, MsgType}, Trailer}, version::FixVersion};

/// MsgType = 3
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Reject {
    #[serde(flatten)]
    pub header: Header,
    /// MsgSeqNum of rejected message
    #[serde(rename = "45")]
    pub ref_seq_num: u64,
    /// The tag number of the FIX field being referenced.
    #[serde(rename = "371")]
    #[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
    pub ref_tag_id: Option<i32>,
    /// The MsgType (35) of the FIX message being referenced.
    #[serde(rename = "372")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_msg_type: Option<MsgType>,
    /// Recommended when rejecting an application message that does not explicitly provide ApplVerID ( 1128) on the message being rejected. In this case the value from the DefaultApplVerID(1137) or the default value specified in the NoMsgTypes repeating group on the logon message should be provided.
    #[serde(rename = "1130")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_appl_ver_id: Option<crate::entities::ApplVerID>,
    /// Recommended when rejecting an application message that does not explicitly provide ApplExtID(1156) on the rejected message. In this case the value from the DefaultApplExtID(1407) or the default value specified in the NoMsgTypes repeating group on the logon message should be provided.
    #[serde(rename = "1406")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_appl_ext_id: Option<String>,
    /// Recommended when rejecting an application message that does not explicitly provide CstmApplVerID(1129) on the message being rejected. In this case the value from the DefaultCstmApplVerID(1408) or the default value specified in the NoMsgTypes repeating group on the logon message should be provided
    #[serde(rename = "1131")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_cstm_appl_ver_id: Option<String>,
    /// Code to identify reason for a session-level Reject message.
    #[serde(rename = "373")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_reject_reason: Option<SessionRejectReason>,
    /// Where possible, message to explain reason for rejection
    #[serde(rename = "58")]
    pub text: Option<String>,
    /// Must be set if EncodedText (355) field is specified and must immediately precede it.
    #[serde(rename = "354")]
    /// Encoded (non-ASCII characters) representation of the Text (58) field in the encoded format specified via the MessageEncoding (347) field. 
    #[serde(alias = "355")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_text: Option<super::logon::EncodedText>,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl Reject {
    pub fn new() -> Self {
        Reject {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::Reject),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl HasHeader for Reject {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl From<Header> for Reject {
    fn from(header: Header) -> Self {
        let mut reply_header = header.clone();
        reply_header.reply(MsgType::Reject);
        Reject {
            header: reply_header,
            ref_seq_num: header.msg_seq_num,
            ref_msg_type: header.msg_type,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SessionRejectReason {
    /// Invalid Tag Number
    #[serde(rename = "0")]
    InvalidTagNumber,
    /// Required Tag Missing
    #[serde(rename = "1")]
    RequiredTagMissing,
    /// Tag not defined for this message type
    #[serde(rename = "2")]
    TagNotDefinedForThisMessageType,
    /// Undefined tag
    #[serde(rename = "3")]
    UndefinedTag,
    /// Tag specified without a value
    #[serde(rename = "4")]
    TagSpecifiedWithoutAValue,
    /// Value is incorrect (out of range) for this tag
    #[serde(rename = "5")]
    ValueIsIncorrectForThisTag,
    /// Incorrect data format for value
    #[serde(rename = "6")]
    IncorrectDataFormatForValue,
    /// Decryption problem
    #[serde(rename = "7")]
    DecryptionProblem,
    /// Signature problem
    #[serde(rename = "8")]
    SignatureProblem,
    /// CompID problem
    #[serde(rename = "9")]
    CompIdProblem,
    /// SendingTime Accuracy Problem
    #[serde(rename = "10")]
    SendingTimeAccuracyProblem,
    /// Invalid MsgType
    #[serde(rename = "11")]
    InvalidMsgType,
    /// XML Validation Error
    #[serde(rename = "12")]
    XMLValidationError,
    /// Tag appears more than once
    #[serde(rename = "13")]
    TagAppearsMoreThanOnce,
    /// Tag specified out of required order
    #[serde(rename = "14")]
    TagSpecifiedOutOfRequiredOrder,
    /// Repeating group fields out of order
    #[serde(rename = "15")]
    RepeatingGroupFieldsOutOfOrder,
    /// Incorrect NumInGroup count for repeating group
    #[serde(rename = "16")]
    IncorrectNumInGroupCountForRepeatingGroup,
    /// Non "Data" value includes field delimiter (<SOH> character)"
    #[serde(rename = "17")]
    NonDataValueIncludesFieldDelimiter,
    /// Invalid/Unsupported Application Version
    #[serde(rename = "18")]
    InvalidUnsupportedApplicationVersion,
    /// Other
    #[serde(rename = "99")]
    Other,
}
