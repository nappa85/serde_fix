
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NestedInstrumentAttribute {
	/// NoNestedInstrAttrib
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1312")]
	pub nested_instr_attrib: Option<fix_common::RepeatingValues<NestedInstrAttri>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NestedInstrAttri {
	/// Code to represent the type of instrument attribute.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1210")]
	pub nested_instr_attrib_type: Option<NestedInstrAttribType>,
	/// Attribute value appropriate to the <a href="tag_1210_NestedInstrAttribType.html" target="bottom">NestedInstrAttribType field&nbsp;(1210)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1211")]
	pub nested_instr_attrib_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NestedInstrAttribType {
	/// Flat (securities pay interest on a current basis but are traded without interest)
	#[serde(rename = "1")]
	N1,
	/// Zero coupon
	#[serde(rename = "2")]
	N2,
	/// Interest bearing (for Euro commercial paper when not issued at discount)
	#[serde(rename = "3")]
	N3,
	/// No periodic payments
	#[serde(rename = "4")]
	N4,
	/// Variable rate
	#[serde(rename = "5")]
	N5,
	/// Less fee for put
	#[serde(rename = "6")]
	N6,
	/// Stepped coupon
	#[serde(rename = "7")]
	N7,
	/// Coupon period (if not semi-annual). Supply redemption date in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "8")]
	N8,
	/// When [and if] issued
	#[serde(rename = "9")]
	N9,
	/// Original issue discount
	#[serde(rename = "10")]
	N10,
	/// Callable, puttable
	#[serde(rename = "11")]
	N11,
	/// Escrowed to Maturity
	#[serde(rename = "12")]
	N12,
	/// Escrowed to redemption date - callable. Supply redemption date in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "13")]
	N13,
	/// Prerefunded
	#[serde(rename = "14")]
	N14,
	/// In default
	#[serde(rename = "15")]
	N15,
	/// Unrated
	#[serde(rename = "16")]
	N16,
	/// Taxable
	#[serde(rename = "17")]
	N17,
	/// Indexed
	#[serde(rename = "18")]
	N18,
	/// Subject to Alternative Minimum Tax
	#[serde(rename = "19")]
	N19,
	/// Original issue discount price. Supply price in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "20")]
	N20,
	/// Callable below maturity value
	#[serde(rename = "21")]
	N21,
	/// Callable without notice by mail to holder unless registered
	#[serde(rename = "22")]
	N22,
	/// Price tick rules for security.
	#[serde(rename = "23")]
	N23,
	/// Trade type eligibility details for security.
	#[serde(rename = "24")]
	N24,
	/// Instrument Denominator
	#[serde(rename = "25")]
	N25,
	/// Instrument Numerator
	#[serde(rename = "26")]
	N26,
	/// Instrument Price Precision
	#[serde(rename = "27")]
	N27,
	/// Instrument Strike Price
	#[serde(rename = "28")]
	N28,
	/// Tradeable Indicator
	#[serde(rename = "29")]
	N29,
	/// Text. Supply the text of the attribute or disclaimer in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "99")]
	N99,
	/// Instrument is eligible to accept anonymous orders
	#[serde(rename = "30")]
	N30,
	/// Minimum guaranteed fill volume
	#[serde(rename = "31")]
	N31,
	/// Minimum Guaranteed Fill Status
	#[serde(rename = "33")]
	N33,
	/// Test instrument
	#[serde(rename = "34")]
	N34,
	/// Dummy instrument
	#[serde(rename = "35")]
	N35,
	/// Minimum guaranteed fill status
	#[serde(rename = "32")]
	N32,
	/// Negative settlement price eligibility
	#[serde(rename = "36")]
	N36,
	/// Negative strike price eligibility
	#[serde(rename = "37")]
	N37,
	/// US standard contract indicator
	#[serde(rename = "38")]
	N38,
	/// Admitted to trading on a trading venue
	#[serde(rename = "39")]
	N39,
	/// Average daily notional amount
	#[serde(rename = "40")]
	N40,
	/// Average daily number of trades
	#[serde(rename = "41")]
	N41,
}

impl Default for NestedInstrAttribType {
	fn default() -> Self {
		NestedInstrAttribType::N1
	}
}
