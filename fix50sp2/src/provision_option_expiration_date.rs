
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExpirationDate {
	/// ProvisionOptionExpirationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40145")]
	pub provision_option_expiration_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the provisional option expiration date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40146")]
	pub provision_option_expiration_date_business_day_convention: Option<ProvisionOptionExpirationDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the provisional option expiration date.
	#[serde(flatten)]
	pub provision_option_expiration_date_business_center_grp: Option<super::provision_option_expiration_date_business_center_grp::ProvisionOptionExpirationDateBusinessCenterGrp>,
	/// ProvisionOptionExpirationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40148")]
	pub provision_option_expiration_date_relative_to: Option<ProvisionOptionExpirationDateRelativeTo>,
	/// Conditionally required when ProvisionOptionExpirationDateOffsetUnit(40150) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40149")]
	pub provision_option_expiration_date_offset_period: Option<i32>,
	/// Conditionally required when ProvisionOptionExpirationDateOffsetPeriod(40149) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40150")]
	pub provision_option_expiration_date_offset_unit: Option<ProvisionOptionExpirationDateOffsetUnit>,
	/// ProvisionOptionExpirationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40151")]
	pub provision_option_expiration_date_offset_day_type: Option<ProvisionOptionExpirationDateOffsetDayType>,
	/// ProvisionOptionExpirationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40152")]
	pub provision_option_expiration_date_adjusted: Option<fix_common::LocalMktDate>,
	/// ProvisionOptionExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40153")]
	pub provision_option_expiration_time: Option<String>,
	/// ProvisionOptionExpirationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40154")]
	pub provision_option_expiration_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionExpirationDateBusinessDayConvention {
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

impl Default for ProvisionOptionExpirationDateBusinessDayConvention {
	fn default() -> Self {
		ProvisionOptionExpirationDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionExpirationDateRelativeTo {
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

impl Default for ProvisionOptionExpirationDateRelativeTo {
	fn default() -> Self {
		ProvisionOptionExpirationDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionExpirationDateOffsetUnit {
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

impl Default for ProvisionOptionExpirationDateOffsetUnit {
	fn default() -> Self {
		ProvisionOptionExpirationDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ProvisionOptionExpirationDateOffsetDayType {
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

impl Default for ProvisionOptionExpirationDateOffsetDayType {
	fn default() -> Self {
		ProvisionOptionExpirationDateOffsetDayType::Business
	}
}
