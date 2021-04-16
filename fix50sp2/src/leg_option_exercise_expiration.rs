
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseExpiration {
	/// LegOptionExerciseExpirationDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41517")]
	pub leg_option_exercise_expiration_date_business_day_convention: Option<LegOptionExerciseExpirationDateBusinessDayConvention>,
	/// LegOptionExerciseExpirationDateBusinessCenterGrp
	#[serde(flatten)]
	pub leg_option_exercise_expiration_date_business_center_grp: Option<super::leg_option_exercise_expiration_date_business_center_grp::LegOptionExerciseExpirationDateBusinessCenterGrp>,
	/// LegOptionExerciseExpirationDateGrp
	#[serde(flatten)]
	pub leg_option_exercise_expiration_date_grp: Option<super::leg_option_exercise_expiration_date_grp::LegOptionExerciseExpirationDateGrp>,
	/// LegOptionExerciseExpirationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41518")]
	pub leg_option_exercise_expiration_date_relative_to: Option<LegOptionExerciseExpirationDateRelativeTo>,
	/// Conditionally required when LegOptionExerciseExpirationDateOffsetUnit(41520) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41519")]
	pub leg_option_exercise_expiration_date_offset_period: Option<i32>,
	/// Conditionally required when LegOptionExerciseExpirationDateOffsetPeriod(41519) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41520")]
	pub leg_option_exercise_expiration_date_offset_unit: Option<LegOptionExerciseExpirationDateOffsetUnit>,
	/// Conditionally required when LegOptionExerciseExpirationFrequencyUnit(41522) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41521")]
	pub leg_option_exercise_expiration_frequency_period: Option<i32>,
	/// Conditionally required when LegOptionExerciseExpirationFrequencyPeriod(41521) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41522")]
	pub leg_option_exercise_expiration_frequency_unit: Option<LegOptionExerciseExpirationFrequencyUnit>,
	/// LegOptionExerciseExpirationRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41523")]
	pub leg_option_exercise_expiration_roll_convention: Option<LegOptionExerciseExpirationRollConvention>,
	/// LegOptionExerciseExpirationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41524")]
	pub leg_option_exercise_expiration_date_offset_day_type: Option<LegOptionExerciseExpirationDateOffsetDayType>,
	/// LegOptionExerciseExpirationTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41525")]
	pub leg_option_exercise_expiration_time: Option<String>,
	/// LegOptionExerciseExpirationTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41526")]
	pub leg_option_exercise_expiration_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationDateBusinessDayConvention {
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

impl Default for LegOptionExerciseExpirationDateBusinessDayConvention {
	fn default() -> Self {
		LegOptionExerciseExpirationDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationDateRelativeTo {
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

impl Default for LegOptionExerciseExpirationDateRelativeTo {
	fn default() -> Self {
		LegOptionExerciseExpirationDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationDateOffsetUnit {
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

impl Default for LegOptionExerciseExpirationDateOffsetUnit {
	fn default() -> Self {
		LegOptionExerciseExpirationDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationFrequencyUnit {
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

impl Default for LegOptionExerciseExpirationFrequencyUnit {
	fn default() -> Self {
		LegOptionExerciseExpirationFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationRollConvention {
	/// 1st day of the month
	#[serde(rename = "1")]
	N1StDayOfTheMonth,
	/// 2nd day of the month
	#[serde(rename = "2")]
	N2NdDayOfTheMonth,
	/// 3rd day of the month
	#[serde(rename = "3")]
	N3RdDayOfTheMonth,
	/// 4th day of the month
	#[serde(rename = "4")]
	N4ThDayOfTheMonth,
	/// 5th day of the month
	#[serde(rename = "5")]
	N5ThDayOfTheMonth,
	/// 6thd day of the month
	#[serde(rename = "6")]
	N6ThdDayOfTheMonth,
	/// 7th day of the month
	#[serde(rename = "7")]
	N7ThDayOfTheMonth,
	/// 8th day of the month
	#[serde(rename = "8")]
	N8ThDayOfTheMonth,
	/// 9th day of the month
	#[serde(rename = "9")]
	N9ThDayOfTheMonth,
	/// 10th day of the month
	#[serde(rename = "10")]
	N10ThDayOfTheMonth,
	/// 11th day of the month
	#[serde(rename = "11")]
	N11ThDayOfTheMonth,
	/// 12th day of the month
	#[serde(rename = "12")]
	N12ThDayOfTheMonth,
	/// 13th day of the month
	#[serde(rename = "13")]
	N13ThDayOfTheMonth,
	/// 14th day of the month
	#[serde(rename = "14")]
	N14ThDayOfTheMonth,
	/// 15th day of the month
	#[serde(rename = "15")]
	N15ThDayOfTheMonth,
	/// 16th day of the month
	#[serde(rename = "16")]
	N16ThDayOfTheMonth,
	/// 17th day of the month
	#[serde(rename = "17")]
	N17ThDayOfTheMonth,
	/// 18th day of the month
	#[serde(rename = "18")]
	N18ThDayOfTheMonth,
	/// 19th day of the month
	#[serde(rename = "19")]
	N19ThDayOfTheMonth,
	/// 20th day of the month
	#[serde(rename = "20")]
	N20ThDayOfTheMonth,
	/// 21st day of the month
	#[serde(rename = "21")]
	N21StDayOfTheMonth,
	/// 22nd day of the month
	#[serde(rename = "22")]
	N22NdDayOfTheMonth,
	/// 23rd day of the month
	#[serde(rename = "23")]
	N23RdDayOfTheMonth,
	/// 24th day of the month
	#[serde(rename = "24")]
	N24ThDayOfTheMonth,
	/// 25th day of the month
	#[serde(rename = "25")]
	N25ThDayOfTheMonth,
	/// 26th day of the month
	#[serde(rename = "26")]
	N26ThDayOfTheMonth,
	/// 27th day of the month
	#[serde(rename = "27")]
	N27ThDayOfTheMonth,
	/// 28th day of the month
	#[serde(rename = "28")]
	N28ThDayOfTheMonth,
	/// 29th day of the month
	#[serde(rename = "29")]
	N29ThDayOfTheMonth,
	/// 30th day of the month
	#[serde(rename = "30")]
	N30ThDayOfTheMonth,
	/// The end of the month.
	#[serde(rename = "EOM")]
	TheEndOfTheMonth,
	/// The floating rate note convention or Eurodollar convention.
	#[serde(rename = "FRN")]
	TheFloatingRateNoteConventionOrEurodollarConvention,
	/// The International Money Market settlement date, i.e. the 3rd Wednesday of the month.
	#[serde(rename = "IMM")]
	TheInternationalMoneyMarketSettlementDateIEThe3RdWednesdayOfTheMonth,
	/// The last trading day/expiration day of the Canadian Derivatives Exchange.
	#[serde(rename = "IMMCAD")]
	TheLastTradingDayExpirationDayOfTheCanadianDerivativesExchange,
	/// The last trading day of the Sydney Futures Exchange Australian 90-day bank accepted bill futures contract.
	#[serde(rename = "IMMAUD")]
	TheLastTradingDayOfTheSydneyFuturesExchangeAustralian90DayBankAcceptedBillFuturesContract,
	/// The last trading day of the Sydney Futures Exchange New Zealand 90-day bank bill futures contract.
	#[serde(rename = "IMMNZD")]
	TheLastTradingDayOfTheSydneyFuturesExchangeNewZealand90DayBankBillFuturesContract,
	/// The Sydney Futures Exchange 90-day bank accepted bill futures settlement dates.
	#[serde(rename = "SFE")]
	TheSydneyFuturesExchange90DayBankAcceptedBillFuturesSettlementDates,
	/// No adjustment
	#[serde(rename = "NONE")]
	NoAdjustment,
	/// The 13-week and 26-week U.S. Treasury Bill auction dates.
	#[serde(rename = "TBILL")]
	The13WeekAnd26WeekUSTreasuryBillAuctionDates,
	/// Monday
	#[serde(rename = "MON")]
	Monday,
	/// Tuesday
	#[serde(rename = "TUE")]
	Tuesday,
	/// Wednesday
	#[serde(rename = "WED")]
	Wednesday,
	/// Thursday
	#[serde(rename = "THU")]
	Thursday,
	/// Friday
	#[serde(rename = "FRI")]
	Friday,
	/// Saturday
	#[serde(rename = "SAT")]
	Saturday,
	/// Sunday
	#[serde(rename = "SUN")]
	Sunday,
}

impl Default for LegOptionExerciseExpirationRollConvention {
	fn default() -> Self {
		LegOptionExerciseExpirationRollConvention::N1StDayOfTheMonth
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegOptionExerciseExpirationDateOffsetDayType {
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

impl Default for LegOptionExerciseExpirationDateOffsetDayType {
	fn default() -> Self {
		LegOptionExerciseExpirationDateOffsetDayType::Business
	}
}
