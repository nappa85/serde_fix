
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdCollGrp {
	/// Trades for which collateral is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "897")]
	pub trades: Option<fix_common::RepeatingValues<Trade>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Trade {
	/// Required if NoTrades &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "571")]
	pub trade_report_id: Option<String>,
	/// SecondaryTradeReportID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "818")]
	pub secondary_trade_report_id: Option<String>,
}
