
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClearingAccountTypeGrp {
	/// NoClearingAccountTypes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1918")]
	pub clearing_account_types: Option<fix_common::RepeatingValues<ClearingAccountType>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClearingAccountType {
	/// Required if NoClearingAccountTypes(1918) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1816")]
	pub clearing_account_type_item: Option<ClearingAccountTypeItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClearingAccountTypeItem {
	/// Customer
	#[serde(rename = "1")]
	Customer,
	/// Firm
	#[serde(rename = "2")]
	Firm,
	/// Market maker
	#[serde(rename = "3")]
	MarketMaker,
}

impl Default for ClearingAccountTypeItem {
	fn default() -> Self {
		ClearingAccountTypeItem::Customer
	}
}
