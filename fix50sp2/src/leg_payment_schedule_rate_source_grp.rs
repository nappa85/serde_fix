
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleRateSourceGrp {
	/// NoLegPaymentScheduleRateSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40414")]
	pub leg_payment_schedule_rate_sources: Option<fix_common::RepeatingValues<LegPaymentScheduleRateSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleRateSource {
	/// Required if NoLegPaymentScheduleRateSources(40414) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40415")]
	pub leg_payment_schedule_rate_source_item: Option<LegPaymentScheduleRateSourceItem>,
	/// Required if NoLegPaymentScheduleRateSources(40414) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40416")]
	pub leg_payment_schedule_rate_source_type: Option<LegPaymentScheduleRateSourceType>,
	/// Conditionally required when LegPaymentScheduleRateSource(40415) = 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40417")]
	pub leg_payment_schedule_reference_page: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentScheduleRateSourceItem {
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

impl Default for LegPaymentScheduleRateSourceItem {
	fn default() -> Self {
		LegPaymentScheduleRateSourceItem::Bloomberg
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentScheduleRateSourceType {
	/// Primary
	#[serde(rename = "0")]
	Primary,
	/// Secondary
	#[serde(rename = "1")]
	Secondary,
}

impl Default for LegPaymentScheduleRateSourceType {
	fn default() -> Self {
		LegPaymentScheduleRateSourceType::Primary
	}
}
