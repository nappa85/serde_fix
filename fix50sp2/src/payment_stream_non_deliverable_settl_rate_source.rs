
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamNonDeliverableSettlRateSource {
	/// PaymentStreamNonDeliverableSettlRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40371")]
	pub payment_stream_non_deliverable_settl_rate_source_item: Option<PaymentStreamNonDeliverableSettlRateSourceItem>,
	/// Conditionally required when PaymentStreamNonDeliverableSettlRateSource(40371) = 3 (ISDA Settlement Rate Option) or 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40372")]
	pub payment_stream_non_deliverable_settl_reference_page: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentStreamNonDeliverableSettlRateSourceItem {
	/// Bloomberg
	#[serde(rename = "0")]
	Bloomberg,
	/// Reuters
	#[serde(rename = "1")]
	Reuters,
	/// Telerate
	#[serde(rename = "2")]
	Telerate,
	/// ISDA Settlement Rate Option (Elaboration: The source of the currency conversion as specified by the ISDA terms in Annex A
	/// to the 1998 FX and Currency Option Definitions. See <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.fpml.org/coding-scheme/settlement-rate-option" target="_blank">http://www.fpml.org/coding-scheme/settlement-rate-option</a> )
	#[serde(rename = "3")]
	IsdaSettlementRateOption,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for PaymentStreamNonDeliverableSettlRateSourceItem {
	fn default() -> Self {
		PaymentStreamNonDeliverableSettlRateSourceItem::Bloomberg
	}
}
