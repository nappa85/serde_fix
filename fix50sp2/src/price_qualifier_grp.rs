
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceQualifierGrp {
	/// NoPriceQualifiers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2709")]
	pub price_qualifiers: Option<fix_common::RepeatingValues<PriceQualifier>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceQualifier {
	/// Required if NoPriceQualifiers(2709) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2710")]
	pub price_qualifier_item: Option<PriceQualifierItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PriceQualifierItem {
	/// Accrued interest (if any) is factored into the price
	#[serde(rename = "0")]
	AccruedInterestIsFactoredIntoThePrice,
	/// Tax is factored into the price
	#[serde(rename = "1")]
	TaxIsFactoredIntoThePrice,
	/// The effect of bond amortization or the floating rate index offset is factored into the price
	#[serde(rename = "2")]
	TheEffectOfBondAmortizationOrTheFloatingRateIndexOffsetIsFactoredIntoThePrice,
}
