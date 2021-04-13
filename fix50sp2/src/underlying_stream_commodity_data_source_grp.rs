
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommodityDataSourceGrp {
	/// NoUnderlyingStreamCommodityDataSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41993")]
	pub underlying_stream_commodity_data_sources: Option<fix_common::RepeatingValues<UnderlyingStreamCommodityDataSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommodityDataSource {
	/// Required if NoUnderlyingStreamCommodityDataSources(41993) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41994")]
	pub underlying_stream_commodity_data_source_id: Option<String>,
	/// Required if NoUnderlyingStreamCommodityDataSources(41993) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41995")]
	pub underlying_stream_commodity_data_source_id_type: Option<UnderlyingStreamCommodityDataSourceIDType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommodityDataSourceIDType {
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

impl Default for UnderlyingStreamCommodityDataSourceIDType {
	fn default() -> Self {
		UnderlyingStreamCommodityDataSourceIDType::City
	}
}
