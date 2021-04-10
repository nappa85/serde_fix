
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommodity {
	/// UnderlyingStreamCommodityBase
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41964")]
	pub underlying_stream_commodity_base: Option<String>,
	/// UnderlyingStreamCommodityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41965")]
	pub underlying_stream_commodity_type: Option<String>,
	/// Conditionally required when UnderlyingStreamCommoditySecurityIDSource(41967) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41966")]
	pub underlying_stream_commodity_security_id: Option<String>,
	/// Conditionally required when UnderlyingStreamCommoditySecurityID(41966) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41967")]
	pub underlying_stream_commodity_security_id_source: Option<UnderlyingStreamCommoditySecurityIDSource>,
	/// UnderlyingStreamCommodityAltIDGrp
	#[serde(flatten)]
	pub underlying_stream_commodity_alt_id_grp: Option<super::underlying_stream_commodity_alt_id_grp::UnderlyingStreamCommodityAltIDGrp>,
	/// UnderlyingStreamCommodityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41968")]
	pub underlying_stream_commodity_desc: Option<String>,
	/// Must be set if EncodedUnderlyingStreamCommodityDesc(41970) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41969")]
	pub encoded_underlying_stream_commodity_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the UnderlyingStreamCommodityDesc(41968) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41970")]
	pub encoded_underlying_stream_commodity_desc: Option<String>,
	/// UnderlyingStreamAssetAttributeGrp
	#[serde(flatten)]
	pub underlying_stream_asset_attribute_grp: Option<super::underlying_stream_asset_attribute_grp::UnderlyingStreamAssetAttributeGrp>,
	/// UnderlyingStreamCommodityUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41971")]
	pub underlying_stream_commodity_unit_of_measure: Option<UnderlyingStreamCommodityUnitOfMeasure>,
	/// UnderlyingStreamCommodityCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41972")]
	pub underlying_stream_commodity_currency: Option<String>,
	/// UnderlyingStreamCommodityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41973")]
	pub underlying_stream_commodity_exchange: Option<String>,
	/// UnderlyingStreamCommodityRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41974")]
	pub underlying_stream_commodity_rate_source: Option<i32>,
	/// UnderlyingStreamCommodityRateReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41975")]
	pub underlying_stream_commodity_rate_reference_page: Option<String>,
	/// UnderlyingStreamCommodityRatePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41976")]
	pub underlying_stream_commodity_rate_page_heading: Option<String>,
	/// UnderlyingStreamDataProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41977")]
	pub underlying_stream_data_provider: Option<String>,
	/// UnderlyingStreamCommodityDataSourceGrp
	#[serde(flatten)]
	pub underlying_stream_commodity_data_source_grp: Option<super::underlying_stream_commodity_data_source_grp::UnderlyingStreamCommodityDataSourceGrp>,
	/// UnderlyingStreamCommodityPricingType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41978")]
	pub underlying_stream_commodity_pricing_type: Option<String>,
	/// Conditionally required when UnderlyingStreamCommodityNearbySettlDayUnit(41980) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41979")]
	pub underlying_stream_commodity_nearby_settl_day_period: Option<i32>,
	/// Conditionally required when UnderlyingStreamCommodityNearbySettlDayPeriod(41979) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41980")]
	pub underlying_stream_commodity_nearby_settl_day_unit: Option<UnderlyingStreamCommodityNearbySettlDayUnit>,
	/// UnderlyingStreamCommoditySettlDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41981")]
	pub underlying_stream_commodity_settl_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingStreamCommoditySettlDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41982")]
	pub underlying_stream_commodity_settl_date_business_day_convention: Option<UnderlyingStreamCommoditySettlDateBusinessDayConvention>,
	/// UnderlyingStreamCommoditySettlBusinessCenterGrp
	#[serde(flatten)]
	pub underlying_stream_commodity_settl_business_center_grp: Option<super::underlying_stream_commodity_settl_business_center_grp::UnderlyingStreamCommoditySettlBusinessCenterGrp>,
	/// UnderlyingStreamCommoditySettlDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41983")]
	pub underlying_stream_commodity_settl_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingStreamCommoditySettlMonth
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41984")]
	pub underlying_stream_commodity_settl_month: Option<i32>,
	/// Conditionally required when UnderlyingStreamCommoditySettlDateRollUnit(41986) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41985")]
	pub underlying_stream_commodity_settl_date_roll_period: Option<i32>,
	/// Conditionally required when UnderlyingStreamCommoditySettlDateRollPeriod(41985) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41986")]
	pub underlying_stream_commodity_settl_date_roll_unit: Option<UnderlyingStreamCommoditySettlDateRollUnit>,
	/// UnderlyingStreamCommoditySettlDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41987")]
	pub underlying_stream_commodity_settl_day_type: Option<UnderlyingStreamCommoditySettlDayType>,
	/// UnderlyingStreamCommoditySettlPeriodGrp
	#[serde(flatten)]
	pub underlying_stream_commodity_settl_period_grp: Option<super::underlying_stream_commodity_settl_period_grp::UnderlyingStreamCommoditySettlPeriodGrp>,
	/// UnderlyingStreamCommodityXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41988")]
	pub underlying_stream_commodity_xid: Option<String>,
	/// UnderlyingStreamCommodityXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41989")]
	pub underlying_stream_commodity_xid_ref: Option<String>,
	/// May be used to specify the delivery or pricing region of a non-standard commodity swap contract (e.g. when InstrAttribType(871)=38
	/// (US standard contract indicator) and InstrAttribValue(872)=N).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42589")]
	pub underlying_stream_commodity_delivery_pricing_region: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySecurityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN
	#[serde(rename = "4")]
	Isin,
	/// RIC
	#[serde(rename = "5")]
	Ric,
	/// ISO Currency Code
	#[serde(rename = "6")]
	IsoCurrencyCode,
	/// ISO Country Code
	#[serde(rename = "7")]
	IsoCountryCode,
	/// Exchange Symbol
	#[serde(rename = "8")]
	ExchangeSymbol,
	/// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
	#[serde(rename = "9")]
	ConsolidatedTapeAssociationSymbol,
	/// Bloomberg Symbol
	#[serde(rename = "A")]
	BloombergSymbol,
	/// Wertpapier
	#[serde(rename = "B")]
	Wertpapier,
	/// Dutch
	#[serde(rename = "C")]
	Dutch,
	/// Valoren
	#[serde(rename = "D")]
	Valoren,
	/// Sicovam
	#[serde(rename = "E")]
	Sicovam,
	/// Belgian
	#[serde(rename = "F")]
	Belgian,
	/// "Common" (Clearstream and Euroclear)
	#[serde(rename = "G")]
	Common,
	/// Clearing House / Clearing Organization
	#[serde(rename = "H")]
	ClearingHouseClearingOrganization,
	/// ISDA/FpML Product Specification
	#[serde(rename = "I")]
	IsdaFpMlProductSpecification,
	/// Option Price Reporting Authority
	#[serde(rename = "J")]
	OptionPriceReportingAuthority,
	/// ISDA/FpML Product URL (URL in SecurityID)
	#[serde(rename = "K")]
	IsdaFpMlProductUrl,
	/// Letter of Credit
	#[serde(rename = "L")]
	LetterOfCredit,
	/// Marketplace-assigned Identifier
	#[serde(rename = "M")]
	MarketplaceAssignedIdentifier,
	/// Markit RED entity CLIP
	#[serde(rename = "N")]
	MarkitRedEntityClip,
	/// Markit RED pair CLIP
	#[serde(rename = "P")]
	MarkitRedPairClip,
	/// CFTC commodity code
	#[serde(rename = "Q")]
	CftcCommodityCode,
	/// ISDA Commodity Reference Price
	#[serde(rename = "R")]
	IsdaCommodityReferencePrice,
	/// Financial Instrument Global Identifier
	#[serde(rename = "S")]
	FinancialInstrumentGlobalIdentifier,
	/// Legal Entity Identifier
	#[serde(rename = "T")]
	LegalEntityIdentifier,
	/// Synthetic
	#[serde(rename = "U")]
	Synthetic,
	/// Fidessa Instrument Mnemonic (FIM)
	#[serde(rename = "V")]
	FidessaInstrumentMnemonic,
	/// Index name
	#[serde(rename = "W")]
	IndexName,
	/// Uniform Symbol (UMTF Symbol)
	#[serde(rename = "X")]
	UniformSymbol,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommodityUnitOfMeasure {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommodityNearbySettlDayUnit {
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlDateBusinessDayConvention {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlDateRollUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlDayType {
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
