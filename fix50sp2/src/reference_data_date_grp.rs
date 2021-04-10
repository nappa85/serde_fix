
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReferenceDataDateGrp {
	/// NoReferenceDataDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2746")]
	pub reference_data_dates: Option<crate::entities::RepeatingValues<ReferenceDataDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReferenceDataDate {
	/// Required if NoReferenceDataDates(2746) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2747")]
	pub reference_data_date: Option<crate::entities::UTCTimestamp>,
	/// ReferenceDataDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2748")]
	pub reference_data_date_type: Option<ReferenceDataDateType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReferenceDataDateType {
	/// Date of request for admission to trading
	#[serde(rename = "0")]
	DateOfRequestForAdmissionToTrading,
	/// Date of approval of admission to trading
	#[serde(rename = "1")]
	DateOfApprovalOfAdmissionToTrading,
	/// Date of admission to trading or date of first trade
	#[serde(rename = "2")]
	DateOfAdmissionToTradingOrDateOfFirstTrade,
	/// Termination date
	#[serde(rename = "3")]
	TerminationDate,
}
