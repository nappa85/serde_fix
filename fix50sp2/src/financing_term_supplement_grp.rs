
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancingTermSupplementGrp {
	/// NoFinancingTermSupplements
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40046")]
	pub financing_term_supplements: Option<crate::entities::RepeatingValues<FinancingTermSupplement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancingTermSupplement {
	/// Required if NoFinancingTermSupplements(40046) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40047")]
	pub financing_term_supplement_desc: Option<String>,
	/// FinancingTermSupplementDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40048")]
	pub financing_term_supplement_date: Option<crate::entities::LocalMktDate>,
}
