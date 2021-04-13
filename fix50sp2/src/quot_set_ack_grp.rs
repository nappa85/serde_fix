
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuotSetAckGrp {
	/// The number of sets of quotes in the message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "296")]
	pub quote_sets: Option<fix_common::RepeatingValues<QuoteSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteSet {
	/// First field in repeating group. Required if NoQuoteSets &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "302")]
	pub quote_set_id: Option<String>,
	/// QuoteSetValidUntilTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "367")]
	pub quote_set_valid_until_time: Option<fix_common::UTCTimestamp>,
	/// Total number of quotes for the QuoteSet across all messages. Should be the sum of all NoQuoteEntries in each message that
	/// has repeating quotes that are part of the same QuoteSet. Required if NoQuoteEntries &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "304")]
	pub tot_no_quote_entries: Option<i32>,
	/// Total number of quotes canceled for the QuoteSet across all messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1168")]
	pub tot_no_cxld_quotes: Option<i32>,
	/// Total number of quotes accepted for the QuoteSet across all messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1169")]
	pub tot_no_acc_quotes: Option<i32>,
	/// Total number of quotes rejected for the QuoteSet across all messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1170")]
	pub tot_no_rej_quotes: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
