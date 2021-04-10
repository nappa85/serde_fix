
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocAckGrp {
	/// Indicates number of allocation groups to follow.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "78")]
	pub allocs: Option<fix_common::RepeatingValues<Alloc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Alloc {
	/// Required if NoAllocs &gt; 0. Must be first field in repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "79")]
	pub alloc_account: Option<String>,
	/// AllocAcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "661")]
	pub alloc_acct_id_source: Option<AllocAcctIDSource>,
	/// Used when performing "executed price" vs. "average price" allocations (e.g. Japan). AllocAccount plus AllocPrice form a unique
	/// Allocs entry. Used in lieu of AllocAvgPx.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "366")]
	pub alloc_price: Option<f64>,
	/// AllocPositionEffect
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1047")]
	pub alloc_position_effect: Option<AllocPositionEffect>,
	/// IndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "467")]
	pub individual_alloc_id: Option<String>,
	/// Required if NoAllocs &gt; 0 and <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus(87)&nbsp;(87)</a> = 2 (Account level reject).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "776")]
	pub individual_alloc_rej_code: Option<IndividualAllocRejCode>,
	/// AllocHandlInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "209")]
	pub alloc_handl_inst: Option<AllocHandlInst>,
	/// Can be used here to hold text relating to the rejection of this <a href="tag_366_AllocPrice.html" target="bottom">AllocAccount&nbsp;(366)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "161")]
	pub alloc_text: Option<String>,
	/// Must be set if <a href="tag_361_EncodedAllocText.html" target="bottom">EncodedAllocText&nbsp;(361)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "360")]
	pub encoded_alloc_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the AllocText field in the encoded format specified via the MessageEncoding
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "361")]
	pub encoded_alloc_text: Option<String>,
	/// SecondaryIndividualAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "989")]
	pub secondary_individual_alloc_id: Option<String>,
	/// AllocCustomerCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "993")]
	pub alloc_customer_capacity: Option<String>,
	/// IndividualAllocType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "992")]
	pub individual_alloc_type: Option<IndividualAllocType>,
	/// AllocQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "80")]
	pub alloc_qty: Option<f64>,
	/// FirmMnemonic
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1729")]
	pub firm_mnemonic: Option<String>,
	/// FirmAllocText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1732")]
	pub firm_alloc_text: Option<String>,
	/// Must be set if <a href="tag_1734_EncodedFirmAllocText.html" target="bottom">EncodedFirmAllocText (1734)&nbsp;(1734)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1733")]
	pub encoded_firm_alloc_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1732_FirmAllocText.html" target="bottom">FirmAllocText(1732)&nbsp;(1732)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1734")]
	pub encoded_firm_alloc_text: Option<String>,
	/// Only used for specific lot trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1752")]
	pub custodial_lot_id: Option<String>,
	/// Only used for specific lot trades. If this field is used, either <a href="tag_1754_VersusPurchasePrice.html" target="bottom">VersusPurchasePrice(1754)&nbsp;(1754)</a> or <a href="tag_1755_CurrentCostBasis.html" target="bottom">CurrentCostBasis(1755)&nbsp;(1755)</a> should be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1753")]
	pub versus_purchase_date: Option<fix_common::LocalMktDate>,
	/// Only used for specific lot trades. If this field is used, <a href="tag_1753_VersusPurchaseDate.html" target="bottom">VersusPurchaseDate(1753)&nbsp;(1753)</a> should be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1754")]
	pub versus_purchase_price: Option<f64>,
	/// Only used for specific lot trades. If this field is used, <a href="tag_1753_VersusPurchaseDate.html" target="bottom">VersusPurchaseDate(1753)&nbsp;(1753)</a> should be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1755")]
	pub current_cost_basis: Option<f64>,
	/// AllocAvgPxIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2769")]
	pub alloc_avg_px_indicator: Option<AllocAvgPxIndicator>,
	/// AllocAvgPxGroupID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2770")]
	pub alloc_avg_px_group_id: Option<String>,
	/// ParentAllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1593")]
	pub parent_alloc_id: Option<String>,
	/// AllocCalculatedCcyQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2515")]
	pub alloc_calculated_ccy_qty: Option<f64>,
	/// Used to communicate the status of central clearing workflow.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1832")]
	pub cleared_indicator: Option<ClearedIndicator>,
	/// The field may not be used in any message where there are no legs.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2727")]
	pub alloc_leg_ref_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocAcctIDSource {
	/// BIC
	#[serde(rename = "1")]
	Bic,
	/// SID code
	#[serde(rename = "2")]
	SidCode,
	/// TFM (GSPTA)
	#[serde(rename = "3")]
	Tfm,
	/// OMGEO (AlertID)
	#[serde(rename = "4")]
	Omgeo,
	/// DTCC code
	#[serde(rename = "5")]
	DtccCode,
	/// Special Segregated Account ID
	#[serde(rename = "6")]
	SpecialSegregatedAccountId,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocPositionEffect {
	/// Open
	#[serde(rename = "O")]
	Open,
	/// Close
	#[serde(rename = "C")]
	Close,
	/// Rolled
	#[serde(rename = "R")]
	Rolled,
	/// FIFO
	#[serde(rename = "F")]
	Fifo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IndividualAllocRejCode {
	/// Unknown account(s)
	#[serde(rename = "0")]
	UnknownAccount,
	/// Incorrect quantity
	#[serde(rename = "1")]
	IncorrectQuantity,
	/// Incorrect averageg price
	#[serde(rename = "2")]
	IncorrectAveragegPrice,
	/// Unknown executing broker mnemonic
	#[serde(rename = "3")]
	UnknownExecutingBrokerMnemonic,
	/// Commission difference
	#[serde(rename = "4")]
	CommissionDifference,
	/// Unknown OrderID (37)
	#[serde(rename = "5")]
	UnknownOrderId,
	/// Unknown ListID (66)
	#[serde(rename = "6")]
	UnknownListId,
	/// Other (further in Text (58))
	#[serde(rename = "7")]
	Other,
	/// Incorrect allocated quantity
	#[serde(rename = "8")]
	IncorrectAllocatedQuantity,
	/// Calculation difference
	#[serde(rename = "9")]
	CalculationDifference,
	/// Unknown or stale ExecID
	#[serde(rename = "10")]
	UnknownOrStaleExecId,
	/// Mismatched data
	#[serde(rename = "11")]
	MismatchedData,
	/// Unknown ClOrdID
	#[serde(rename = "12")]
	UnknownClOrdId,
	/// Warehouse request rejected
	#[serde(rename = "13")]
	WarehouseRequestRejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocHandlInst {
	/// Match
	#[serde(rename = "1")]
	Match,
	/// Forward
	#[serde(rename = "2")]
	Forward,
	/// Forward and Match
	#[serde(rename = "3")]
	ForwardAndMatch,
	/// Auto claim give-up (Indicates that the give-up and take-up party are the same and that trade give-up is to be claimed automatically)
	#[serde(rename = "4")]
	AutoClaimGiveUp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IndividualAllocType {
	/// Sub Allocate
	#[serde(rename = "1")]
	SubAllocate,
	/// Third Party Allocation
	#[serde(rename = "2")]
	ThirdPartyAllocation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocAvgPxIndicator {
	/// No Average Pricing
	#[serde(rename = "0")]
	NoAveragePricing,
	/// Trade is part of an average price group identified by the AvgPxGroupID(1731)
	#[serde(rename = "1")]
	TradeIsPartOfAnAveragePriceGroupIdentifiedByTheAvgPxGroupId,
	/// Last trade of the average price group identified by the AvgPxGroupID(1731)
	#[serde(rename = "2")]
	LastTradeOfTheAveragePriceGroupIdentifiedByTheAvgPxGroupId,
	/// Trade is part of a notional value average price group
	#[serde(rename = "3")]
	TradeIsPartOfANotionalValueAveragePriceGroup,
	/// Trade is average priced
	#[serde(rename = "4")]
	TradeIsAveragePriced,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClearedIndicator {
	/// Not cleared
	#[serde(rename = "0")]
	NotCleared,
	/// Cleared
	#[serde(rename = "1")]
	Cleared,
	/// Submitted
	#[serde(rename = "2")]
	Submitted,
	/// Rejected
	#[serde(rename = "3")]
	Rejected,
}
