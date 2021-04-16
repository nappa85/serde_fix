
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentGrp {
	/// NoPayments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40212")]
	pub payments: Option<fix_common::RepeatingValues<Payment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Payment {
	/// Required if NoPayments(40212) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40213")]
	pub payment_type: Option<PaymentType>,
	/// PaymentPaySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40214")]
	pub payment_pay_side: Option<PaymentPaySide>,
	/// PaymentReceiveSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40215")]
	pub payment_receive_side: Option<PaymentReceiveSide>,
	/// PaymentCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40216")]
	pub payment_currency: Option<String>,
	/// Either PaymentAmount(40217), PaymentFixedRate(43097) or PaymentRFloatingRateIndex(43098) must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40217")]
	pub payment_amount: Option<f64>,
	/// PaymentPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40218")]
	pub payment_price: Option<f64>,
	/// PaymentPriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40919")]
	pub payment_price_type: Option<f64>,
	/// PaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40219")]
	pub payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment information.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40220")]
	pub payment_business_day_convention: Option<PaymentBusinessDayConvention>,
	/// PaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40222")]
	pub payment_date_adjusted: Option<fix_common::LocalMktDate>,
	/// PaymentDiscountFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40224")]
	pub payment_discount_factor: Option<f64>,
	/// PaymentPresentValueAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40225")]
	pub payment_present_value_amount: Option<f64>,
	/// PaymentPresentValueCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40226")]
	pub payment_present_value_currency: Option<String>,
	/// PaymentSettlStyle
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40227")]
	pub payment_settl_style: Option<PaymentSettlStyle>,
	/// PaymentMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "492")]
	pub payment_method: Option<PaymentMethod>,
	/// PaymentText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40229")]
	pub payment_text: Option<String>,
	/// Must be set if EncodedPaymentText(40985) field is specified and must immediately precede it.
	#[serde(rename = "40984")]
	/// Encoded (non-ASCII characters) representation of the PaymentText(40229) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "40985")]
	pub encoded_payment_text: Option<fix_common::EncodedText<40985>>,
	/// PaymentUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41155")]
	pub payment_unit_of_measure: Option<PaymentUnitOfMeasure>,
	/// PaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41156")]
	pub payment_date_relative_to: Option<PaymentDateRelativeTo>,
	/// Conditionally required when PaymentDateOffsetUnit(41158) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41157")]
	pub payment_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentDateOffsetPeriod(41157) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41158")]
	pub payment_date_offset_unit: Option<PaymentDateOffsetUnit>,
	/// PaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41159")]
	pub payment_date_offset_day_type: Option<PaymentDateOffsetDayType>,
	/// PaymentForwardStartType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41160")]
	pub payment_forward_start_type: Option<PaymentForwardStartType>,
	/// PaymentSubType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40993")]
	pub payment_sub_type: Option<PaymentSubType>,
	/// Used to link a payment back to its parent InstrumentLeg by using the same value as the parent's LegID(1788).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41304")]
	pub payment_leg_ref_id: Option<String>,
	/// PaymentDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43087")]
	pub payment_desc: Option<String>,
	/// PaymentAmountRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42598")]
	pub payment_amount_relative_to: Option<i32>,
	/// PaymentAmountDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42599")]
	pub payment_amount_determination_method: Option<String>,
	/// Either PaymentAmount(40217), PaymentFixedRate(43097) or PaymentFloatingRateIndex(43098) must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43097")]
	pub payment_fixed_rate: Option<f32>,
	/// Either PaymentAmount(40217), PaymentFixedRate(43097) or PaymentFloatingRateIndex(43098) must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43098")]
	pub payment_floating_rate_index: Option<String>,
	/// Conditionally required when PaymentFloatingRateIndexCurvePeriod(43099) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43100")]
	pub payment_floating_rate_index_curve_unit: Option<String>,
	/// Conditionally required when PaymentFloatingRateIndexCurveUnit(43100) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43099")]
	pub payment_floating_rate_index_curve_period: Option<i32>,
	/// Conditionally required when PaymentFloatingRateIndex(43098) is specified and the spread to the index is not "zero". When the
	/// spread to the index is "zero" this may be omitted.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43101")]
	pub payment_floating_rate_spread: Option<f64>,
	/// Conditionally required when PaymentRateResetFrequencyPeriod(43104) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43105")]
	pub payment_rate_reset_frequency_unit: Option<String>,
	/// Conditionally required when PaymentRateResetFrequencyUnit(43105) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43104")]
	pub payment_rate_reset_frequency_period: Option<i32>,
	/// Conditionally required when PaymentFrequencyPeriod(43102) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43103")]
	pub payment_frequency_unit: Option<String>,
	/// Conditionally required when PaymentFrequencyUnitPeriod(43103) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43102")]
	pub payment_frequency_period: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentType {
	/// Brokerage
	#[serde(rename = "0")]
	Brokerage,
	/// Upfront fee
	#[serde(rename = "1")]
	UpfrontFee,
	/// Independent amount / collateral
	#[serde(rename = "2")]
	IndependentAmountCollateral,
	/// Principal exchange
	#[serde(rename = "3")]
	PrincipalExchange,
	/// Novation / termination
	#[serde(rename = "4")]
	NovationTermination,
	/// Early termination provision
	#[serde(rename = "5")]
	EarlyTerminationProvision,
	/// Cancelable provision
	#[serde(rename = "6")]
	CancelableProvision,
	/// Extendible provision
	#[serde(rename = "7")]
	ExtendibleProvision,
	/// Cap rate provision
	#[serde(rename = "8")]
	CapRateProvision,
	/// Floor rate provision
	#[serde(rename = "9")]
	FloorRateProvision,
	/// Option premium
	#[serde(rename = "10")]
	OptionPremium,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Settlement payment
	#[serde(rename = "11")]
	SettlementPayment,
	/// Cash settlement
	#[serde(rename = "12")]
	CashSettlement,
	/// Security lending
	#[serde(rename = "13")]
	SecurityLending,
	/// Rebate
	#[serde(rename = "14")]
	Rebate,
}

impl Default for PaymentType {
	fn default() -> Self {
		PaymentType::Brokerage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentPaySide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for PaymentPaySide {
	fn default() -> Self {
		PaymentPaySide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentReceiveSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for PaymentReceiveSide {
	fn default() -> Self {
		PaymentReceiveSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentBusinessDayConvention {
	/// Not applicable
	#[serde(rename = "0")]
	NotApplicable,
	/// None (current day)
	#[serde(rename = "1")]
	None,
	/// Following day
	#[serde(rename = "2")]
	FollowingDay,
	/// Floating rate note
	#[serde(rename = "3")]
	FloatingRateNote,
	/// Modified following day
	#[serde(rename = "4")]
	ModifiedFollowingDay,
	/// Preceding day
	#[serde(rename = "5")]
	PrecedingDay,
	/// Modified preceding day
	#[serde(rename = "6")]
	ModifiedPrecedingDay,
	/// Nearest day
	#[serde(rename = "7")]
	NearestDay,
}

impl Default for PaymentBusinessDayConvention {
	fn default() -> Self {
		PaymentBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentSettlStyle {
	/// Standard
	#[serde(rename = "0")]
	Standard,
	/// Net
	#[serde(rename = "1")]
	Net,
	/// Standard and net
	#[serde(rename = "2")]
	StandardAndNet,
}

impl Default for PaymentSettlStyle {
	fn default() -> Self {
		PaymentSettlStyle::Standard
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentMethod {
	/// CREST
	#[serde(rename = "1")]
	Crest,
	/// NSCC
	#[serde(rename = "2")]
	Nscc,
	/// Euroclear
	#[serde(rename = "3")]
	Euroclear,
	/// Clearstream
	#[serde(rename = "4")]
	Clearstream,
	/// Cheque
	#[serde(rename = "5")]
	Cheque,
	/// Telegraphic Transfer
	#[serde(rename = "6")]
	TelegraphicTransfer,
	/// Fed Wire
	#[serde(rename = "7")]
	FedWire,
	/// Debit Card
	#[serde(rename = "8")]
	DebitCard,
	/// Direct Debit (BECS)
	#[serde(rename = "9")]
	DirectDebit,
	/// Direct Credit (BECS)
	#[serde(rename = "10")]
	DirectCredit,
	/// Credit Card
	#[serde(rename = "11")]
	CreditCard,
	/// ACH Debit
	#[serde(rename = "12")]
	AchDebit,
	/// ACH Credit
	#[serde(rename = "13")]
	AchCredit,
	/// BPAY
	#[serde(rename = "14")]
	Bpay,
	/// High Value Clearing System (HVACS)
	#[serde(rename = "15")]
	HighValueClearingSystem,
	/// CHIPS
	#[serde(rename = "16")]
	Chips,
	/// S.W.I.F.T.
	#[serde(rename = "17")]
	SWIFT,
	/// CHAPS
	#[serde(rename = "18")]
	Chaps,
	/// SIC
	#[serde(rename = "19")]
	Sic,
	/// euroSIC
	#[serde(rename = "20")]
	EuroSic,
}

impl Default for PaymentMethod {
	fn default() -> Self {
		PaymentMethod::Crest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentUnitOfMeasure {
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

impl Default for PaymentUnitOfMeasure {
	fn default() -> Self {
		PaymentUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentDateRelativeTo {
	/// Trade date
	#[serde(rename = "0")]
	TradeDate,
	/// Settlement date
	#[serde(rename = "1")]
	SettlementDate,
	/// Effective date
	#[serde(rename = "2")]
	EffectiveDate,
}

impl Default for PaymentDateRelativeTo {
	fn default() -> Self {
		PaymentDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentDateOffsetUnit {
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

impl Default for PaymentDateOffsetUnit {
	fn default() -> Self {
		PaymentDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentDateOffsetDayType {
	/// Business
	#[serde(rename = "0")]
	Business,
	/// Calendar
	#[serde(rename = "1")]
	Calendar,
	/// Commodity business
	#[serde(rename = "2")]
	CommodityBusiness,
	/// Currency business
	#[serde(rename = "3")]
	CurrencyBusiness,
	/// Exchange business
	#[serde(rename = "4")]
	ExchangeBusiness,
	/// Scheduled trading day
	#[serde(rename = "5")]
	ScheduledTradingDay,
}

impl Default for PaymentDateOffsetDayType {
	fn default() -> Self {
		PaymentDateOffsetDayType::Business
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentForwardStartType {
	/// Prepaid
	#[serde(rename = "0")]
	Prepaid,
	/// Post-paid
	#[serde(rename = "1")]
	PostPaid,
	/// Variable
	#[serde(rename = "2")]
	Variable,
	/// Fixed
	#[serde(rename = "3")]
	Fixed,
}

impl Default for PaymentForwardStartType {
	fn default() -> Self {
		PaymentForwardStartType::Prepaid
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PaymentSubType {
	/// Initial (principal exchange)
	#[serde(rename = "0")]
	Initial,
	/// Intermediate (principal exchange)
	#[serde(rename = "1")]
	Intermediate,
	/// Final (principal exchange)
	#[serde(rename = "2")]
	Final,
	/// Prepaid (premium forward)
	#[serde(rename = "3")]
	Prepaid,
	/// Postpaid (premium forward)
	#[serde(rename = "4")]
	Postpaid,
	/// Variable (premium forward)
	#[serde(rename = "5")]
	Variable,
	/// Fixed (premium forward)
	#[serde(rename = "6")]
	Fixed,
	/// Swap (premium)
	#[serde(rename = "7")]
	Swap,
	/// Conditional (principal exchange on exercise)
	#[serde(rename = "8")]
	Conditional,
	/// Fixed rate
	#[serde(rename = "9")]
	FixedRate,
	/// Floating rate
	#[serde(rename = "10")]
	FloatingRate,
}

impl Default for PaymentSubType {
	fn default() -> Self {
		PaymentSubType::Initial
	}
}
