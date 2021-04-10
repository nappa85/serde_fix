
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PositionAmountData {
	/// Number of Position Amount entries.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "753")]
	pub pos_amt: Option<crate::entities::RepeatingValues<PosAm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PosAm {
	/// PosAmtType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "707")]
	pub pos_amt_type: Option<PosAmtType>,
	/// PosAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "708")]
	pub pos_amt: Option<f64>,
	/// PositionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1055")]
	pub position_currency: Option<String>,
	/// Specifies the reason for an amount type when reported on a position. Useful when multiple instances of the same amount type
	/// are reported.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1585")]
	pub pos_amt_reason: Option<PosAmtReason>,
	/// Used when the PosAmt(708) value corresponds to a specific stream in of a swap.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2096")]
	pub pos_amt_stream_desc: Option<String>,
	/// PositionFXRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2097")]
	pub position_fx_rate: Option<f64>,
	/// PositionFXRateCalc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2098")]
	pub position_fx_rate_calc: Option<PositionFXRateCalc>,
	/// PosAmtMarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2099")]
	pub pos_amt_market_segment_id: Option<String>,
	/// PosAmtMarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2100")]
	pub pos_amt_market_id: Option<String>,
	/// PosAmtPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2876")]
	pub pos_amt_price: Option<f64>,
	/// PosAmtPriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2877")]
	pub pos_amt_price_type: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PosAmtType {
	/// Final Mark-to-Market Amount
	#[serde(rename = "FMTM")]
	FinalMarkToMarketAmount,
	/// Incremental Mark-to-Market Amount
	#[serde(rename = "IMTM")]
	IncrementalMarkToMarketAmount,
	/// Trade Variation Amount
	#[serde(rename = "TVAR")]
	TradeVariationAmount,
	/// Start-of-Day Mark-to-Market Amount
	#[serde(rename = "SMTM")]
	StartOfDayMarkToMarketAmount,
	/// Premium Amount
	#[serde(rename = "PREM")]
	PremiumAmount,
	/// Cash Residual Amount
	#[serde(rename = "CRES")]
	CashResidualAmount,
	/// Cash Amount (Corporate Event)
	#[serde(rename = "CASH")]
	CashAmount,
	/// Value Adjusted Amount
	#[serde(rename = "VADJ")]
	ValueAdjustedAmount,
	/// Settlement Value
	#[serde(rename = "SETL")]
	SettlementValue,
	/// Initial Trade Coupon Amount
	#[serde(rename = "ICPN")]
	InitialTradeCouponAmount,
	/// Accrued Coupon Amount
	#[serde(rename = "ACPN")]
	AccruedCouponAmount,
	/// Coupon Amount
	#[serde(rename = "CPN")]
	CouponAmount,
	/// Incremental Accrued Coupon
	#[serde(rename = "IACPN")]
	IncrementalAccruedCoupon,
	/// Collateralized Mark to Market
	#[serde(rename = "CMTM")]
	CollateralizedMarkToMarket,
	/// Incremental Collateralized Mark to market
	#[serde(rename = "ICMTM")]
	IncrementalCollateralizedMarkToMarket,
	/// Compensation Amount
	#[serde(rename = "DLV")]
	CompensationAmount,
	/// Total Banked Amount
	#[serde(rename = "BANK")]
	TotalBankedAmount,
	/// Total Collateralized Amount
	#[serde(rename = "COLAT")]
	TotalCollateralizedAmount,
	/// Long paired swap or swaption notional value
	#[serde(rename = "LSNV")]
	LongPairedSwapOrSwaptionNotionalValue,
	/// Short paired swap or swaption notional value
	#[serde(rename = "SSNV")]
	ShortPairedSwapOrSwaptionNotionalValue,
	/// Start-of-day accrued coupon
	#[serde(rename = "SACPN")]
	StartOfDayAccruedCoupon,
	/// Net present value
	#[serde(rename = "NPV")]
	NetPresentValue,
	/// Start-of-day net present value
	#[serde(rename = "SNPV")]
	StartOfDayNetPresentValue,
	/// Net cash flow
	#[serde(rename = "NCF")]
	NetCashFlow,
	/// Present value of all fees
	#[serde(rename = "PVFEES")]
	PresentValueOfAllFees,
	/// Present value of one basis points
	#[serde(rename = "PV01")]
	PresentValueOfOneBasisPoints,
	/// The five year equivalent notional amount
	#[serde(rename = "5YREN")]
	TheFiveYearEquivalentNotionalAmount,
	/// Undiscounted mark-to-market
	#[serde(rename = "UMTM")]
	UndiscountedMarkToMarket,
	/// Mark-to-model
	#[serde(rename = "MTD")]
	MarkToModel,
	/// Mark-to-market variance
	#[serde(rename = "VMTM")]
	MarkToMarketVariance,
	/// Mark-to-model variance
	#[serde(rename = "VMTD")]
	MarkToModelVariance,
	/// Upfront payment
	#[serde(rename = "UPFRNT")]
	UpfrontPayment,
	/// End value
	#[serde(rename = "ENDV")]
	EndValue,
	/// Outstanding margin loan
	#[serde(rename = "MGNLN")]
	OutstandingMarginLoan,
	/// Loan value
	#[serde(rename = "LNVL")]
	LoanValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PosAmtReason {
	/// Options settlement
	#[serde(rename = "0")]
	OptionsSettlement,
	/// Pending erosion adjustment
	#[serde(rename = "1")]
	PendingErosionAdjustment,
	/// Final erosion adjustment
	#[serde(rename = "2")]
	FinalErosionAdjustment,
	/// Tear-up coupon amount
	#[serde(rename = "3")]
	TearUpCouponAmount,
	/// Price Alignment Interest
	#[serde(rename = "4")]
	PriceAlignmentInterest,
	/// Delivery invoice charges
	#[serde(rename = "5")]
	DeliveryInvoiceCharges,
	/// Delivery storage charges
	#[serde(rename = "6")]
	DeliveryStorageCharges,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PositionFXRateCalc {
	/// Divided
	#[serde(rename = "D")]
	Divided,
	/// Multiply
	#[serde(rename = "M")]
	Multiply,
}
