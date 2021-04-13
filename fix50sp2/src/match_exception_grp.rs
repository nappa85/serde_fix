
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MatchExceptionGrp {
	/// NoMatchExceptions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2772")]
	pub match_exceptions: Option<fix_common::RepeatingValues<MatchException>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MatchException {
	/// Required if NoMatchExceptions(2772) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2773")]
	pub match_exception_type: Option<MatchExceptionType>,
	/// Required if NoMatchExceptions(2772) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2774")]
	pub match_exception_element_type: Option<MatchExceptionElementType>,
	/// MatchExceptionElementName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2775")]
	pub match_exception_element_name: Option<String>,
	/// MatchExceptionAllocValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2776")]
	pub match_exception_alloc_value: Option<String>,
	/// MatchExceptionConfirmValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2777")]
	pub match_exception_confirm_value: Option<String>,
	/// MatchExceptionToleranceValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2778")]
	pub match_exception_tolerance_value: Option<f64>,
	/// MatchExceptionToleranceValueType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2779")]
	pub match_exception_tolerance_value_type: Option<MatchExceptionToleranceValueType>,
	/// MatchExceptionText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2780")]
	pub match_exception_text: Option<String>,
	/// Must be set if EncodedMatchExceptio nText(2780) field is specified and must immediately precede it.
	#[serde(rename = "2797")]
	/// Encoded (non-ASCII characters) representation of the MatchExceptionText(2780) field in the encoded format specified via the
	/// MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2798")]
	pub encoded_match_exception_text: Option<fix_common::EncodedText<2798>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchExceptionType {
	/// No matching confirmation
	#[serde(rename = "0")]
	NoMatchingConfirmation,
	/// No matching allocation
	#[serde(rename = "1")]
	NoMatchingAllocation,
	/// Allocation data element missing
	#[serde(rename = "2")]
	AllocationDataElementMissing,
	/// Confirmation data element missing
	#[serde(rename = "3")]
	ConfirmationDataElementMissing,
	/// Data difference not within tolerance
	#[serde(rename = "4")]
	DataDifferenceNotWithinTolerance,
	/// Match within tolerance
	#[serde(rename = "5")]
	MatchWithinTolerance,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for MatchExceptionType {
	fn default() -> Self {
		MatchExceptionType::NoMatchingConfirmation
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchExceptionElementType {
	/// Accrued interest
	#[serde(rename = "1")]
	AccruedInterest,
	/// Deal price
	#[serde(rename = "2")]
	DealPrice,
	/// Trade date (Tolerance not applicable)
	#[serde(rename = "3")]
	TradeDate,
	/// Settlement date (Tolerance not applicable)
	#[serde(rename = "4")]
	SettlementDate,
	/// Side indicator (Tolerance not applicable)
	#[serde(rename = "5")]
	SideIndicator,
	/// Traded currency (Tolerance not applicable)
	#[serde(rename = "6")]
	TradedCurrency,
	/// Account ID (Tolerance not applicable)
	#[serde(rename = "7")]
	AccountId,
	/// Executing broker ID (Tolerance not applicable)
	#[serde(rename = "8")]
	ExecutingBrokerId,
	/// Settlement currency and amount
	#[serde(rename = "9")]
	SettlementCurrencyAndAmount,
	/// Net amount
	#[serde(rename = "11")]
	NetAmount,
	/// Place of settlement (Tolerance not applicable)
	#[serde(rename = "12")]
	PlaceOfSettlement,
	/// Commissions
	#[serde(rename = "13")]
	Commissions,
	/// Security identifier (Tolerance not applicable)
	#[serde(rename = "14")]
	SecurityIdentifier,
	/// Quantity allocated
	#[serde(rename = "15")]
	QuantityAllocated,
	/// Principal
	#[serde(rename = "16")]
	Principal,
	/// Fees
	#[serde(rename = "17")]
	Fees,
	/// Tax
	#[serde(rename = "18")]
	Tax,
}

impl Default for MatchExceptionElementType {
	fn default() -> Self {
		MatchExceptionElementType::AccruedInterest
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchExceptionToleranceValueType {
	/// Fixed amount (Default if not specified)
	#[serde(rename = "1")]
	FixedAmount,
	/// Percentage
	#[serde(rename = "2")]
	Percentage,
}

impl Default for MatchExceptionToleranceValueType {
	fn default() -> Self {
		MatchExceptionToleranceValueType::FixedAmount
	}
}
