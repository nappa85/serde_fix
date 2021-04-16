
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCalculationPeriodDates {
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the calculation period dates of the stream.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40073")]
	pub stream_calculation_period_business_day_convention: Option<StreamCalculationPeriodBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the calculation period dates of the stream.
	#[serde(flatten)]
	pub stream_calculation_period_business_center_grp: Option<super::stream_calculation_period_business_center_grp::StreamCalculationPeriodBusinessCenterGrp>,
	/// StreamFirstPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40075")]
	pub stream_first_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the calculation period dates of the stream.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40076")]
	pub stream_first_period_start_date_business_day_convention: Option<StreamFirstPeriodStartDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the calculation period dates of the stream.
	#[serde(flatten)]
	pub stream_first_period_start_date_business_center_grp: Option<super::stream_first_period_start_date_business_center_grp::StreamFirstPeriodStartDateBusinessCenterGrp>,
	/// StreamFirstPeriodStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40078")]
	pub stream_first_period_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// StreamFirstRegularPeriodStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40079")]
	pub stream_first_regular_period_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// StreamFirstCompoundingPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40080")]
	pub stream_first_compounding_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// StreamLastRegularPeriodEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40081")]
	pub stream_last_regular_period_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when StreamCalculationFrequencyUnit(40083) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40082")]
	pub stream_calculation_frequency_period: Option<i32>,
	/// Conditionally required when StreamCalculationFrequencyPeriod(40082) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40083")]
	pub stream_calculation_frequency_unit: Option<StreamCalculationFrequencyUnit>,
	/// StreamCalculationRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40084")]
	pub stream_calculation_roll_convention: Option<StreamCalculationRollConvention>,
	/// StreamCalculationPeriodDatesXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41244")]
	pub stream_calculation_period_dates_xid: Option<String>,
	/// StreamCalculationPeriodDatesXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41245")]
	pub stream_calculation_period_dates_xid_ref: Option<String>,
	/// StreamCalculationPeriodDateGrp
	#[serde(flatten)]
	pub stream_calculation_period_date_grp: Option<super::stream_calculation_period_date_grp::StreamCalculationPeriodDateGrp>,
	/// StreamCalculationBalanceOfFirstPeriod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41246")]
	pub stream_calculation_balance_of_first_period: Option<fix_common::Boolean>,
	/// Conditionally required when StreamCalculationCorrectionUnit(41248) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41247")]
	pub stream_calculation_correction_period: Option<i32>,
	/// Conditionally required when StreamCalculationCorrectionPeriod(41247) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41248")]
	pub stream_calculation_correction_unit: Option<StreamCalculationCorrectionUnit>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StreamCalculationPeriodBusinessDayConvention {
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

impl Default for StreamCalculationPeriodBusinessDayConvention {
	fn default() -> Self {
		StreamCalculationPeriodBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StreamFirstPeriodStartDateBusinessDayConvention {
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

impl Default for StreamFirstPeriodStartDateBusinessDayConvention {
	fn default() -> Self {
		StreamFirstPeriodStartDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StreamCalculationFrequencyUnit {
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

impl Default for StreamCalculationFrequencyUnit {
	fn default() -> Self {
		StreamCalculationFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StreamCalculationRollConvention {
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

impl Default for StreamCalculationRollConvention {
	fn default() -> Self {
		StreamCalculationRollConvention::N1StDayOfTheMonth
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StreamCalculationCorrectionUnit {
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

impl Default for StreamCalculationCorrectionUnit {
	fn default() -> Self {
		StreamCalculationCorrectionUnit::Day
	}
}
