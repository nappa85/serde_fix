
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlQuoteSource {
	/// ProvisionCashSettlQuoteSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40112")]
	pub provision_cash_settl_quote_source_item: Option<ProvisionCashSettlQuoteSourceItem>,
	/// ProvisionCashSettlQuoteReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41406")]
	pub provision_cash_settl_quote_reference_page: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlQuoteSourceItem {
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
