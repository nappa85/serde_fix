
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Application {
	/// MsgType = BY
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Identifier for the Application Message Report.
	#[serde(rename = "1356")]
	pub appl_report_id: String,
	/// If the application message report is generated in response to an <a href="message_Application_Message_Request_BW.html" target="main">ApplicationMessageRequest(MsgType=BW)&nbsp;(BW)</a> , then this tag contain the ApplReqID(1346) of that request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1346")]
	pub appl_req_id: Option<String>,
	/// Type of report.
	#[serde(rename = "1426")]
	pub appl_report_type: ApplReportType,
	/// ApplIDReportGrp
	#[serde(flatten)]
	pub appl_id_report_grp: Option<super::super::appl_id_report_grp::ApplIDReportGrp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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
	/// Application message re-send completed
	#[serde(rename = "3")]
	ApplicationMessageReSendCompleted,
}
