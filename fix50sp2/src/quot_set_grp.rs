
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuotSetGrp {
	/// The number of sets of quotes in the message
	#[serde(rename = "296")]
	pub quote_sets: fix_common::RepeatingValues<QuoteSet>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteSet {
	/// Sequential number for the QuoteSet. For a given QuoteID - assumed to start at 1. Must be the first field in the repeating
	/// group.
	#[serde(rename = "302")]
	pub quote_set_id: String,
	/// QuoteSetValidUntilTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "367")]
	pub quote_set_valid_until_time: Option<fix_common::UTCTimestamp>,
	/// Total number of quotes for the QuoteSet across all messages. Should be the sum of all NoQuoteEntries in each message that
	/// has repeating quotes that are part of the same QuoteSet.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "304")]
	pub tot_no_quote_entries: i32,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

impl Default for LastFragment {
	fn default() -> Self {
		LastFragment::NotLastMessage
	}
}
