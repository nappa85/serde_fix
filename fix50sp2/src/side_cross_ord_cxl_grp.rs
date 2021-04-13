
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideCrossOrdCxlGrp {
	/// Must be 1 or 2
	#[serde(rename = "552")]
	pub no_sides: NoSides,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NoSide {
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Required when referring to orders that were electronically submitted over FIX or otherwise assigned a <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID(11)&nbsp;(11)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// Unique identifier of the order as assigned by institution or by the intermediary with closest association with the investor.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// ClOrdLinkID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "583")]
	pub cl_ord_link_id: Option<String>,
	/// OrigOrdModTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "586")]
	pub orig_ord_mod_time: Option<fix_common::UTCTimestamp>,
	/// TradeOriginationDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "229")]
	pub trade_origination_date: Option<fix_common::LocalMktDate>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
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
	/// ComplianceText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2404")]
	pub compliance_text: Option<String>,
	/// Must be set if EncodedComplianceText(2352) field is specified and must immediately precede it.
	#[serde(rename = "2351")]
	/// Encoded (non-ASCII characters) representation of the ComplianceText(2404) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2352")]
	pub encoded_compliance_text: Option<fix_common::EncodedText<2352>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NoSides {
	/// One Side
	#[serde(rename = "1")]
	OneSide,
	/// Both Sides
	#[serde(rename = "2")]
	BothSides,
}

impl Default for NoSides {
	fn default() -> Self {
		NoSides::OneSide
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Side {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for IOI and List Order messages only)
	#[serde(rename = "7")]
	Undisclosed,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
	/// Cross short exempt
	#[serde(rename = "A")]
	CrossShortExempt,
	/// "As Defined" (for use with multileg instruments)
	#[serde(rename = "B")]
	AsDefined,
	/// "Opposite" (for use with multileg instruments)
	#[serde(rename = "C")]
	Opposite,
	/// Subscribe (e.g. CIV)
	#[serde(rename = "D")]
	Subscribe,
	/// Redeem (e.g. CIV)
	#[serde(rename = "E")]
	Redeem,
	/// Lend (FINANCING - identifies direction of collateral)
	#[serde(rename = "F")]
	Lend,
	/// Borrow (FINANCING - identifies direction of collateral)
	#[serde(rename = "G")]
	Borrow,
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}
