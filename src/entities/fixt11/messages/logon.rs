use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::entities::{ApplVerID, Boolean, data_field, fixt11::{header::{Header, HasHeader, MsgType}, Trailer}, version::FixVersion};

/// MsgType = A
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Logon {
    #[serde(flatten)]
    pub header: Header,
    /// (Always unencrypted, always last field in message)
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,
    /// Note same value used by both sides
    #[serde(rename = "108")]
    #[serde(deserialize_with = "crate::entities::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub heart_bt_int: i64,
    /// Required for some authentication methods
    #[serde(rename = "95")]
    #[serde(alias = "96")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<RawData>,
    /// Indicates both sides of a FIX session should reset sequence numbers
    #[serde(rename = "141")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_seq_num_flag: Option<Boolean>,
    /// Optional, alternative via counterparty bi-lateral agreement message gap detection and recovery approach (see "Logon Message NextExpectedMsgSeqNum (789) Processing" section)
    #[serde(rename = "789")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub next_expected_msg_seq_num: Option<u64>,
    /// Can be used to specify the maximum number of bytes supported for messages received
    #[serde(rename = "383")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub max_message_size: Option<usize>,
    /// Specifies the number of repeating RefMsgTypes specified
    #[serde(rename = "384")]
    #[serde(alias = "372")]
    #[serde(alias = "385")]
    #[serde(alias = "1130")]
    #[serde(alias = "1406")]
    #[serde(alias = "1131")]
    #[serde(alias = "1410")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_msg: Option<RefMsgs>,
    /// Can be used to specify that this FIX session will be sending and receiving "test" vs. "production" messages.
    #[serde(rename = "464")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_message_indicator: Option<Boolean>,
    /// Userid or username.
    #[serde(rename = "553")]
    pub username: Option<String>,
    /// Note: minimal security exists without transport-level encryption.
    #[serde(rename = "554")]
    pub password: Option<String>,
    /// Specifies a new password for the FIX Logon. The new password is used for subsequent logons.
    #[serde(rename = "925")]
    pub new_password: Option<String>,
    /// EncryptedPasswordMethod
    #[serde(rename = "1400")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_password_method: Option<String>,
    /// EncryptedPassword
    #[serde(rename = "1401")]
    #[serde(alias = "1402")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_password: Option<EncryptedPassword>,
    /// Encrypted new password- encrypted via the method specified in the field EncryptedPasswordMethod(1400)
    #[serde(rename = "1403")]
    #[serde(alias = "1404")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_new_password: Option<EncryptedNewPassword>,
    /// Session status at time of logon. Field is intended to be used when the logon is sent as an acknowledgement from acceptor of the FIX session.
    #[serde(rename = "1409")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_status: Option<SessionStatus>,
    /// The default version of FIX being carried over this FIXT session
    #[serde(rename = "1137")]
    pub default_appl_ver_id: ApplVerID,
    /// The default extension pack for FIX messages used in this session
    #[serde(rename = "1407")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_appl_ext_id: Option<String>,
    /// The default custom application version (dictionary) for FIX messages used in this session
    #[serde(rename = "1408")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cstm_appl_ver_id: Option<String>,
    /// Available to provide a response to logon when used as a logon acknowledgement from acceptor back to the logon initiator.
    #[serde(rename = "58")]
    pub text: Option<String>,
    /// Must be set if EncodedText field is specified and must immediately precede it.
    #[serde(rename = "354")]
    /// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
    #[serde(alias = "355")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_text: Option<EncodedText>,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl Logon {
    pub fn new() -> Self {
        Logon {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::Logon),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl HasHeader for Logon {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Default for Logon {
    fn default() -> Self {
        Logon {
            header: Header::default(),
            encrypt_method: EncryptMethod::None,
            heart_bt_int: 0,
            raw_data: None,
            reset_seq_num_flag: None,
            next_expected_msg_seq_num: None,
            max_message_size: None,
            ref_msg: None,
            test_message_indicator: None,
            username: None,
            password: None,
            new_password: None,
            encrypted_password_method: None,
            encrypted_password: None,
            encrypted_new_password: None,
            session_status: None,
            default_appl_ver_id: ApplVerID::FIX50SP2,
            default_appl_ext_id: None,
            default_cstm_appl_ver_id: None,
            text: None,
            encoded_text: None,
            trailer: Trailer::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EncryptMethod {
    /// None / other
    #[serde(rename = "0")]
    None,
    /// PKCS (proprietary)
    #[serde(rename = "1")]
    Pkcs,
    /// DES (ECB mode)
    #[serde(rename = "2")]
    Des,
    /// PKCS/DES (proprietary)
    #[serde(rename = "3")]
    PkcsDes,
    /// PGP/DES (defunct)
    #[serde(rename = "4")]
    PgpDes,
    /// PGP/DES-MD5 (see app note on FIX web site)
    #[serde(rename = "5")]
    PgpDesMd5,
    /// PEM/DES-MD5 (see app note on FIX web site)
    #[serde(rename = "6")]
    PemDesMd5,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawData {
    // #[serde(rename = "95")]
    len: usize,
    // #[serde(rename = "96")]
    data: String,
}

impl data_field::DataField for RawData {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for RawData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "96")
    }
}

impl Serialize for RawData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "96")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncryptedPassword {
    // #[serde(rename = "1401")]
    len: usize,
    // #[serde(rename = "1402")]
    data: String,
}

impl data_field::DataField for EncryptedPassword {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncryptedPassword {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "1402")
    }
}

impl Serialize for EncryptedPassword {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "1402")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncryptedNewPassword {
    // #[serde(rename = "1403")]
    len: usize,
    // #[serde(rename = "1404")]
    data: String,
}

impl data_field::DataField for EncryptedNewPassword {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncryptedNewPassword {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "1404")
    }
}

impl Serialize for EncryptedNewPassword {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "1404")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncodedText {
    // #[serde(rename = "354")]
    len: usize,
    // #[serde(rename = "355")]
    data: String,
}

impl data_field::DataField for EncodedText {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncodedText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "355")
    }
}

