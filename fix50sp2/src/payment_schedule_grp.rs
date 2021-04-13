
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleGrp {
	/// NoPaymentSchedules
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40828")]
	pub payment_schedules: Option<fix_common::RepeatingValues<PaymentSchedule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentSchedule {
	/// Required if NoPaymentSchedules(40828) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40829")]
	pub payment_schedule_type: Option<PaymentScheduleType>,
	/// PaymentScheduleStubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40830")]
	pub payment_schedule_stub_type: Option<i32>,
	/// PaymentScheduleStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40831")]
	pub payment_schedule_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentScheduleEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40832")]
	pub payment_schedule_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentSchedulePaySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40833")]
	pub payment_schedule_pay_side: Option<i32>,
	/// PaymentScheduleReceiveSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40834")]
	pub payment_schedule_receive_side: Option<i32>,
	/// PaymentScheduleNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40835")]
	pub payment_schedule_notional: Option<f64>,
	/// PaymentScheduleCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40836")]
	pub payment_schedule_currency: Option<String>,
	/// PaymentScheduleRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40837")]
	pub payment_schedule_rate: Option<f32>,
	/// PaymentScheduleRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40838")]
	pub payment_schedule_rate_multiplier: Option<f64>,
	/// PaymentScheduleRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40839")]
	pub payment_schedule_rate_spread: Option<f64>,
	/// PaymentScheduleRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40840")]
	pub payment_schedule_rate_spread_position_type: Option<i32>,
	/// PaymentScheduleRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40841")]
	pub payment_schedule_rate_treatment: Option<i32>,
	/// PaymentScheduleFixedAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40842")]
	pub payment_schedule_fixed_amount: Option<f64>,
	/// PaymentScheduleFixedCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40843")]
	pub payment_schedule_fixed_currency: Option<String>,
	/// Conditionally required when PaymentScheduleStepFrequencyUnit(40845) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40844")]
	pub payment_schedule_step_frequency_period: Option<i32>,
	/// Conditionally required when PaymentScheduleStepFrequencyPeriod(40844) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40845")]
	pub payment_schedule_step_frequency_unit: Option<String>,
	/// PaymentScheduleStepOffsetValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40846")]
	pub payment_schedule_step_offset_value: Option<f64>,
	/// PaymentScheduleStepRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40847")]
	pub payment_schedule_step_rate: Option<f32>,
	/// PaymentScheduleStepOffsetRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40848")]
	pub payment_schedule_step_offset_rate: Option<f32>,
	/// PaymentScheduleStepRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40849")]
	pub payment_schedule_step_relative_to: Option<PaymentScheduleStepRelativeTo>,
	/// PaymentScheduleFixingDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40850")]
	pub payment_schedule_fixing_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentScheduleWeight
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40851")]
	pub payment_schedule_weight: Option<f64>,
	/// PaymentScheduleFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40852")]
	pub payment_schedule_fixing_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment schedule.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40853")]
	pub payment_schedule_fixing_date_business_day_convention: Option<i32>,
	/// Conditionally required when PaymentScheduleFixingDateOffsetUnit(40856) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40855")]
	pub payment_schedule_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentScheduleFixingDateOffsetPeriod(40855) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40856")]
	pub payment_schedule_fixing_date_offset_unit: Option<String>,
	/// PaymentScheduleFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40857")]
	pub payment_schedule_fixing_date_offset_day_type: Option<i32>,
	/// PaymentScheduleFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40858")]
	pub payment_schedule_fixing_date_adjusted: Option<String>,
	/// PaymentScheduleFixingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40859")]
	pub payment_schedule_fixing_time: Option<String>,
	/// PaymentScheduleFixingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40860")]
	pub payment_schedule_fixing_time_business_center: Option<String>,
	/// PaymentScheduleInterimExchangePaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40861")]
	pub payment_schedule_interim_exchange_payment_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment schedule.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40862")]
	pub payment_schedule_interim_exchange_dates_business_day_convention: Option<i32>,
	/// Conditionally required when PaymentScheduleInterimExchangeDatesOffsetUnit(40865) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40864")]
	pub payment_schedule_interim_exchange_dates_offset_period: Option<i32>,
	/// Conditionally required when PaymentScheduleInterimExchangeDatesOffsetPeriod(40864) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40865")]
	pub payment_schedule_interim_exchange_dates_offset_unit: Option<String>,
	/// PaymentScheduleInterimExchangeDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40866")]
	pub payment_schedule_interim_exchange_dates_offset_day_type: Option<i32>,
	/// PaymentScheduleInterimExchangeDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40867")]
	pub payment_schedule_interim_exchange_date_adjusted: Option<fix_common::LocalMktDate>,
	/// PaymentScheduleXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41164")]
	pub payment_schedule_xid: Option<String>,
	/// PaymentScheduleXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41165")]
	pub payment_schedule_xid_ref: Option<String>,
	/// PaymentScheduleRateCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41166")]
	pub payment_schedule_rate_currency: Option<String>,
	/// PaymentScheduleRateUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41167")]
	pub payment_schedule_rate_unit_of_measure: Option<PaymentScheduleRateUnitOfMeasure>,
	/// PaymentScheduleRateConversionFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41168")]
	pub payment_schedule_rate_conversion_factor: Option<f64>,
	/// PaymentScheduleRateSpreadType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41169")]
	pub payment_schedule_rate_spread_type: Option<PaymentScheduleRateSpreadType>,
	/// PaymentScheduleSettlPeriodPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41170")]
	pub payment_schedule_settl_period_price: Option<f64>,
	/// PaymentScheduleSettlPeriodPriceCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41171")]
	pub payment_schedule_settl_period_price_currency: Option<String>,
	/// PaymentScheduleSettlPeriodPriceUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41172")]
	pub payment_schedule_settl_period_price_unit_of_measure: Option<PaymentScheduleSettlPeriodPriceUnitOfMeasure>,
	/// PaymentScheduleStepUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41173")]
	pub payment_schedule_step_unit_of_measure: Option<PaymentScheduleStepUnitOfMeasure>,
	/// PaymentScheduleFixingDayDistribution
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41174")]
	pub payment_schedule_fixing_day_distribution: Option<PaymentScheduleFixingDayDistribution>,
	/// PaymentScheduleFixingDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41175")]
	pub payment_schedule_fixing_day_count: Option<i32>,
	/// Conditionally required when PaymentScheduleFixingLagUnit(41177) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41176")]
	pub payment_schedule_fixing_lag_period: Option<i32>,
	/// Conditionally required when PaymentScheduleFixingLagPeriod(41176) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41177")]
	pub payment_schedule_fixing_lag_unit: Option<PaymentScheduleFixingLagUnit>,
	/// Conditionally required when PaymentScheduleFixingFirstObservationDateOffsetUnit(41179) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41178")]
	pub payment_schedule_fixing_first_observation_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentScheduleFixingFirstObservationDateOffsetPeriod(41178) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41179")]
	pub payment_schedule_fixing_first_observation_date_offset_unit: Option<PaymentScheduleFixingFirstObservationDateOffsetUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleType {
	/// Notional
	#[serde(rename = "0")]
	Notional,
	/// Cash flow
	#[serde(rename = "1")]
	CashFlow,
	/// FX linked notional
	#[serde(rename = "2")]
	FxLinkedNotional,
	/// Fixed rate
	#[serde(rename = "3")]
	FixedRate,
	/// Future value notional
	#[serde(rename = "4")]
	FutureValueNotional,
	/// Known amount
	#[serde(rename = "5")]
	KnownAmount,
	/// Floating rate multiplier
	#[serde(rename = "6")]
	FloatingRateMultiplier,
	/// Spread
	#[serde(rename = "7")]
	Spread,
	/// Cap rate
	#[serde(rename = "8")]
	CapRate,
	/// Floor rate
	#[serde(rename = "9")]
	FloorRate,
	/// Non-deliverable settlement payment dates
	#[serde(rename = "10")]
	NonDeliverableSettlementPaymentDates,
	/// Non-deliverable settlement calculation dates
	#[serde(rename = "11")]
	NonDeliverableSettlementCalculationDates,
	/// Non-deliverable fixing dates.
	#[serde(rename = "12")]
	NonDeliverableFixingDates,
	/// Settlement period notional
	#[serde(rename = "13")]
	SettlementPeriodNotional,
	/// Settlement period price
	#[serde(rename = "14")]
	SettlementPeriodPrice,
	/// Calculation period
	#[serde(rename = "15")]
	CalculationPeriod,
	/// Dividend accrual rate multiplier
	#[serde(rename = "16")]
	DividendAccrualRateMultiplier,
	/// Dividend accrual rate spread
	#[serde(rename = "17")]
	DividendAccrualRateSpread,
	/// Dividend accrual cap rate
	#[serde(rename = "18")]
	DividendAccrualCapRate,
	/// Dividend accrual floor rate
	#[serde(rename = "19")]
	DividendAccrualFloorRate,
	/// Compounding rate multiplier
	#[serde(rename = "20")]
	CompoundingRateMultiplier,
	/// Compounding rate spread
	#[serde(rename = "21")]
	CompoundingRateSpread,
	/// Compounding cap rate
	#[serde(rename = "22")]
	CompoundingCapRate,
	/// Compounding floor rate
	#[serde(rename = "23")]
	CompoundingFloorRate,
}

impl Default for PaymentScheduleType {
	fn default() -> Self {
		PaymentScheduleType::Notional
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleStepRelativeTo {
	/// Initial
	#[serde(rename = "0")]
	Initial,
	/// Previous
	#[serde(rename = "1")]
	Previous,
}

impl Default for PaymentScheduleStepRelativeTo {
	fn default() -> Self {
		PaymentScheduleStepRelativeTo::Initial
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleRateUnitOfMeasure {
	/// Barrels
	#[serde(rename = "Bbl")]
	Bbl,
	/// Billion cubic feet
	#[serde(rename = "Bcf")]
	Bcf,
	/// Bushels
	#[serde(rename = "Bu")]
	Bu,
	/// Pounds
	#[serde(rename = "lbs")]
	Lbs,
	/// Gallons
	#[serde(rename = "Gal")]
	Gal,
	/// Million Barrels (deprecated in FIX 5.0 SP1)
	#[serde(rename = "MMbbl")]
	MMbbl,
	/// One Million BTU
	#[serde(rename = "MMBtu")]
	MmBtu,
	/// Megawatt hours
	#[serde(rename = "MWh")]
	MWh,
	/// Troy Ounces
	#[serde(rename = "oz_tr")]
	OzTr,
	/// Metric Tons (aka Tonne)
	#[serde(rename = "t")]
	_T,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tn,
	/// US Dollars
	#[serde(rename = "USD")]
	Usd,
	/// Allowances
	#[serde(rename = "Alw")]
	Alw,
	/// Cubic Meters
	#[serde(rename = "CBM")]
	Cbm,
	/// Certified Emissions Reduction
	#[serde(rename = "CER")]
	Cer,
	/// Principal with relation to debt instrument
	#[serde(rename = "PRINC")]
	Princ,
	/// ClimateReserveTonnes (NOTE: This is added as a Variable Quantity UOM)
	#[serde(rename = "CRT")]
	Crt,
	/// Amount of currency
	#[serde(rename = "Ccy")]
	Ccy,
	/// Board feet
	#[serde(rename = "BDFT")]
	Bdft,
	/// Index point
	#[serde(rename = "IPNT")]
	Ipnt,
	/// Day
	#[serde(rename = "day")]
	Day,
	/// Hundredweight (US)
	#[serde(rename = "cwt")]
	Cwt,
	/// Grams
	#[serde(rename = "g")]
	_G,
	/// Dry metric tons
	#[serde(rename = "dt")]
	Dt,
	/// Kilowatt hours
	#[serde(rename = "kWh")]
	KWh,
	/// Environmental Offset
	#[serde(rename = "EnvOfst")]
	EnvOfst,
	/// Environmental Credit
	#[serde(rename = "EnvCrd")]
	EnvCrd,
	/// Kilowatt-Minute(electrical capacity)
	#[serde(rename = "kW-min")]
	KWMin,
	/// therms
	#[serde(rename = "thm")]
	Thm,
	/// gigajoules
	#[serde(rename = "GJ")]
	Gj,
	/// liters
	#[serde(rename = "L")]
	L,
	/// kiloliters
	#[serde(rename = "kL")]
	KL,
	/// Gross Tons(Elaboration: long tons or imperial tons, equal to 2240 lbs)
	#[serde(rename = "GT")]
	Gt,
	/// Kilograms
	#[serde(rename = "kg")]
	Kg,
	/// Metric tons
	#[serde(rename = "T")]
	T,
	/// Cooling degree day
	#[serde(rename = "CDD")]
	Cdd,
	/// Critical precipitation day
	#[serde(rename = "CPD")]
	Cpd,
	/// Environmental allowance certificates
	#[serde(rename = "EnvAllwnc")]
	EnvAllwnc,
	/// Heating degree day
	#[serde(rename = "HDD")]
	Hdd,
	/// Heat rate. The number of BTUs required to produce one kilowatt hour of electricity, typically 3,412.14 BTUs per 1 kWh
	#[serde(rename = "kHR")]
	KHr,
	/// Mega heat rate. The number of million BTUs required to produce one megawatt hour of electricity, typically 3.41214 million
	/// BTUs per 1 MWh
	#[serde(rename = "MHR")]
	Mhr,
	/// Kilowatt year (electrical capacity)
	#[serde(rename = "kW-a")]
	KWA,
	/// Kilowatt day (electrical capacity)
	#[serde(rename = "kW-d")]
	KWD,
	/// Kilowatt hour (electrical capacity)
	#[serde(rename = "kW-h")]
	KWH,
	/// Kilowatt month (electrical capacity)
	#[serde(rename = "kW-M")]
	KWM,
	/// Megawatt year (electrical capacity)
	#[serde(rename = "MW-a")]
	MwA,
	/// Megawatt day (electrical capacity)
	#[serde(rename = "MW-d")]
	MwD,
	/// Megawatt hour (electrical capacity)
	#[serde(rename = "MW-h")]
	MwH,
	/// Megawatt month (electrical capacity)
	#[serde(rename = "MW-M")]
	MwM,
	/// Megawatt minute (electrical capacity)
	#[serde(rename = "MW-min")]
	MwMin,
	/// Tons of carbon dioxide
	#[serde(rename = "tnCO2")]
	TnCo2,
	/// Are
	#[serde(rename = "a")]
	_A,
	/// Acre
	#[serde(rename = "ac")]
	Ac,
	/// Centiliter
	#[serde(rename = "cL")]
	CL,
	/// Centimeter
	#[serde(rename = "cM")]
	CM,
	/// Diesel gallon equivalent
	#[serde(rename = "DGE")]
	Dge,
	/// Foot
	#[serde(rename = "ft")]
	Ft,
	/// GB Gallon
	#[serde(rename = "Gal_gb")]
	GalGb,
	/// Gasonline gallon equivalent
	#[serde(rename = "GGE")]
	Gge,
	/// Hectare
	#[serde(rename = "ha")]
	Ha,
	/// Inch
	#[serde(rename = "in")]
	In,
	/// Kilometer
	#[serde(rename = "kM")]
	KM,
	/// Meter
	#[serde(rename = "M")]
	M,
	/// Mile
	#[serde(rename = "mi")]
	Mi,
	/// Milliliter
	#[serde(rename = "mL")]
	ML,
	/// Millimeter
	#[serde(rename = "mM")]
	MM,
	/// US ounce
	#[serde(rename = "oz")]
	Oz,
	/// Piece
	#[serde(rename = "pc")]
	Pc,
	/// US Pint
	#[serde(rename = "pt")]
	Pt,
	/// GB pint
	#[serde(rename = "pt_gb")]
	PtGb,
	/// US Quart
	#[serde(rename = "qt")]
	Qt,
	/// GB Quart
	#[serde(rename = "qt_gb")]
	QtGb,
	/// Square centimeter
	#[serde(rename = "SqcM")]
	SqcM,
	/// Square foot
	#[serde(rename = "Sqft")]
	Sqft,
	/// Square inch
	#[serde(rename = "Sqin")]
	Sqin,
	/// Square kilometer
	#[serde(rename = "SqkM")]
	SqkM,
	/// Square meter
	#[serde(rename = "SqM")]
	SqM,
	/// Square mile
	#[serde(rename = "Sqmi")]
	Sqmi,
	/// Square millimeter
	#[serde(rename = "SqmM")]
	SqmM,
	/// Square yard
	#[serde(rename = "Sqyd")]
	Sqyd,
	/// Yard
	#[serde(rename = "yd")]
	Yd,
}

impl Default for PaymentScheduleRateUnitOfMeasure {
	fn default() -> Self {
		PaymentScheduleRateUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleRateSpreadType {
	/// Absolute
	#[serde(rename = "0")]
	Absolute,
	/// Percentage
	#[serde(rename = "1")]
	Percentage,
}

impl Default for PaymentScheduleRateSpreadType {
	fn default() -> Self {
		PaymentScheduleRateSpreadType::Absolute
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleSettlPeriodPriceUnitOfMeasure {
	/// Barrels
	#[serde(rename = "Bbl")]
	Bbl,
	/// Billion cubic feet
	#[serde(rename = "Bcf")]
	Bcf,
	/// Bushels
	#[serde(rename = "Bu")]
	Bu,
	/// Pounds
	#[serde(rename = "lbs")]
	Lbs,
	/// Gallons
	#[serde(rename = "Gal")]
	Gal,
	/// Million Barrels (deprecated in FIX 5.0 SP1)
	#[serde(rename = "MMbbl")]
	MMbbl,
	/// One Million BTU
	#[serde(rename = "MMBtu")]
	MmBtu,
	/// Megawatt hours
	#[serde(rename = "MWh")]
	MWh,
	/// Troy Ounces
	#[serde(rename = "oz_tr")]
	OzTr,
	/// Metric Tons (aka Tonne)
	#[serde(rename = "t")]
	_T,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tn,
	/// US Dollars
	#[serde(rename = "USD")]
	Usd,
	/// Allowances
	#[serde(rename = "Alw")]
	Alw,
	/// Cubic Meters
	#[serde(rename = "CBM")]
	Cbm,
	/// Certified Emissions Reduction
	#[serde(rename = "CER")]
	Cer,
	/// Principal with relation to debt instrument
	#[serde(rename = "PRINC")]
	Princ,
	/// ClimateReserveTonnes (NOTE: This is added as a Variable Quantity UOM)
	#[serde(rename = "CRT")]
	Crt,
	/// Amount of currency
	#[serde(rename = "Ccy")]
	Ccy,
	/// Board feet
	#[serde(rename = "BDFT")]
	Bdft,
	/// Index point
	#[serde(rename = "IPNT")]
	Ipnt,
	/// Day
	#[serde(rename = "day")]
	Day,
	/// Hundredweight (US)
	#[serde(rename = "cwt")]
	Cwt,
	/// Grams
	#[serde(rename = "g")]
	_G,
	/// Dry metric tons
	#[serde(rename = "dt")]
	Dt,
	/// Kilowatt hours
	#[serde(rename = "kWh")]
	KWh,
	/// Environmental Offset
	#[serde(rename = "EnvOfst")]
	EnvOfst,
	/// Environmental Credit
	#[serde(rename = "EnvCrd")]
	EnvCrd,
	/// Kilowatt-Minute(electrical capacity)
	#[serde(rename = "kW-min")]
	KWMin,
	/// therms
	#[serde(rename = "thm")]
	Thm,
	/// gigajoules
	#[serde(rename = "GJ")]
	Gj,
	/// liters
	#[serde(rename = "L")]
	L,
	/// kiloliters
	#[serde(rename = "kL")]
	KL,
	/// Gross Tons(Elaboration: long tons or imperial tons, equal to 2240 lbs)
	#[serde(rename = "GT")]
	Gt,
	/// Kilograms
	#[serde(rename = "kg")]
	Kg,
	/// Metric tons
	#[serde(rename = "T")]
	T,
	/// Cooling degree day
	#[serde(rename = "CDD")]
	Cdd,
	/// Critical precipitation day
	#[serde(rename = "CPD")]
	Cpd,
	/// Environmental allowance certificates
	#[serde(rename = "EnvAllwnc")]
	EnvAllwnc,
	/// Heating degree day
	#[serde(rename = "HDD")]
	Hdd,
	/// Heat rate. The number of BTUs required to produce one kilowatt hour of electricity, typically 3,412.14 BTUs per 1 kWh
	#[serde(rename = "kHR")]
	KHr,
	/// Mega heat rate. The number of million BTUs required to produce one megawatt hour of electricity, typically 3.41214 million
	/// BTUs per 1 MWh
	#[serde(rename = "MHR")]
	Mhr,
	/// Kilowatt year (electrical capacity)
	#[serde(rename = "kW-a")]
	KWA,
	/// Kilowatt day (electrical capacity)
	#[serde(rename = "kW-d")]
	KWD,
	/// Kilowatt hour (electrical capacity)
	#[serde(rename = "kW-h")]
	KWH,
	/// Kilowatt month (electrical capacity)
	#[serde(rename = "kW-M")]
	KWM,
	/// Megawatt year (electrical capacity)
	#[serde(rename = "MW-a")]
	MwA,
	/// Megawatt day (electrical capacity)
	#[serde(rename = "MW-d")]
	MwD,
	/// Megawatt hour (electrical capacity)
	#[serde(rename = "MW-h")]
	MwH,
	/// Megawatt month (electrical capacity)
	#[serde(rename = "MW-M")]
	MwM,
	/// Megawatt minute (electrical capacity)
	#[serde(rename = "MW-min")]
	MwMin,
	/// Tons of carbon dioxide
	#[serde(rename = "tnCO2")]
	TnCo2,
	/// Are
	#[serde(rename = "a")]
	_A,
	/// Acre
	#[serde(rename = "ac")]
	Ac,
	/// Centiliter
	#[serde(rename = "cL")]
	CL,
	/// Centimeter
	#[serde(rename = "cM")]
	CM,
	/// Diesel gallon equivalent
	#[serde(rename = "DGE")]
	Dge,
	/// Foot
	#[serde(rename = "ft")]
	Ft,
	/// GB Gallon
	#[serde(rename = "Gal_gb")]
	GalGb,
	/// Gasonline gallon equivalent
	#[serde(rename = "GGE")]
	Gge,
	/// Hectare
	#[serde(rename = "ha")]
	Ha,
	/// Inch
	#[serde(rename = "in")]
	In,
	/// Kilometer
	#[serde(rename = "kM")]
	KM,
	/// Meter
	#[serde(rename = "M")]
	M,
	/// Mile
	#[serde(rename = "mi")]
	Mi,
	/// Milliliter
	#[serde(rename = "mL")]
	ML,
	/// Millimeter
	#[serde(rename = "mM")]
	MM,
	/// US ounce
	#[serde(rename = "oz")]
	Oz,
	/// Piece
	#[serde(rename = "pc")]
	Pc,
	/// US Pint
	#[serde(rename = "pt")]
	Pt,
	/// GB pint
	#[serde(rename = "pt_gb")]
	PtGb,
	/// US Quart
	#[serde(rename = "qt")]
	Qt,
	/// GB Quart
	#[serde(rename = "qt_gb")]
	QtGb,
	/// Square centimeter
	#[serde(rename = "SqcM")]
	SqcM,
	/// Square foot
	#[serde(rename = "Sqft")]
	Sqft,
	/// Square inch
	#[serde(rename = "Sqin")]
	Sqin,
	/// Square kilometer
	#[serde(rename = "SqkM")]
	SqkM,
	/// Square meter
	#[serde(rename = "SqM")]
	SqM,
	/// Square mile
	#[serde(rename = "Sqmi")]
	Sqmi,
	/// Square millimeter
	#[serde(rename = "SqmM")]
	SqmM,
	/// Square yard
	#[serde(rename = "Sqyd")]
	Sqyd,
	/// Yard
	#[serde(rename = "yd")]
	Yd,
}

impl Default for PaymentScheduleSettlPeriodPriceUnitOfMeasure {
	fn default() -> Self {
		PaymentScheduleSettlPeriodPriceUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleStepUnitOfMeasure {
	/// Barrels
	#[serde(rename = "Bbl")]
	Bbl,
	/// Billion cubic feet
	#[serde(rename = "Bcf")]
	Bcf,
	/// Bushels
	#[serde(rename = "Bu")]
	Bu,
	/// Pounds
	#[serde(rename = "lbs")]
	Lbs,
	/// Gallons
	#[serde(rename = "Gal")]
	Gal,
	/// Million Barrels (deprecated in FIX 5.0 SP1)
	#[serde(rename = "MMbbl")]
	MMbbl,
	/// One Million BTU
	#[serde(rename = "MMBtu")]
	MmBtu,
	/// Megawatt hours
	#[serde(rename = "MWh")]
	MWh,
	/// Troy Ounces
	#[serde(rename = "oz_tr")]
	OzTr,
	/// Metric Tons (aka Tonne)
	#[serde(rename = "t")]
	_T,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tn,
	/// US Dollars
	#[serde(rename = "USD")]
	Usd,
	/// Allowances
	#[serde(rename = "Alw")]
	Alw,
	/// Cubic Meters
	#[serde(rename = "CBM")]
	Cbm,
	/// Certified Emissions Reduction
	#[serde(rename = "CER")]
	Cer,
	/// Principal with relation to debt instrument
	#[serde(rename = "PRINC")]
	Princ,
	/// ClimateReserveTonnes (NOTE: This is added as a Variable Quantity UOM)
	#[serde(rename = "CRT")]
	Crt,
	/// Amount of currency
	#[serde(rename = "Ccy")]
	Ccy,
	/// Board feet
	#[serde(rename = "BDFT")]
	Bdft,
	/// Index point
	#[serde(rename = "IPNT")]
	Ipnt,
	/// Day
	#[serde(rename = "day")]
	Day,
	/// Hundredweight (US)
	#[serde(rename = "cwt")]
	Cwt,
	/// Grams
	#[serde(rename = "g")]
	_G,
	/// Dry metric tons
	#[serde(rename = "dt")]
	Dt,
	/// Kilowatt hours
	#[serde(rename = "kWh")]
	KWh,
	/// Environmental Offset
	#[serde(rename = "EnvOfst")]
	EnvOfst,
	/// Environmental Credit
	#[serde(rename = "EnvCrd")]
	EnvCrd,
	/// Kilowatt-Minute(electrical capacity)
	#[serde(rename = "kW-min")]
	KWMin,
	/// therms
	#[serde(rename = "thm")]
	Thm,
	/// gigajoules
	#[serde(rename = "GJ")]
	Gj,
	/// liters
	#[serde(rename = "L")]
	L,
	/// kiloliters
	#[serde(rename = "kL")]
	KL,
	/// Gross Tons(Elaboration: long tons or imperial tons, equal to 2240 lbs)
	#[serde(rename = "GT")]
	Gt,
	/// Kilograms
	#[serde(rename = "kg")]
	Kg,
	/// Metric tons
	#[serde(rename = "T")]
	T,
	/// Cooling degree day
	#[serde(rename = "CDD")]
	Cdd,
	/// Critical precipitation day
	#[serde(rename = "CPD")]
	Cpd,
	/// Environmental allowance certificates
	#[serde(rename = "EnvAllwnc")]
	EnvAllwnc,
	/// Heating degree day
	#[serde(rename = "HDD")]
	Hdd,
	/// Heat rate. The number of BTUs required to produce one kilowatt hour of electricity, typically 3,412.14 BTUs per 1 kWh
	#[serde(rename = "kHR")]
	KHr,
	/// Mega heat rate. The number of million BTUs required to produce one megawatt hour of electricity, typically 3.41214 million
	/// BTUs per 1 MWh
	#[serde(rename = "MHR")]
	Mhr,
	/// Kilowatt year (electrical capacity)
	#[serde(rename = "kW-a")]
	KWA,
	/// Kilowatt day (electrical capacity)
	#[serde(rename = "kW-d")]
	KWD,
	/// Kilowatt hour (electrical capacity)
	#[serde(rename = "kW-h")]
	KWH,
	/// Kilowatt month (electrical capacity)
	#[serde(rename = "kW-M")]
	KWM,
	/// Megawatt year (electrical capacity)
	#[serde(rename = "MW-a")]
	MwA,
	/// Megawatt day (electrical capacity)
	#[serde(rename = "MW-d")]
	MwD,
	/// Megawatt hour (electrical capacity)
	#[serde(rename = "MW-h")]
	MwH,
	/// Megawatt month (electrical capacity)
	#[serde(rename = "MW-M")]
	MwM,
	/// Megawatt minute (electrical capacity)
	#[serde(rename = "MW-min")]
	MwMin,
	/// Tons of carbon dioxide
	#[serde(rename = "tnCO2")]
	TnCo2,
	/// Are
	#[serde(rename = "a")]
	_A,
	/// Acre
	#[serde(rename = "ac")]
	Ac,
	/// Centiliter
	#[serde(rename = "cL")]
	CL,
	/// Centimeter
	#[serde(rename = "cM")]
	CM,
	/// Diesel gallon equivalent
	#[serde(rename = "DGE")]
	Dge,
	/// Foot
	#[serde(rename = "ft")]
	Ft,
	/// GB Gallon
	#[serde(rename = "Gal_gb")]
	GalGb,
	/// Gasonline gallon equivalent
	#[serde(rename = "GGE")]
	Gge,
	/// Hectare
	#[serde(rename = "ha")]
	Ha,
	/// Inch
	#[serde(rename = "in")]
	In,
	/// Kilometer
	#[serde(rename = "kM")]
	KM,
	/// Meter
	#[serde(rename = "M")]
	M,
	/// Mile
	#[serde(rename = "mi")]
	Mi,
	/// Milliliter
	#[serde(rename = "mL")]
	ML,
	/// Millimeter
	#[serde(rename = "mM")]
	MM,
	/// US ounce
	#[serde(rename = "oz")]
	Oz,
	/// Piece
	#[serde(rename = "pc")]
	Pc,
	/// US Pint
	#[serde(rename = "pt")]
	Pt,
	/// GB pint
	#[serde(rename = "pt_gb")]
	PtGb,
	/// US Quart
	#[serde(rename = "qt")]
	Qt,
	/// GB Quart
	#[serde(rename = "qt_gb")]
	QtGb,
	/// Square centimeter
	#[serde(rename = "SqcM")]
	SqcM,
	/// Square foot
	#[serde(rename = "Sqft")]
	Sqft,
	/// Square inch
	#[serde(rename = "Sqin")]
	Sqin,
	/// Square kilometer
	#[serde(rename = "SqkM")]
	SqkM,
	/// Square meter
	#[serde(rename = "SqM")]
	SqM,
	/// Square mile
	#[serde(rename = "Sqmi")]
	Sqmi,
	/// Square millimeter
	#[serde(rename = "SqmM")]
	SqmM,
	/// Square yard
	#[serde(rename = "Sqyd")]
	Sqyd,
	/// Yard
	#[serde(rename = "yd")]
	Yd,
}

impl Default for PaymentScheduleStepUnitOfMeasure {
	fn default() -> Self {
		PaymentScheduleStepUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleFixingDayDistribution {
	/// All
	#[serde(rename = "0")]
	All,
	/// First
	#[serde(rename = "1")]
	First,
	/// Last
	#[serde(rename = "2")]
	Last,
	/// Penultimate
	#[serde(rename = "3")]
	Penultimate,
}

impl Default for PaymentScheduleFixingDayDistribution {
	fn default() -> Self {
		PaymentScheduleFixingDayDistribution::All
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleFixingLagUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

impl Default for PaymentScheduleFixingLagUnit {
	fn default() -> Self {
		PaymentScheduleFixingLagUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentScheduleFixingFirstObservationDateOffsetUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

impl Default for PaymentScheduleFixingFirstObservationDateOffsetUnit {
	fn default() -> Self {
		PaymentScheduleFixingFirstObservationDateOffsetUnit::Day
	}
}
