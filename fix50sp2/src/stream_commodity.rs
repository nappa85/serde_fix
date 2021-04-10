
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommodity {
	/// StreamCommodityBase
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41251")]
	pub stream_commodity_base: Option<String>,
	/// StreamCommodityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41252")]
	pub stream_commodity_type: Option<String>,
	/// Conditionally required when StreamCommoditySecurityIDSource(41254) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41253")]
	pub stream_commodity_security_id: Option<String>,
	/// Conditionally required when StreamCommoditySecurityID(41253) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41254")]
	pub stream_commodity_security_id_source: Option<StreamCommoditySecurityIDSource>,
	/// StreamCommodityAltIDGrp
	#[serde(flatten)]
	pub stream_commodity_alt_id_grp: Option<super::stream_commodity_alt_id_grp::StreamCommodityAltIDGrp>,
	/// StreamCommodityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41255")]
	pub stream_commodity_desc: Option<String>,
	/// Must be set if EncodedCommodityDesc(41257) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41256")]
	pub encoded_stream_commodity_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the StreamCommodityDesc(41255) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41257")]
	pub encoded_stream_commodity_desc: Option<String>,
	/// StreamAssetAttributeGrp
	#[serde(flatten)]
	pub stream_asset_attribute_grp: Option<super::stream_asset_attribute_grp::StreamAssetAttributeGrp>,
	/// StreamCommodityUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41258")]
	pub stream_commodity_unit_of_measure: Option<StreamCommodityUnitOfMeasure>,
	/// StreamCommodityCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41259")]
	pub stream_commodity_currency: Option<String>,
	/// StreamCommodityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41260")]
	pub stream_commodity_exchange: Option<String>,
	/// StreamCommodityRateSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41261")]
	pub stream_commodity_rate_source: Option<i32>,
	/// StreamCommodityRateReferencePage
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41262")]
	pub stream_commodity_rate_reference_page: Option<String>,
	/// StreamCommodityRatePageHeading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41263")]
	pub stream_commodity_rate_page_heading: Option<String>,
	/// StreamDataProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41264")]
	pub stream_data_provider: Option<String>,
	/// StreamCommodityDataSourceGrp
	#[serde(flatten)]
	pub stream_commodity_data_source_grp: Option<super::stream_commodity_data_source_grp::StreamCommodityDataSourceGrp>,
	/// StreamCommodityPricingType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41265")]
	pub stream_commodity_pricing_type: Option<String>,
	/// Conditionally required when StreamCommodityNearbySettlDayUnit(41267) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41266")]
	pub stream_commodity_nearby_settl_day_period: Option<i32>,
	/// Conditionally required when StreamCommodityNearbySettlDayPeriod(41266) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41267")]
	pub stream_commodity_nearby_settl_day_unit: Option<StreamCommodityNearbySettlDayUnit>,
	/// StreamCommoditySettlDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41268")]
	pub stream_commodity_settl_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// StreamCommoditySettlDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41269")]
	pub stream_commodity_settl_date_business_day_convention: Option<StreamCommoditySettlDateBusinessDayConvention>,
	/// StreamCommoditySettlBusinessCenterGrp
	#[serde(flatten)]
	pub stream_commodity_settl_business_center_grp: Option<super::stream_commodity_settl_business_center_grp::StreamCommoditySettlBusinessCenterGrp>,
	/// StreamCommoditySettlDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41270")]
	pub stream_commodity_settl_date_adjusted: Option<fix_common::LocalMktDate>,
	/// StreamCommoditySettlMonth
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41271")]
	pub stream_commodity_settl_month: Option<i32>,
	/// Conditionally required when StreamCommoditySettlDateRollUnit(41273) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41272")]
	pub stream_commodity_settl_date_roll_period: Option<i32>,
	/// Conditionally required when StreamCommoditySettlDateRollPeriod(41272) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41273")]
	pub stream_commodity_settl_date_roll_unit: Option<StreamCommoditySettlDateRollUnit>,
	/// StreamCommoditySettlDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41274")]
	pub stream_commodity_settl_day_type: Option<StreamCommoditySettlDayType>,
	/// StreamCommoditySettlPeriodGrp
	#[serde(flatten)]
	pub stream_commodity_settl_period_grp: Option<super::stream_commodity_settl_period_grp::StreamCommoditySettlPeriodGrp>,
	/// StreamCommodityXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41275")]
	pub stream_commodity_xid: Option<String>,
	/// StreamCommodityXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41276")]
	pub stream_commodity_xid_ref: Option<String>,
	/// May be used to specify the delivery or pricing region of a non-standard commodity swap contract (e.g. when InstrAttribType(871)=38
	/// (US standard contract indicator) and InstrAttribValue(872)=N).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42587")]
	pub stream_commodity_delivery_pricing_region: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamCommoditySecurityIDSource {
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
pub enum StreamCommodityUnitOfMeasure {
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
pub enum StreamCommodityNearbySettlDayUnit {
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamCommoditySettlDateBusinessDayConvention {
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
pub enum StreamCommoditySettlDateRollUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamCommoditySettlDayType {
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
