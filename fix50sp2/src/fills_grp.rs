
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FillsGrp {
	/// Specifies the number of partial fills included in this Execution Report.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1362")]
	pub fills: Option<fix_common::RepeatingValues<Fill>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Fill {
	/// Unique identifier of execution as assigned by sell-side (broker, exchange, ECN). Must not overlap ExecID(17). Required if
	/// NoFills(1362) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1363")]
	pub fill_exec_id: Option<String>,
	/// Price of this partial fill. Required if NoFills(1362) &gt; 0. Refer to LastPx(31).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1364")]
	pub fill_px: Option<f64>,
	/// Quantity (e.g. shares) bought/sold on this partial fill. Required if NoFills(1362) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1365")]
	pub fill_qty: Option<f64>,
	/// FillLiquidityInd
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1443")]
	pub fill_liquidity_ind: Option<FillLiquidityInd>,
	/// FillYieldType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1622")]
	pub fill_yield_type: Option<FillYieldType>,
	/// FillYield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1623")]
	pub fill_yield: Option<f32>,
	/// Can be used to refer to the related match event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2673")]
	pub fill_match_id: Option<String>,
	/// Can be used to refer to a price level (e.g. match step, clip) within the related match event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2674")]
	pub fill_match_sub_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FillLiquidityInd {
	/// Added Liquidity
	#[serde(rename = "1")]
	AddedLiquidity,
	/// Removed Liquidity
	#[serde(rename = "2")]
	RemovedLiquidity,
	/// Liquidity Routed Out
	#[serde(rename = "3")]
	LiquidityRoutedOut,
	/// Auction
	#[serde(rename = "4")]
	Auction,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FillYieldType {
	/// After Tax Yield (Municipals)
	#[serde(rename = "AFTERTAX")]
	AfterTaxYield,
	/// Annual Yield
	#[serde(rename = "ANNUAL")]
	AnnualYield,
	/// Yield At Issue (Municipals)
	#[serde(rename = "ATISSUE")]
	YieldAtIssue,
	/// Yield To Avg Maturity
	#[serde(rename = "AVGMATURITY")]
	YieldToAvgMaturity,
	/// Book Yield
	#[serde(rename = "BOOK")]
	BookYield,
	/// Yield to Next Call
	#[serde(rename = "CALL")]
	YieldToNextCall,
	/// Yield Change Since Close
	#[serde(rename = "CHANGE")]
	YieldChangeSinceClose,
	/// Closing Yield
	#[serde(rename = "CLOSE")]
	ClosingYield,
	/// Compound Yield
	#[serde(rename = "COMPOUND")]
	CompoundYield,
	/// Current Yield
	#[serde(rename = "CURRENT")]
	CurrentYield,
	/// Gvnt Equivalent Yield
	#[serde(rename = "GOVTEQUIV")]
	GvntEquivalentYield,
	/// True Gross Yield
	#[serde(rename = "GROSS")]
	TrueGrossYield,
	/// Yield with Inflation Assumption
	#[serde(rename = "INFLATION")]
	YieldWithInflationAssumption,
	/// Inverse Floater Bond Yield
	#[serde(rename = "INVERSEFLOATER")]
	InverseFloaterBondYield,
	/// Most Recent Closing Yield
	#[serde(rename = "LASTCLOSE")]
	MostRecentClosingYield,
	/// Closing Yield Most Recent Month
	#[serde(rename = "LASTMONTH")]
	ClosingYieldMostRecentMonth,
	/// Closing Yield Most Recent Quarter
	#[serde(rename = "LASTQUARTER")]
	ClosingYieldMostRecentQuarter,
	/// Closing Yield Most Recent Year
	#[serde(rename = "LASTYEAR")]
	ClosingYieldMostRecentYear,
	/// Yield to Longest Average Life
	#[serde(rename = "LONGAVGLIFE")]
	YieldToLongestAverageLife,
	/// Mark to Market Yield
	#[serde(rename = "MARK")]
	MarkToMarketYield,
	/// Yield to Maturity
	#[serde(rename = "MATURITY")]
	YieldToMaturity,
	/// Yield to Next Refund (Sinking Fund Bonds)
	#[serde(rename = "NEXTREFUND")]
	YieldToNextRefund,
	/// Open Average Yield
	#[serde(rename = "OPENAVG")]
	OpenAverageYield,
	/// Previous Close Yield
	#[serde(rename = "PREVCLOSE")]
	PreviousCloseYield,
	/// Proceeds Yield
	#[serde(rename = "PROCEEDS")]
	ProceedsYield,
	/// Yield to Next Put
	#[serde(rename = "PUT")]
	YieldToNextPut,
	/// Semi-annual Yield
	#[serde(rename = "SEMIANNUAL")]
	SemiAnnualYield,
	/// Yield to Shortest Average Life
	#[serde(rename = "SHORTAVGLIFE")]
	YieldToShortestAverageLife,
	/// Simple Yield
	#[serde(rename = "SIMPLE")]
	SimpleYield,
	/// Tax Equivalent Yield
	#[serde(rename = "TAXEQUIV")]
	TaxEquivalentYield,
	/// Yield to Tender Date
	#[serde(rename = "TENDER")]
	YieldToTenderDate,
	/// True Yield
	#[serde(rename = "TRUE")]
	TrueYield,
	/// Yield Value Of 1/32
	#[serde(rename = "VALUE1_32")]
	YieldValueOf132,
	/// Yield To Worst
	#[serde(rename = "WORST")]
	YieldToWorst,
}
