
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrategyParametersGrp {
	/// Indicates number of strategy parameters.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "957")]
	pub strategy_parameters: Option<fix_common::RepeatingValues<StrategyParameter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StrategyParameter {
	/// Name of parameter.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "958")]
	pub strategy_parameter_name: Option<String>,
	/// Datatype of the parameter.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "959")]
	pub strategy_parameter_type: Option<StrategyParameterType>,
	/// Value of the parameter.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "960")]
	pub strategy_parameter_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrategyParameterType {
	/// Int
	#[serde(rename = "1")]
	Int,
	/// Length
	#[serde(rename = "2")]
	Length,
	/// NumInGroup
	#[serde(rename = "3")]
	NumInGroup,
	/// SeqNum
	#[serde(rename = "4")]
	SeqNum,
	/// TagNum
	#[serde(rename = "5")]
	TagNum,
	/// Float
	#[serde(rename = "6")]
	Float,
	/// Qty
	#[serde(rename = "7")]
	Qty,
	/// Price
	#[serde(rename = "8")]
	Price,
	/// PriceOffset
	#[serde(rename = "9")]
	PriceOffset,
	/// Amt
	#[serde(rename = "10")]
	Amt,
	/// Percentage
	#[serde(rename = "11")]
	Percentage,
	/// Char
	#[serde(rename = "12")]
	Char,
	/// Boolean
	#[serde(rename = "13")]
	Boolean,
	/// String
	#[serde(rename = "14")]
	String,
	/// MultipleCharValue
	#[serde(rename = "15")]
	MultipleCharValue,
	/// Currency
	#[serde(rename = "16")]
	Currency,
	/// Exchange
	#[serde(rename = "17")]
	Exchange,
	/// Month-Year
	#[serde(rename = "18")]
	MonthYear,
	/// UTCTimeStamp
	#[serde(rename = "19")]
	UtcTimeStamp,
	/// UTCTimeOnly
	#[serde(rename = "20")]
	UtcTimeOnly,
	/// LocalMktDate
	#[serde(rename = "21")]
	LocalMktDate,
	/// UTCDateOnly
	#[serde(rename = "22")]
	UtcDateOnly,
	/// Data
	#[serde(rename = "23")]
	Data,
	/// MultipleStringValue
	#[serde(rename = "24")]
	MultipleStringValue,
	/// Country
	#[serde(rename = "25")]
	Country,
	/// Language
	#[serde(rename = "26")]
	Language,
	/// TZTimeOnly
	#[serde(rename = "27")]
	TzTimeOnly,
	/// TZTimestamp
	#[serde(rename = "28")]
	TzTimestamp,
	/// Tenor
	#[serde(rename = "29")]
	Tenor,
}

impl Default for StrategyParameterType {
	fn default() -> Self {
		StrategyParameterType::Int
	}
}