impl Serialize for EncodedText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "355")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RefMsgs {
    // #[serde(rename = "384")]
    len: usize,
    inner: Vec<RefMsg>,
}

impl<'de> Deserialize<'de> for RefMsgs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<RefMsgs, D::Error> {
        let temp = <Cow<'_, str> as Deserialize>::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        let mut msgs = RefMsgs::default();
        let mut actual = RefMsg::default();
        let iterator = temp.split('\u{1}').map(|a| match a.find('=') {
            Some(i) => (&a[0..i], &a[(i + 1)..]),
            None => ("", &a[0..]),
        });
        for (code, value) in iterator {
            match &*code {
                "372" => {
                    if actual.ref_msg_type.is_some() ||
                        actual.msg_direction.is_some() ||
                        actual.ref_appl_ver_id.is_some() ||
                        actual.ref_appl_ext_id.is_some() ||
                        actual.ref_cstm_appl_ver_id.is_some() ||
                        actual.default_ver_indicator.is_some() {
                        msgs.inner.push(actual);
                        actual = RefMsg::default();
                    }
                    actual.ref_msg_type = Some(value.to_owned());
                },
                "385" => {
                    if actual.msg_direction.is_some() ||
                        actual.ref_appl_ver_id.is_some() ||
                        actual.ref_appl_ext_id.is_some() ||
                        actual.ref_cstm_appl_ver_id.is_some() ||
                        actual.default_ver_indicator.is_some() {
                        msgs.inner.push(actual);
                        actual = RefMsg::default();
                    }
                    actual.msg_direction = crate::from_str(value).ok();//TODO: error management
                },
                "1130" => {
                    if actual.ref_appl_ver_id.is_some() ||
                        actual.ref_appl_ext_id.is_some() ||
                        actual.ref_cstm_appl_ver_id.is_some() ||
                        actual.default_ver_indicator.is_some() {
                        msgs.inner.push(actual);
                        actual = RefMsg::default();
                    }
                    actual.ref_appl_ver_id = crate::from_str(value).ok();//TODO: error management
                },
                "1406" => {
                    if actual.ref_appl_ext_id.is_some() ||
                        actual.ref_cstm_appl_ver_id.is_some() ||
                        actual.default_ver_indicator.is_some() {
                        msgs.inner.push(actual);
                        actual = RefMsg::default();
                    }
                    actual.ref_appl_ext_id = Some(value.to_owned());
                },
                "1131" => {
                    if actual.ref_cstm_appl_ver_id.is_some() ||
                        actual.default_ver_indicator.is_some() {
                        msgs.inner.push(actual);
                        actual = RefMsg::default();
                    }
                    actual.ref_cstm_appl_ver_id = Some(value.to_owned());
                },
                "1410" => {
                    if actual.default_ver_indicator.is_some() {
                        msgs.inner.push(actual);
                        actual = RefMsg::default();
                    }
                    actual.default_ver_indicator = Some(value.to_owned());
                },
                // 384
                _ => {
                    msgs.len = value.parse().map_err(serde::de::Error::custom)?;
                },
            }
        }
        if actual.ref_msg_type.is_some() ||
            actual.msg_direction.is_some() ||
            actual.ref_appl_ver_id.is_some() ||
            actual.ref_appl_ext_id.is_some() ||
            actual.ref_cstm_appl_ver_id.is_some() ||
            actual.default_ver_indicator.is_some() {
            msgs.inner.push(actual);
        }
        Ok(msgs)
    }
}

