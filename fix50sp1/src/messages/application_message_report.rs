
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplicationMessageReport {
	/// MsgType = BY
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'Y'>,
	/// Identifier for the Application Message Report.
	#[serde(rename = "1356")]
	pub appl_report_id: String,
	/// Type of report.
	#[serde(rename = "1426")]
	pub appl_report_type: ApplReportType,
	/// Number of applications.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1351")]
	pub appl_i_ds: Option<fix_common::RepeatingValues<ApplID>>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplID {
	/// RefApplID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1355")]
	pub ref_appl_id: Option<String>,
	/// ApplNewSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1399")]
	pub appl_new_seq_num: Option<usize>,
	/// RefApplLastSeqNum
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1357")]
	pub ref_appl_last_seq_num: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ApplReportType {
	/// Reset ApplSeqNum to new value specified in <a href="tag_1399_ApplNewSeqNum.html" target="bottom">ApplNewSeqNum&nbsp;(1399)</a>
	#[serde(rename = "0")]
	ResetApplSeqNumToNewValueSpecifiedInAHrefTag1399ApplNewSeqNumHtmlTargetBottomApplNewSeqNumNbspA,
	/// Reports that the last message has been sent for the ApplIDs Refer to <a href="tag_1357_RefApplLastSeqNum.html" target="bottom">RefApplLastSeqNum&nbsp;(1357)</a> for the application sequence number of the last message
	#[serde(rename = "1")]
	ReportsThatTheLastMessageHasBeenSentForTheApplIDsReferToAHrefTag1357RefApplLastSeqNumHtmlTargetBottomRefApplLastSeqNumNbspAForTheApplicationSequenceNumberOfTheLastMessage,
	/// Heartbeat message indicating that Application identified by <a href="tag_1355_RefApplID.html" target="bottom">RefApplID&nbsp;(1355)</a> is still alive. Refer to <a href="tag_1357_RefApplLastSeqNum.html" target="bottom">RefApplLastSeqNum&nbsp;(1357)</a> for the application sequence number of the previous message.
	#[serde(rename = "2")]
	HeartbeatMessageIndicatingThatApplicationIdentifiedByAHrefTag1355RefApplIdHtmlTargetBottomRefApplIdNbspAIsStillAliveReferToAHrefTag1357RefApplLastSeqNumHtmlTargetBottomRefApplLastSeqNumNbspAForTheApplicationSequenceNumberOfThePreviousMessage,
}

impl Default for ApplReportType {
	fn default() -> Self {
		ApplReportType::ResetApplSeqNumToNewValueSpecifiedInAHrefTag1399ApplNewSeqNumHtmlTargetBottomApplNewSeqNumNbspA
	}
}
