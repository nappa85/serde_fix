
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeAllocAmtGrp {
	/// NoTradeAllocAmts
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1844")]
	pub trade_alloc_amts: Option<crate::entities::RepeatingValues<TradeAllocAmt>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeAllocAmt {
	/// Required if NoTradeAllocAmts &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1845")]
	pub trade_alloc_amt_type: Option<TradeAllocAmtType>,
	/// TradeAllocAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1846")]
	pub trade_alloc_amt: Option<f64>,
	/// TradeAllocCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1847")]
	pub trade_alloc_currency: Option<String>,
	/// TradeAllocAmtReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1850")]
	pub trade_alloc_amt_reason: Option<TradeAllocAmtReason>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeAllocAmtType {
	/// Cash Amount (Corporate Event)
	#[serde(rename = "CASH")]
	CashAmount,
	/// Cash Residual Amount
	#[serde(rename = "CRES")]
	CashResidualAmount,
	/// Final Mark-to-Market Amount
	#[serde(rename = "FMTM")]
	FinalMarkToMarketAmount,
	/// Incremental Mark-to-Market Amount
	#[serde(rename = "IMTM")]
	IncrementalMarkToMarketAmount,
	/// Premium Amount
	#[serde(rename = "PREM")]
	PremiumAmount,
	/// Start-of-Day Mark-to-Market Amount
	#[serde(rename = "SMTM")]
	StartOfDayMarkToMarketAmount,
	/// Trade Variation Amount
	#[serde(rename = "TVAR")]
	TradeVariationAmount,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeAllocAmtReason {
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
	/// Price alignment interest
	#[serde(rename = "4")]
	PriceAlignmentInterest,
	/// Delivery invoice charges
	#[serde(rename = "5")]
	DeliveryInvoiceCharges,
	/// Delivery storage charges
	#[serde(rename = "6")]
	DeliveryStorageCharges,
}
