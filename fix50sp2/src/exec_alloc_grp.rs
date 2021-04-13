
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecAllocGrp {
	/// Indicates number of individual execution or trade entries. Absence indicates that no individual execution or trade entries
	/// are included. Primarily used to support stepouts.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "124")]
	pub execs: Option<fix_common::RepeatingValues<Exec>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Exec {
	/// Amount of quantity (e.g. number of shares) in individual execution. Required if NoExecs &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// ExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "17")]
	pub exec_id: Option<String>,
	/// SecondaryExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "527")]
	pub secondary_exec_id: Option<String>,
	/// Price of individual execution. Required if NoExecs &gt; 0.For FX, if specified, expressed in terms of <a href="tag_15_Currency.html" target="bottom">Currency(15)&nbsp;(15)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// Last price expressed in percent-of-par. Conditionally required for Fixed Income trades when LastPx is expressed in Yield,
	/// Spread, Discount or any other price type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "669")]
	pub last_par_px: Option<f64>,
	/// Used to identify whether the trade was executed on an agency or principal basis.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "29")]
	pub last_capacity: Option<LastCapacity>,
	/// TradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// FirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1041")]
	pub firm_trade_id: Option<String>,
	/// ExecutionTimestamp
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2749")]
	pub execution_timestamp: Option<fix_common::UTCTimestamp>,
	/// TradeReportingIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2524")]
	pub trade_reporting_indicator: Option<TradeReportingIndicator>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastCapacity {
	/// Agent
	#[serde(rename = "1")]
	Agent,
	/// Cross as agent
	#[serde(rename = "2")]
	CrossAsAgent,
	/// Cross as principal
	#[serde(rename = "3")]
	CrossAsPrincipal,
	/// Principal
	#[serde(rename = "4")]
	Principal,
	/// Riskless principal
	#[serde(rename = "5")]
	RisklessPrincipal,
}

impl Default for LastCapacity {
	fn default() -> Self {
		LastCapacity::Agent
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeReportingIndicator {
	/// Trade has not (yet) been reported
	#[serde(rename = "0")]
	TradeHasNotBeenReported,
	/// Trade has been or will be reported by a trading venue as an "on-book" trade
	#[serde(rename = "1")]
	TradeHasBeenOrWillBeReportedByATradingVenueAsAnOnBookTrade,
	/// Trade has been or will be reported as a "systematic internaliser" seller trade
	#[serde(rename = "2")]
	TradeHasBeenOrWillBeReportedAsASystematicInternaliserSellerTrade,
	/// Trade has been or will be reported as a "systematic internaliser" buyer trade
	#[serde(rename = "3")]
	TradeHasBeenOrWillBeReportedAsASystematicInternaliserBuyerTrade,
	/// Trade has been or will be reported as a "non-systematic internaliser" seller trade
	#[serde(rename = "4")]
	TradeHasBeenOrWillBeReportedAsANonSystematicInternaliserSellerTrade,
	/// Trade has been or will be reported under a sub-delegation arrangement by an investment firm to a reporting facility (e.g.
	/// APA) on behalf of another investment firm
	#[serde(rename = "5")]
	TradeHasBeenOrWillBeReportedUnderASubDelegationArrangementByAnInvestmentFirmToAReportingFacilityOnBehalfOfAnotherInvestmentFirm,
	/// Trade has been or will be reported
	#[serde(rename = "6")]
	TradeHasBeenOrWillBeReported,
	/// Trade has been or will be reported as a "non-Systematic Internaliser" buyer trade
	#[serde(rename = "7")]
	TradeHasBeenOrWillBeReportedAsANonSystematicInternaliserBuyerTrade,
	/// Trade has been or will be reported by a trading venue as an "off-book" trade
	#[serde(rename = "8")]
	TradeHasBeenOrWillBeReportedByATradingVenueAsAnOffBookTrade,
	/// Trade is not reportable
	#[serde(rename = "9")]
	TradeIsNotReportable,
}

impl Default for TradeReportingIndicator {
	fn default() -> Self {
		TradeReportingIndicator::TradeHasNotBeenReported
	}
}