impl Serialize for RefMsgs {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut temp = Vec::new();
        temp.push(self.len.to_string());
        for msg in &self.inner {
            if let Some(s) = &msg.ref_msg_type {
                temp.push(format!("372={}", s));
            }
            if let Some(s) = &msg.msg_direction {
                temp.push(format!("385={}", s.to_string()));
            }
            if let Some(s) = &msg.ref_appl_ver_id {
                temp.push(format!("1130={}", s.to_string()));
            }
            if let Some(s) = &msg.ref_appl_ext_id {
                temp.push(format!("1406={}", s));
            }
            if let Some(s) = &msg.ref_cstm_appl_ver_id {
                temp.push(format!("1131={}", s));
            }
            if let Some(s) = &msg.default_ver_indicator {
                temp.push(format!("1410={}", s));
            }
        }
        temp.join("\u{1}").serialize(serializer)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RefMsg {
    /// Specifies a specific, supported MsgType. Required if NoMsgTypes (384) is > 0. Should be specified from the point of view of the sender of the Logon (A) message
    // #[serde(rename = "372")]
    pub ref_msg_type: Option<String>,
    /// Indicates direction (send vs. receive) of a supported MsgType. Required if NoMsgTypes (384) is > 0. Should be specified from the point of view of the sender of the Logon (A) message
    // #[serde(rename = "385")]
    pub msg_direction: Option<Direction>,
    /// Specifies the service pack release being applied to a message at the session level. Enumerated field with values assigned at time of service pack release
    // #[serde(rename = "1130")]
    pub ref_appl_ver_id: Option<ApplVerID>,
    /// Specified the extension pack being applied to a message
    // #[serde(rename = "1406")]
    pub ref_appl_ext_id: Option<String>,
    /// Specifies a custom extension to a message being applied at the session level.
    // #[serde(rename = "1131")]
    pub ref_cstm_appl_ver_id: Option<String>,
    /// Indicates that this Application Version (RefApplVerID(1130), RefApplExtID(1406),RefCstmApplVerID(1131)) is the default for the RefMsgType(372) field
    // #[serde(rename = "1410")]
    pub default_ver_indicator: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    #[serde(rename = "R")]
    Receive,
    #[serde(rename = "S")]
    Send,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        crate::to_string(self).expect("Direction serialize failed")// should be safe
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SessionStatus {
    /// Session active
    #[serde(rename = "0")]
    SessionActive,
    /// Session password changed
    #[serde(rename = "1")]
    SessionPasswordChanged,
    /// Session password due to expire
    #[serde(rename = "2")]
    SessionPasswordDueToExpire,
    /// New session password does not comply with policy
    #[serde(rename = "3")]
    NewSessionPasswordDoesNotComplyWithPolicy,
    /// Session logout complete
    #[serde(rename = "4")]
    SessionLogoutComplete,
    /// Invalid username and password
    #[serde(rename = "5")]
    InvalidUsernameAndPassword,
    /// Account locked
    #[serde(rename = "6")]
    AccountLocked,
    /// Logons are not allowed at this time
    #[serde(rename = "7")]
    LogonsAreNotAllowedAtThisTime,
    /// Password expired
    #[serde(rename = "8")]
    PasswordExpired,
}
