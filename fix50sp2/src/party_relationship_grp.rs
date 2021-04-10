
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRelationshipGrp {
	/// NoPartyRelationships
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1514")]
	pub party_relationships: Option<crate::entities::RepeatingValues<PartyRelationship>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRelationship {
	/// Identifies the type of party relationship requested. Required when <a href="tag_1514_NoPartyRelationships.html" target="bottom">NoPartyRelationships (1514)&nbsp;(1514)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1515")]
	pub party_relationship_item: Option<PartyRelationshipItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PartyRelationshipItem {
	/// IsAlso
	#[serde(rename = "0")]
	IsAlso,
	/// ClearsFor
	#[serde(rename = "1")]
	ClearsFor,
	/// ClearsThrough
	#[serde(rename = "2")]
	ClearsThrough,
	/// TradesFor
	#[serde(rename = "3")]
	TradesFor,
	/// TradesThrough
	#[serde(rename = "4")]
	TradesThrough,
	/// Sponsors
	#[serde(rename = "5")]
	Sponsors,
	/// SponsoredThrough
	#[serde(rename = "6")]
	SponsoredThrough,
	/// ProvidesGuaranteeFor
	#[serde(rename = "7")]
	ProvidesGuaranteeFor,
	/// IsGuaranteedBy
	#[serde(rename = "8")]
	IsGuaranteedBy,
	/// MemberOf
	#[serde(rename = "9")]
	MemberOf,
	/// HasMembers
	#[serde(rename = "10")]
	HasMembers,
	/// ProvidesMarketplaceFor
	#[serde(rename = "11")]
	ProvidesMarketplaceFor,
	/// ParticipantOfMarketplace
	#[serde(rename = "12")]
	ParticipantOfMarketplace,
	/// CarriesPositionsFor
	#[serde(rename = "13")]
	CarriesPositionsFor,
	/// PostsTradesTo
	#[serde(rename = "14")]
	PostsTradesTo,
	/// EntersTradesFor
	#[serde(rename = "15")]
	EntersTradesFor,
	/// EntersTradesThrough
	#[serde(rename = "16")]
	EntersTradesThrough,
	/// ProvidesQuotesTo
	#[serde(rename = "17")]
	ProvidesQuotesTo,
	/// RequestsQuotesFrom
	#[serde(rename = "18")]
	RequestsQuotesFrom,
	/// InvestsFor
	#[serde(rename = "19")]
	InvestsFor,
	/// InvestsThrough
	#[serde(rename = "20")]
	InvestsThrough,
	/// BrokersTradesFor
	#[serde(rename = "21")]
	BrokersTradesFor,
	/// BrokersTradesThrough
	#[serde(rename = "22")]
	BrokersTradesThrough,
	/// ProvidesTradingServicesFor
	#[serde(rename = "23")]
	ProvidesTradingServicesFor,
	/// UsesTradingServicesOf
	#[serde(rename = "24")]
	UsesTradingServicesOf,
	/// ApprovesOf
	#[serde(rename = "25")]
	ApprovesOf,
	/// ApprovedBy
	#[serde(rename = "26")]
	ApprovedBy,
	/// ParentFirmFor
	#[serde(rename = "27")]
	ParentFirmFor,
	/// SubsidiaryOf
	#[serde(rename = "28")]
	SubsidiaryOf,
	/// RegulatoryOwnerOf
	#[serde(rename = "29")]
	RegulatoryOwnerOf,
	/// OwnedByRegulatory
	#[serde(rename = "30")]
	OwnedByRegulatory,
	/// Controls
	#[serde(rename = "31")]
	Controls,
	/// IsControlledBy
	#[serde(rename = "32")]
	IsControlledBy,
	/// LegalOwnerOf
	#[serde(rename = "33")]
	LegalOwnerOf,
	/// OwnedByLegal
	#[serde(rename = "34")]
	OwnedByLegal,
	/// BeneficialOwnerOf
	#[serde(rename = "35")]
	BeneficialOwnerOf,
	/// OwnedByBeneficial
	#[serde(rename = "36")]
	OwnedByBeneficial,
	/// SettlesFor
	#[serde(rename = "37")]
	SettlesFor,
	/// SettlesThrough
	#[serde(rename = "38")]
	SettlesThrough,
}
