
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataFeedTypes {
	/// The number of feed types and corresponding book depths associated with a security
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1141")]
	pub md_feed_types: Option<fix_common::RepeatingValues<MDFeedType>>,
	/// MDSubFeedType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1683")]
	pub md_sub_feed_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDFeedType {
	/// Required if NoMDFeedTypes(1141) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1022")]
	pub md_feed_type: Option<String>,
	/// Specifies the depth of book (or levels of market depth) for the feed type.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "264")]
	pub market_depth: Option<MarketDepth>,
	/// MDBookType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1021")]
	pub md_book_type: Option<MDBookType>,
	/// Conditionally required when MarketDepthTimeIntervalUnit(2564) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2563")]
	pub market_depth_time_interval: Option<i32>,
	/// Conditionally required when MarketDataTimeInterval(2563) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2564")]
	pub market_depth_time_interval_unit: Option<i32>,
	/// Conditionally required when MDRecoveryTimeIntervalUnit(2566) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2565")]
	pub md_recovery_time_interval: Option<i32>,
	/// Conditionally required when MDRecoveryTimeInterval(2565) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2566")]
	pub md_recovery_time_interval_unit: Option<i32>,
	/// MDSubBookType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1173")]
	pub md_sub_book_type: Option<i32>,
	/// PrimaryServiceLocationID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2567")]
	pub primary_service_location_id: Option<String>,
	/// SecondaryServiceLocationID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2568")]
	pub secondary_service_location_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MarketDepth {
	/// full book depth
	#[serde(rename = "0")]
	FullBookDepth,
	/// top of book
	#[serde(rename = "1")]
	TopOfBook,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDBookType {
	/// Top of Book
	#[serde(rename = "1")]
	TopOfBook,
	/// Price Depth
	#[serde(rename = "2")]
	PriceDepth,
	/// Order Depth
	#[serde(rename = "3")]
	OrderDepth,
}
