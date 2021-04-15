
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExpirationDate {
	/// UnderlyingProvisionOptionExpirationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42133")]
	pub underlying_provision_option_expiration_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the busienss day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the provisional option expiration date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42134")]
	pub underlying_provision_option_expiration_date_business_day_convention: Option<UnderlyingProvisionOptionExpirationDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the provisional option expiration date.
	#[serde(flatten)]
	pub underlying_provision_option_expiration_date_business_center_grp: Option<super::underlying_provision_option_expiration_date_business_center_grp::UnderlyingProvisionOptionExpirationDateBusinessCenterGrp>,
	/// UnderlyingProvisionOptionExpirationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42135")]
	pub underlying_provision_option_expiration_date_relative_to: Option<UnderlyingProvisionOptionExpirationDateRelativeTo>,
	/// Conditionally required when UnderlyingProvisionOptionExpirationDateOffsetUnit(42137) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42136")]
	pub underlying_provision_option_expiration_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionOptionExpirationDateOffsetPeriod(42136) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42137")]
	pub underlying_provision_option_expiration_date_offset_unit: Option<UnderlyingProvisionOptionExpirationDateOffsetUnit>,
	/// UnderlyingProvisionOptionExpirationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42138")]
	pub underlying_provision_option_expiration_date_offset_day_type: Option<UnderlyingProvisionOptionExpirationDateOffsetDayType>,
	/// UnderlyingProvisionOptionExpirationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42139")]
	pub underlying_provision_option_expiration_date_adjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionOptionExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42140")]
	pub underlying_provision_option_expiration_time: Option<String>,
	/// UnderlyingProvisionOptionExpirationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42141")]
	pub underlying_provision_option_expiration_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExpirationDateBusinessDayConvention {
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

impl Default for UnderlyingProvisionOptionExpirationDateBusinessDayConvention {
	fn default() -> Self {
		UnderlyingProvisionOptionExpirationDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExpirationDateRelativeTo {
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

impl Default for UnderlyingProvisionOptionExpirationDateRelativeTo {
	fn default() -> Self {
		UnderlyingProvisionOptionExpirationDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExpirationDateOffsetUnit {
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

impl Default for UnderlyingProvisionOptionExpirationDateOffsetUnit {
	fn default() -> Self {
		UnderlyingProvisionOptionExpirationDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExpirationDateOffsetDayType {
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

impl Default for UnderlyingProvisionOptionExpirationDateOffsetDayType {
	fn default() -> Self {
		UnderlyingProvisionOptionExpirationDateOffsetDayType::Business
	}
}
