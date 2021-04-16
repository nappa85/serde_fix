
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Stipulations {
	/// NoStipulations
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "232")]
	pub stipulations: Option<fix_common::RepeatingValues<Stipulation>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Stipulation {
	/// Required if <a href="tag_232_NoStipulations.html" target="bottom">NoStipulations&nbsp;(232)</a> &gt;0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "233")]
	pub stipulation_type: Option<StipulationType>,
	/// StipulationValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "234")]
	pub stipulation_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StipulationType {
	/// AMT (y/n)
	#[serde(rename = "AMT")]
	Amt,
	/// Auto Reinvestment at &lt;rate&gt; or better
	#[serde(rename = "AUTOREINV")]
	AutoReinvestmentAtLtRateGtOrBetter,
	/// Bank qualified (y/n)
	#[serde(rename = "BANKQUAL")]
	BankQualified,
	/// Bargain Conditions- see (234) for values
	#[serde(rename = "BGNCON")]
	BargainConditionsSeeForValues,
	/// Coupon range
	#[serde(rename = "COUPON")]
	CouponRange,
	/// ISO Currency code
	#[serde(rename = "CURRENCY")]
	IsoCurrencyCode,
	/// Geographics and % Range (ex. 234=CA 0-80 [minimum of 80% California assets])
	#[serde(rename = "GEOG")]
	GeographicsAndRange,
	/// Valuation discount
	#[serde(rename = "HAIRCUT")]
	ValuationDiscount,
	/// Insured (y/n)
	#[serde(rename = "INSURED")]
	Insured,
	/// Year or Year/Month of Issue (ex. 234=2002/09)
	#[serde(rename = "ISSUE")]
	YearOrYearMonthOfIssue,
	/// Issuer's ticker
	#[serde(rename = "ISSUER")]
	IssuerSTicker,
	/// Issue size range
	#[serde(rename = "ISSUESIZE")]
	IssueSizeRange,
	/// Lookback days
	#[serde(rename = "LOOKBACK")]
	LookbackDays,
	/// Explicit lot identifier
	#[serde(rename = "LOT")]
	ExplicitLotIdentifier,
	/// Lot Variance (value in percent maximum over- or under-allocation allowed)
	#[serde(rename = "LOTVAR")]
	LotVariance,
	/// Maturity Year and Month
	#[serde(rename = "MAT")]
	MaturityYearAndMonth,
	/// Maturity range
	#[serde(rename = "MATURITY")]
	MaturityRange,
	/// Maximum substitutions (Repo)
	#[serde(rename = "MAXSUBS")]
	MaximumSubstitutions,
	/// Minimum quantity
	#[serde(rename = "MINQTY")]
	MinimumQuantity,
	/// Minimum increment
	#[serde(rename = "MININCR")]
	MinimumIncrement,
	/// Minimum denomination
	#[serde(rename = "MINDNOM")]
	MinimumDenomination,
	/// Payment frequency, calendar
	#[serde(rename = "PAYFREQ")]
	PaymentFrequencyCalendar,
	/// Number of Pieces
	#[serde(rename = "PIECES")]
	NumberOfPieces,
	/// Pools Maximum
	#[serde(rename = "PMAX")]
	PoolsMaximum,
	/// Pools per Million
	#[serde(rename = "PPM")]
	PoolsPerMillion,
	/// Pools per Lot
	#[serde(rename = "PPL")]
	PoolsPerLot,
	/// Pools per Trade
	#[serde(rename = "PPT")]
	PoolsPerTrade,
	/// Price range
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
	/// Type of redemption - values are: NonCallable, Callable, Prefunded, EscrowedToMaturity, Putable, Convertible
	#[serde(rename = "REDEMPTION")]
	TypeOfRedemptionValuesAreNonCallableCallablePrefundedEscrowedToMaturityPutableConvertible,
	/// Restricted (y/n)
	#[serde(rename = "RESTRICTED")]
	Restricted,
	/// Market sector
	#[serde(rename = "SECTOR")]
	MarketSector,
	/// <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> included or excluded
	#[serde(rename = "SECTYPE")]
	AHrefTag167SecurityTypeHtmlTargetBottomSecurityTypeNbspAIncludedOrExcluded,
	/// Structure
	#[serde(rename = "STRUCT")]
	Structure,
	/// Substitutions frequency (Repo)
	#[serde(rename = "SUBSFREQ")]
	SubstitutionsFrequency,
	/// Substitutions left (Repo)
	#[serde(rename = "SUBSLEFT")]
	SubstitutionsLeft,
	/// Freeform text
	#[serde(rename = "TEXT")]
	FreeformText,
	/// Trade Variance (value in percent maximum over- or under-allocation allowed)
	#[serde(rename = "TRDVAR")]
	TradeVariance,
	/// Weighted Average Coupon:value in percent (exact or range) plus 'Gross' or 'Net' of servicing spread (the default) (ex. 234=6.5-
	/// Net [minimum of 6.5% net of servicing fee])
	#[serde(rename = "WAC")]
	WeightedAverageCouponValueInPercentPlusGrossOrNetOfServicingSpread,
	/// Weighted Average Life Coupon: value in percent (exact or range)
	#[serde(rename = "WAL")]
	WeightedAverageLifeCouponValueInPercent,
	/// Weighted Average Loan Age: value in months (exact or range)
	#[serde(rename = "WALA")]
	WeightedAverageLoanAgeValueInMonths,
	/// Weighted Average Maturity : value in months (exact or range)
	#[serde(rename = "WAM")]
	WeightedAverageMaturityValueInMonths,
	/// Whole Pool (y/n)
	#[serde(rename = "WHOLE")]
	WholePool,
	/// Yield range
	#[serde(rename = "YIELD")]
	YieldRange,
	/// Single Monthly Mortality
	#[serde(rename = "SMM")]
	SingleMonthlyMortality,
	/// Constant Prepayment Rate
	#[serde(rename = "CPR")]
	ConstantPrepaymentRate,
	/// Constant Prepayment Yield
	#[serde(rename = "CPY")]
	ConstantPrepaymentYield,
	/// Constant Prepayment Penalty
	#[serde(rename = "CPP")]
	ConstantPrepaymentPenalty,
	/// Absolute Prepayment Speed
	#[serde(rename = "ABS")]
	AbsolutePrepaymentSpeed,
	/// Monthly Prepayment Rate
	#[serde(rename = "MPR")]
	MonthlyPrepaymentRate,
	/// % of BMA Prepayment Curve
	#[serde(rename = "PSA")]
	OfBmaPrepaymentCurve,
	/// % of Prospectus Prepayment Curve
	#[serde(rename = "PPC")]
	OfProspectusPrepaymentCurve,
	/// % of Manufactured Housing Prepayment Curve
	#[serde(rename = "MHP")]
	OfManufacturedHousingPrepaymentCurve,
	/// final CPR of Home Equity Prepayment Curve
	#[serde(rename = "HEP")]
	FinalCprOfHomeEquityPrepaymentCurve,
}

impl Default for StipulationType {
	fn default() -> Self {
		StipulationType::Amt
	}
}
