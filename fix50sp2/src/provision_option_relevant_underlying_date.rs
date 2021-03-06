
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionRelevantUnderlyingDate {
	/// ProvisionOptionRelevantUnderlyingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40155")]
	pub provision_option_relevant_underlying_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the provisional option relevant underlying date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40156")]
	pub provision_option_relevant_underlying_date_business_day_convention: Option<ProvisionOptionRelevantUnderlyingDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the provisional option relevent underlying date.
	#[serde(flatten)]
	pub provision_option_relevant_underlying_date_business_center_grp: Option<super::provision_option_relevant_underlying_date_business_center_grp::ProvisionOptionRelevantUnderlyingDateBusinessCenterGrp>,
	/// ProvisionOptionRelevantUnderlyingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40158")]
	pub provision_option_relevant_underlying_date_relative_to: Option<ProvisionOptionRelevantUnderlyingDateRelativeTo>,
	/// Conditionally required when ProvisionOptionRelevantUnderlyingDateOffsetUnit(40160) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40159")]
	pub provision_option_relevant_underlying_date_offset_period: Option<i32>,
	/// Conditionally required when ProvisionOptionRelevantUnderlyingDateOffsetPeriod(40159) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40160")]
	pub provision_option_relevant_underlying_date_offset_unit: Option<ProvisionOptionRelevantUnderlyingDateOffsetUnit>,
	/// ProvisionOptionRelevantUnderlyingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40161")]
	pub provision_option_relevant_underlying_date_offset_day_type: Option<ProvisionOptionRelevantUnderlyingDateOffsetDayType>,
	/// ProvisionOptionRelevantUnderlyingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40162")]
	pub provision_option_relevant_underlying_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionRelevantUnderlyingDateBusinessDayConvention {
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

impl Default for ProvisionOptionRelevantUnderlyingDateBusinessDayConvention {
	fn default() -> Self {
		ProvisionOptionRelevantUnderlyingDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionRelevantUnderlyingDateRelativeTo {
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

impl Default for ProvisionOptionRelevantUnderlyingDateRelativeTo {
	fn default() -> Self {
		ProvisionOptionRelevantUnderlyingDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionRelevantUnderlyingDateOffsetUnit {
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

impl Default for ProvisionOptionRelevantUnderlyingDateOffsetUnit {
	fn default() -> Self {
		ProvisionOptionRelevantUnderlyingDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionRelevantUnderlyingDateOffsetDayType {
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

impl Default for ProvisionOptionRelevantUnderlyingDateOffsetDayType {
	fn default() -> Self {
		ProvisionOptionRelevantUnderlyingDateOffsetDayType::Business
	}
}
