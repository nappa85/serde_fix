
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDeliveryStream {
	/// UnderlyingDeliveryStreamType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41777")]
	pub underlying_delivery_stream_type: Option<UnderlyingDeliveryStreamType>,
	/// UnderlyingDeliveryStreamCommoditySourceGrp
	#[serde(flatten)]
	pub underlying_delivery_stream_commodity_source_grp: Option<super::underlying_delivery_stream_commodity_source_grp::UnderlyingDeliveryStreamCommoditySourceGrp>,
	/// UnderlyingDeliveryStreamPipeline
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41778")]
	pub underlying_delivery_stream_pipeline: Option<String>,
	/// UnderlyingDeliveryStreamEntryPoint
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41779")]
	pub underlying_delivery_stream_entry_point: Option<String>,
	/// UnderlyingDeliveryStreamWithdrawalPoint
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41780")]
	pub underlying_delivery_stream_withdrawal_point: Option<String>,
	/// UnderlyingDeliveryStreamDeliveryPoint
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41781")]
	pub underlying_delivery_stream_delivery_point: Option<String>,
	/// UnderlyingDeliveryStreamDeliveryRestriction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41782")]
	pub underlying_delivery_stream_delivery_restriction: Option<UnderlyingDeliveryStreamDeliveryRestriction>,
	/// UnderlyingDeliveryStreamDeliveryContingency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41783")]
	pub underlying_delivery_stream_delivery_contingency: Option<String>,
	/// UnderlyingDeliveryStreamDeliveryContingentPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41784")]
	pub underlying_delivery_stream_delivery_contingent_party_side: Option<UnderlyingDeliveryStreamDeliveryContingentPartySide>,
	/// UnderlyingDeliveryStreamDeliverAtSourceIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41785")]
	pub underlying_delivery_stream_deliver_at_source_indicator: Option<fix_common::Boolean>,
	/// UnderlyingDeliveryStreamRiskApportionment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41786")]
	pub underlying_delivery_stream_risk_apportionment: Option<String>,
	/// UnderlyingDeliveryStreamRiskApportionmentSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41587")]
	pub underlying_delivery_stream_risk_apportionment_source: Option<String>,
	/// UnderlyingDeliveryStreamCycleGrp
	#[serde(flatten)]
	pub underlying_delivery_stream_cycle_grp: Option<super::underlying_delivery_stream_cycle_grp::UnderlyingDeliveryStreamCycleGrp>,
	/// UnderlyingDeliveryStreamTitleTransferLocation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41787")]
	pub underlying_delivery_stream_title_transfer_location: Option<String>,
	/// UnderlyingDeliveryStreamTitleTransferCondition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41788")]
	pub underlying_delivery_stream_title_transfer_condition: Option<UnderlyingDeliveryStreamTitleTransferCondition>,
	/// UnderlyingDeliveryStreamImporterOfRecord
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41789")]
	pub underlying_delivery_stream_importer_of_record: Option<String>,
	/// UnderlyingDeliveryStreamNegativeTolerance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41790")]
	pub underlying_delivery_stream_negative_tolerance: Option<f64>,
	/// UnderlyingDeliveryStreamPositiveTolerance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41791")]
	pub underlying_delivery_stream_positive_tolerance: Option<f64>,
	/// UnderlyingDeliveryStreamToleranceUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41792")]
	pub underlying_delivery_stream_tolerance_unit_of_measure: Option<UnderlyingDeliveryStreamToleranceUnitOfMeasure>,
	/// UnderlyingDeliveryStreamToleranceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41793")]
	pub underlying_delivery_stream_tolerance_type: Option<UnderlyingDeliveryStreamToleranceType>,
	/// UnderlyingDeliveryStreamToleranceOptionSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41794")]
	pub underlying_delivery_stream_tolerance_option_side: Option<UnderlyingDeliveryStreamToleranceOptionSide>,
	/// UnderlyingDeliveryStreamTotalPositiveTolerance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41795")]
	pub underlying_delivery_stream_total_positive_tolerance: Option<f32>,
	/// UnderlyingDeliveryStreamTotalNegativeTolerance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41796")]
	pub underlying_delivery_stream_total_negative_tolerance: Option<f32>,
	/// UnderlyingDeliveryStreamNotionalConversionFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41797")]
	pub underlying_delivery_stream_notional_conversion_factor: Option<f64>,
	/// UnderlyingDeliveryStreamTransportEquipment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41798")]
	pub underlying_delivery_stream_transport_equipment: Option<String>,
	/// UnderlyingDeliveryStreamElectingPartySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41799")]
	pub underlying_delivery_stream_electing_party_side: Option<UnderlyingDeliveryStreamElectingPartySide>,
	/// UnderlyingDeliveryStreamDeliveryPointSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42196")]
	pub underlying_delivery_stream_delivery_point_source: Option<UnderlyingDeliveryStreamDeliveryPointSource>,
	/// UnderlyingDeliveryStreamDeliveryPointDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42197")]
	pub underlying_delivery_stream_delivery_point_desc: Option<String>,
	/// UnderlyingDeliveryStreamRouteOrCharter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43096")]
	pub underlying_delivery_stream_route_or_charter: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamType {
	/// Periodic (default if not specified)
	#[serde(rename = "0")]
	Periodic,
	/// Initial
	#[serde(rename = "1")]
	Initial,
	/// Single
	#[serde(rename = "2")]
	Single,
}

impl Default for UnderlyingDeliveryStreamType {
	fn default() -> Self {
		UnderlyingDeliveryStreamType::Periodic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamDeliveryRestriction {
	/// Firm (Never excused of delivery obligations)
	#[serde(rename = "1")]
	Firm,
	/// Interruptable or non-firm (Excused when interrupted for any reason or for no reason without liability)
	#[serde(rename = "2")]
	InterruptableOrNonFirm,
	/// Force majeure (Excused when prevented by force majeure)
	#[serde(rename = "3")]
	ForceMajeure,
	/// System firm (Must be supplied from the owned or controlled generation or pre-existing purchased power assets of the system
	/// specified)
	#[serde(rename = "4")]
	SystemFirm,
	/// Unit firm (Must be supplied from the generation asset specified)
	#[serde(rename = "5")]
	UnitFirm,
}

impl Default for UnderlyingDeliveryStreamDeliveryRestriction {
	fn default() -> Self {
		UnderlyingDeliveryStreamDeliveryRestriction::Firm
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamDeliveryContingentPartySide {
	/// Buyer
	#[serde(rename = "0")]
	Buyer,
	/// Seller
	#[serde(rename = "1")]
	Seller,
}

impl Default for UnderlyingDeliveryStreamDeliveryContingentPartySide {
	fn default() -> Self {
		UnderlyingDeliveryStreamDeliveryContingentPartySide::Buyer
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamTitleTransferCondition {
	/// Transfers with risk of loss
	#[serde(rename = "0")]
	TransfersWithRiskOfLoss,
	/// Does not transfer with risk of loss
	#[serde(rename = "1")]
	DoesNotTransferWithRiskOfLoss,
}

impl Default for UnderlyingDeliveryStreamTitleTransferCondition {
	fn default() -> Self {
		UnderlyingDeliveryStreamTitleTransferCondition::TransfersWithRiskOfLoss
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamToleranceUnitOfMeasure {
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

impl Default for UnderlyingDeliveryStreamToleranceUnitOfMeasure {
	fn default() -> Self {
		UnderlyingDeliveryStreamToleranceUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamToleranceType {
	/// Absolute
	#[serde(rename = "0")]
	Absolute,
	/// Percentage
	#[serde(rename = "1")]
	Percentage,
}

impl Default for UnderlyingDeliveryStreamToleranceType {
	fn default() -> Self {
		UnderlyingDeliveryStreamToleranceType::Absolute
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamToleranceOptionSide {
	/// Buyer
	#[serde(rename = "1")]
	Buyer,
	/// Seller
	#[serde(rename = "2")]
	Seller,
}

impl Default for UnderlyingDeliveryStreamToleranceOptionSide {
	fn default() -> Self {
		UnderlyingDeliveryStreamToleranceOptionSide::Buyer
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamElectingPartySide {
	/// Buyer
	#[serde(rename = "0")]
	Buyer,
	/// Seller
	#[serde(rename = "1")]
	Seller,
}

impl Default for UnderlyingDeliveryStreamElectingPartySide {
	fn default() -> Self {
		UnderlyingDeliveryStreamElectingPartySide::Buyer
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingDeliveryStreamDeliveryPointSource {
	/// Proprietary
	#[serde(rename = "0")]
	Proprietary,
	/// Energy Identification Code
	#[serde(rename = "1")]
	EnergyIdentificationCode,
}

impl Default for UnderlyingDeliveryStreamDeliveryPointSource {
	fn default() -> Self {
		UnderlyingDeliveryStreamDeliveryPointSource::Proprietary
	}
}
