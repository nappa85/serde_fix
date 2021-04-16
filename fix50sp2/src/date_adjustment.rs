
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DateAdjustment {
	/// BusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40921")]
	pub business_day_convention: Option<BusinessDayConvention>,
	/// BusinessCenterGrp
	#[serde(flatten)]
	pub business_center_grp: Option<super::business_center_grp::BusinessCenterGrp>,
	/// DateRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40922")]
	pub date_roll_convention: Option<DateRollConvention>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum BusinessDayConvention {
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

impl Default for BusinessDayConvention {
	fn default() -> Self {
		BusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DateRollConvention {
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

impl Default for DateRollConvention {
	fn default() -> Self {
		DateRollConvention::N1StDayOfTheMonth
	}
}
