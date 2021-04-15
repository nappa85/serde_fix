
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSecondaryAssetGrp {
	/// NoUnderlyingSecondaryAssetClasses
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2080")]
	pub underlying_secondary_asset_classes: Option<fix_common::RepeatingValues<UnderlyingSecondaryAssetClasse>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSecondaryAssetClasse {
	/// Required if NoUnderlyingSecondaryAssetClasses(2080) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2081")]
	pub underlying_secondary_asset_class: Option<UnderlyingSecondaryAssetClass>,
	/// Required if UnderlyingSecondaryAssetType(2083) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2082")]
	pub underlying_secondary_asset_sub_class: Option<UnderlyingSecondaryAssetSubClass>,
	/// Required if UnderlyingSecondaryAssetSubType(2745) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2083")]
	pub underlying_secondary_asset_type: Option<String>,
	/// UnderlyingSecondaryAssetSubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2745")]
	pub underlying_secondary_asset_sub_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingSecondaryAssetClass {
	/// Interest rate
	#[serde(rename = "1")]
	InterestRate,
	/// Currency
	#[serde(rename = "2")]
	Currency,
	/// Credit
	#[serde(rename = "3")]
	Credit,
	/// Equity
	#[serde(rename = "4")]
	Equity,
	/// Commodity
	#[serde(rename = "5")]
	Commodity,
	/// Other
	#[serde(rename = "6")]
	Other,
	/// Cash
	#[serde(rename = "7")]
	Cash,
	/// Debt
	#[serde(rename = "8")]
	Debt,
	/// Fund (Such as mutual fund, collective investment vehicle, investment program, specialized account program.)
	#[serde(rename = "9")]
	Fund,
	/// Loan facility
	#[serde(rename = "10")]
	LoanFacility,
	/// Index
	#[serde(rename = "11")]
	Index,
}

impl Default for UnderlyingSecondaryAssetClass {
	fn default() -> Self {
		UnderlyingSecondaryAssetClass::InterestRate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingSecondaryAssetSubClass {
	/// Single currency
	#[serde(rename = "1")]
	SingleCurrency,
	/// Cross currency
	#[serde(rename = "2")]
	CrossCurrency,
	/// Basket [for multi-currency]
	#[serde(rename = "3")]
	BasketForMultiCurrency,
	/// Single name
	#[serde(rename = "4")]
	SingleName,
	/// Credit index
	#[serde(rename = "5")]
	CreditIndex,
	/// Index tranche
	#[serde(rename = "6")]
	IndexTranche,
	/// Credit basket
	#[serde(rename = "7")]
	CreditBasket,
	/// Exotic
	#[serde(rename = "8")]
	Exotic,
	/// Common
	#[serde(rename = "9")]
	Common,
	/// Preferred
	#[serde(rename = "10")]
	Preferred,
	/// Equity index
	#[serde(rename = "11")]
	EquityIndex,
	/// Equity basket
	#[serde(rename = "12")]
	EquityBasket,
	/// Metals
	#[serde(rename = "13")]
	Metals,
	/// Bullion
	#[serde(rename = "14")]
	Bullion,
	/// Energy
	#[serde(rename = "15")]
	Energy,
	/// Commodity index
	#[serde(rename = "16")]
	CommodityIndex,
	/// Agricultural
	#[serde(rename = "17")]
	Agricultural,
	/// Environmental
	#[serde(rename = "18")]
	Environmental,
	/// Freight
	#[serde(rename = "19")]
	Freight,
	/// Government
	#[serde(rename = "20")]
	Government,
	/// Agency
	#[serde(rename = "21")]
	Agency,
	/// Corporate
	#[serde(rename = "22")]
	Corporate,
	/// Financing
	#[serde(rename = "23")]
	Financing,
	/// Money market
	#[serde(rename = "24")]
	MoneyMarket,
	/// Mortgage
	#[serde(rename = "25")]
	Mortgage,
	/// Municipal
	#[serde(rename = "26")]
	Municipal,
	/// Mutual fund
	#[serde(rename = "27")]
	MutualFund,
	/// Collective investment vehicle
	#[serde(rename = "28")]
	CollectiveInvestmentVehicle,
	/// Investment program (generalized fund for major investors)
	#[serde(rename = "29")]
	InvestmentProgram,
	/// Specialized account (specialized fund setup for a particular account or group of accounts.)
	#[serde(rename = "30")]
	SpecializedAccount,
	/// Term loan
	#[serde(rename = "31")]
	TermLoan,
	/// Bridge loan
	#[serde(rename = "32")]
	BridgeLoan,
	/// Letter of credit
	#[serde(rename = "33")]
	LetterOfCredit,
	/// Dividend index
	#[serde(rename = "34")]
	DividendIndex,
	/// Stock dividend
	#[serde(rename = "35")]
	StockDividend,
	/// Exchange traded fund
	#[serde(rename = "36")]
	ExchangeTradedFund,
	/// Volatility index
	#[serde(rename = "37")]
	VolatilityIndex,
	/// FX cross rates
	#[serde(rename = "38")]
	FxCrossRates,
	/// FX emerging markets
	#[serde(rename = "39")]
	FxEmergingMarkets,
	/// FX Majors
	#[serde(rename = "40")]
	FxMajors,
	/// Fertilizer
	#[serde(rename = "41")]
	Fertilizer,
	/// Industrial product
	#[serde(rename = "42")]
	IndustrialProduct,
	/// Inflation
	#[serde(rename = "43")]
	Inflation,
	/// Paper
	#[serde(rename = "44")]
	Paper,
	/// Polypropylene
	#[serde(rename = "45")]
	Polypropylene,
	/// Official economic statistics
	#[serde(rename = "46")]
	OfficialEconomicStatistics,
	/// Other C10
	#[serde(rename = "47")]
	OtherC10,
	/// Other
	#[serde(rename = "48")]
	Other,
}

impl Default for UnderlyingSecondaryAssetSubClass {
	fn default() -> Self {
		UnderlyingSecondaryAssetSubClass::SingleCurrency
	}
}
