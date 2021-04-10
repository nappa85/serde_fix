
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradePriceConditionGrp {
	/// NoTradePriceConditions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1838")]
	pub trade_price_conditions: Option<crate::entities::RepeatingValues<TradePriceCondition>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradePriceCondition {
	/// Required if NoTradePriceConditions &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1839")]
	pub trade_price_condition_item: Option<TradePriceConditionItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradePriceConditionItem {
	/// Special cum dividend(CD)
	#[serde(rename = "0")]
	SpecialCumDividend,
	/// Special cum rights(CR)
	#[serde(rename = "1")]
	SpecialCumRights,
	/// Special exdividend(XD)
	#[serde(rename = "2")]
	SpecialExdividend,
	/// Special ex rights(XR)
	#[serde(rename = "3")]
	SpecialExRights,
	/// Special cum coupon(CC)
	#[serde(rename = "4")]
	SpecialCumCoupon,
	/// Special cum capital repayments(CP)
	#[serde(rename = "5")]
	SpecialCumCapitalRepayments,
	/// Special ex coupon(XC)
	#[serde(rename = "6")]
	SpecialExCoupon,
	/// Special ex capital repayments(XP)
	#[serde(rename = "7")]
	SpecialExCapitalRepayments,
	/// Cash settlement(CS)
	#[serde(rename = "8")]
	CashSettlement,
	/// Special cum bonus (CB)
	#[serde(rename = "9")]
	SpecialCumBonus,
	/// Special price(usually net- or all-in price)(SP)
	#[serde(rename = "10")]
	SpecialPrice,
	/// Special ex bonus(XB)
	#[serde(rename = "11")]
	SpecialExBonus,
	/// Guaranteed delivery(GD)
	#[serde(rename = "12")]
	GuaranteedDelivery,
	/// Special dividend
	#[serde(rename = "13")]
	SpecialDividend,
	/// Price improvement
	#[serde(rename = "14")]
	PriceImprovement,
	/// Non-price forming trade
	#[serde(rename = "15")]
	NonPriceFormingTrade,
	/// Trade exempted from trading obligation
	#[serde(rename = "16")]
	TradeExemptedFromTradingObligation,
	/// Price or strike price is pending
	#[serde(rename = "17")]
	PriceOrStrikePriceIsPending,
	/// Price is not applicable
	#[serde(rename = "18")]
	PriceIsNotApplicable,
}
