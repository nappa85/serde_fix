
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarginAmount {
	/// Number of margin amount entries
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1643")]
	pub margin_amt: Option<fix_common::RepeatingValues<MarginAm>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarginAm {
	/// MarginAmt
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1645")]
	pub margin_amt: Option<f64>,
	/// Total margin requirement if not provided
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1644")]
	pub margin_amt_type: Option<MarginAmtType>,
	/// Can be used to specify the base settlement currency if Currency(15) is not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1646")]
	pub margin_amt_ccy: Option<MarginAmtCcy>,
	/// MarginAmountMarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1714")]
	pub margin_amount_market_segment_id: Option<String>,
	/// MarginAmountMarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1715")]
	pub margin_amount_market_id: Option<String>,
	/// MarginAmtFXRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2088")]
	pub margin_amt_fx_rate: Option<f64>,
	/// MarginAmtFXRateCalc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2089")]
	pub margin_amt_fx_rate_calc: Option<MarginAmtFXRateCalc>,
	/// MarginDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2851")]
	pub margin_direction: Option<MarginDirection>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarginAmtType {
	/// Additional Margin
	#[serde(rename = "1")]
	AdditionalMargin,
	/// Adjusted Margin
	#[serde(rename = "2")]
	AdjustedMargin,
	/// Unadjusted Margin
	#[serde(rename = "3")]
	UnadjustedMargin,
	/// Binary Add-On Amount
	#[serde(rename = "4")]
	BinaryAddOnAmount,
	/// Cash Balance Amount
	#[serde(rename = "5")]
	CashBalanceAmount,
	/// Concentration Margin
	#[serde(rename = "6")]
	ConcentrationMargin,
	/// Core Margin
	#[serde(rename = "7")]
	CoreMargin,
	/// Delivery Margin
	#[serde(rename = "8")]
	DeliveryMargin,
	/// Discretionary Margin
	#[serde(rename = "9")]
	DiscretionaryMargin,
	/// Futures Spread Margin
	#[serde(rename = "10")]
	FuturesSpreadMargin,
	/// Initial Margin
	#[serde(rename = "11")]
	InitialMargin,
	/// Liquidating Margin
	#[serde(rename = "12")]
	LiquidatingMargin,
	/// Margin Call Amount
	#[serde(rename = "13")]
	MarginCallAmount,
	/// Margin Deficit Amount (Shortfall)
	#[serde(rename = "14")]
	MarginDeficitAmount,
	/// Margin Excess Amount (Surplus)
	#[serde(rename = "15")]
	MarginExcessAmount,
	/// Option Premium Amount
	#[serde(rename = "16")]
	OptionPremiumAmount,
	/// Premium Margin
	#[serde(rename = "17")]
	PremiumMargin,
	/// Reserve Margin
	#[serde(rename = "18")]
	ReserveMargin,
	/// Security Collateral Amount
	#[serde(rename = "19")]
	SecurityCollateralAmount,
	/// Stress Test Add-On Amount
	#[serde(rename = "20")]
	StressTestAddOnAmount,
	/// Super Margin
	#[serde(rename = "21")]
	SuperMargin,
	/// Total Margin
	#[serde(rename = "22")]
	TotalMargin,
	/// Variation Margin
	#[serde(rename = "23")]
	VariationMargin,
	/// Secondary Variation Margin
	#[serde(rename = "24")]
	SecondaryVariationMargin,
	/// Rolled up margin deficit
	#[serde(rename = "25")]
	RolledUpMarginDeficit,
	/// Spread response margin (Elaboration: Risk factor component associated with spread moves, curve shape changes and recovery
	/// rates)
	#[serde(rename = "26")]
	SpreadResponseMargin,
	/// Systemic risk margin (Elaboration: Risk factor component to capture parallel shift of credit spreads.)
	#[serde(rename = "27")]
	SystemicRiskMargin,
	/// Curve risk margin (Elaboration: Risk factor captures curve shifts based on portfolio)
	#[serde(rename = "28")]
	CurveRiskMargin,
	/// Index spread risk margin (Elaboration: Risk factor component associated with risks due to widening/tightening spreads of CDS
	/// indices relative to each other)
	#[serde(rename = "29")]
	IndexSpreadRiskMargin,
	/// Sector risk margin (Elaboration: Risk factor component to capture sector risk)
	#[serde(rename = "30")]
	SectorRiskMargin,
	/// Jump-to-default risk margin (Elaboration: Risk factor component to capture extreme widening of credit spreads of a reference
	/// entity. Also known as Idiosyncratic Risk)
	#[serde(rename = "31")]
	JumpToDefaultRiskMargin,
	/// Basis risk margin (Elaboration: Risk factor component to capture basis risk between index and index constituent reference
	/// entities)
	#[serde(rename = "32")]
	BasisRiskMargin,
	/// Interest rate risk margin (Elaboration: Risk factor component associated with parallel shift movements in interest rates)
	#[serde(rename = "33")]
	InterestRateRiskMargin,
	/// Jump-to-health risk margin (Elaboration: Risk factor component to capture extreme narrowing of credit spreads of a reference
	/// entity. Also known as Idiosyncratic Risk)
	#[serde(rename = "34")]
	JumpToHealthRiskMargin,
	/// Other risk margin (Elaboration: Any other risk factors include in the Margin Model)
	#[serde(rename = "35")]
	OtherRiskMargin,
}

impl Default for MarginAmtType {
	fn default() -> Self {
		MarginAmtType::AdditionalMargin
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarginAmtCcy {
	/// Afghani
	#[serde(rename = "AFA")]
	Afa,
	/// Algerian Dinar
	#[serde(rename = "DZD")]
	Dzd,
	/// Andorran Peseta
	#[serde(rename = "ADP")]
	Adp,
	/// Argentine Peso
	#[serde(rename = "ARS")]
	Ars,
	/// Armenian Dram
	#[serde(rename = "AMD")]
	Amd,
	/// Aruban Guilder
	#[serde(rename = "AWG")]
	Awg,
	/// Australian Dollar
	#[serde(rename = "AUD")]
	Aud,
	/// Azerbaijanian Manat
	#[serde(rename = "AZM")]
	Azm,
	/// Bahamian Dollar
	#[serde(rename = "BSD")]
	Bsd,
	/// Bahraini Dinar
	#[serde(rename = "BHD")]
	Bhd,
	/// Baht
	#[serde(rename = "THB")]
	Thb,
	/// Balboa
	#[serde(rename = "PAB")]
	Pab,
	/// Barbados Dollar
	#[serde(rename = "BBD")]
	Bbd,
	/// Belarussian Ruble
	#[serde(rename = "BYB")]
	Byb,
	/// Belgian Franc
	#[serde(rename = "BEF")]
	Bef,
	/// Belize Dollar
	#[serde(rename = "BZD")]
	Bzd,
	/// Bermudian Dollar
	#[serde(rename = "BMD")]
	Bmd,
	/// Bolivar
	#[serde(rename = "VEB")]
	Veb,
	/// Boliviano
	#[serde(rename = "BOB")]
	Bob,
	/// Brazilian Real
	#[serde(rename = "BRL")]
	Brl,
	/// Brunei Dollar
	#[serde(rename = "BND")]
	Bnd,
	/// Burundi Franc
	#[serde(rename = "BIF")]
	Bif,
	/// CFA Franc BCEAO+
	#[serde(rename = "XOF")]
	Xof,
	/// CFA Franc BEAC#
	#[serde(rename = "XAF")]
	Xaf,
	/// CFP Franc
	#[serde(rename = "XPF")]
	Xpf,
	/// Canadian Dollar
	#[serde(rename = "CAD")]
	Cad,
	/// Cape Verde Escudo
	#[serde(rename = "CVE")]
	Cve,
	/// Cayman Islands Dollar
	#[serde(rename = "KYD")]
	Kyd,
	/// Cedi
	#[serde(rename = "GHC")]
	Ghc,
	/// Chilean Peso
	#[serde(rename = "CLP")]
	Clp,
	/// Colombian Peso
	#[serde(rename = "COP")]
	Cop,
	/// Comoro Franc
	#[serde(rename = "KMF")]
	Kmf,
	/// Convertible Marks
	#[serde(rename = "BAM")]
	Bam,
	/// Cordoba Oro
	#[serde(rename = "NIO")]
	Nio,
	/// Costa Rican Colon
	#[serde(rename = "CRC")]
	Crc,
	/// Cuban Peso
	#[serde(rename = "CUP")]
	Cup,
	/// Cyprus Pound
	#[serde(rename = "CYP")]
	Cyp,
	/// Czech Koruna
	#[serde(rename = "CZK")]
	Czk,
	/// Dalasi
	#[serde(rename = "GMD")]
	Gmd,
	/// Danish Krone
	#[serde(rename = "DKK")]
	Dkk,
	/// Denar
	#[serde(rename = "MKD")]
	Mkd,
	/// Deutsche Mark
	#[serde(rename = "DEM")]
	Dem,
	/// Djibouti Franc
	#[serde(rename = "DJF")]
	Djf,
	/// Dobra
	#[serde(rename = "STD")]
	Std,
	/// Dominican Peso
	#[serde(rename = "DOP")]
	Dop,
	/// Dong
	#[serde(rename = "VND")]
	Vnd,
	/// Drachma
	#[serde(rename = "GRD")]
	Grd,
	/// East Caribbean Dollar
	#[serde(rename = "XCD")]
	Xcd,
	/// Egyptian Pound
	#[serde(rename = "EGP")]
	Egp,
	/// El Salvador Colon
	#[serde(rename = "SVC")]
	Svc,
	/// Ethiopian Birr
	#[serde(rename = "ETB")]
	Etb,
	/// Euro
	#[serde(rename = "EUR")]
	Eur,
	/// Falkland Islands Pound
	#[serde(rename = "FKP")]
	Fkp,
	/// Fiji Dollar
	#[serde(rename = "FJD")]
	Fjd,
	/// Forint
	#[serde(rename = "HUF")]
	Huf,
	/// Franc Congolais
	#[serde(rename = "CDF")]
	Cdf,
	/// French Franc
	#[serde(rename = "FRF")]
	Frf,
	/// Gibraltar Pound
	#[serde(rename = "GIP")]
	Gip,
	/// Gourde
	#[serde(rename = "HTG")]
	Htg,
	/// Guarani
	#[serde(rename = "PYG")]
	Pyg,
	/// Guinea Franc
	#[serde(rename = "GNF")]
	Gnf,
	/// Guinea-Bissau Peso
	#[serde(rename = "GWP")]
	Gwp,
	/// Guyana Dollar
	#[serde(rename = "GYD")]
	Gyd,
	/// Hong Kong Dollar
	#[serde(rename = "HKD")]
	Hkd,
	/// Hryvnia
	#[serde(rename = "UAH")]
	Uah,
	/// Iceland Krona
	#[serde(rename = "ISK")]
	Isk,
	/// Indian Rupee
	#[serde(rename = "INR")]
	Inr,
	/// Iranian Rial
	#[serde(rename = "IRR")]
	Irr,
	/// Iraqi Dinar
	#[serde(rename = "IQD")]
	Iqd,
	/// Irish Pound
	#[serde(rename = "IEP")]
	Iep,
	/// Italian Lira
	#[serde(rename = "ITL")]
	Itl,
	/// Jamaican Dollar
	#[serde(rename = "JMD")]
	Jmd,
	/// Jordanian Dinar
	#[serde(rename = "JOD")]
	Jod,
	/// Kenyan Shilling
	#[serde(rename = "KES")]
	Kes,
	/// Kina
	#[serde(rename = "PGK")]
	Pgk,
	/// Kip
	#[serde(rename = "LAK")]
	Lak,
	/// Kroon
	#[serde(rename = "EEK")]
	Eek,
	/// Kuna
	#[serde(rename = "HRK")]
	Hrk,
	/// Kuwaiti Dinar
	#[serde(rename = "KWD")]
	Kwd,
	/// Kwacha
	#[serde(rename = "MWK")]
	Mwk,
	/// Kwacha
	#[serde(rename = "ZMK")]
	Zmk,
	/// Kwanza Reajustado
	#[serde(rename = "AOR")]
	Aor,
	/// Kyat
	#[serde(rename = "MMK")]
	Mmk,
	/// Lari
	#[serde(rename = "GEL")]
	Gel,
	/// Latvian Lats
	#[serde(rename = "LVL")]
	Lvl,
	/// Lebanese Pound
	#[serde(rename = "LBP")]
	Lbp,
	/// Lek
	#[serde(rename = "ALL")]
	All,
	/// Lempira
	#[serde(rename = "HNL")]
	Hnl,
	/// Leone
	#[serde(rename = "SLL")]
	Sll,
	/// Leu
	#[serde(rename = "ROL")]
	Rol,
	/// Lev
	#[serde(rename = "BGL")]
	Bgl,
	/// Liberian Dollar
	#[serde(rename = "LRD")]
	Lrd,
	/// Libyan Dinar
	#[serde(rename = "LYD")]
	Lyd,
	/// Lilangeni
	#[serde(rename = "SZL")]
	Szl,
	/// Lithuanian Litas
	#[serde(rename = "LTL")]
	Ltl,
	/// Loti
	#[serde(rename = "LSL")]
	Lsl,
	/// Luxembourg Franc
	#[serde(rename = "LUF")]
	Luf,
	/// Malagasy Franc
	#[serde(rename = "MGF")]
	Mgf,
	/// Malaysian Ringgit
	#[serde(rename = "MYR")]
	Myr,
	/// Maltese Lira
	#[serde(rename = "MTL")]
	Mtl,
	/// Manat
	#[serde(rename = "TMM")]
	Tmm,
	/// Markka
	#[serde(rename = "FIM")]
	Fim,
	/// Mauritius Rupee
	#[serde(rename = "MUR")]
	Mur,
	/// Metical
	#[serde(rename = "MZM")]
	Mzm,
	/// Mexican Peso
	#[serde(rename = "MXN")]
	Mxn,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "MXV")]
	Mxv,
	/// Moldovan Leu
	#[serde(rename = "MDL")]
	Mdl,
	/// Moroccan Dirham
	#[serde(rename = "MAD")]
	Mad,
	/// Mvdol
	#[serde(rename = "BOV")]
	Bov,
	/// Naira
	#[serde(rename = "NGN")]
	Ngn,
	/// Nakfa
	#[serde(rename = "ERN")]
	Ern,
	/// Namibia Dollar
	#[serde(rename = "NAD")]
	Nad,
	/// Nepalese Rupee
	#[serde(rename = "NPR")]
	Npr,
	/// Netherlands Antillian Guilder
	#[serde(rename = "ANG")]
	Ang,
	/// Netherlands Guilder
	#[serde(rename = "NLG")]
	Nlg,
	/// New Dinar
	#[serde(rename = "YUM")]
	Yum,
	/// New Israeli Sheqel
	#[serde(rename = "ILS")]
	Ils,
	/// New Kwanza
	#[serde(rename = "AON")]
	Aon,
	/// New Taiwan Dollar
	#[serde(rename = "TWD")]
	Twd,
	/// New Zaire
	#[serde(rename = "ZRN")]
	Zrn,
	/// New Zealand Dollar
	#[serde(rename = "NZD")]
	Nzd,
	/// Next day
	#[serde(rename = "USN")]
	Usn,
	/// Ngultrum
	#[serde(rename = "BTN")]
	Btn,
	/// North Korean Won
	#[serde(rename = "KPW")]
	Kpw,
	/// Norwegian Krone
	#[serde(rename = "NOK")]
	Nok,
	/// Nuevo Sol
	#[serde(rename = "PEN")]
	Pen,
	/// Ouguiya
	#[serde(rename = "MRO")]
	Mro,
	/// Pa'anga
	#[serde(rename = "TOP")]
	Top,
	/// Pakistan Rupee
	#[serde(rename = "PKR")]
	Pkr,
	/// Pataca
	#[serde(rename = "MOP")]
	Mop,
	/// Peso Uruguayo
	#[serde(rename = "UYU")]
	Uyu,
	/// Philippine Peso
	#[serde(rename = "PHP")]
	Php,
	/// Portuguese Escudo
	#[serde(rename = "PTE")]
	Pte,
	/// Pound Sterling
	#[serde(rename = "GBP")]
	Gbp,
	/// Pula
	#[serde(rename = "BWP")]
	Bwp,
	/// Qatari Rial
	#[serde(rename = "QAR")]
	Qar,
	/// Quetzal
	#[serde(rename = "GTQ")]
	Gtq,
	/// Rand
	#[serde(rename = "ZAR")]
	Zar,
	/// Rial Omani
	#[serde(rename = "OMR")]
	Omr,
	/// Riel
	#[serde(rename = "KHR")]
	Khr,
	/// Rufiyaa
	#[serde(rename = "MVR")]
	Mvr,
	/// Rupiah
	#[serde(rename = "IDR")]
	Idr,
	/// Russian Ruble
	#[serde(rename = "RUB")]
	Rub,
	/// Russian Ruble
	#[serde(rename = "RUR")]
	Rur,
	/// Rwanda Franc
	#[serde(rename = "RWF")]
	Rwf,
	/// SDR
	#[serde(rename = "XDR")]
	Xdr,
	/// Same day
	#[serde(rename = "USS")]
	Uss,
	/// Saudi Riyal
	#[serde(rename = "SAR")]
	Sar,
	/// Schilling
	#[serde(rename = "ATS")]
	Ats,
	/// Seychelles Rupee
	#[serde(rename = "SCR")]
	Scr,
	/// Singapore Dollar
	#[serde(rename = "SGD")]
	Sgd,
	/// Slovak Koruna
	#[serde(rename = "SKK")]
	Skk,
	/// Solomon Islands Dollar
	#[serde(rename = "SBD")]
	Sbd,
	/// Som
	#[serde(rename = "KGS")]
	Kgs,
	/// Somali Shilling
	#[serde(rename = "SOS")]
	Sos,
	/// Spanish Peseta
	#[serde(rename = "ESP")]
	Esp,
	/// Sri Lanka Rupee
	#[serde(rename = "LKR")]
	Lkr,
	/// St Helena Pound
	#[serde(rename = "SHP")]
	Shp,
	/// Sucre
	#[serde(rename = "ECS")]
	Ecs,
	/// Sudanese Dinar
	#[serde(rename = "SDD")]
	Sdd,
	/// Surinam Guilder
	#[serde(rename = "SRG")]
	Srg,
	/// Swedish Krona
	#[serde(rename = "SEK")]
	Sek,
	/// Swiss Franc
	#[serde(rename = "CHF")]
	Chf,
	/// Syrian Pound
	#[serde(rename = "SYP")]
	Syp,
	/// Tajik Ruble
	#[serde(rename = "TJR")]
	Tjr,
	/// Taka
	#[serde(rename = "BDT")]
	Bdt,
	/// Tala
	#[serde(rename = "WST")]
	Wst,
	/// Tanzanian Shilling
	#[serde(rename = "TZS")]
	Tzs,
	/// Tenge
	#[serde(rename = "KZT")]
	Kzt,
	/// Timor Escudo
	#[serde(rename = "TPE")]
	Tpe,
	/// Tolar
	#[serde(rename = "SIT")]
	Sit,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "TTD")]
	Ttd,
	/// Tugrik
	#[serde(rename = "MNT")]
	Mnt,
	/// Tunisian Dinar
	#[serde(rename = "TND")]
	Tnd,
	/// Turkish Lira
	#[serde(rename = "TRL")]
	Trl,
	/// UAE Dirham
	#[serde(rename = "AED")]
	Aed,
	/// US Dollar
	#[serde(rename = "USD")]
	Usd,
	/// Uganda Shilling
	#[serde(rename = "UGX")]
	Ugx,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "ECV")]
	Ecv,
	/// Unidades de fomento
	#[serde(rename = "CLF")]
	Clf,
	/// Uzbekistan Sum
	#[serde(rename = "UZS")]
	Uzs,
	/// Vatu
	#[serde(rename = "VUV")]
	Vuv,
	/// Won
	#[serde(rename = "KRW")]
	Krw,
	/// Yemeni Rial
	#[serde(rename = "YER")]
	Yer,
	/// Yen
	#[serde(rename = "JPY")]
	Jpy,
	/// Yuan Renminbi
	#[serde(rename = "CNY")]
	Cny,
	/// Zimbabwe Dollar
	#[serde(rename = "ZWD")]
	Zwd,
	/// Zloty
	#[serde(rename = "PLN")]
	Pln,
	/// financial Rand
	#[serde(rename = "ZAL")]
	Zal,
	/// Afghani
	#[serde(rename = "004")]
	N004,
	/// Algerian Dinar
	#[serde(rename = "01")]
	N01,
	/// Andorran Peseta
	#[serde(rename = "020")]
	N020,
	/// Argentine Peso
	#[serde(rename = "032")]
	N032,
	/// Armenian Dram
	#[serde(rename = "051")]
	N051,
	/// Aruban Guilder
	#[serde(rename = "533")]
	N533,
	/// Australian Dollar
	#[serde(rename = "036")]
	N036,
	/// Azerbaijanian Manat
	#[serde(rename = "031")]
	N031,
	/// Bahamian Dollar
	#[serde(rename = "044")]
	N044,
	/// Bahraini Dinar
	#[serde(rename = "048")]
	N048,
	/// Baht
	#[serde(rename = "764")]
	N764,
	/// Balboa
	#[serde(rename = "590")]
	N590,
	/// Barbados Dollar
	#[serde(rename = "052")]
	N052,
	/// Belarussian Ruble
	#[serde(rename = "112")]
	N112,
	/// Belgian Franc
	#[serde(rename = "056")]
	N056,
	/// Belize Dollar
	#[serde(rename = "084")]
	N084,
	/// Bermudian Dollar
	#[serde(rename = "060")]
	N060,
	/// Bolivar
	#[serde(rename = "862")]
	N862,
	/// Boliviano
	#[serde(rename = "068")]
	N068,
	/// Brazilian Real
	#[serde(rename = "986")]
	N986,
	/// Brunei Dollar
	#[serde(rename = "096")]
	N096,
	/// Burundi Franc
	#[serde(rename = "108")]
	N108,
	/// CFA Franc BCEAO+
	#[serde(rename = "952")]
	N952,
	/// CFA Franc BEAC#
	#[serde(rename = "950")]
	N950,
	/// CFP Franc
	#[serde(rename = "953")]
	N953,
	/// Canadian Dollar
	#[serde(rename = "124")]
	N124,
	/// Cape Verde Escudo
	#[serde(rename = "132")]
	N132,
	/// Cayman Islands Dollar
	#[serde(rename = "136")]
	N136,
	/// Cedi
	#[serde(rename = "288")]
	N288,
	/// Chilean Peso
	#[serde(rename = "152")]
	N152,
	/// Colombian Peso
	#[serde(rename = "170")]
	N170,
	/// Comoro Franc
	#[serde(rename = "174")]
	N174,
	/// Convertible Marks
	#[serde(rename = "977")]
	N977,
	/// Cordoba Oro
	#[serde(rename = "558")]
	N558,
	/// Costa Rican Colon
	#[serde(rename = "188")]
	N188,
	/// Cuban Peso
	#[serde(rename = "192")]
	N192,
	/// Cyprus Pound
	#[serde(rename = "196")]
	N196,
	/// Czech Koruna
	#[serde(rename = "203")]
	N203,
	/// Dalasi
	#[serde(rename = "270")]
	N270,
	/// Danish Krone
	#[serde(rename = "208")]
	N208,
	/// Denar
	#[serde(rename = "807")]
	N807,
	/// Deutsche Mark
	#[serde(rename = "280")]
	N280,
	/// Djibouti Franc
	#[serde(rename = "262")]
	N262,
	/// Dobra
	#[serde(rename = "678")]
	N678,
	/// Dominican Peso
	#[serde(rename = "214")]
	N214,
	/// Dong
	#[serde(rename = "704")]
	N704,
	/// Drachma
	#[serde(rename = "300")]
	N300,
	/// East Caribbean Dollar
	#[serde(rename = "951")]
	N951,
	/// Egyptian Pound
	#[serde(rename = "818")]
	N818,
	/// El Salvador Colon
	#[serde(rename = "222")]
	N222,
	/// Ethiopian Birr
	#[serde(rename = "230")]
	N230,
	/// Euro
	#[serde(rename = "978")]
	N978,
	/// Falkland Islands Pound
	#[serde(rename = "238")]
	N238,
	/// Fiji Dollar
	#[serde(rename = "242")]
	N242,
	/// Forint
	#[serde(rename = "348")]
	N348,
	/// Franc Congolais
	#[serde(rename = "976")]
	N976,
	/// French Franc
	#[serde(rename = "250")]
	N250,
	/// Gibraltar Pound
	#[serde(rename = "292")]
	N292,
	/// Gourde
	#[serde(rename = "332")]
	N332,
	/// Guarani
	#[serde(rename = "600")]
	N600,
	/// Guinea Franc
	#[serde(rename = "324")]
	N324,
	/// Guinea-Bissau Peso
	#[serde(rename = "624")]
	N624,
	/// Guyana Dollar
	#[serde(rename = "328")]
	N328,
	/// Hong Kong Dollar
	#[serde(rename = "344")]
	N344,
	/// Hryvnia
	#[serde(rename = "980")]
	N980,
	/// Iceland Krona
	#[serde(rename = "352")]
	N352,
	/// Indian Rupee
	#[serde(rename = "356")]
	N356,
	/// Iranian Rial
	#[serde(rename = "364")]
	N364,
	/// Iraqi Dinar
	#[serde(rename = "368")]
	N368,
	/// Irish Pound
	#[serde(rename = "372")]
	N372,
	/// Italian Lira
	#[serde(rename = "380")]
	N380,
	/// Jamaican Dollar
	#[serde(rename = "388")]
	N388,
	/// Jordanian Dinar
	#[serde(rename = "400")]
	N400,
	/// Kenyan Shilling
	#[serde(rename = "404")]
	N404,
	/// Kina
	#[serde(rename = "598")]
	N598,
	/// Kip
	#[serde(rename = "418")]
	N418,
	/// Kroon
	#[serde(rename = "233")]
	N233,
	/// Kuna
	#[serde(rename = "191")]
	N191,
	/// Kuwaiti Dinar
	#[serde(rename = "414")]
	N414,
	/// Kwacha
	#[serde(rename = "454")]
	N454,
	/// Kwacha
	#[serde(rename = "894")]
	N894,
	/// Kwanza Reajustado
	#[serde(rename = "982")]
	N982,
	/// Kyat
	#[serde(rename = "104")]
	N104,
	/// Lari
	#[serde(rename = "981")]
	N981,
	/// Latvian Lats
	#[serde(rename = "428")]
	N428,
	/// Lebanese Pound
	#[serde(rename = "422")]
	N422,
	/// Lek
	#[serde(rename = "008")]
	N008,
	/// Lempira
	#[serde(rename = "340")]
	N340,
	/// Leone
	#[serde(rename = "694")]
	N694,
	/// Leu
	#[serde(rename = "642")]
	N642,
	/// Lev
	#[serde(rename = "100")]
	N100,
	/// Liberian Dollar
	#[serde(rename = "430")]
	N430,
	/// Libyan Dinar
	#[serde(rename = "434")]
	N434,
	/// Lilangeni
	#[serde(rename = "748")]
	N748,
	/// Lithuanian Litas
	#[serde(rename = "440")]
	N440,
	/// Loti
	#[serde(rename = "426")]
	N426,
	/// Luxembourg Franc
	#[serde(rename = "442")]
	N442,
	/// Malagasy Franc
	#[serde(rename = "450")]
	N450,
	/// Malaysian Ringgit
	#[serde(rename = "458")]
	N458,
	/// Maltese Lira
	#[serde(rename = "470")]
	N470,
	/// Manat
	#[serde(rename = "795")]
	N795,
	/// Markka
	#[serde(rename = "246")]
	N246,
	/// Mauritius Rupee
	#[serde(rename = "480")]
	N480,
	/// Metical
	#[serde(rename = "508")]
	N508,
	/// Mexican Peso
	#[serde(rename = "484")]
	N484,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "979")]
	N979,
	/// Moldovan Leu
	#[serde(rename = "498")]
	N498,
	/// Moroccan Dirham
	#[serde(rename = "504")]
	N504,
	/// Mvdol
	#[serde(rename = "984")]
	N984,
	/// Naira
	#[serde(rename = "566")]
	N566,
	/// Nakfa
	#[serde(rename = "232")]
	N232,
	/// Namibia Dollar
	#[serde(rename = "516")]
	N516,
	/// Nepalese Rupee
	#[serde(rename = "524")]
	N524,
	/// Netherlands Antillian Guilder
	#[serde(rename = "532")]
	N532,
	/// Netherlands Guilder
	#[serde(rename = "528")]
	N528,
	/// New Dinar
	#[serde(rename = "891")]
	N891,
	/// New Israeli Sheqel
	#[serde(rename = "376")]
	N376,
	/// New Kwanza
	#[serde(rename = "02")]
	N02,
	/// New Taiwan Dollar
	#[serde(rename = "901")]
	N901,
	/// New Zaire
	#[serde(rename = "180")]
	N180,
	/// New Zealand Dollar
	#[serde(rename = "554")]
	N554,
	/// Next day
	#[serde(rename = "997")]
	N997,
	/// Ngultrum
	#[serde(rename = "064")]
	N064,
	/// North Korean Won
	#[serde(rename = "408")]
	N408,
	/// Norwegian Krone
	#[serde(rename = "578")]
	N578,
	/// Nuevo Sol
	#[serde(rename = "604")]
	N604,
	/// Ouguiya
	#[serde(rename = "478")]
	N478,
	/// Pa'anga
	#[serde(rename = "776")]
	N776,
	/// Pakistan Rupee
	#[serde(rename = "586")]
	N586,
	/// Pataca
	#[serde(rename = "446")]
	N446,
	/// Peso Uruguayo
	#[serde(rename = "858")]
	N858,
	/// Philippine Peso
	#[serde(rename = "608")]
	N608,
	/// Portuguese Escudo
	#[serde(rename = "620")]
	N620,
	/// Pound Sterling
	#[serde(rename = "826")]
	N826,
	/// Pula
	#[serde(rename = "072")]
	N072,
	/// Qatari Rial
	#[serde(rename = "634")]
	N634,
	/// Quetzal
	#[serde(rename = "320")]
	N320,
	/// Rand
	#[serde(rename = "710")]
	N710,
	/// Rial Omani
	#[serde(rename = "512")]
	N512,
	/// Riel
	#[serde(rename = "116")]
	N116,
	/// Rufiyaa
	#[serde(rename = "462")]
	N462,
	/// Rupiah
	#[serde(rename = "360")]
	N360,
	/// Russian Ruble
	#[serde(rename = "643")]
	N643,
	/// Russian Ruble
	#[serde(rename = "810")]
	N810,
	/// Rwanda Franc
	#[serde(rename = "646")]
	N646,
	/// SDR
	#[serde(rename = "960")]
	N960,
	/// Same day
	#[serde(rename = "998")]
	N998,
	/// Saudi Riyal
	#[serde(rename = "682")]
	N682,
	/// Schilling
	#[serde(rename = "040")]
	N040,
	/// Seychelles Rupee
	#[serde(rename = "690")]
	N690,
	/// Singapore Dollar
	#[serde(rename = "702")]
	N702,
	/// Slovak Koruna
	#[serde(rename = "703")]
	N703,
	/// Solomon Islands Dollar
	#[serde(rename = "090")]
	N090,
	/// Som
	#[serde(rename = "417")]
	N417,
	/// Somali Shilling
	#[serde(rename = "706")]
	N706,
	/// Spanish Peseta
	#[serde(rename = "724")]
	N724,
	/// Sri Lanka Rupee
	#[serde(rename = "144")]
	N144,
	/// St Helena Pound
	#[serde(rename = "654")]
	N654,
	/// Sucre
	#[serde(rename = "218")]
	N218,
	/// Sudanese Dinar
	#[serde(rename = "736")]
	N736,
	/// Surinam Guilder
	#[serde(rename = "740")]
	N740,
	/// Swedish Krona
	#[serde(rename = "752")]
	N752,
	/// Swiss Franc
	#[serde(rename = "756")]
	N756,
	/// Syrian Pound
	#[serde(rename = "760")]
	N760,
	/// Tajik Ruble
	#[serde(rename = "762")]
	N762,
	/// Taka
	#[serde(rename = "050")]
	N050,
	/// Tala
	#[serde(rename = "882")]
	N882,
	/// Tanzanian Shilling
	#[serde(rename = "834")]
	N834,
	/// Tenge
	#[serde(rename = "398")]
	N398,
	/// Timor Escudo
	#[serde(rename = "626")]
	N626,
	/// Tolar
	#[serde(rename = "705")]
	N705,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "780")]
	N780,
	/// Tugrik
	#[serde(rename = "496")]
	N496,
	/// Tunisian Dinar
	#[serde(rename = "788")]
	N788,
	/// Turkish Lira
	#[serde(rename = "792")]
	N792,
	/// UAE Dirham
	#[serde(rename = "784")]
	N784,
	/// US Dollar
	#[serde(rename = "840")]
	N840,
	/// Uganda Shilling
	#[serde(rename = "800")]
	N800,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "983")]
	N983,
	/// Unidades de fomento
	#[serde(rename = "990")]
	N990,
	/// Uzbekistan Sum
	#[serde(rename = "860")]
	N860,
	/// Vatu
	#[serde(rename = "548")]
	N548,
	/// Won
	#[serde(rename = "410")]
	N410,
	/// Yemeni Rial
	#[serde(rename = "886")]
	N886,
	/// Yen
	#[serde(rename = "392")]
	N392,
	/// Yuan Renminbi
	#[serde(rename = "156")]
	N156,
	/// Zimbabwe Dollar
	#[serde(rename = "716")]
	N716,
	/// Zloty
	#[serde(rename = "985")]
	N985,
	/// financial Rand
	#[serde(rename = "991")]
	N991,
	/// Gold
	#[serde(rename = "XAU")]
	Xau,
	/// European Composite Unit (EURCO)
	#[serde(rename = "XBA")]
	Xba,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "XBB")]
	Xbb,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "XBC")]
	Xbc,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "XBD")]
	Xbd,
	/// Palladium
	#[serde(rename = "XPD")]
	Xpd,
	/// Platinum
	#[serde(rename = "XPT")]
	Xpt,
	/// Silver
	#[serde(rename = "XAG")]
	Xag,
	/// UIC-Franc
	#[serde(rename = "XFU")]
	Xfu,
	/// Gold-Franc
	#[serde(rename = "XFO")]
	Xfo,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "XTS")]
	Xts,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "XXX")]
	Xxx,
	/// Gold
	#[serde(rename = "959")]
	N959,
	/// European Composite Unit (EURCO)
	#[serde(rename = "955")]
	N955,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "956")]
	N956,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "957")]
	N957,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "958")]
	N958,
	/// Palladium
	#[serde(rename = "964")]
	N964,
	/// Platinum
	#[serde(rename = "962")]
	N962,
	/// Silver
	#[serde(rename = "961")]
	N961,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "963")]
	N963,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "999")]
	N999,
}

impl Default for MarginAmtCcy {
	fn default() -> Self {
		MarginAmtCcy::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarginAmtFXRateCalc {
	/// Divided
	#[serde(rename = "D")]
	Divided,
	/// Multiply
	#[serde(rename = "M")]
	Multiply,
}

impl Default for MarginAmtFXRateCalc {
	fn default() -> Self {
		MarginAmtFXRateCalc::Divided
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarginDirection {
	/// Posted
	#[serde(rename = "0")]
	Posted,
	/// Received
	#[serde(rename = "1")]
	Received,
}

impl Default for MarginDirection {
	fn default() -> Self {
		MarginDirection::Posted
	}
}
