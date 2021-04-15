
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommoditySettlPeriodGrp {
	/// NoUnderlyingStreamCommoditySettlPeriods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42002")]
	pub underlying_stream_commodity_settl_periods: Option<fix_common::RepeatingValues<UnderlyingStreamCommoditySettlPeriod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommoditySettlPeriod {
	/// Required if NoUnderlyingStreamCommoditySettlPeriods(42002) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42003")]
	pub underlying_stream_commodity_settl_country: Option<String>,
	/// UnderlyingStreamCommoditySettlTimeZone
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42004")]
	pub underlying_stream_commodity_settl_time_zone: Option<String>,
	/// UnderlyingStreamCommoditySettlFlowType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42005")]
	pub underlying_stream_commodity_settl_flow_type: Option<UnderlyingStreamCommoditySettlFlowType>,
	/// UnderlyingStreamCommoditySettlPeriodNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42006")]
	pub underlying_stream_commodity_settl_period_notional: Option<f64>,
	/// UnderlyingStreamCommoditySettlPeriodNotionalUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42007")]
	pub underlying_stream_commodity_settl_period_notional_unit_of_measure: Option<UnderlyingStreamCommoditySettlPeriodNotionalUnitOfMeasure>,
	/// Conditionally required when UnderlyingStreamCommoditySettlPeriodFrequencyUnit(42009) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42008")]
	pub underlying_stream_commodity_settl_period_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingStreamCommoditySettlPeriodFrequencyPeriod(42008) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42009")]
	pub underlying_stream_commodity_settl_period_frequency_unit: Option<UnderlyingStreamCommoditySettlPeriodFrequencyUnit>,
	/// UnderlyingStreamCommoditySettlPeriodPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42010")]
	pub underlying_stream_commodity_settl_period_price: Option<f64>,
	/// UnderlyingStreamCommoditySettlPeriodPriceUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42011")]
	pub underlying_stream_commodity_settl_period_price_unit_of_measure: Option<UnderlyingStreamCommoditySettlPeriodPriceUnitOfMeasure>,
	/// UnderlyingStreamCommoditySettlPeriodPriceCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42012")]
	pub underlying_stream_commodity_settl_period_price_currency: Option<String>,
	/// UnderlyingStreamCommoditySettlHolidaysProcessingInstruction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42013")]
	pub underlying_stream_commodity_settl_holidays_processing_instruction: Option<UnderlyingStreamCommoditySettlHolidaysProcessingInstruction>,
	/// UnderlyingStreamCommoditySettlPeriodXID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42014")]
	pub underlying_stream_commodity_settl_period_xid: Option<String>,
	/// UnderlyingStreamCommoditySettlPeriodXIDRef
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42015")]
	pub underlying_stream_commodity_settl_period_xid_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlFlowType {
	/// All times
	#[serde(rename = "0")]
	AllTimes,
	/// On peak
	#[serde(rename = "1")]
	OnPeak,
	/// Off peak
	#[serde(rename = "2")]
	OffPeak,
	/// Base
	#[serde(rename = "3")]
	Base,
	/// Block hours
	#[serde(rename = "4")]
	BlockHours,
	/// Other
	#[serde(rename = "5")]
	Other,
}

impl Default for UnderlyingStreamCommoditySettlFlowType {
	fn default() -> Self {
		UnderlyingStreamCommoditySettlFlowType::AllTimes
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlPeriodNotionalUnitOfMeasure {
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

impl Default for UnderlyingStreamCommoditySettlPeriodNotionalUnitOfMeasure {
	fn default() -> Self {
		UnderlyingStreamCommoditySettlPeriodNotionalUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlPeriodFrequencyUnit {
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

impl Default for UnderlyingStreamCommoditySettlPeriodFrequencyUnit {
	fn default() -> Self {
		UnderlyingStreamCommoditySettlPeriodFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlPeriodPriceUnitOfMeasure {
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

impl Default for UnderlyingStreamCommoditySettlPeriodPriceUnitOfMeasure {
	fn default() -> Self {
		UnderlyingStreamCommoditySettlPeriodPriceUnitOfMeasure::Bbl
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlHolidaysProcessingInstruction {
	/// Do not include holidays
	#[serde(rename = "0")]
	DoNotIncludeHolidays,
	/// Include holidays
	#[serde(rename = "1")]
	IncludeHolidays,
}

impl Default for UnderlyingStreamCommoditySettlHolidaysProcessingInstruction {
	fn default() -> Self {
		UnderlyingStreamCommoditySettlHolidaysProcessingInstruction::DoNotIncludeHolidays
	}
}
