
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCalculationPeriodDates {
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg stream calculation period dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40265")]
	pub leg_stream_calculation_period_business_day_convention: Option<LegStreamCalculationPeriodBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg stream calculation period dates.
	#[serde(flatten)]
	pub leg_stream_calculation_period_business_center_grp: Option<super::leg_stream_calculation_period_business_center_grp::LegStreamCalculationPeriodBusinessCenterGrp>,
	/// LegStreamFirstPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40267")]
	pub leg_stream_first_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg stream calculation period dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40268")]
	pub leg_stream_first_period_start_date_business_day_convention: Option<LegStreamFirstPeriodStartDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg stream calculation period dates.
	#[serde(flatten)]
	pub leg_stream_first_period_start_date_business_center_grp: Option<super::leg_stream_first_period_start_date_business_center_grp::LegStreamFirstPeriodStartDateBusinessCenterGrp>,
	/// LegStreamFirstPeriodStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40270")]
	pub leg_stream_first_period_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// LegStreamFirstRegularPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40271")]
	pub leg_stream_first_regular_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegStreamFirstCompoundingPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40272")]
	pub leg_stream_first_compounding_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegStreamLastRegularPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40273")]
	pub leg_stream_last_regular_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when LegStreamCalculationFrequencyUnit(40275) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40274")]
	pub leg_stream_calculation_frequency_period: Option<i32>,
	/// Conditionally required when LegStreamCalculationFrequencyPeriod(40274) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40275")]
	pub leg_stream_calculation_frequency_unit: Option<LegStreamCalculationFrequencyUnit>,
	/// LegStreamCalculationRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40276")]
	pub leg_stream_calculation_roll_convention: Option<LegStreamCalculationRollConvention>,
	/// LegStreamCalculationPeriodDatesXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41641")]
	pub leg_stream_calculation_period_dates_xid: Option<String>,
	/// LegStreamCalculationPeriodDatesXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41642")]
	pub leg_stream_calculation_period_dates_xid_ref: Option<String>,
	/// LegStreamCalculationPeriodDateGrp
	#[serde(flatten)]
	pub leg_stream_calculation_period_date_grp: Option<super::leg_stream_calculation_period_date_grp::LegStreamCalculationPeriodDateGrp>,
	/// LegStreamCalculationBalanceOfFirstPeriod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41643")]
	pub leg_stream_calculation_balance_of_first_period: Option<fix_common::Boolean>,
	/// Conditionally required when LegStreamCalculationCorrectionUnit(41645) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41644")]
	pub leg_stream_calculation_correction_period: Option<i32>,
	/// Conditionally required when LegStreamCalculationCorrectionPeriod(41644) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41645")]
	pub leg_stream_calculation_correction_unit: Option<LegStreamCalculationCorrectionUnit>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegStreamCalculationPeriodBusinessDayConvention {
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

impl Default for LegStreamCalculationPeriodBusinessDayConvention {
	fn default() -> Self {
		LegStreamCalculationPeriodBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegStreamFirstPeriodStartDateBusinessDayConvention {
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

impl Default for LegStreamFirstPeriodStartDateBusinessDayConvention {
	fn default() -> Self {
		LegStreamFirstPeriodStartDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegStreamCalculationFrequencyUnit {
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
	/// Hour
	#[serde(rename = "H")]
	Hour,
	/// Minute
	#[serde(rename = "Min")]
	Minute,
	/// Second
	#[serde(rename = "S")]
	Second,
	/// Term
	#[serde(rename = "T")]
	Term,
}

impl Default for LegStreamCalculationFrequencyUnit {
	fn default() -> Self {
		LegStreamCalculationFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegStreamCalculationRollConvention {
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

impl Default for LegStreamCalculationRollConvention {
	fn default() -> Self {
		LegStreamCalculationRollConvention::N1StDayOfTheMonth
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegStreamCalculationCorrectionUnit {
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

impl Default for LegStreamCalculationCorrectionUnit {
	fn default() -> Self {
		LegStreamCalculationCorrectionUnit::Day
	}
}
