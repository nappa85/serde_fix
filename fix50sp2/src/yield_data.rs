
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct YieldData {
	/// YieldType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "235")]
	pub yield_type: Option<YieldType>,
	/// Yield
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "236")]
	pub _yield: Option<f32>,
	/// YieldCalcDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "701")]
	pub yield_calc_date: Option<fix_common::LocalMktDate>,
	/// YieldRedemptionDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "696")]
	pub yield_redemption_date: Option<fix_common::LocalMktDate>,
	/// YieldRedemptionPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "697")]
	pub yield_redemption_price: Option<f64>,
	/// YieldRedemptionPriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "698")]
	pub yield_redemption_price_type: Option<YieldRedemptionPriceType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum YieldType {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum YieldRedemptionPriceType {
	/// Percentage (e.g. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Per unit (i.e. per share or contract)
	#[serde(rename = "2")]
	PerUnit,
	/// Fixed Amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
	/// Discount - percentage points below par
	#[serde(rename = "4")]
	DiscountPercentagePointsBelowPar,
	/// Premium - percentage points over par
	#[serde(rename = "5")]
	PremiumPercentagePointsOverPar,
	/// Spread
	#[serde(rename = "6")]
	Spread,
	/// TED price
	#[serde(rename = "7")]
	TedPrice,
	/// TED yield
	#[serde(rename = "8")]
	TedYield,
	/// Yield
	#[serde(rename = "9")]
	Yield,
	/// Fixed cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "10")]
	FixedCabinetTradePrice,
	/// Variable cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "11")]
	VariableCabinetTradePrice,
	/// Price spread
	#[serde(rename = "12")]
	PriceSpread,
	/// Product ticks in halves
	#[serde(rename = "13")]
	ProductTicksInHalves,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eighths
	#[serde(rename = "15")]
	ProductTicksInEighths,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-fourths
	#[serde(rename = "18")]
	ProductTicksInSixtyFourths,
	/// Product ticks in one-twenty-eighths
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEighths,
	/// Normal rate representation (e.g. FX rate) (represents the number of units of currency 2 that is required to purchase one unit
	/// of currency 1.)
	#[serde(rename = "20")]
	NormalRateRepresentation,
	/// Inverse rate representation (e.g. FX rate) (represents the number of units of 1 that is needed to purchase one unit of currency
	/// 2.)
	#[serde(rename = "21")]
	InverseRateRepresentation,
	/// Basis points (when the price is not spread based)
	#[serde(rename = "22")]
	BasisPoints,
	/// Upfront points (used specifically for CDS pricing)
	#[serde(rename = "23")]
	UpfrontPoints,
	/// Interest rate (Elaboration: When the price is an interest rate. For example, used with benchmark reference rate)
	#[serde(rename = "24")]
	InterestRate,
	/// Percentage of notional
	#[serde(rename = "25")]
	PercentageOfNotional,
}
