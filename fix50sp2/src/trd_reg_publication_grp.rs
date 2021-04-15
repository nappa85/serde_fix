
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegPublicationGrp {
	/// NoTrdRegPublications
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2668")]
	pub trd_reg_publications: Option<fix_common::RepeatingValues<TrdRegPublication>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegPublication {
	/// Required if NoTrdRegPublications(2668) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2669")]
	pub trd_reg_publication_type: Option<TrdRegPublicationType>,
	/// TrdRegPublicationReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2670")]
	pub trd_reg_publication_reason: Option<TrdRegPublicationReason>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TrdRegPublicationType {
	/// Pre-trade transparency waiver
	#[serde(rename = "0")]
	PreTradeTransparencyWaiver,
	/// Post-trade deferral
	#[serde(rename = "1")]
	PostTradeDeferral,
	/// Exempt from publication
	#[serde(rename = "2")]
	ExemptFromPublication,
	/// </td>
	/// </tr>
	/// <tr valign="top">
	/// <td class="val">'4'</td>
	/// <td class="val-descr">
	#[serde(rename = "3")]
	TdTrTrValignTopTdClassVal4TdTdClassValDescr,
	/// </td>
	/// </tr>
	/// <tr valign="top">
	/// <td class="val">'6'</td>
	/// <td class="val-descr">
	#[serde(rename = "5")]
	TdTrTrValignTopTdClassVal6TdTdClassValDescr,
}

impl Default for TrdRegPublicationType {
	fn default() -> Self {
		TrdRegPublicationType::PreTradeTransparencyWaiver
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TrdRegPublicationReason {
	/// No preceding order in book as transaction price set within average spread of a liquid instrument
	#[serde(rename = "0")]
	NoPrecedingOrderInBookAsTransactionPriceSetWithinAverageSpreadOfALiquidInstrument,
	/// No preceding order in book as transaction price depends on system-set reference price for an illiquid instrument
	#[serde(rename = "1")]
	NoPrecedingOrderInBookAsTransactionPriceDependsOnSystemSetReferencePriceForAnIlliquidInstrument,
	/// No preceding order in book as transaction price is for transaction subject to conditions other than current market price
	#[serde(rename = "2")]
	NoPrecedingOrderInBookAsTransactionPriceIsForTransactionSubjectToConditionsOtherThanCurrentMarketPrice,
	/// No public price for preceding order as public reference price was used for matching orders
	#[serde(rename = "3")]
	NoPublicPriceForPrecedingOrderAsPublicReferencePriceWasUsedForMatchingOrders,
	/// No public price quoted as instrument is illiquid
	#[serde(rename = "4")]
	NoPublicPriceQuotedAsInstrumentIsIlliquid,
	/// No public price quoted due to "Size"
	#[serde(rename = "5")]
	NoPublicPriceQuotedDueToSize,
	/// Deferral due to "Large in Scale"
	#[serde(rename = "6")]
	DeferralDueToLargeInScale,
	/// Deferral due to "Illiquid Instrument"
	#[serde(rename = "7")]
	DeferralDueToIlliquidInstrument,
	/// Deferral due to "Size Specific"
	#[serde(rename = "8")]
	DeferralDueToSizeSpecific,
	/// No public price and/or size quoted as transaction is "large in scale"
	#[serde(rename = "9")]
	NoPublicPriceAndOrSizeQuotedAsTransactionIsLargeInScale,
	/// No public price and/or size quoted due to order being hidden
	#[serde(rename = "10")]
	NoPublicPriceAndOrSizeQuotedDueToOrderBeingHidden,
	/// Exempted due to securities financing transaction
	#[serde(rename = "11")]
	ExemptedDueToSecuritiesFinancingTransaction,
	/// Exempted due to European System of Central Banks (ESCB) policy transaction
	#[serde(rename = "12")]
	ExemptedDueToEuropeanSystemOfCentralBanksPolicyTransaction,
	/// Exception due to report by paper
	#[serde(rename = "13")]
	ExceptionDueToReportByPaper,
	/// Exception due to trade with non-reporting party
	#[serde(rename = "14")]
	ExceptionDueToTradeWithNonReportingParty,
	/// Exception due to intra-firm order
	#[serde(rename = "15")]
	ExceptionDueToIntraFirmOrder,
}

impl Default for TrdRegPublicationReason {
	fn default() -> Self {
		TrdRegPublicationReason::NoPrecedingOrderInBookAsTransactionPriceSetWithinAverageSpreadOfALiquidInstrument
	}
}
