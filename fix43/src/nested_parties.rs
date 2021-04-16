
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NestedParties {
	/// Repeating group below should contain unique combinations of <a href="tag_524_NestedPartyID.html" target="bottom">NestedPartyID&nbsp;(524)</a> , <a href="tag_525_NestedPartyIDSource.html" target="bottom">NestedPartyIDSource&nbsp;(525)</a> , and <a href="tag_538_NestedPartyRole.html" target="bottom">NestedPartyRole&nbsp;(538)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "539")]
	pub nested_party_i_ds: Option<fix_common::RepeatingValues<NestedPartyID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NestedPartyID {
	/// Used to identify source of NestedPartyID. Required if <a href="tag_525_NestedPartyIDSource.html" target="bottom">NestedPartyIDSource&nbsp;(525)</a> is specified. Required if <a href="tag_539_NoNestedPartyIDs.html" target="bottom">NoNestedPartyIDs&nbsp;(539)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "524")]
	pub nested_party_id: Option<String>,
	/// Used to identify class source of <a href="tag_524_NestedPartyID.html" target="bottom">NestedPartyID&nbsp;(524)</a> value (e.g. BIC). Required if <a href="tag_524_NestedPartyID.html" target="bottom">NestedPartyID&nbsp;(524)</a> is specified. Required if <a href="tag_539_NoNestedPartyIDs.html" target="bottom">NoNestedPartyIDs&nbsp;(539)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "525")]
	pub nested_party_id_source: Option<char>,
	/// Identifies the type of <a href="tag_539_NoNestedPartyIDs.html" target="bottom">NoNestedPartyIDs&nbsp;(539)</a> (e.g. Executing Broker). Required if <a href="tag_539_NoNestedPartyIDs.html" target="bottom">NoNestedPartyIDs&nbsp;(539)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "538")]
	pub nested_party_role: Option<NestedPartyRole>,
	/// Sub-identifier (e.g. Clearing Acct for <a href="tag_524_NestedPartyID.html" target="bottom">NestedPartyID&nbsp;(524)</a> =Clearing Firm) if applicable
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "545")]
	pub nested_party_sub_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NestedPartyRole {
	/// Executing Firm (formerly FIX 4.2 ExecBroker)
	#[serde(rename = "1")]
	ExecutingFirm,
	/// Broker of Credit (formerly FIX 4.2 BrokerOfCredit)
	#[serde(rename = "2")]
	BrokerOfCredit,
	/// Client ID (formerly FIX 4.2 ClientID)
	#[serde(rename = "3")]
	ClientId,
	/// Clearing Firm (formerly FIX 4.2 ClearingFirm)
	#[serde(rename = "4")]
	ClearingFirm,
	/// Investor ID
	#[serde(rename = "5")]
	InvestorId,
	/// Introducing Firm
	#[serde(rename = "6")]
	IntroducingFirm,
	/// Entering Firm
	#[serde(rename = "7")]
	EnteringFirm,
	/// Locate/Lending Firm (for short-sales)
	#[serde(rename = "8")]
	LocateLendingFirm,
	/// Fund manager Client ID (for CIV)
	#[serde(rename = "9")]
	FundManagerClientId,
	/// Settlement Location (formerly FIX 4.2 SettlLocation)
	#[serde(rename = "10")]
	SettlementLocation,
	/// Order Origination Trader (associated with Order Origination Firm - e.g. trader who initiates/submits the order)
	#[serde(rename = "11")]
	OrderOriginationTrader,
	/// Executing Trader (associated with Executing Firm - actually executes)
	#[serde(rename = "12")]
	ExecutingTrader,
	/// Order Origination Firm (e.g. buyside firm)
	#[serde(rename = "13")]
	OrderOriginationFirm,
	/// Giveup Clearing Firm (firm to which trade is given up)
	#[serde(rename = "14")]
	GiveupClearingFirm,
	/// Correspondant Clearing Firm
	#[serde(rename = "15")]
	CorrespondantClearingFirm,
	/// Executing System
	#[serde(rename = "16")]
	ExecutingSystem,
	/// Contra Firm
	#[serde(rename = "17")]
	ContraFirm,
	/// Contra Clearing Firm
	#[serde(rename = "18")]
	ContraClearingFirm,
	/// Sponsoring Firm
	#[serde(rename = "19")]
	SponsoringFirm,
	/// Underlying Contra Firm
	#[serde(rename = "20")]
	UnderlyingContraFirm,
}

impl Default for NestedPartyRole {
	fn default() -> Self {
		NestedPartyRole::ExecutingFirm
	}
}
