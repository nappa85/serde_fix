
use serde::{Serialize, Deserialize};

use crate::entities::Boolean;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationSequenceControl {
    /// Identifies the application with which a message is associated. Used only if application sequencing is in effect.
    #[serde(rename = "1180")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appl_id: Option<String>,
    /// Application sequence number assigned to the message by the application generating the message. Used only if application sequencing is in effect. Conditionally required if ApplID has been specified.
    #[serde(rename = "1181")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appl_seq_num: Option<u64>,
    /// The previous sequence number in the application sequence stream. Permits an application to publish messages with sequence gaps where it cannot be avoided. Used only if application sequencing is in effect. Conditionally required if ApplID has been specified
    #[serde(rename = "1350")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appl_last_seq_num: Option<u64>,
    /// Used to indicate that a message is being sent in response to an Application Message Request. Used only if application sequencing is in effect. It is possible for both ApplResendFlag and PossDupFlag to be set on the same message if the Sender's cache size is greater than zero and the message is being resent due to a session level resend request. 
    #[serde(rename = "1352")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appl_resend_flag: Option<Boolean>,
}
