
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionRelevantUnderlyingDate {
	/// UnderlyingProvisionOptionRelevantUnderlyingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42142")]
	pub underlying_provision_option_relevant_underlying_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the provisional option relevant underlying date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42143")]
	pub underlying_provision_option_relevant_underlying_date_business_day_convention: Option<UnderlyingProvisionOptionRelevantUnderlyingDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the provisional option relevent underlying date.
	#[serde(flatten)]
	pub underlying_provision_option_relevant_underlying_date_business_center_grp: Option<super::underlying_provision_option_relevant_underlying_date_business_center_grp::UnderlyingProvisionOptionRelevantUnderlyingDateBusinessCenterGrp>,
	/// UnderlyingProvisionOptionRelevantUnderlyingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42144")]
	pub underlying_provision_option_relevant_underlying_date_relative_to: Option<UnderlyingProvisionOptionRelevantUnderlyingDateRelativeTo>,
	/// Conditionally required when UnderlyingProvisionOptionRelevantUnderlyingDateOffsetUnit(42146) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42145")]
	pub underlying_provision_option_relevant_underlying_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionOptionRelevantUnderlyingDateOffsetPeriod(42145) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42146")]
	pub underlying_provision_option_relevant_underlying_date_offset_unit: Option<UnderlyingProvisionOptionRelevantUnderlyingDateOffsetUnit>,
	/// UnderlyingProvisionOptionRelevantUnderlyingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42147")]
	pub underlying_provision_option_relevant_underlying_date_offset_day_type: Option<UnderlyingProvisionOptionRelevantUnderlyingDateOffsetDayType>,
	/// UnderlyingProvisionOptionRelevantUnderlyingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42148")]
	pub underlying_provision_option_relevant_underlying_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionRelevantUnderlyingDateBusinessDayConvention {
	/// Not applicable
	#[serde(rename = "0")]
	NotApplicable,
	/// None (current day)
	#[serde(rename = "1")]
	None,
	/// Following day
	#[serde(rename = "2")]
	FollowingDay,
	/// Floating rate note
	#[serde(rename = "3")]
	FloatingRateNote,
	/// Modified following day
	#[serde(rename = "4")]
	ModifiedFollowingDay,
	/// Preceding day
	#[serde(rename = "5")]
	PrecedingDay,
	/// Modified preceding day
	#[serde(rename = "6")]
	ModifiedPrecedingDay,
	/// Nearest day
	#[serde(rename = "7")]
	NearestDay,
}

impl Default for UnderlyingProvisionOptionRelevantUnderlyingDateBusinessDayConvention {
	fn default() -> Self {
		UnderlyingProvisionOptionRelevantUnderlyingDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionRelevantUnderlyingDateRelativeTo {
	/// Trade date
	#[serde(rename = "0")]
	TradeDate,
	/// Settlement date
	#[serde(rename = "1")]
	SettlementDate,
	/// Effective date
	#[serde(rename = "2")]
	EffectiveDate,
	/// Calculation period start date
	#[serde(rename = "3")]
	CalculationPeriodStartDate,
	/// Calculation period end date
	#[serde(rename = "4")]
	CalculationPeriodEndDate,
	/// Reset date
	#[serde(rename = "5")]
	ResetDate,
	/// Last pricing date
	#[serde(rename = "6")]
	LastPricingDate,
	/// Valuation date
	#[serde(rename = "7")]
	ValuationDate,
	/// Cash settlement date
	#[serde(rename = "8")]
	CashSettlementDate,
	/// Option exercise start date
	#[serde(rename = "9")]
	OptionExerciseStartDate,
}

impl Default for UnderlyingProvisionOptionRelevantUnderlyingDateRelativeTo {
	fn default() -> Self {
		UnderlyingProvisionOptionRelevantUnderlyingDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionRelevantUnderlyingDateOffsetUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

impl Default for UnderlyingProvisionOptionRelevantUnderlyingDateOffsetUnit {
	fn default() -> Self {
		UnderlyingProvisionOptionRelevantUnderlyingDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionRelevantUnderlyingDateOffsetDayType {
	/// Business
	#[serde(rename = "0")]
	Business,
	/// Calendar
	#[serde(rename = "1")]
	Calendar,
	/// Commodity business
	#[serde(rename = "2")]
	CommodityBusiness,
	/// Currency business
	#[serde(rename = "3")]
	CurrencyBusiness,
	/// Exchange business
	#[serde(rename = "4")]
	ExchangeBusiness,
	/// Scheduled trading day
	#[serde(rename = "5")]
	ScheduledTradingDay,
}

impl Default for UnderlyingProvisionOptionRelevantUnderlyingDateOffsetDayType {
	fn default() -> Self {
		UnderlyingProvisionOptionRelevantUnderlyingDateOffsetDayType::Business
	}
}
