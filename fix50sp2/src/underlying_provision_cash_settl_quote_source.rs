
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlQuoteSource {
	/// UnderlyingProvisionCashSettlQuoteSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42102")]
	pub underlying_provision_cash_settl_quote_source_item: Option<UnderlyingProvisionCashSettlQuoteSourceItem>,
	/// UnderlyingProvisionCashSettlQuoteReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42103")]
	pub underlying_provision_cash_settl_quote_reference_page: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlQuoteSourceItem {
	/// Bloomberg
	#[serde(rename = "0")]
	Bloomberg,
	/// Reuters
	#[serde(rename = "1")]
	Reuters,
	/// Telerate
	#[serde(rename = "2")]
	Telerate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for UnderlyingProvisionCashSettlQuoteSourceItem {
	fn default() -> Self {
		UnderlyingProvisionCashSettlQuoteSourceItem::Bloomberg
	}
}
