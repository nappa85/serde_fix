
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CollInqQualGrp {
	/// Number of qualifiers to inquiry.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "938")]
	pub coll_inquiry_qualifier: Option<crate::entities::RepeatingValues<CollInquiryQualifie>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CollInquiryQualifie {
	/// Required if <a href="tag_938_NoCollInquiryQualifier.html" target="bottom">NoCollInquiryQualifier&nbsp;(938)</a> &gt; 0. Type of collateral inquiry.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "896")]
	pub coll_inquiry_qualifier: Option<CollInquiryQualifier>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollInquiryQualifier {
	/// Trade Date
	#[serde(rename = "0")]
	TradeDate,
	/// GC Instrument
	#[serde(rename = "1")]
	GcInstrument,
	/// Collateral Instrument
	#[serde(rename = "2")]
	CollateralInstrument,
	/// Substitution Eligible
	#[serde(rename = "3")]
	SubstitutionEligible,
	/// Not Assigned
	#[serde(rename = "4")]
	NotAssigned,
	/// Partially Assigned
	#[serde(rename = "5")]
	PartiallyAssigned,
	/// Fully Assigned
	#[serde(rename = "6")]
	FullyAssigned,
	/// Outstanding Trades (Today &lt; end date)
	#[serde(rename = "7")]
	OutstandingTrades,
}
