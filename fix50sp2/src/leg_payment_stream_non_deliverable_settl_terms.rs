
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamNonDeliverableSettlTerms {
	/// LegPaymentStreamNonDeliverableRefCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40359")]
	pub leg_payment_stream_non_deliverable_ref_currency: Option<String>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the non-deliverable currency's fixing date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40360")]
	pub leg_payment_stream_non_deliverable_fixing_dates_business_day_convention: Option<LegPaymentStreamNonDeliverableFixingDatesBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the non-deliverable currency's fixing date.
	#[serde(flatten)]
	pub leg_payment_stream_non_deliverable_fixing_dates_business_center_grp: Option<super::leg_payment_stream_non_deliverable_fixing_dates_business_center_grp::LegPaymentStreamNonDeliverableFixingDatesBusinessCenterGrp>,
	/// LegPaymentStreamNonDeliverableFixingDatesRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40362")]
	pub leg_payment_stream_non_deliverable_fixing_dates_relative_to: Option<LegPaymentStreamNonDeliverableFixingDatesRelativeTo>,
	/// Conditionally required when LegPaymentStreamNonDeliverableFixingDateOffsetUnit(40364) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40363")]
	pub leg_payment_stream_non_deliverable_fixing_dates_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamNonDeliverableFixingDateOffsetPeriod(40363) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40364")]
	pub leg_payment_stream_non_deliverable_fixing_dates_offset_unit: Option<LegPaymentStreamNonDeliverableFixingDatesOffsetUnit>,
	/// LegPaymentStreamNonDeliverableFixingDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40365")]
	pub leg_payment_stream_non_deliverable_fixing_dates_offset_day_type: Option<LegPaymentStreamNonDeliverableFixingDatesOffsetDayType>,
	/// LegPaymentStreamNonDeliverableSettlRateSource
	#[serde(flatten)]
	pub leg_payment_stream_non_deliverable_settl_rate_source: Option<super::leg_payment_stream_non_deliverable_settl_rate_source::LegPaymentStreamNonDeliverableSettlRateSource>,
	/// LegPaymentStreamNonDeliverableFixingDateGrp
	#[serde(flatten)]
	pub leg_payment_stream_non_deliverable_fixing_date_grp: Option<super::leg_payment_stream_non_deliverable_fixing_date_grp::LegPaymentStreamNonDeliverableFixingDateGrp>,
	/// LegSettlRateDisruptionFallbackGrp
	#[serde(flatten)]
	pub leg_settl_rate_disruption_fallback_grp: Option<super::leg_settl_rate_disruption_fallback_grp::LegSettlRateDisruptionFallbackGrp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStreamNonDeliverableFixingDatesBusinessDayConvention {
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
pub enum LegPaymentStreamNonDeliverableFixingDatesRelativeTo {
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
pub enum LegPaymentStreamNonDeliverableFixingDatesOffsetUnit {
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
pub enum LegPaymentStreamNonDeliverableFixingDatesOffsetDayType {
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
