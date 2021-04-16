
use serde::{Serialize, Deserialize};

use fix_common::{ApplVerID, Boolean, EncodedText, RepeatingValues};
use crate::{StandardMessageHeader, StandardMessageTrailer};

/// MsgType = A
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Logon {
    #[serde(flatten)]
    pub standard_message_header: StandardMessageHeader<5, 'A', ' '>,
    /// (Always unencrypted, always last field in message)
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,
    /// Note same value used by both sides
    #[serde(rename = "108")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub heart_bt_int: i64,
    /// Required for some authentication methods
    #[serde(rename = "95")]
    #[serde(alias = "96")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<EncodedText<96>>,
    /// Indicates both sides of a FIX session should reset sequence numbers
    #[serde(rename = "141")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_seq_num_flag: Option<Boolean>,
    /// Optional, alternative via counterparty bi-lateral agreement message gap detection and recovery approach (see "Logon Message NextExpectedMsgSeqNum (789) Processing" section)
    #[serde(rename = "789")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub next_expected_msg_seq_num: Option<u64>,
    /// Can be used to specify the maximum number of bytes supported for messages received
    #[serde(rename = "383")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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
    pub ref_msg: Option<RepeatingValues<RefMsg>>,
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
    pub encrypted_password: Option<EncodedText<1402>>,
    /// Encrypted new password- encrypted via the method specified in the field EncryptedPasswordMethod(1400)
    #[serde(rename = "1403")]
    #[serde(alias = "1404")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_new_password: Option<EncodedText<1404>>,
    /// Session status at time of logon. Field is intended to be used when the logon is sent as an acknowledgement from acceptor of the FIX session.
    #[serde(rename = "1409")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_status: Option<SessionStatus>,
    /// The default version of FIX being carried over this FIXT session
    #[serde(rename = "1137")]
    pub default_appl_ver_id: ApplVerID<9>,
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
    pub encoded_text: Option<EncodedText<355>>,
    #[serde(flatten)]
    pub standard_message_trailer: StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for EncryptMethod {
    fn default() -> Self {
        EncryptMethod::None
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct RefMsg {
    /// Specifies a specific, supported MsgType. Required if NoMsgTypes (384) is > 0. Should be specified from the point of view of the sender of the Logon (A) message
    #[serde(rename = "372")]
    pub ref_msg_type: Option<String>,
    /// Indicates direction (send vs. receive) of a supported MsgType. Required if NoMsgTypes (384) is > 0. Should be specified from the point of view of the sender of the Logon (A) message
    #[serde(rename = "385")]
    pub msg_direction: Option<Direction>,
    /// Specifies the service pack release being applied to a message at the session level. Enumerated field with values assigned at time of service pack release
    #[serde(rename = "1130")]
    pub ref_appl_ver_id: Option<ApplVerID<9>>,
    /// Specified the extension pack being applied to a message
    #[serde(rename = "1406")]
    pub ref_appl_ext_id: Option<String>,
    /// Specifies a custom extension to a message being applied at the session level.
    #[serde(rename = "1131")]
    pub ref_cstm_appl_ver_id: Option<String>,
    /// Indicates that this Application Version (RefApplVerID(1130), RefApplExtID(1406),RefCstmApplVerID(1131)) is the default for the RefMsgType(372) field
    #[serde(rename = "1410")]
    pub default_ver_indicator: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    #[serde(rename = "R")]
    Receive,
    #[serde(rename = "S")]
    Send,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
