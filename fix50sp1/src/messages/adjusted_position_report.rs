
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdjustedPositionReport {
	/// MsgType = BL
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this Adjusted Position report
	#[serde(rename = "721")]
	pub pos_maint_rpt_id: String,
	/// PosReqType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "724")]
	pub pos_req_type: Option<PosReqType>,
	/// The Clearing Business Date referred to by this maintenance request
	#[serde(rename = "715")]
	pub clearing_business_date: fix_common::LocalMktDate,
	/// SettlSessID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "716")]
	pub settl_sess_id: Option<SettlSessID>,
	/// PosMaintRptRefID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "714")]
	pub pos_maint_rpt_ref_id: Option<String>,
	/// Position Account
	#[serde(flatten)]
	pub parties: super::super::parties::Parties,
	/// Insert here here the set of "Position Qty" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub position_qty: super::super::position_qty::PositionQty,
	/// Specifies the number of repeating symbols (instruments) specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "146")]
	pub related_sym: Option<fix_common::RepeatingValues<RelatedSy>>,
	/// Settlement Price
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "730")]
	pub settl_price: Option<f64>,
	/// Prior Settlement Price
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "734")]
	pub prior_settl_price: Option<f64>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PosReqType {
	/// Positions
	#[serde(rename = "0")]
	Positions,
	/// Trades
	#[serde(rename = "1")]
	Trades,
	/// Exercises
	#[serde(rename = "2")]
	Exercises,
	/// Assignments
	#[serde(rename = "3")]
	Assignments,
	/// Settlement Activity
	#[serde(rename = "4")]
	SettlementActivity,
	/// Backout Message
	#[serde(rename = "5")]
	BackoutMessage,
}

impl Default for PosReqType {
	fn default() -> Self {
		PosReqType::Positions
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlSessID {
	/// Intraday
	#[serde(rename = "ITD")]
	Intraday,
	/// Regular Trading Hours
	#[serde(rename = "RTH")]
	RegularTradingHours,
	/// Electronic Trading Hours
	#[serde(rename = "ETH")]
	ElectronicTradingHours,
	/// End Of Day
	#[serde(rename = "EOD")]
	EndOfDay,
}

impl Default for SettlSessID {
	fn default() -> Self {
		SettlSessID::Intraday
	}
}
