
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContraGrp {
	/// Number of ContraBrokers repeating group instances.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "382")]
	pub contra_brokers: Option<fix_common::RepeatingValues<ContraBroker>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContraBroker {
	/// First field in repeating group. Required if NoContraBrokers &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "375")]
	pub contra_broker: Option<String>,
	/// ContraTrader
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "337")]
	pub contra_trader: Option<String>,
	/// ContraTradeQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "437")]
	pub contra_trade_qty: Option<f64>,
	/// ContraTradeTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "438")]
	pub contra_trade_time: Option<fix_common::UTCTimestamp>,
	/// ContraLegRefID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "655")]
	pub contra_leg_ref_id: Option<String>,
}
