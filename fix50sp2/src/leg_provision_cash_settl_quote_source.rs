
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlQuoteSource {
	/// LegProvisionCashSettlQuoteSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40470")]
	pub leg_provision_cash_settl_quote_source_item: Option<LegProvisionCashSettlQuoteSourceItem>,
	/// LegProvisionCashSettlQuoteReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41407")]
	pub leg_provision_cash_settl_quote_reference_page: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionCashSettlQuoteSourceItem {
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

impl Default for LegProvisionCashSettlQuoteSourceItem {
	fn default() -> Self {
		LegProvisionCashSettlQuoteSourceItem::Bloomberg
	}
}
