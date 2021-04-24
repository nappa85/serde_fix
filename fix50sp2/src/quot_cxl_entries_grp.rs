
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuotCxlEntriesGrp {
	/// The number of securities (instruments) whose quotes are to be canceled Not required when cancelling all quotes.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "295")]
	pub quote_entries: Option<fix_common::RepeatingValues<QuoteEntrie>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteEntrie {
    #[serde(flatten)]
    pub instrument: Option<super::instrument::Instrument>,
    #[serde(flatten)]
    pub financing_details: Option<super::financing_details::FinancingDetails>,
    #[serde(flatten)]
    pub und_instrmt_grp: Option<super::und_instrmt_grp::UndInstrmtGrp>,
    #[serde(flatten)]
    pub instrmt_leg_grp: Option<super::instrmt_leg_grp::InstrmtLegGrp>,
}
