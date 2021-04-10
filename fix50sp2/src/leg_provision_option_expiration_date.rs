
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionExpirationDate {
	/// LegProvisionOptionExpirationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40498")]
	pub leg_provision_option_expiration_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg provision option expiration date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40499")]
	pub leg_provision_option_expiration_date_business_day_convention: Option<LegProvisionOptionExpirationDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg provision option expiration date.
	#[serde(flatten)]
	pub leg_provision_option_expiration_date_business_center_grp: Option<super::leg_provision_option_expiration_date_business_center_grp::LegProvisionOptionExpirationDateBusinessCenterGrp>,
	/// LegProvisionOptionExpirationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40501")]
	pub leg_provision_option_expiration_date_relative_to: Option<LegProvisionOptionExpirationDateRelativeTo>,
	/// Conditionally required when LegProvisionOptionExpirationDateOffsetUnit(40503) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40502")]
	pub leg_provision_option_expiration_date_offset_period: Option<i32>,
	/// Conditionally required when LegProvisionOptionExpirationDateOffsetPeriod(40502) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40503")]
	pub leg_provision_option_expiration_date_offset_unit: Option<LegProvisionOptionExpirationDateOffsetUnit>,
	/// LegProvisionOptionExpirationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40504")]
	pub leg_provision_option_expiration_date_offset_day_type: Option<LegProvisionOptionExpirationDateOffsetDayType>,
	/// LegProvisionOptionExpirationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40505")]
	pub leg_provision_option_expiration_date_adjusted: Option<fix_common::LocalMktDate>,
	/// LegProvisionOptionExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40506")]
	pub leg_provision_option_expiration_time: Option<String>,
	/// LegProvisionOptionExpirationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40507")]
	pub leg_provision_option_expiration_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionExpirationDateBusinessDayConvention {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionExpirationDateRelativeTo {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionExpirationDateOffsetUnit {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionExpirationDateOffsetDayType {
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
