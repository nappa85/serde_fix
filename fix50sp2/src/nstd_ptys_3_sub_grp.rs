
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NstdPtys3SubGrp {
	/// NoNested3PartySubIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "952")]
	pub nested_3_party_sub_i_ds: Option<fix_common::RepeatingValues<Nested3PartySubID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Nested3PartySubID {
	/// Nested3PartySubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "953")]
	pub nested_3_party_sub_id: Option<String>,
	/// Nested3PartySubIDType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "954")]
	pub nested_3_party_sub_id_type: Option<Nested3PartySubIDType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Nested3PartySubIDType {
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
	/// Postal address
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
	/// CSD participant member code
	#[serde(rename = "17")]
	N17,
	/// Registered address
	#[serde(rename = "18")]
	N18,
	/// Fund account name
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
	/// Location desk
	#[serde(rename = "25")]
	N25,
	/// Position account type
	#[serde(rename = "26")]
	N26,
	/// Security locate ID
	#[serde(rename = "27")]
	N27,
	/// Market maker
	#[serde(rename = "28")]
	N28,
	/// Eligible counterparty
	#[serde(rename = "29")]
	N29,
	/// Professional client
	#[serde(rename = "30")]
	N30,
	/// Location
	#[serde(rename = "31")]
	N31,
	/// Execution venue
	#[serde(rename = "32")]
	N32,
	/// Currency delivery identifier
	#[serde(rename = "33")]
	N33,
	/// Address City
	#[serde(rename = "34")]
	N34,
	/// Address State/Province
	#[serde(rename = "35")]
	N35,
	/// Address Postal Code
	#[serde(rename = "36")]
	N36,
	/// Address Street
	#[serde(rename = "37")]
	N37,
	/// Address Country (ISO country code)
	#[serde(rename = "38")]
	N38,
	/// ISO country code
	#[serde(rename = "39")]
	N39,
	/// MarketSegment
	#[serde(rename = "40")]
	N40,
	/// Customer account type
	#[serde(rename = "41")]
	N41,
	/// Omnibus account
	#[serde(rename = "42")]
	N42,
	/// Funds segregation type
	#[serde(rename = "43")]
	N43,
	/// Guarantee fund (Elaboration: Identifies a guarantee fund related to an account. Used when one account has multiple funds of
	/// collateral, each guaranteeing different positions. Can be used for PartyRole(452) = Customer Account(24))
	#[serde(rename = "44")]
	N44,
	/// Swap dealer
	#[serde(rename = "45")]
	N45,
	/// Major participant
	#[serde(rename = "46")]
	N46,
	/// Financial entity
	#[serde(rename = "47")]
	N47,
	/// U.S. person
	#[serde(rename = "48")]
	N48,
	/// Reporting entity indicator
	#[serde(rename = "49")]
	N49,
	/// Elected clearing requirement exception
	#[serde(rename = "50")]
	N50,
	/// Business center
	#[serde(rename = "51")]
	N51,
	/// Reference text
	#[serde(rename = "52")]
	N52,
	/// Short-marking exempt account
	#[serde(rename = "53")]
	N53,
	/// Parent firm identifier (Implementation-specific identifier of this party's parent entity)
	#[serde(rename = "54")]
	N54,
	/// Parent firm name
	#[serde(rename = "55")]
	N55,
	/// Deal identifier (The internal identifier assigned to the trade by this party, particularly by a Clearing Organization)
	#[serde(rename = "56")]
	N56,
	/// System trade identifier
	#[serde(rename = "57")]
	N57,
	/// System trade sub-identifier
	#[serde(rename = "58")]
	N58,
	/// Futures Commission Merchant (FCM) code (The FCM's code or identifier in relation to the PartyRole(452). For example, if PartyRole(452)
	/// is the exchange or clearinghouse, the FCM code/ID specified in PartySubID(523) is the FCM's identifier at the exchange or
	/// clearinghouse)
	#[serde(rename = "59")]
	N59,
	/// Delivery terminal customer account/code (Usually used for gas delivery to identify whose account the gas is allocated to at
	/// the delivery terminal. Often referred to as "HUB" code)
	#[serde(rename = "60")]
	N60,
	/// Voluntary reporting entity (The entity voluntarily reporting the trade to the regulator. Set PartySubID(523)=Y if true)
	#[serde(rename = "61")]
	N61,
	/// Reporting obligation jurisdiction (For a trade that falls under multiple jurisdictions this may be used to identify, through
	/// PartySubID(523), the reporting jurisdiction to which the party is obligated to report)
	#[serde(rename = "62")]
	N62,
	/// Voluntary reporting jurisdiction (For a trade that falls under multiple jurisdictions this may be used to identify, through
	/// PartySubID(523), the regulatory jurisdiction to which the party is submitting a voluntary report)
	#[serde(rename = "63")]
	N63,
	/// Company Activities
	#[serde(rename = "64")]
	N64,
	/// European Economic Area domiciled
	#[serde(rename = "65")]
	N65,
	/// Contract linked to commercial or treasury financing for this counterparty
	#[serde(rename = "66")]
	N66,
	/// Contract above clearing threshold for this counterparty
	#[serde(rename = "67")]
	N67,
	/// Voluntary reporting party (When PartySubID(523)=Y, identifies that the party is reporting voluntarily when VoluntaryRegulatoryReport(1935)
	/// = Y)
	#[serde(rename = "68")]
	N68,
	/// End user (When PartySubID(523)=Y the counterparty is neither the swap dealer, major swap participant nor financial entity
	/// as defined in the regulations)
	#[serde(rename = "69")]
	N69,
	/// Location or jurisdiction
	#[serde(rename = "70")]
	N70,
	/// Derivatives dealer (Elaboration: Indicates whether the party is a derivatives dealer or not (Y/N). The Canadian regulator's
	/// defined term for identifying the trade counterparty as "a person or company engaging in or holding himself, herself or itself
	/// out as engaging in the business of trading in derivatives in Ontario as principal or agent")
	#[serde(rename = "71")]
	N71,
	/// Domicile (Elaboration: Country and optionally province, state or region of domicile. The party sub-ID value is either a 2-character
	/// ISO 3166 country code or a hyphenated combination of the country code and the standard post-office abbreviation of province,
	/// state or region if necessary. E.g. "US" for United States or "CA-QC" for Quebec Canada)
	#[serde(rename = "72")]
	N72,
	/// Exempt from recognition (Elaboration: Used with party role 21 "Clearing Organization" to indicate exemption (Y/N). Identifies
	/// a clearing agency as exempt from oversight in Ontario, i.e. one that 1) only provides limited services and does not present
	/// significant risks or 2) is foreign-based, indends to operate in Ontario but is subject to regulatory oversight in another
	/// jurisdiction)
	#[serde(rename = "73")]
	N73,
	/// Payer
	#[serde(rename = "74")]
	N74,
	/// Receiver
	#[serde(rename = "75")]
	N75,
	/// Systematic Internaliser (SI)
	#[serde(rename = "76")]
	N76,
	/// Publishing entity indicator
	#[serde(rename = "77")]
	N77,
	/// First name
	#[serde(rename = "78")]
	N78,
	/// Surname
	#[serde(rename = "79")]
	N79,
	/// Date of birth
	#[serde(rename = "80")]
	N80,
	/// Order transmitting firm
	#[serde(rename = "81")]
	N81,
	/// Order transmitting firm for buyer
	#[serde(rename = "82")]
	N82,
	/// Order transmitter for seller
	#[serde(rename = "83")]
	N83,
	/// Legal Entity Identifier (ISO 17442) LEI
	#[serde(rename = "84")]
	N84,
	/// Sub-sector classification
	#[serde(rename = "85")]
	N85,
	/// Party side
	#[serde(rename = "86")]
	N86,
	/// Legal registration country
	#[serde(rename = "87")]
	N87,
}

impl Default for Nested3PartySubIDType {
	fn default() -> Self {
		Nested3PartySubIDType::N1
	}
}
