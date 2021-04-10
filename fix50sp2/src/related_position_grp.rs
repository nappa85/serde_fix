
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedPositionGrp {
	/// NoRelatedPositions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1861")]
	pub related_positions: Option<crate::entities::RepeatingValues<RelatedPosition>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedPosition {
	/// Required if NoRelatedPositions (1861) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1862")]
	pub related_position_id: Option<String>,
	/// RelatedPositionIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1863")]
	pub related_position_id_source: Option<RelatedPositionIDSource>,
	/// RelatedPositionDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1864")]
	pub related_position_date: Option<crate::entities::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelatedPositionIDSource {
	/// Position maintenance report ID
	#[serde(rename = "1")]
	PositionMaintenanceReportId,
	/// Position transfer ID - TransferID(2437)
	#[serde(rename = "2")]
	PositionTransferIdTransferId,
	/// Position entity ID - PositionID(2618)
	#[serde(rename = "3")]
	PositionEntityIdPositionId,
}
