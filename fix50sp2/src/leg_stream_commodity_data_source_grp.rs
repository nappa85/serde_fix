
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCommodityDataSourceGrp {
	/// NoLegStreamCommodityDataSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41677")]
	pub leg_stream_commodity_data_sources: Option<crate::entities::RepeatingValues<LegStreamCommodityDataSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCommodityDataSource {
	/// Required if NoLegStreamCommodityDataSources(41677) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41678")]
	pub leg_stream_commodity_data_source_id: Option<String>,
	/// Required if NoLegStreamCommodityDataSources(41677) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41679")]
	pub leg_stream_commodity_data_source_id_type: Option<LegStreamCommodityDataSourceIDType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegStreamCommodityDataSourceIDType {
	/// City (4 character business center code)
	#[serde(rename = "0")]
	City,
	/// Airport (IATA standard)
	#[serde(rename = "1")]
	Airport,
	/// Weather station WBAN (Weather Bureau Army Navy)
	#[serde(rename = "2")]
	WeatherStationWban,
	/// Weather index WMO (World Meteorological Organization)
	#[serde(rename = "3")]
	WeatherIndexWmo,
}
