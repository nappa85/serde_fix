
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventScheduleGrp {
	/// NoUnderlyingComplexEventSchedules
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41750")]
	pub underlying_complex_event_schedules: Option<fix_common::RepeatingValues<UnderlyingComplexEventSchedule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventSchedule {
	/// Required if NoUnderlyingComplexEventSchedules(41750) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41751")]
	pub underlying_complex_event_schedule_start_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingComplexEventScheduleEndDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41752")]
	pub underlying_complex_event_schedule_end_date: Option<fix_common::LocalMktDate>,
	/// Conditionally required when UnderlyingComplexEventScheduleFrequencyUnit(41754) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41753")]
	pub underlying_complex_event_schedule_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingComplexEventScheduleFrequencyPeriod(41753) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41754")]
	pub underlying_complex_event_schedule_frequency_unit: Option<UnderlyingComplexEventScheduleFrequencyUnit>,
	/// UnderlyingComplexEventScheduleRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41755")]
	pub underlying_complex_event_schedule_roll_convention: Option<UnderlyingComplexEventScheduleRollConvention>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingComplexEventScheduleFrequencyUnit {
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

impl Default for UnderlyingComplexEventScheduleFrequencyUnit {
	fn default() -> Self {
		UnderlyingComplexEventScheduleFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingComplexEventScheduleRollConvention {
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

impl Default for UnderlyingComplexEventScheduleRollConvention {
	fn default() -> Self {
		UnderlyingComplexEventScheduleRollConvention::N1StDayOfTheMonth
	}
}
