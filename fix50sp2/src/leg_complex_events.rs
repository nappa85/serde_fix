
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEvents {
	/// NoLegComplexEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2218")]
	pub leg_complex_events: Option<fix_common::RepeatingValues<LegComplexEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEvent {
	/// Required if NoLegComplexEvents(2218)) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2219")]
	pub leg_complex_event_type: Option<LegComplexEventType>,
	/// LegComplexOptPayoutPaySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2220")]
	pub leg_complex_opt_payout_pay_side: Option<LegComplexOptPayoutPaySide>,
	/// LegComplexOptPayoutReceiveSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2221")]
	pub leg_complex_opt_payout_receive_side: Option<LegComplexOptPayoutReceiveSide>,
	/// LegComplexOptPayoutUnderlier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2222")]
	pub leg_complex_opt_payout_underlier: Option<String>,
	/// LegComplexOptPayoutAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2223")]
	pub leg_complex_opt_payout_amount: Option<f64>,
	/// LegComplexOptPayoutPercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2224")]
	pub leg_complex_opt_payout_percentage: Option<f32>,
	/// LegComplexOptPayoutTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2225")]
	pub leg_complex_opt_payout_time: Option<LegComplexOptPayoutTime>,
	/// LegComplexOptPayoutCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2226")]
	pub leg_complex_opt_payout_currency: Option<String>,
	/// LegComplexEventPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2227")]
	pub leg_complex_event_price: Option<f64>,
	/// LegComplexEventPricePercentage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2228")]
	pub leg_complex_event_price_percentage: Option<f32>,
	/// LegComplexEventPriceBoundaryMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2229")]
	pub leg_complex_event_price_boundary_method: Option<LegComplexEventPriceBoundaryMethod>,
	/// LegComplexEventPriceBoundaryPrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2230")]
	pub leg_complex_event_price_boundary_precision: Option<f32>,
	/// LegComplexEventPriceTimeType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2231")]
	pub leg_complex_event_price_time_type: Option<LegComplexEventPriceTimeType>,
	/// Conditionally required when there are more than one LegComplexEvents occurrences. A chain of LegComplexEvents must be linked
	/// together through use of the LegComplexEventCondition(2232) in which the relationship between any two events is described.
	/// For any two LegComplexEvents the first occurrence will specify the LegComplexEventCondition(2232) which links it with the
	/// second event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2232")]
	pub leg_complex_event_condition: Option<LegComplexEventCondition>,
	/// LegComplexEventCurrencyOne
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2233")]
	pub leg_complex_event_currency_one: Option<String>,
	/// LegComplexEventCurrencyTwo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2234")]
	pub leg_complex_event_currency_two: Option<String>,
	/// LegComplexEventQuoteBasis
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2235")]
	pub leg_complex_event_quote_basis: Option<LegComplexEventQuoteBasis>,
	/// LegComplexEventFixedFXRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2236")]
	pub leg_complex_event_fixed_fx_rate: Option<f64>,
	/// LegComplexEventDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2237")]
	pub leg_complex_event_determination_method: Option<String>,
	/// LegComplexEventCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2238")]
	pub leg_complex_event_calculation_agent: Option<LegComplexEventCalculationAgent>,
	/// LegComplexEventStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2239")]
	pub leg_complex_event_strike_price: Option<f64>,
	/// LegComplexEventStrikeFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2240")]
	pub leg_complex_event_strike_factor: Option<f64>,
	/// LegComplexEventStrikeNumberOfOptions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2241")]
	pub leg_complex_event_strike_number_of_options: Option<i32>,
	/// LegComplexEventCreditEventsXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2242")]
	pub leg_complex_event_credit_events_xid_ref: Option<String>,
	/// LegComplexEventCreditEventNotifyingParty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2243")]
	pub leg_complex_event_credit_event_notifying_party: Option<LegComplexEventCreditEventNotifyingParty>,
	/// LegComplexEventCreditEventBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2244")]
	pub leg_complex_event_credit_event_business_center: Option<String>,
	/// LegComplexEventCreditEventStandardSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2245")]
	pub leg_complex_event_credit_event_standard_sources: Option<fix_common::Boolean>,
	/// LegComplexEventCreditEventMinimumSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2246")]
	pub leg_complex_event_credit_event_minimum_sources: Option<i32>,
	/// LegComplexEventXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2248")]
	pub leg_complex_event_xid: Option<String>,
	/// LegComplexEventXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2249")]
	pub leg_complex_event_xid_ref: Option<String>,
	/// LegComplexEventSpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2409")]
	pub leg_complex_event_spot_rate: Option<f64>,
	/// LegComplexEventForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2410")]
	pub leg_complex_event_forward_points: Option<f64>,
	/// LegComplexEventFuturesPriceValuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2608")]
	pub leg_complex_event_futures_price_valuation: Option<fix_common::Boolean>,
	/// LegComplexEventOptionsPriceValuation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2609")]
	pub leg_complex_event_options_price_valuation: Option<fix_common::Boolean>,
	/// LegComplexEventPVFinalPriceElectionFallback
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2610")]
	pub leg_complex_event_pv_final_price_election_fallback: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventType {
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
}

impl Default for LegComplexEventType {
	fn default() -> Self {
		LegComplexEventType::Capped
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexOptPayoutPaySide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for LegComplexOptPayoutPaySide {
	fn default() -> Self {
		LegComplexOptPayoutPaySide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexOptPayoutReceiveSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for LegComplexOptPayoutReceiveSide {
	fn default() -> Self {
		LegComplexOptPayoutReceiveSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexOptPayoutTime {
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

impl Default for LegComplexOptPayoutTime {
	fn default() -> Self {
		LegComplexOptPayoutTime::Close
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventPriceBoundaryMethod {
	/// Less than ComplexEventPrice(1486)
	#[serde(rename = "1")]
	LessThanComplexEventPrice,
	/// Less than or equal to ComplexEventPrice(1486)
	#[serde(rename = "2")]
	LessThanOrEqualToComplexEventPrice,
	/// Equal to ComplexEventPrice(1486)
	#[serde(rename = "3")]
	EqualToComplexEventPrice,
	/// Greater than or equal to ComplexEventPrice(1486)
	#[serde(rename = "4")]
	GreaterThanOrEqualToComplexEventPrice,
	/// Greater than ComplexEventPrice(1486)
	#[serde(rename = "5")]
	GreaterThanComplexEventPrice,
}

impl Default for LegComplexEventPriceBoundaryMethod {
	fn default() -> Self {
		LegComplexEventPriceBoundaryMethod::LessThanComplexEventPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventPriceTimeType {
	/// Expiration
	#[serde(rename = "1")]
	Expiration,
	/// Immediate (At Any Time)
	#[serde(rename = "2")]
	Immediate,
	/// Specified Date/Time
	#[serde(rename = "3")]
	SpecifiedDateTime,
}

impl Default for LegComplexEventPriceTimeType {
	fn default() -> Self {
		LegComplexEventPriceTimeType::Expiration
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventCondition {
	/// And
	#[serde(rename = "1")]
	And,
	/// Or
	#[serde(rename = "2")]
	Or,
}

impl Default for LegComplexEventCondition {
	fn default() -> Self {
		LegComplexEventCondition::And
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventQuoteBasis {
	/// Currency 1 per currency 2
	#[serde(rename = "0")]
	Currency1PerCurrency2,
	/// Currency 2 per currency 1
	#[serde(rename = "1")]
	Currency2PerCurrency1,
}

impl Default for LegComplexEventQuoteBasis {
	fn default() -> Self {
		LegComplexEventQuoteBasis::Currency1PerCurrency2
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventCalculationAgent {
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

impl Default for LegComplexEventCalculationAgent {
	fn default() -> Self {
		LegComplexEventCalculationAgent::ExercisingParty
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventCreditEventNotifyingParty {
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

impl Default for LegComplexEventCreditEventNotifyingParty {
	fn default() -> Self {
		LegComplexEventCreditEventNotifyingParty::SellerNotifies
	}
}
