
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PositionAmountData {
	/// Number of Position Amount entries
	#[serde(rename = "753")]
	pub pos_amt: fix_common::RepeatingValues<PosAm>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PosAm {
	/// PosAmtType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "707")]
	pub pos_amt_type: Option<PosAmtType>,
	/// PosAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "708")]
	pub pos_amt: Option<f64>,
	/// PositionCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1055")]
	pub position_currency: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for PosAmtType {
	fn default() -> Self {
		PosAmtType::FinalMarkToMarketAmount
	}
}
