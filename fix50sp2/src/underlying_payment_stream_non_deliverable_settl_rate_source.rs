
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamNonDeliverableSettlRateSource {
	/// UnderlyingPaymentStreamNonDeliverableSettlRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40661")]
	pub underlying_payment_stream_non_deliverable_settl_rate_source: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamNonDeliverableSettlRateSource(40661) = 3 (ISDA Settlement Rate Option)
	/// or 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40824")]
	pub underlying_payment_stream_non_deliverable_settl_reference_page: Option<String>,
}
