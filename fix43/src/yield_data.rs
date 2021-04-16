
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
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum YieldType {
	/// After Tax Yield (Municipals) - The yield on the bond net of any tax consequences from holding the bond. The discount on municipal
	/// securities can be subject to both capital gains taxes and ordinary income taxes. Calculated from dollar price.
	#[serde(rename = "AFTERTAX")]
	AfterTaxYieldTheYieldOnTheBondNetOfAnyTaxConsequencesFromHoldingTheBondTheDiscountOnMunicipalSecuritiesCanBeSubjectToBothCapitalGainsTaxesAndOrdinaryIncomeTaxesCalculatedFromDollarPrice,
	/// Annual Yield - The annual interest or dividend income an investment earns, expressed as a percentage of the investment's total
	/// value.
	#[serde(rename = "ANNUAL")]
	AnnualYieldTheAnnualInterestOrDividendIncomeAnInvestmentEarnsExpressedAsAPercentageOfTheInvestmentSTotalValue,
	/// Yield At Issue (Municipals) - The yield of the bond offered on the issue date.
	#[serde(rename = "ATISSUE")]
	YieldAtIssueTheYieldOfTheBondOfferedOnTheIssueDate,
	/// Yield To Average Life - The yield assuming that all sinks (mandatory and voluntary) are taken at par. This results in a faster
	/// paydown of debt; the yield is then calculated to the average life date.
	#[serde(rename = "AVGLIFE")]
	YieldToAverageLifeTheYieldAssumingThatAllSinksAreTakenAtParThisResultsInAFasterPaydownOfDebtTheYieldIsThenCalculatedToTheAverageLifeDate,
	/// Yield To Average Maturity - The yield achieved by substituting a bond's average maturity for the issue's final maturity date.
	#[serde(rename = "AVGMATURITY")]
	YieldToAverageMaturityTheYieldAchievedBySubstitutingABondSAverageMaturityForTheIssueSFinalMaturityDate,
	/// Book Yield - The yield of a security calculated by using its book value instead of the current market price. This term is
	/// typically used in the US domestic market.
	#[serde(rename = "BOOK")]
	BookYieldTheYieldOfASecurityCalculatedByUsingItsBookValueInsteadOfTheCurrentMarketPriceThisTermIsTypicallyUsedInTheUsDomesticMarket,
	/// Yield to Next Call - The yield of a bond to the next possible call date.
	#[serde(rename = "CALL")]
	YieldToNextCallTheYieldOfABondToTheNextPossibleCallDate,
	/// Yield Change Since Close - The change in the yield since the previous day's closing yield.
	#[serde(rename = "CHANGE")]
	YieldChangeSinceCloseTheChangeInTheYieldSinceThePreviousDaySClosingYield,
	/// Closing Yield - The yield of a bond based on the closing price.
	#[serde(rename = "CLOSE")]
	ClosingYieldTheYieldOfABondBasedOnTheClosingPrice,
	/// Compound Yield - The yield of certain Japanese bonds based on its price. Certain Japanese bonds have irregular first or last
	/// coupons, and the yield is calculated compound for these irregular periods.
	#[serde(rename = "COMPOUND")]
	CompoundYieldTheYieldOfCertainJapaneseBondsBasedOnItsPriceCertainJapaneseBondsHaveIrregularFirstOrLastCouponsAndTheYieldIsCalculatedCompoundForTheseIrregularPeriods,
	/// Current Yield - Annual interest on a bond divided by the market value. The actual income rate of return as opposed to the
	/// coupon rate expressed as a percentage.
	#[serde(rename = "CURRENT")]
	CurrentYieldAnnualInterestOnABondDividedByTheMarketValueTheActualIncomeRateOfReturnAsOpposedToTheCouponRateExpressedAsAPercentage,
	/// True Gross Yield - Yield calculated using the price including accrued interest, where coupon dates are moved from holidays
	/// and weekends to the next trading day.
	#[serde(rename = "GROSS")]
	TrueGrossYieldYieldCalculatedUsingThePriceIncludingAccruedInterestWhereCouponDatesAreMovedFromHolidaysAndWeekendsToTheNextTradingDay,
	/// Government Equivalent Yield - Ask yield based on semi-annual coupons compounding in all periods and actual/actual calendar.
	#[serde(rename = "GOVTEQUIV")]
	GovernmentEquivalentYieldAskYieldBasedOnSemiAnnualCouponsCompoundingInAllPeriodsAndActualActualCalendar,
	/// Yield with Inflation Assumption - Based on price, the return an investor would require on a normal bond that would make the
	/// real return equal to that of the inflation-indexed bond, assuming a constant inflation rate.
	#[serde(rename = "INFLATION")]
	YieldWithInflationAssumptionBasedOnPriceTheReturnAnInvestorWouldRequireOnANormalBondThatWouldMakeTheRealReturnEqualToThatOfTheInflationIndexedBondAssumingAConstantInflationRate,
	/// Inverse Floater Bond Yield - Inverse floater semi-annual bond equivalent rate.
	#[serde(rename = "INVERSEFLOATER")]
	InverseFloaterBondYieldInverseFloaterSemiAnnualBondEquivalentRate,
	/// Most Recent Closing Yield - The last available yield stored in history, computed using price.
	#[serde(rename = "LASTCLOSE")]
	MostRecentClosingYieldTheLastAvailableYieldStoredInHistoryComputedUsingPrice,
	/// Closing Yield Most Recent Month - The yield of a bond based on the closing price as of the most recent month's end.
	#[serde(rename = "LASTMONTH")]
	ClosingYieldMostRecentMonthTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentMonthSEnd,
	/// Closing Yield Most Recent Quarter - The yield of a bond based on the closing price as of the most recent quarter's end.
	#[serde(rename = "LASTQUARTER")]
	ClosingYieldMostRecentQuarterTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentQuarterSEnd,
	/// Closing Yield Most Recent Year - The yield of a bond based on the closing price as of the most recent year's end.
	#[serde(rename = "LASTYEAR")]
	ClosingYieldMostRecentYearTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentYearSEnd,
	/// Yield to Longest Average Life - The yield assuming only mandatory sinks are taken. This results in a lower paydown of debt;
	/// the yield is then calculated to the final payment date.
	#[serde(rename = "LONGAVGLIFE")]
	YieldToLongestAverageLifeTheYieldAssumingOnlyMandatorySinksAreTakenThisResultsInALowerPaydownOfDebtTheYieldIsThenCalculatedToTheFinalPaymentDate,
	/// Yield to Longest Average (Sinking Fund Bonds) - The yield assuming only mandatory sinks are taken. This results in a slower
	/// paydown of debt; the yield is then calculated to the final payment date.
	#[serde(rename = "LONGEST")]
	YieldToLongestAverageTheYieldAssumingOnlyMandatorySinksAreTakenThisResultsInASlowerPaydownOfDebtTheYieldIsThenCalculatedToTheFinalPaymentDate,
	/// Mark To Market Yield - An adjustment in the valuation of a securities portfolio to reflect the current market values of the
	/// respective securities in the portfolio.
	#[serde(rename = "MARK")]
	MarkToMarketYieldAnAdjustmentInTheValuationOfASecuritiesPortfolioToReflectTheCurrentMarketValuesOfTheRespectiveSecuritiesInThePortfolio,
	/// Yield to Maturity - The yield of a bond to its maturity date.
	#[serde(rename = "MATURITY")]
	YieldToMaturityTheYieldOfABondToItsMaturityDate,
	/// Yield To Next Refund (Sinking Fund Bonds) - Yield assuming all bonds are redeemed at the next refund date at the redemption
	/// price.
	#[serde(rename = "NEXTREFUND")]
	YieldToNextRefundYieldAssumingAllBondsAreRedeemedAtTheNextRefundDateAtTheRedemptionPrice,
	/// Open Average Yield - The average yield of the respective securities in the portfolio.
	#[serde(rename = "OPENAVG")]
	OpenAverageYieldTheAverageYieldOfTheRespectiveSecuritiesInThePortfolio,
	/// Yield to Next Put - The yield to the date at which the bond holder can next put the bond to the issuer.
	#[serde(rename = "PUT")]
	YieldToNextPutTheYieldToTheDateAtWhichTheBondHolderCanNextPutTheBondToTheIssuer,
	/// Previous Close Yield - The yield of a bond based on the closing price 1 day ago.
	#[serde(rename = "PREVCLOSE")]
	PreviousCloseYieldTheYieldOfABondBasedOnTheClosingPrice1DayAgo,
	/// Proceeds Yield - The CD equivalent yield when the remaining time to maturity is less than two years.
	#[serde(rename = "PROCEEDS")]
	ProceedsYieldTheCdEquivalentYieldWhenTheRemainingTimeToMaturityIsLessThanTwoYears,
	/// Semi-annual Yield - The yield of a bond whose coupon payments are reinvested semi-annually
	#[serde(rename = "SEMIANNUAL")]
	SemiAnnualYieldTheYieldOfABondWhoseCouponPaymentsAreReinvestedSemiAnnually,
	/// Yield to Shortest Average Life - same as AVGLIFE above.
	#[serde(rename = "SHORTAVGLIFE")]
	YieldToShortestAverageLifeSameAsAvglifeAbove,
	/// Yield to Shortest Average (Sinking Fund Bonds) - The yield assuming that all sinks (mandatory and voluntary) are taken. This
	/// results in a faster paydown of debt; the yield is then calculated to the final payment date.
	#[serde(rename = "SHORTEST")]
	YieldToShortestAverageTheYieldAssumingThatAllSinksAreTakenThisResultsInAFasterPaydownOfDebtTheYieldIsThenCalculatedToTheFinalPaymentDate,
	/// Simple Yield - The yield of a bond assuming no reinvestment of coupon payments. (Act/360 day count)
	#[serde(rename = "SIMPLE")]
	SimpleYieldTheYieldOfABondAssumingNoReinvestmentOfCouponPayments,
	/// Tax Equivalent Yield - The after tax yield grossed up by the maximum federal tax rate of 39.6%. For comparison to taxable
	/// yields.
	#[serde(rename = "TAXEQUIV")]
	TaxEquivalentYieldTheAfterTaxYieldGrossedUpByTheMaximumFederalTaxRateOf396ForComparisonToTaxableYields,
	/// Yield to Tender Date - The yield on a Municipal bond to its mandatory tender date.
	#[serde(rename = "TENDER")]
	YieldToTenderDateTheYieldOnAMunicipalBondToItsMandatoryTenderDate,
	/// True Yield - The yield calculated with coupon dates moved from a weekend or holiday to the next valid settlement date.
	#[serde(rename = "TRUE")]
	TrueYieldTheYieldCalculatedWithCouponDatesMovedFromAWeekendOrHolidayToTheNextValidSettlementDate,
	/// Yield Value Of 1/32 - The amount that the yield will change for a 1/32nd change in price.
	#[serde(rename = "VALUE1/32")]
	YieldValueOf132TheAmountThatTheYieldWillChangeForA132NdChangeInPrice,
	/// Yield To Worst Convention - The lowest yield to all possible redemption date scenarios.
	#[serde(rename = "WORST")]
	YieldToWorstConventionTheLowestYieldToAllPossibleRedemptionDateScenarios,
}

impl Default for YieldType {
	fn default() -> Self {
		YieldType::AfterTaxYieldTheYieldOnTheBondNetOfAnyTaxConsequencesFromHoldingTheBondTheDiscountOnMunicipalSecuritiesCanBeSubjectToBothCapitalGainsTaxesAndOrdinaryIncomeTaxesCalculatedFromDollarPrice
	}
}
