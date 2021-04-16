
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
}
