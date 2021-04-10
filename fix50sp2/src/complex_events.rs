
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEvents {
	/// NoComplexEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1483")]
	pub complex_events: Option<crate::entities::RepeatingValues<ComplexEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEvent {
	/// Required if NoComplexEvents(1483) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1484")]
	pub complex_event_type: Option<ComplexEventType>,
	/// ComplexOptPayoutAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1485")]
	pub complex_opt_payout_amount: Option<f64>,
	/// ComplexEventPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1486")]
	pub complex_event_price: Option<f64>,
	/// ComplexEventPriceBoundaryMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1487")]
	pub complex_event_price_boundary_method: Option<ComplexEventPriceBoundaryMethod>,
	/// ComplexEventPriceBoundaryPrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1488")]
	pub complex_event_price_boundary_precision: Option<f32>,
	/// ComplexEventPriceTimeType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1489")]
	pub complex_event_price_time_type: Option<ComplexEventPriceTimeType>,
	/// Conditionally required when there are more than one ComplexEvents occurrences. A chain of ComplexEvents must be linked together
	/// through use of the ComplexEventCondition(1490) in which the relationship between any two events is described. For any two
	/// ComplexEvents the first occurrence will specify the ComplexEventCondition(1490) which links it with the second event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1490")]
	pub complex_event_condition: Option<ComplexEventCondition>,
	/// ComplexOptPayoutPaySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2117")]
	pub complex_opt_payout_pay_side: Option<ComplexOptPayoutPaySide>,
	/// ComplexOptPayoutReceiveSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2118")]
	pub complex_opt_payout_receive_side: Option<ComplexOptPayoutReceiveSide>,
	/// ComplexOptPayoutUnderlier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2119")]
	pub complex_opt_payout_underlier: Option<String>,
	/// ComplexOptPayoutPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2120")]
	pub complex_opt_payout_percentage: Option<f32>,
	/// ComplexOptPayoutTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2121")]
	pub complex_opt_payout_time: Option<ComplexOptPayoutTime>,
	/// ComplexOptPayoutCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2122")]
	pub complex_opt_payout_currency: Option<String>,
	/// ComplexEventPricePercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2123")]
	pub complex_event_price_percentage: Option<f32>,
	/// ComplexEventCurrencyOne
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2124")]
	pub complex_event_currency_one: Option<String>,
	/// ComplexEventCurrencyTwo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2125")]
	pub complex_event_currency_two: Option<String>,
	/// ComplexEventQuoteBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2126")]
	pub complex_event_quote_basis: Option<ComplexEventQuoteBasis>,
	/// ComplexEventFixedFXRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2127")]
	pub complex_event_fixed_fx_rate: Option<f64>,
	/// ComplexEventDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2128")]
	pub complex_event_determination_method: Option<String>,
	/// ComplexEventCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2129")]
	pub complex_event_calculation_agent: Option<ComplexEventCalculationAgent>,
	/// ComplexEventStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2130")]
	pub complex_event_strike_price: Option<f64>,
	/// ComplexEventStrikeFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2131")]
	pub complex_event_strike_factor: Option<f64>,
	/// ComplexEventStrikeNumberOfOptions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2132")]
	pub complex_event_strike_number_of_options: Option<i32>,
	/// ComplexEventCreditEventsXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2133")]
	pub complex_event_credit_events_xid_ref: Option<String>,
	/// ComplexEventCreditEventNotifyingParty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2134")]
	pub complex_event_credit_event_notifying_party: Option<ComplexEventCreditEventNotifyingParty>,
	/// ComplexEventCreditEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2135")]
	pub complex_event_credit_event_business_center: Option<String>,
	/// ComplexEventCreditEventStandardSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2136")]
	pub complex_event_credit_event_standard_sources: Option<crate::entities::Boolean>,
	/// ComplexEventCreditEventMinimumSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2137")]
	pub complex_event_credit_event_minimum_sources: Option<i32>,
	/// ComplexEventXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2138")]
	pub complex_event_xid: Option<String>,
	/// ComplexEventXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2139")]
	pub complex_event_xid_ref: Option<String>,
	/// ComplexEventSpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2407")]
	pub complex_event_spot_rate: Option<f64>,
	/// ComplexEventForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2408")]
	pub complex_event_forward_points: Option<f64>,
	/// ComplexEventFuturesPriceValuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2597")]
	pub complex_event_futures_price_valuation: Option<crate::entities::Boolean>,
	/// ComplexEventOptionsPriceValuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2598")]
	pub complex_event_options_price_valuation: Option<crate::entities::Boolean>,
	/// ComplexEventPVFinalPriceElectionFallback
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2599")]
	pub complex_event_pv_final_price_election_fallback: Option<ComplexEventPVFinalPriceElectionFallback>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventType {
	/// Capped
	#[serde(rename = "1")]
	Capped,
	/// Trigger
	#[serde(rename = "2")]
	Trigger,
	/// Knock-in up
	#[serde(rename = "3")]
	KnockInUp,
	/// Knock-in down
	#[serde(rename = "4")]
	KnockInDown,
	/// Knock-out up
	#[serde(rename = "5")]
	KnockOutUp,
	/// Knock-out down
	#[serde(rename = "6")]
	KnockOutDown,
	/// Underlying
	#[serde(rename = "7")]
	Underlying,
	/// Reset Barrier
	#[serde(rename = "8")]
	ResetBarrier,
	/// Rolling Barrier
	#[serde(rename = "9")]
	RollingBarrier,
	/// One-touch
	#[serde(rename = "10")]
	OneTouch,
	/// No-touch
	#[serde(rename = "11")]
	NoTouch,
	/// Double one-touch
	#[serde(rename = "12")]
	DoubleOneTouch,
	/// Double no-touch
	#[serde(rename = "13")]
	DoubleNoTouch,
	/// Foreign exchange composite
	#[serde(rename = "14")]
	ForeignExchangeComposite,
	/// Foreign exchange Quanto
	#[serde(rename = "15")]
	ForeignExchangeQuanto,
	/// Foreign exchange cross currency
	#[serde(rename = "16")]
	ForeignExchangeCrossCurrency,
	/// Strike spread
	#[serde(rename = "17")]
	StrikeSpread,
	/// Calendar spread
	#[serde(rename = "18")]
	CalendarSpread,
	/// Price observation (Asian or Lookback)
	#[serde(rename = "19")]
	PriceObservation,
	/// Pass-through
	#[serde(rename = "20")]
	PassThrough,
	/// Strike schedule
	#[serde(rename = "21")]
	StrikeSchedule,
	/// Equity valuation
	#[serde(rename = "22")]
	EquityValuation,
	/// Dividend valuation
	#[serde(rename = "23")]
	DividendValuation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventPriceBoundaryMethod {
	/// Less than <a href="tag_1486_ComplexEventPrice.html" target="bottom">ComplexEventPrice&nbsp;(1486)</a>
	#[serde(rename = "1")]
	LessThanAHrefTag1486ComplexEventPriceHtmlTargetBottomComplexEventPriceNbspA,
	/// Less than or equal to <a href="tag_1486_ComplexEventPrice.html" target="bottom">ComplexEventPrice&nbsp;(1486)</a>
	#[serde(rename = "2")]
	LessThanOrEqualToAHrefTag1486ComplexEventPriceHtmlTargetBottomComplexEventPriceNbspA,
	/// Equal to <a href="tag_1486_ComplexEventPrice.html" target="bottom">ComplexEventPrice&nbsp;(1486)</a>
	#[serde(rename = "3")]
	EqualToAHrefTag1486ComplexEventPriceHtmlTargetBottomComplexEventPriceNbspA,
	/// Greater than or equal to <a href="tag_1486_ComplexEventPrice.html" target="bottom">ComplexEventPrice&nbsp;(1486)</a>
	#[serde(rename = "4")]
	GreaterThanOrEqualToAHrefTag1486ComplexEventPriceHtmlTargetBottomComplexEventPriceNbspA,
	/// Greater than <a href="tag_1486_ComplexEventPrice.html" target="bottom">ComplexEventPrice&nbsp;(1486)</a>
	#[serde(rename = "5")]
	GreaterThanAHrefTag1486ComplexEventPriceHtmlTargetBottomComplexEventPriceNbspA,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventPriceTimeType {
	/// Expiration
	#[serde(rename = "1")]
	Expiration,
	/// Immediate (At Any Time)
	#[serde(rename = "2")]
	Immediate,
	/// Specified Date/Time
	#[serde(rename = "3")]
	SpecifiedDateTime,
	/// Close
	#[serde(rename = "4")]
	Close,
	/// Open
	#[serde(rename = "5")]
	Open,
	/// Official settlement price
	#[serde(rename = "6")]
	OfficialSettlementPrice,
	/// Derivatives close
	#[serde(rename = "7")]
	DerivativesClose,
	/// As specified in Master Confirmation
	#[serde(rename = "8")]
	AsSpecifiedInMasterConfirmation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventCondition {
	/// And
	#[serde(rename = "1")]
	And,
	/// Or
	#[serde(rename = "2")]
	Or,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexOptPayoutPaySide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexOptPayoutReceiveSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexOptPayoutTime {
	/// Close
	#[serde(rename = "0")]
	Close,
	/// Open
	#[serde(rename = "1")]
	Open,
	/// Official settlement
	#[serde(rename = "2")]
	OfficialSettlement,
	/// Valuation time
	#[serde(rename = "3")]
	ValuationTime,
	/// Exchange settlement time
	#[serde(rename = "4")]
	ExchangeSettlementTime,
	/// Derivatives close
	#[serde(rename = "5")]
	DerivativesClose,
	/// As specified in master confirmation
	#[serde(rename = "6")]
	AsSpecifiedInMasterConfirmation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventQuoteBasis {
	/// Currency 1 per currency 2
	#[serde(rename = "0")]
	Currency1PerCurrency2,
	/// Currency 2 per currency 1
	#[serde(rename = "1")]
	Currency2PerCurrency1,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventCalculationAgent {
	/// Exercising party
	#[serde(rename = "0")]
	ExercisingParty,
	/// Non-exercising party
	#[serde(rename = "1")]
	NonExercisingParty,
	/// As specified in the master agreement
	#[serde(rename = "2")]
	AsSpecifiedInTheMasterAgreement,
	/// As specified in the standard terms supplement
	#[serde(rename = "3")]
	AsSpecifiedInTheStandardTermsSupplement,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventCreditEventNotifyingParty {
	/// Seller notifies
	#[serde(rename = "0")]
	SellerNotifies,
	/// Buyer notifies
	#[serde(rename = "1")]
	BuyerNotifies,
	/// Seller or buyer notifies
	#[serde(rename = "2")]
	SellerOrBuyerNotifies,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventPVFinalPriceElectionFallback {
	/// Close
	#[serde(rename = "0")]
	Close,
	/// Hedge election
	#[serde(rename = "1")]
	HedgeElection,
}
