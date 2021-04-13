
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamEffectiveDate {
	/// UnderlyingStreamEffectiveDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40057")]
	pub underlying_stream_effective_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's stream effective dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40058")]
	pub underlying_stream_effective_date_business_day_convention: Option<UnderlyingStreamEffectiveDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's stream effective dates.
	#[serde(flatten)]
	pub underlying_stream_effective_date_business_center_grp: Option<super::underlying_stream_effective_date_business_center_grp::UnderlyingStreamEffectiveDateBusinessCenterGrp>,
	/// UnderlyingStreamEffectiveDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40060")]
	pub underlying_stream_effective_date_relative_to: Option<UnderlyingStreamEffectiveDateRelativeTo>,
	/// Conditionally required when UnderlyingStreamEffectiveDateOffsetUnit(40062) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40061")]
	pub underlying_stream_effective_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingStreamEffectiveDateOffsetPeriod(40061) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40062")]
	pub underlying_stream_effective_date_offset_unit: Option<UnderlyingStreamEffectiveDateOffsetUnit>,
	/// UnderlyingStreamEffectiveDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40063")]
	pub underlying_stream_effective_date_offset_day_type: Option<UnderlyingStreamEffectiveDateOffsetDayType>,
	/// UnderlyingStreamEffectiveDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40064")]
	pub underlying_stream_effective_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamEffectiveDateBusinessDayConvention {
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

impl Default for UnderlyingStreamEffectiveDateBusinessDayConvention {
	fn default() -> Self {
		UnderlyingStreamEffectiveDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamEffectiveDateRelativeTo {
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

impl Default for UnderlyingStreamEffectiveDateRelativeTo {
	fn default() -> Self {
		UnderlyingStreamEffectiveDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamEffectiveDateOffsetUnit {
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

impl Default for UnderlyingStreamEffectiveDateOffsetUnit {
	fn default() -> Self {
		UnderlyingStreamEffectiveDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamEffectiveDateOffsetDayType {
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

impl Default for UnderlyingStreamEffectiveDateOffsetDayType {
	fn default() -> Self {
		UnderlyingStreamEffectiveDateOffsetDayType::Business
	}
}
