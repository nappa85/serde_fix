
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEvents {
	/// NoUnderlyingComplexEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2045")]
	pub underlying_complex_events: Option<fix_common::RepeatingValues<UnderlyingComplexEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEvent {
	/// Required if NoUnderlyingComplexEvents(2045) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2046")]
	pub underlying_complex_event_type: Option<UnderlyingComplexEventType>,
	/// UnderlyingComplexOptPayoutAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2047")]
	pub underlying_complex_opt_payout_amount: Option<f64>,
	/// UnderlyingComplexEventPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2048")]
	pub underlying_complex_event_price: Option<f64>,
	/// UnderlyingComplexEventPriceBoundaryMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2049")]
	pub underlying_complex_event_price_boundary_method: Option<UnderlyingComplexEventPriceBoundaryMethod>,
	/// UnderlyingComplexEventPriceBoundaryPrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2050")]
	pub underlying_complex_event_price_boundary_precision: Option<f32>,
	/// UnderlyingComplexEventPriceTimeType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2051")]
	pub underlying_complex_event_price_time_type: Option<UnderlyingComplexEventPriceTimeType>,
	/// Conditionally required when there are more than one UnderlyingComplexEvent occurrences. A chain of events must be linked together
	/// through use of the UnderlyingComplexEventCondition(2052) in which the relationship between any two events is described. For
	/// any two occurances of events the first occurrence will specify the UnderlyingComplexEventCondition(2052) which links it with
	/// the second event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2052")]
	pub underlying_complex_event_condition: Option<UnderlyingComplexEventCondition>,
	/// UnderlyingComplexOptPayoutPaySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2261")]
	pub underlying_complex_opt_payout_pay_side: Option<UnderlyingComplexOptPayoutPaySide>,
	/// UnderlyingComplexOptPayoutReceiveSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2262")]
	pub underlying_complex_opt_payout_receive_side: Option<UnderlyingComplexOptPayoutReceiveSide>,
	/// UnderlyingComplexOptPayoutUnderlier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2263")]
	pub underlying_complex_opt_payout_underlier: Option<String>,
	/// UnderlyingComplexOptPayoutPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2264")]
	pub underlying_complex_opt_payout_percentage: Option<f32>,
	/// UnderlyingComplexOptPayoutTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2265")]
	pub underlying_complex_opt_payout_time: Option<UnderlyingComplexOptPayoutTime>,
	/// UnderlyingComplexOptPayoutCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2266")]
	pub underlying_complex_opt_payout_currency: Option<String>,
	/// UnderlyingComplexEventPricePercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2267")]
	pub underlying_complex_event_price_percentage: Option<f32>,
	/// UnderlyingComplexEventCurrencyOne
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2268")]
	pub underlying_complex_event_currency_one: Option<String>,
	/// UnderlyingComplexEventCurrencyTwo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2269")]
	pub underlying_complex_event_currency_two: Option<String>,
	/// UnderlyingComplexEventQuoteBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2270")]
	pub underlying_complex_event_quote_basis: Option<UnderlyingComplexEventQuoteBasis>,
	/// UnderlyingComplexEventFixedFXRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2271")]
	pub underlying_complex_event_fixed_fx_rate: Option<f64>,
	/// UnderlyingComplexEventDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2272")]
	pub underlying_complex_event_determination_method: Option<String>,
	/// UnderlyingComplexEventCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2273")]
	pub underlying_complex_event_calculation_agent: Option<UnderlyingComplexEventCalculationAgent>,
	/// UnderlyingComplexEventStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2274")]
	pub underlying_complex_event_strike_price: Option<f64>,
	/// UnderlyingComplexEventStrikeFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2275")]
	pub underlying_complex_event_strike_factor: Option<f64>,
	/// UnderlyingComplexEventStrikeNumberOfOptions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2276")]
	pub underlying_complex_event_strike_number_of_options: Option<i32>,
	/// UnderlyingComplexEventCreditEventsXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2277")]
	pub underlying_complex_event_credit_events_xid_ref: Option<String>,
	/// UnderlyingComplexEventCreditEventNotifyingParty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2278")]
	pub underlying_complex_event_credit_event_notifying_party: Option<UnderlyingComplexEventCreditEventNotifyingParty>,
	/// UnderlyingComplexEventCreditEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2279")]
	pub underlying_complex_event_credit_event_business_center: Option<String>,
	/// UnderlyingComplexEventCreditEventStandardSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2280")]
	pub underlying_complex_event_credit_event_standard_sources: Option<fix_common::Boolean>,
	/// UnderlyingComplexEventCreditEventMinimumSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2281")]
	pub underlying_complex_event_credit_event_minimum_sources: Option<i32>,
	/// UnderlyingComplexEventXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2282")]
	pub underlying_complex_event_xid: Option<String>,
	/// UnderlyingComplexEventXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2283")]
	pub underlying_complex_event_xid_ref: Option<String>,
	/// UnderlyingComplexEventSpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2419")]
	pub underlying_complex_event_spot_rate: Option<f64>,
	/// UnderlyingComplexEventForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2420")]
	pub underlying_complex_event_forward_points: Option<f64>,
	/// UnderlyingComplexEventFuturesPriceValuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2611")]
	pub underlying_complex_event_futures_price_valuation: Option<fix_common::Boolean>,
	/// UnderlyingComplexEventOptionsPriceValuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2612")]
	pub underlying_complex_event_options_price_valuation: Option<fix_common::Boolean>,
	/// UnderlyingComplexEventPVFinalPriceElectionFallback
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2613")]
	pub underlying_complex_event_pv_final_price_election_fallback: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventType {
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
}

impl Default for UnderlyingComplexEventType {
	fn default() -> Self {
		UnderlyingComplexEventType::Capped
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventPriceBoundaryMethod {
	/// Less than UnderlyingComplexEventPrice(2048)
	#[serde(rename = "1")]
	LessThanUnderlyingComplexEventPrice,
	/// Less than or equal to UnderlyingComplexEventPrice(2048)
	#[serde(rename = "2")]
	LessThanOrEqualToUnderlyingComplexEventPrice,
	/// Equal to UnderlyingComplexEventPrice(2048)
	#[serde(rename = "3")]
	EqualToUnderlyingComplexEventPrice,
	/// Greater than or equal to UnderlyingComplexEventPrice(2048)
	#[serde(rename = "4")]
	GreaterThanOrEqualToUnderlyingComplexEventPrice,
	/// Greater than UnderlyingComplexEventPrice(2048)
	#[serde(rename = "5")]
	GreaterThanUnderlyingComplexEventPrice,
}

impl Default for UnderlyingComplexEventPriceBoundaryMethod {
	fn default() -> Self {
		UnderlyingComplexEventPriceBoundaryMethod::LessThanUnderlyingComplexEventPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventPriceTimeType {
	/// Expiration
	#[serde(rename = "1")]
	Expiration,
	/// Immediate (At Any Time)
	#[serde(rename = "2")]
	Immediate,
	/// Specified Date and Time
	#[serde(rename = "3")]
	SpecifiedDateAndTime,
}

impl Default for UnderlyingComplexEventPriceTimeType {
	fn default() -> Self {
		UnderlyingComplexEventPriceTimeType::Expiration
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventCondition {
	/// And
	#[serde(rename = "1")]
	And,
	/// Or
	#[serde(rename = "2")]
	Or,
}

impl Default for UnderlyingComplexEventCondition {
	fn default() -> Self {
		UnderlyingComplexEventCondition::And
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexOptPayoutPaySide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for UnderlyingComplexOptPayoutPaySide {
	fn default() -> Self {
		UnderlyingComplexOptPayoutPaySide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexOptPayoutReceiveSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for UnderlyingComplexOptPayoutReceiveSide {
	fn default() -> Self {
		UnderlyingComplexOptPayoutReceiveSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexOptPayoutTime {
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

impl Default for UnderlyingComplexOptPayoutTime {
	fn default() -> Self {
		UnderlyingComplexOptPayoutTime::Close
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventQuoteBasis {
	/// Currency 1 per currency 2
	#[serde(rename = "0")]
	Currency1PerCurrency2,
	/// Currency 2 per currency 1
	#[serde(rename = "1")]
	Currency2PerCurrency1,
}

impl Default for UnderlyingComplexEventQuoteBasis {
	fn default() -> Self {
		UnderlyingComplexEventQuoteBasis::Currency1PerCurrency2
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventCalculationAgent {
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

impl Default for UnderlyingComplexEventCalculationAgent {
	fn default() -> Self {
		UnderlyingComplexEventCalculationAgent::ExercisingParty
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingComplexEventCreditEventNotifyingParty {
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

impl Default for UnderlyingComplexEventCreditEventNotifyingParty {
	fn default() -> Self {
		UnderlyingComplexEventCreditEventNotifyingParty::SellerNotifies
	}
}
