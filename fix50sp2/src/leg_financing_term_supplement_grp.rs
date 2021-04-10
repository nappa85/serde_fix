
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegFinancingTermSupplementGrp {
	/// NoLegFinancingTermSupplements
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42200")]
	pub leg_financing_term_supplements: Option<crate::entities::RepeatingValues<LegFinancingTermSupplement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegFinancingTermSupplement {
	/// Required if <a href="tag_42200_NoLegFinancingTermSupplements.html" target="bottom">NoLegFinancingTermSupplements&nbsp;(42200)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42201")]
	pub leg_financing_term_supplement_desc: Option<String>,
	/// LegFinancingTermSupplementDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42202")]
	pub leg_financing_term_supplement_date: Option<crate::entities::LocalMktDate>,
}
