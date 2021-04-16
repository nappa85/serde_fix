
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
	/// Geographics
	#[serde(rename = "GEOG")]
	Geographics,
	/// Year of Issue
	#[serde(rename = "ISSUE")]
	YearOfIssue,
	/// Lot Variance (value in percent maximum over- or under-allocation allowed)
	#[serde(rename = "LOTVAR")]
	LotVariance,
	/// Maturity Year and Month
	#[serde(rename = "MAT")]
	MaturityYearAndMonth,
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
	/// Production Year
	#[serde(rename = "PROD")]
	ProductionYear,
	/// Trade Variance (value in percent maximum over- or under-allocation allowed)
	#[serde(rename = "TRDVAR")]
	TradeVariance,
	/// Weighted Average Coupon (value in percent)
	#[serde(rename = "WAC")]
	WeightedAverageCoupon,
	/// Weighted Average Life (value in months)
	#[serde(rename = "WAL")]
	WeightedAverageLife,
	/// Weighted Average Loan Age (value in months)
	#[serde(rename = "WALA")]
	WeightedAverageLoanAge,
	/// Weighted Average Maturity (value in months)
	#[serde(rename = "WAM")]
	WeightedAverageMaturity,
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
		StipulationType::Geographics
	}
}
