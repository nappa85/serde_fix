
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPayout {
	/// UnderlyingDividendPayoutRatio
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42860")]
	pub underlying_dividend_payout_ratio: Option<f64>,
	/// UnderlyingDividendPayoutConditions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42861")]
	pub underlying_dividend_payout_conditions: Option<String>,
	/// UnderlyingDividendPaymentGrp
	#[serde(flatten)]
	pub underlying_dividend_payment_grp: Option<super::underlying_dividend_payment_grp::UnderlyingDividendPaymentGrp>,
}
