
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
	/// Used to identify class source of NestedPartyID value (e.g. BIC). Required if <a href="tag_524_NestedPartyID.html" target="bottom">NestedPartyID&nbsp;(524)</a> is specified. Required if <a href="tag_539_NoNestedPartyIDs.html" target="bottom">NoNestedPartyIDs&nbsp;(539)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "525")]
	pub nested_party_id_source: Option<char>,
	/// Identifies the type of NestedPartyID (e.g. Executing Broker). Required if <a href="tag_539_NoNestedPartyIDs.html" target="bottom">NoNestedPartyIDs&nbsp;(539)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "538")]
	pub nested_party_role: Option<NestedPartyRole>,
	/// Repeating group of NestedParty sub-identifiers.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "804")]
	pub no_nested_party_sub_i_ds: Option<usize>,
	/// Sub-identifier (e.g. Clearing Acct for <a href="tag_448_PartyID.html" target="bottom">PartyID&nbsp;(448)</a> =Clearing Firm) if applicable. Required if <a href="tag_804_NoNestedPartySubIDs.html" target="bottom">NoNestedPartySubIDs&nbsp;(804)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "545")]
	pub nested_party_sub_id: Option<String>,
	/// Type of Sub-identifier. Required if <a href="tag_804_NoNestedPartySubIDs.html" target="bottom">NoNestedPartySubIDs&nbsp;(804)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "805")]
	pub nested_party_sub_id_type: Option<NestedPartySubIDType>,
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
	/// Clearing Organization
	#[serde(rename = "21")]
	ClearingOrganization,
	/// Exchange
	#[serde(rename = "22")]
	Exchange,
	/// Customer Account
	#[serde(rename = "24")]
	CustomerAccount,
	/// Correspondent Clearing Organization
	#[serde(rename = "25")]
	CorrespondentClearingOrganization,
	/// Correspondent Broker
	#[serde(rename = "26")]
	CorrespondentBroker,
	/// Buyer/Seller (Receiver/Deliverer)
	#[serde(rename = "27")]
	BuyerSeller,
	/// Custodian
	#[serde(rename = "28")]
	Custodian,
	/// Intermediary
	#[serde(rename = "29")]
	Intermediary,
	/// Agent
	#[serde(rename = "30")]
	Agent,
	/// Sub custodian
	#[serde(rename = "31")]
	SubCustodian,
	/// Beneficiary
	#[serde(rename = "32")]
	Beneficiary,
	/// Interested party
	#[serde(rename = "33")]
	InterestedParty,
	/// Regulatory body
	#[serde(rename = "34")]
	RegulatoryBody,
	/// Liquidity provider
	#[serde(rename = "35")]
	LiquidityProvider,
	/// Entering trader
	#[serde(rename = "36")]
	EnteringTrader,
	/// Contra Trader
	#[serde(rename = "37")]
	ContraTrader,
	/// Position Account
	#[serde(rename = "38")]
	PositionAccount,
}

impl Default for NestedPartyRole {
	fn default() -> Self {
		NestedPartyRole::ExecutingFirm
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NestedPartySubIDType {
	/// Firm
	#[serde(rename = "1")]
	N1,
	/// Person
	#[serde(rename = "2")]
	N2,
	/// System
	#[serde(rename = "3")]
	N3,
	/// Application
	#[serde(rename = "4")]
	N4,
	/// Full legal name of firm
	#[serde(rename = "5")]
	N5,
	/// Postal address (inclusive of street address, location, and postal code)
	#[serde(rename = "6")]
	N6,
	/// Phone number
	#[serde(rename = "7")]
	N7,
	/// Email address
	#[serde(rename = "8")]
	N8,
	/// Contact name
	#[serde(rename = "9")]
	N9,
	/// Securities account number (for settlement instructions)
	#[serde(rename = "10")]
	N10,
	/// Registration number (for settlement instructions and confirmations)
	#[serde(rename = "11")]
	N11,
	/// Registered address (for confirmation purposes)
	#[serde(rename = "12")]
	N12,
	/// Regulatory status (for confirmation purposes)
	#[serde(rename = "13")]
	N13,
	/// Registration name (for settlement instructions)
	#[serde(rename = "14")]
	N14,
	/// Cash account number (for settlement instructions)
	#[serde(rename = "15")]
	N15,
	/// BIC
	#[serde(rename = "16")]
	N16,
	/// CSD participant/member code (e.g. Euroclear, DTC, CREST or Kassenverein number)
	#[serde(rename = "17")]
	N17,
	/// Registered address
	#[serde(rename = "18")]
	N18,
	/// Fund/account name
	#[serde(rename = "19")]
	N19,
	/// Telex number
	#[serde(rename = "20")]
	N20,
	/// Fax number
	#[serde(rename = "21")]
	N21,
	/// Securities account name
	#[serde(rename = "22")]
	N22,
	/// Cash account name
	#[serde(rename = "23")]
	N23,
	/// Department
	#[serde(rename = "24")]
	N24,
	/// Location / Desk
	#[serde(rename = "25")]
	N25,
	/// Position Account Type
	#[serde(rename = "26")]
	N26,
}

impl Default for NestedPartySubIDType {
	fn default() -> Self {
		NestedPartySubIDType::N1
	}
}