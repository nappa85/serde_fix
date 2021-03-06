
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentExtension {
	/// Identifies the form of delivery.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "668")]
	pub delivery_form: Option<DeliveryForm>,
	/// Percent at risk due to lowest possible call.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "869")]
	pub pct_at_risk: Option<f32>,
	/// Number of repeating InstrAttrib group entries.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "870")]
	pub instr_attrib: Option<fix_common::RepeatingValues<InstrAttri>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrAttri {
	/// Type of instrument attribute
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "871")]
	pub instr_attrib_type: Option<InstrAttribType>,
	/// Value of instrument attribute, if applicable
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "872")]
	pub instr_attrib_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DeliveryForm {
	/// BookEntry
	#[serde(rename = "1")]
	BookEntry,
	/// Bearer
	#[serde(rename = "2")]
	Bearer,
}

impl Default for DeliveryForm {
	fn default() -> Self {
		DeliveryForm::BookEntry
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum InstrAttribType {
	/// Flat (securities pay interest on a current basis but are traded without interest)
	#[serde(rename = "1")]
	Flat,
	/// Zero coupon
	#[serde(rename = "2")]
	ZeroCoupon,
	/// Interest bearing (for Euro commercial paper when not issued at discount)
	#[serde(rename = "3")]
	InterestBearing,
	/// No periodic payments
	#[serde(rename = "4")]
	NoPeriodicPayments,
	/// Variable rate
	#[serde(rename = "5")]
	VariableRate,
	/// Less fee for put
	#[serde(rename = "6")]
	LessFeeForPut,
	/// Stepped coupon
	#[serde(rename = "7")]
	SteppedCoupon,
	/// Coupon period (if not semi-annual). Supply redemption date in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "8")]
	CouponPeriodSupplyRedemptionDateInTheInstrAttribValueField,
	/// When [and if] issued
	#[serde(rename = "9")]
	WhenAndIfIssued,
	/// Original issue discount
	#[serde(rename = "10")]
	OriginalIssueDiscount,
	/// Callable, puttable
	#[serde(rename = "11")]
	CallablePuttable,
	/// Escrowed to Maturity
	#[serde(rename = "12")]
	EscrowedToMaturity,
	/// Escrowed to redemption date - callable. Supply redemption date in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "13")]
	EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrAttribValueField,
	/// Prerefunded
	#[serde(rename = "14")]
	Prerefunded,
	/// In default
	#[serde(rename = "15")]
	InDefault,
	/// Unrated
	#[serde(rename = "16")]
	Unrated,
	/// Taxable
	#[serde(rename = "17")]
	Taxable,
	/// Indexed
	#[serde(rename = "18")]
	Indexed,
	/// Subject to Alternative Minimum Tax
	#[serde(rename = "19")]
	SubjectToAlternativeMinimumTax,
	/// Original issue discount price. Supply price in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "20")]
	OriginalIssueDiscountPriceSupplyPriceInTheInstrAttribValueField,
	/// Callable below maturity value
	#[serde(rename = "21")]
	CallableBelowMaturityValue,
	/// Callable without notice by mail to holder unless registered
	#[serde(rename = "22")]
	CallableWithoutNoticeByMailToHolderUnlessRegistered,
	/// Text. Supply the text of the attribute or disclaimer in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "99")]
	TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrAttribValueField,
}

impl Default for InstrAttribType {
	fn default() -> Self {
		InstrAttribType::Flat
	}
}
