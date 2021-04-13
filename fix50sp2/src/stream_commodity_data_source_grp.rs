
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommodityDataSourceGrp {
	/// NoStreamCommodityDataSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41280")]
	pub stream_commodity_data_sources: Option<fix_common::RepeatingValues<StreamCommodityDataSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommodityDataSource {
	/// Required if NoStreamCommodityDataSources(41280) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41281")]
	pub stream_commodity_data_source_id: Option<String>,
	/// Required if NoStreamCommodityDataSources(41280) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41282")]
	pub stream_commodity_data_source_id_type: Option<StreamCommodityDataSourceIDType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamCommodityDataSourceIDType {
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

impl Default for StreamCommodityDataSourceIDType {
	fn default() -> Self {
		StreamCommodityDataSourceIDType::City
	}
}
