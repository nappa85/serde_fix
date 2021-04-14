
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataStatisticsRequest {
	/// MsgType = DO
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// <p>Unique message identifier for the request or the identifier of a previous request when unsubscribing.</p>
	#[serde(rename = "2452")]
	pub md_statistic_req_id: String,
	/// <p>Unique message identifier for the request or the identifier of a previous request when unsubscribing.</p>
	#[serde(rename = "263")]
	pub subscription_request_type: SubscriptionRequestType,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// <p>Used to specify the business date</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// <p>Used to specify a single market</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// <p>Used to specify a single market segment</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// MarketSegmentDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1396")]
	pub market_segment_desc: Option<String>,
	/// Must be set if <a href="tag_1398_EncodedMktSegmDesc.html" target="bottom">EncodedMktSegmDesc(1398)&nbsp;(1398)</a> field is specified and must immediately precede it.
	#[serde(rename = "1397")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1396_MarketSegmentDesc.html" target="bottom">MarketSegmentDesc(1396)&nbsp;(1396)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1398")]
	pub encoded_mkt_segm_desc: Option<fix_common::EncodedText<1398>>,
	/// Used to reference an entire group of instruments for which a single set of statistics is to be calculated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1465")]
	pub security_list_id: Option<String>,
	/// <p>Used to specify an individual instrument or instrument attributes for which a single set of statistics is to be calculated.</p>
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// UndInstrmtGrp
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// InstrmtLegGrp
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// <p>Used to specify the parameters for the calculation of statistics.</p>
	#[serde(flatten)]
	pub md_statistic_req_grp: super::super::md_statistic_req_grp::MDStatisticReqGrp,
	/// Time that the request was submitted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// InstrumentExtension
	#[serde(flatten)]
	pub instrument_extension: Option<super::super::instrument_extension::InstrumentExtension>,
	/// FinancingDetails
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// RelatedInstrumentGrp
	#[serde(flatten)]
	pub related_instrument_grp: Option<super::super::related_instrument_grp::RelatedInstrumentGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}
