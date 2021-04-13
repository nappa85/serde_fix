
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamTerminationDate {
	/// StreamTerminationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40065")]
	pub stream_termination_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the termination date of the stream.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40066")]
	pub stream_termination_date_business_day_convention: Option<StreamTerminationDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the termination date of the stream.
	#[serde(flatten)]
	pub stream_termination_date_business_center_grp: Option<super::stream_termination_date_business_center_grp::StreamTerminationDateBusinessCenterGrp>,
	/// StreamTerminationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40068")]
	pub stream_termination_date_relative_to: Option<StreamTerminationDateRelativeTo>,
	/// Conditionally required when StreamTerminationDateOffsetUnit(40070) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40069")]
	pub stream_termination_date_offset_period: Option<i32>,
	/// Conditionally required when StreamTerminationDateOffsetPeriod(40069) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40070")]
	pub stream_termination_date_offset_unit: Option<StreamTerminationDateOffsetUnit>,
	/// StreamTerminationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40071")]
	pub stream_termination_date_offset_day_type: Option<StreamTerminationDateOffsetDayType>,
	/// StreamTerminationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40072")]
	pub stream_termination_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamTerminationDateBusinessDayConvention {
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

impl Default for StreamTerminationDateBusinessDayConvention {
	fn default() -> Self {
		StreamTerminationDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamTerminationDateRelativeTo {
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

impl Default for StreamTerminationDateRelativeTo {
	fn default() -> Self {
		StreamTerminationDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamTerminationDateOffsetUnit {
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

impl Default for StreamTerminationDateOffsetUnit {
	fn default() -> Self {
		StreamTerminationDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamTerminationDateOffsetDayType {
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

impl Default for StreamTerminationDateOffsetDayType {
	fn default() -> Self {
		StreamTerminationDateOffsetDayType::Business
	}
}
