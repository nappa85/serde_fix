
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStipulations {
	/// NoUnderlyingStips
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "887")]
	pub underlying_stips: Option<fix_common::RepeatingValues<UnderlyingStip>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStip {
	/// Required if <a href="tag_887_NoUnderlyingStips.html" target="bottom">NoUnderlyingStips&nbsp;(887)</a> &gt;0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "888")]
	pub underlying_stip_type: Option<UnderlyingStipType>,
	/// UnderlyingStipValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "889")]
	pub underlying_stip_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStipType {
	/// Alternative Minimum Tax (Y/N)
	#[serde(rename = "AMT")]
	AlternativeMinimumTax,
	/// Auto Reinvestment at &lt;rate&gt; or better
	#[serde(rename = "AUTOREINV")]
	AutoReinvestmentAtLtRateGtOrBetter,
	/// Bank qualified (Y/N)
	#[serde(rename = "BANKQUAL")]
	BankQualified,
	/// Bargain conditions (see <a href="tag_234_StipulationValue.html" target="bottom">StipulationValue&nbsp;(234)</a> for values)
	#[serde(rename = "BGNCON")]
	BargainConditionsAForValues,
	/// Coupon range
	#[serde(rename = "COUPON")]
	CouponRange,
	/// ISO Currency Code
	#[serde(rename = "CURRENCY")]
	IsoCurrencyCode,
	/// Custom start/end date
	#[serde(rename = "CUSTOMDATE")]
	CustomStartEndDate,
	/// Geographics and % range (ex. 234=CA 0-80 [minimum of 80% California assets])
	#[serde(rename = "GEOG")]
	GeographicsAndRange,
	/// Valuation Discount
	#[serde(rename = "HAIRCUT")]
	ValuationDiscount,
	/// Insured (Y/N)
	#[serde(rename = "INSURED")]
	Insured,
	/// Year Or Year/Month of Issue (ex. 234=2002/09)
	#[serde(rename = "ISSUE")]
	YearOrYearMonthOfIssue,
	/// Issuer's ticker
	#[serde(rename = "ISSUER")]
	IssuerSTicker,
	/// issue size range
	#[serde(rename = "ISSUESIZE")]
	IssueSizeRange,
	/// Lookback Days
	#[serde(rename = "LOOKBACK")]
	LookbackDays,
	/// Explicit lot identifier
	#[serde(rename = "LOT")]
	ExplicitLotIdentifier,
	/// Lot Variance (value in percent maximum over- or under-allocation allowed)
	#[serde(rename = "LOTVAR")]
	LotVariance,
	/// Maturity Year And Month
	#[serde(rename = "MAT")]
	MaturityYearAndMonth,
	/// Maturity range
	#[serde(rename = "MATURITY")]
	MaturityRange,
	/// Maximum substitutions (Repo)
	#[serde(rename = "MAXSUBS")]
	MaximumSubstitutions,
	/// Minimum denomination
	#[serde(rename = "MINDNOM")]
	MinimumDenomination,
	/// Minimum increment
	#[serde(rename = "MININCR")]
	MinimumIncrement,
	/// Minimum quantity
	#[serde(rename = "MINQTY")]
	MinimumQuantity,
	/// Payment frequency, calendar
	#[serde(rename = "PAYFREQ")]
	PaymentFrequencyCalendar,
	/// Number Of Pieces
	#[serde(rename = "PIECES")]
	NumberOfPieces,
	/// Pools Maximum
	#[serde(rename = "PMAX")]
	PoolsMaximum,
	/// Pools per Lot
	#[serde(rename = "PPL")]
	PoolsPerLot,
	/// Pools per Million
	#[serde(rename = "PPM")]
	PoolsPerMillion,
	/// Pools per Trade
	#[serde(rename = "PPT")]
	PoolsPerTrade,
	/// Price Range
	#[serde(rename = "PRICE")]
	PriceRange,
	/// Pricing frequency
	#[serde(rename = "PRICEFREQ")]
	PricingFrequency,
	/// Production Year
	#[serde(rename = "PROD")]
	ProductionYear,
	/// Call protection
	#[serde(rename = "PROTECT")]
	CallProtection,
	/// Purpose
	#[serde(rename = "PURPOSE")]
	Purpose,
	/// Benchmark price source
	#[serde(rename = "PXSOURCE")]
	BenchmarkPriceSource,
	/// Rating source and range
	#[serde(rename = "RATING")]
	RatingSourceAndRange,
	/// Type Of Redemption - values are: NonCallable, Prefunded, EscrowedToMaturity, Putable, Convertible
	#[serde(rename = "REDEMPTION")]
	TypeOfRedemptionValuesAreNonCallablePrefundedEscrowedToMaturityPutableConvertible,
	/// Restricted (Y/N)
	#[serde(rename = "RESTRICTED")]
	Restricted,
	/// Market Sector
	#[serde(rename = "SECTOR")]
	MarketSector,
	/// Security Type included or excluded
	#[serde(rename = "SECTYPE")]
	SecurityTypeIncludedOrExcluded,
	/// Structure
	#[serde(rename = "STRUCT")]
	Structure,
	/// Substitutions frequency (Repo)
	#[serde(rename = "SUBSFREQ")]
	SubstitutionsFrequency,
	/// Substitutions left (Repo)
	#[serde(rename = "SUBSLEFT")]
	SubstitutionsLeft,
	/// Freeform Text
	#[serde(rename = "TEXT")]
	FreeformText,
	/// Trade Variance (value in percent maximum over- or under-allocation allowed)
	#[serde(rename = "TRDVAR")]
	TradeVariance,
	/// Weighted Average Coupon - value in percent (exact or range) plus "Gross" or "Net" of servicing spread (the default) (ex. 234=6.5-Net
	/// [minimum of 6.5% net of servicing fee])"
	#[serde(rename = "WAC")]
	WeightedAverageCouponValueInPercentPlusGrossOrNetOfServicingSpread,
	/// Weighted Average Life Coupon - value in percent (exact or range)
	#[serde(rename = "WAL")]
	WeightedAverageLifeCouponValueInPercent,
	/// Weighted Average Loan Age - value in months (exact or range)
	#[serde(rename = "WALA")]
	WeightedAverageLoanAgeValueInMonths,
	/// Weighted Average Maturity - value in months (exact or range)
	#[serde(rename = "WAM")]
	WeightedAverageMaturityValueInMonths,
	/// Whole Pool (Y/N)
	#[serde(rename = "WHOLE")]
	WholePool,
	/// Yield Range
	#[serde(rename = "YIELD")]
	YieldRange,
	/// Absolute Prepayment Speed
	#[serde(rename = "ABS")]
	AbsolutePrepaymentSpeed,
	/// Constant Prepayment Penalty
	#[serde(rename = "CPP")]
	ConstantPrepaymentPenalty,
	/// Constant Prepayment Rate
	#[serde(rename = "CPR")]
	ConstantPrepaymentRate,
	/// Constant Prepayment Yield
	#[serde(rename = "CPY")]
	ConstantPrepaymentYield,
	/// Final CPR of Home Equity Prepayment Curve
	#[serde(rename = "HEP")]
	FinalCprOfHomeEquityPrepaymentCurve,
	/// Percent of Manufactured Housing Prepayment Curve
	#[serde(rename = "MHP")]
	PercentOfManufacturedHousingPrepaymentCurve,
	/// Monthly Prepayment Rate
	#[serde(rename = "MPR")]
	MonthlyPrepaymentRate,
	/// Percent of Prospectus Prepayment Curve
	#[serde(rename = "PPC")]
	PercentOfProspectusPrepaymentCurve,
	/// Percent of BMA Prepayment Curve
	#[serde(rename = "PSA")]
	PercentOfBmaPrepaymentCurve,
	/// Single Monthly Mortality
	#[serde(rename = "SMM")]
	SingleMonthlyMortality,
}

impl Default for UnderlyingStipType {
	fn default() -> Self {
		UnderlyingStipType::AlternativeMinimumTax
	}
}