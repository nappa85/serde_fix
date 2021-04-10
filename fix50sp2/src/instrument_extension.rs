
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentExtension {
	/// Identifies the form of delivery.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "668")]
	pub delivery_form: Option<DeliveryForm>,
	/// Percent at risk due to lowest possible call.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "869")]
	pub pct_at_risk: Option<f32>,
	/// Number of repeating InstrAttrib group entries.
	#[serde(flatten)]
	pub attrb_grp: Option<super::attrb_grp::AttrbGrp>,
	/// CommodityFinalPriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2736")]
	pub commodity_final_price_type: Option<CommodityFinalPriceType>,
	/// IndexRollMonthGrp
	#[serde(flatten)]
	pub index_roll_month_grp: Option<super::index_roll_month_grp::IndexRollMonthGrp>,
	/// NextIndexRollDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2738")]
	pub next_index_roll_date: Option<fix_common::LocalMktDate>,
	/// FloatingRateIndex
	#[serde(flatten)]
	pub floating_rate_index: Option<super::floating_rate_index::FloatingRateIndex>,
	/// ReferenceDataDateGrp
	#[serde(flatten)]
	pub reference_data_date_grp: Option<super::reference_data_date_grp::ReferenceDataDateGrp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeliveryForm {
	/// Book Entry (default)
	#[serde(rename = "1")]
	BookEntry,
	/// Bearer
	#[serde(rename = "2")]
	Bearer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CommodityFinalPriceType {
	/// Argus McCloskey
	#[serde(rename = "0")]
	ArgusMcCloskey,
	/// Baltic
	#[serde(rename = "1")]
	Baltic,
	/// Exchange
	#[serde(rename = "2")]
	Exchange,
	/// Global Coal
	#[serde(rename = "3")]
	GlobalCoal,
	/// IHS McCloskey
	#[serde(rename = "4")]
	IhsMcCloskey,
	/// Platts
	#[serde(rename = "5")]
	Platts,
	/// Other
	#[serde(rename = "99")]
	Other,
}
