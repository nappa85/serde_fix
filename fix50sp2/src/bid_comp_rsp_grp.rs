
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BidCompRspGrp {
	/// Used if BidType="Disclosed"
	#[serde(rename = "420")]
	pub bid_components: fix_common::RepeatingValues<BidComponent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BidComponent {
	/// ListID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// ISO Country Code.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "421")]
	pub country: Option<Country>,
	/// When used in request for a "Disclosed" bid indicates that bid is required on assumption that SideValue1 is Buy or Sell. SideValue2
	/// can be derived by inference.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Second element of price
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// PriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// The difference between the value of a future and the value of the underlying equities after allowing for the discounted cash
	/// flows associated with the underlying stocks (E.g. Dividends etc).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "406")]
	pub fair_value: Option<f64>,
	/// Net/Gross
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "430")]
	pub net_gross_ind: Option<NetGrossInd>,
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<fix_common::LocalMktDate>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Country {
	/// AFGHANISTAN
	#[serde(rename = "AF")]
	Afghanistan,
	/// ALBANIA
	#[serde(rename = "AL")]
	Albania,
	/// ALGERIA
	#[serde(rename = "DZ")]
	Algeria,
	/// AMERICAN SAMOA
	#[serde(rename = "AS")]
	AmericanSamoa,
	/// ANDORRA
	#[serde(rename = "AD")]
	Andorra,
	/// ANGOLA
	#[serde(rename = "AO")]
	Angola,
	/// ANGUILLA
	#[serde(rename = "AI")]
	Anguilla,
	/// ANTARCTICA
	#[serde(rename = "AQ")]
	Antarctica,
	/// ANTIGUA AND BARBUDA
	#[serde(rename = "AG")]
	AntiguaAndBarbuda,
	/// ARGENTINA
	#[serde(rename = "AR")]
	Argentina,
	/// ARMENIA
	#[serde(rename = "AM")]
	Armenia,
	/// ARUBA
	#[serde(rename = "AW")]
	Aruba,
	/// AUSTRALIA
	#[serde(rename = "AU")]
	Australia,
	/// AUSTRIA
	#[serde(rename = "AT")]
	Austria,
	/// AZERBAIJAN
	#[serde(rename = "AZ")]
	Azerbaijan,
	/// BAHAMAS
	#[serde(rename = "BS")]
	Bahamas,
	/// BAHRAIN
	#[serde(rename = "BH")]
	Bahrain,
	/// BANGLADESH
	#[serde(rename = "BD")]
	Bangladesh,
	/// BARBADOS
	#[serde(rename = "BB")]
	Barbados,
	/// BELARUS
	#[serde(rename = "BY")]
	Belarus,
	/// BELGIUM
	#[serde(rename = "BE")]
	Belgium,
	/// BELIZE
	#[serde(rename = "BZ")]
	Belize,
	/// BENIN
	#[serde(rename = "BJ")]
	Benin,
	/// BERMUDA
	#[serde(rename = "BM")]
	Bermuda,
	/// BHUTAN
	#[serde(rename = "BT")]
	Bhutan,
	/// BOLIVIA
	#[serde(rename = "BO")]
	Bolivia,
	/// BOSNIA AND HERZEGOVINA
	#[serde(rename = "BA")]
	BosniaAndHerzegovina,
	/// BOTSWANA
	#[serde(rename = "BW")]
	Botswana,
	/// BOUVET ISLAND
	#[serde(rename = "BV")]
	BouvetIsland,
	/// BRAZIL
	#[serde(rename = "BR")]
	Brazil,
	/// BRITISH INDIAN OCEAN TERRITORY
	#[serde(rename = "IO")]
	BritishIndianOceanTerritory,
	/// BRUNEI DARUSSALAM
	#[serde(rename = "BN")]
	BruneiDarussalam,
	/// BULGARIA
	#[serde(rename = "BG")]
	Bulgaria,
	/// BURKINA FASO
	#[serde(rename = "BF")]
	BurkinaFaso,
	/// BURUNDI
	#[serde(rename = "BI")]
	Burundi,
	/// CAMBODIA
	#[serde(rename = "KH")]
	Cambodia,
	/// CAMEROON
	#[serde(rename = "CM")]
	Cameroon,
	/// CANADA
	#[serde(rename = "CA")]
	Canada,
	/// CAPE VERDE
	#[serde(rename = "CV")]
	CapeVerde,
	/// CAYMAN ISLANDS
	#[serde(rename = "KY")]
	CaymanIslands,
	/// CENTRAL AFRICAN REPUBLIC
	#[serde(rename = "CF")]
	CentralAfricanRepublic,
	/// CHAD
	#[serde(rename = "TD")]
	Chad,
	/// CHILE
	#[serde(rename = "CL")]
	Chile,
	/// CHINA
	#[serde(rename = "CN")]
	China,
	/// CHRISTMAS ISLAND
	#[serde(rename = "CX")]
	ChristmasIsland,
	/// COCOS (KEELING) ISLANDS
	#[serde(rename = "CC")]
	CocosIslands,
	/// COLOMBIA
	#[serde(rename = "CO")]
	Colombia,
	/// COMOROS
	#[serde(rename = "KM")]
	Comoros,
	/// CONGO
	#[serde(rename = "CG")]
	Congo,
	/// CONGO, THE DEMOCRATIC REPUBLIC OF THE
	#[serde(rename = "CD")]
	CongoTheDemocraticRepublicOfThe,
	/// COOK ISLANDS
	#[serde(rename = "CK")]
	CookIslands,
	/// COSTA RICA
	#[serde(rename = "CR")]
	CostaRica,
	/// COTE D'IVOIRE
	#[serde(rename = "CI")]
	CoteDIvoire,
	/// CROATIA
	#[serde(rename = "HR")]
	Croatia,
	/// CUBA
	#[serde(rename = "CU")]
	Cuba,
	/// CYPRUS
	#[serde(rename = "CY")]
	Cyprus,
	/// CZECH REPUBLIC
	#[serde(rename = "CZ")]
	CzechRepublic,
	/// DENMARK
	#[serde(rename = "DK")]
	Denmark,
	/// DJIBOUTI
	#[serde(rename = "DJ")]
	Djibouti,
	/// DOMINICA
	#[serde(rename = "DM")]
	Dominica,
	/// DOMINICAN REPUBLIC
	#[serde(rename = "DO")]
	DominicanRepublic,
	/// ECUADOR
	#[serde(rename = "EC")]
	Ecuador,
	/// EGYPT
	#[serde(rename = "EG")]
	Egypt,
	/// EL SALVADOR
	#[serde(rename = "SV")]
	ElSalvador,
	/// EQUATORIAL GUINEA
	#[serde(rename = "GQ")]
	EquatorialGuinea,
	/// ERITREA
	#[serde(rename = "ER")]
	Eritrea,
	/// ESTONIA
	#[serde(rename = "EE")]
	Estonia,
	/// ETHIOPIA
	#[serde(rename = "ET")]
	Ethiopia,
	/// FALKLAND ISLANDS (MALVINAS)
	#[serde(rename = "FK")]
	FalklandIslands,
	/// FAROE ISLANDS
	#[serde(rename = "FO")]
	FaroeIslands,
	/// FIJI
	#[serde(rename = "FJ")]
	Fiji,
	/// FINLAND
	#[serde(rename = "FI")]
	Finland,
	/// FRANCE
	#[serde(rename = "FR")]
	France,
	/// FRENCH GUIANA
	#[serde(rename = "GF")]
	FrenchGuiana,
	/// FRENCH POLYNESIA
	#[serde(rename = "PF")]
	FrenchPolynesia,
	/// FRENCH SOUTHERN TERRITORIES
	#[serde(rename = "TF")]
	FrenchSouthernTerritories,
	/// GABON
	#[serde(rename = "GA")]
	Gabon,
	/// GAMBIA
	#[serde(rename = "GM")]
	Gambia,
	/// GEORGIA
	#[serde(rename = "GE")]
	Georgia,
	/// GERMANY
	#[serde(rename = "DE")]
	Germany,
	/// GHANA
	#[serde(rename = "GH")]
	Ghana,
	/// GIBRALTAR
	#[serde(rename = "GI")]
	Gibraltar,
	/// GREECE
	#[serde(rename = "GR")]
	Greece,
	/// GREENLAND
	#[serde(rename = "GL")]
	Greenland,
	/// GRENADA
	#[serde(rename = "GD")]
	Grenada,
	/// GUADELOUPE
	#[serde(rename = "GP")]
	Guadeloupe,
	/// GUAM
	#[serde(rename = "GU")]
	Guam,
	/// GUATEMALA
	#[serde(rename = "GT")]
	Guatemala,
	/// GUINEA
	#[serde(rename = "GN")]
	Guinea,
	/// GUINEA-BISSAU
	#[serde(rename = "GW")]
	GuineaBissau,
	/// GUYANA
	#[serde(rename = "GY")]
	Guyana,
	/// HAITI
	#[serde(rename = "HT")]
	Haiti,
	/// HEARD ISLAND AND MCDONALD ISLANDS
	#[serde(rename = "HM")]
	HeardIslandAndMcdonaldIslands,
	/// HOLY SEE (VATICAN CITY STATE)
	#[serde(rename = "VA")]
	HolySee,
	/// HONDURAS
	#[serde(rename = "HN")]
	Honduras,
	/// HONG KONG
	#[serde(rename = "HK")]
	HongKong,
	/// HUNGARY
	#[serde(rename = "HU")]
	Hungary,
	/// ICELAND
	#[serde(rename = "IS")]
	Iceland,
	/// INDIA
	#[serde(rename = "IN")]
	India,
	/// INDONESIA
	#[serde(rename = "ID")]
	Indonesia,
	/// IRAN, ISLAMIC REPUBLIC OF
	#[serde(rename = "IR")]
	IranIslamicRepublicOf,
	/// IRAQ
	#[serde(rename = "IQ")]
	Iraq,
	/// IRELAND
	#[serde(rename = "IE")]
	Ireland,
	/// ISRAEL
	#[serde(rename = "IL")]
	Israel,
	/// ITALY
	#[serde(rename = "IT")]
	Italy,
	/// JAMAICA
	#[serde(rename = "JM")]
	Jamaica,
	/// JAPAN
	#[serde(rename = "JP")]
	Japan,
	/// JORDAN
	#[serde(rename = "JO")]
	Jordan,
	/// KAZAKHSTAN
	#[serde(rename = "KZ")]
	Kazakhstan,
	/// KENYA
	#[serde(rename = "KE")]
	Kenya,
	/// KIRIBATI
	#[serde(rename = "KI")]
	Kiribati,
	/// KOREA, DEMOCRATIC PEOPLE'S REPUBLIC OF
	#[serde(rename = "KP")]
	KoreaDemocraticPeopleSRepublicOf,
	/// KOREA, REPUBLIC OF
	#[serde(rename = "KR")]
	KoreaRepublicOf,
	/// KUWAIT
	#[serde(rename = "KW")]
	Kuwait,
	/// KYRGYZSTAN
	#[serde(rename = "KG")]
	Kyrgyzstan,
	/// LAO PEOPLE'S DEMOCRATIC REPUBLIC
	#[serde(rename = "LA")]
	LaoPeopleSDemocraticRepublic,
	/// LATVIA
	#[serde(rename = "LV")]
	Latvia,
	/// LEBANON
	#[serde(rename = "LB")]
	Lebanon,
	/// LESOTHO
	#[serde(rename = "LS")]
	Lesotho,
	/// LIBERIA
	#[serde(rename = "LR")]
	Liberia,
	/// LIBYAN ARAB JAMAHIRIYA
	#[serde(rename = "LY")]
	LibyanArabJamahiriya,
	/// LIECHTENSTEIN
	#[serde(rename = "LI")]
	Liechtenstein,
	/// LITHUANIA
	#[serde(rename = "LT")]
	Lithuania,
	/// LUXEMBOURG
	#[serde(rename = "LU")]
	Luxembourg,
	/// MACAO
	#[serde(rename = "MO")]
	Macao,
	/// MACEDONIA, THE FORMER YUGOSLAV REPUBLIC OF
	#[serde(rename = "MK")]
	MacedoniaTheFormerYugoslavRepublicOf,
	/// MADAGASCAR
	#[serde(rename = "MG")]
	Madagascar,
	/// MALAWI
	#[serde(rename = "MW")]
	Malawi,
	/// MALAYSIA
	#[serde(rename = "MY")]
	Malaysia,
	/// MALDIVES
	#[serde(rename = "MV")]
	Maldives,
	/// MALI
	#[serde(rename = "ML")]
	Mali,
	/// MALTA
	#[serde(rename = "MT")]
	Malta,
	/// MARSHALL ISLANDS
	#[serde(rename = "MH")]
	MarshallIslands,
	/// MARTINIQUE
	#[serde(rename = "MQ")]
	Martinique,
	/// MAURITANIA
	#[serde(rename = "MR")]
	Mauritania,
	/// MAURITIUS
	#[serde(rename = "MU")]
	Mauritius,
	/// MAYOTTE
	#[serde(rename = "YT")]
	Mayotte,
	/// MEXICO
	#[serde(rename = "MX")]
	Mexico,
	/// MICRONESIA, FEDERATED STATES OF
	#[serde(rename = "FM")]
	MicronesiaFederatedStatesOf,
	/// MOLDOVA, REPUBLIC OF
	#[serde(rename = "MD")]
	MoldovaRepublicOf,
	/// MONACO
	#[serde(rename = "MC")]
	Monaco,
	/// MONGOLIA
	#[serde(rename = "MN")]
	Mongolia,
	/// MONTSERRAT
	#[serde(rename = "MS")]
	Montserrat,
	/// MOROCCO
	#[serde(rename = "MA")]
	Morocco,
	/// MOZAMBIQUE
	#[serde(rename = "MZ")]
	Mozambique,
	/// MYANMAR
	#[serde(rename = "MM")]
	Myanmar,
	/// NAMIBIA
	#[serde(rename = "NA")]
	Namibia,
	/// NAURU
	#[serde(rename = "NR")]
	Nauru,
	/// NEPAL
	#[serde(rename = "NP")]
	Nepal,
	/// NETHERLANDS
	#[serde(rename = "NL")]
	Netherlands,
	/// NETHERLANDS ANTILLES
	#[serde(rename = "AN")]
	NetherlandsAntilles,
	/// NEW CALEDONIA
	#[serde(rename = "NC")]
	NewCaledonia,
	/// NEW ZEALAND
	#[serde(rename = "NZ")]
	NewZealand,
	/// NICARAGUA
	#[serde(rename = "NI")]
	Nicaragua,
	/// NIGER
	#[serde(rename = "NE")]
	Niger,
	/// NIGERIA
	#[serde(rename = "NG")]
	Nigeria,
	/// NIUE
	#[serde(rename = "NU")]
	Niue,
	/// NORFOLK ISLAND
	#[serde(rename = "NF")]
	NorfolkIsland,
	/// NORTHERN MARIANA ISLANDS
	#[serde(rename = "MP")]
	NorthernMarianaIslands,
	/// NORWAY
	#[serde(rename = "NO")]
	Norway,
	/// OMAN
	#[serde(rename = "OM")]
	Oman,
	/// PAKISTAN
	#[serde(rename = "PK")]
	Pakistan,
	/// PALAU
	#[serde(rename = "PW")]
	Palau,
	/// PALESTINIAN TERRITORY, OCCUPIED
	#[serde(rename = "PS")]
	PalestinianTerritoryOccupied,
	/// PANAMA
	#[serde(rename = "PA")]
	Panama,
	/// PAPUA NEW GUINEA
	#[serde(rename = "PG")]
	PapuaNewGuinea,
	/// PARAGUAY
	#[serde(rename = "PY")]
	Paraguay,
	/// PERU
	#[serde(rename = "PE")]
	Peru,
	/// PHILIPPINES
	#[serde(rename = "PH")]
	Philippines,
	/// PITCAIRN
	#[serde(rename = "PN")]
	Pitcairn,
	/// POLAND
	#[serde(rename = "PL")]
	Poland,
	/// PORTUGAL
	#[serde(rename = "PT")]
	Portugal,
	/// PUERTO RICO
	#[serde(rename = "PR")]
	PuertoRico,
	/// QATAR
	#[serde(rename = "QA")]
	Qatar,
	/// REUNION
	#[serde(rename = "RE")]
	Reunion,
	/// ROMANIA
	#[serde(rename = "RO")]
	Romania,
	/// RUSSIAN FEDERATION
	#[serde(rename = "RU")]
	RussianFederation,
	/// RWANDA
	#[serde(rename = "RW")]
	Rwanda,
	/// SAINT HELENA
	#[serde(rename = "SH")]
	SaintHelena,
	/// SAINT KITTS AND NEVIS
	#[serde(rename = "KN")]
	SaintKittsAndNevis,
	/// SAINT LUCIA
	#[serde(rename = "LC")]
	SaintLucia,
	/// SAINT PIERRE AND MIQUELON
	#[serde(rename = "PM")]
	SaintPierreAndMiquelon,
	/// SAINT VINCENT AND THE GRENADINES
	#[serde(rename = "VC")]
	SaintVincentAndTheGrenadines,
	/// SAMOA
	#[serde(rename = "WS")]
	Samoa,
	/// SAN MARINO
	#[serde(rename = "SM")]
	SanMarino,
	/// SAO TOME AND PRINCIPE
	#[serde(rename = "ST")]
	SaoTomeAndPrincipe,
	/// SAUDI ARABIA
	#[serde(rename = "SA")]
	SaudiArabia,
	/// SENEGAL
	#[serde(rename = "SN")]
	Senegal,
	/// SEYCHELLES
	#[serde(rename = "SC")]
	Seychelles,
	/// SIERRA LEONE
	#[serde(rename = "SL")]
	SierraLeone,
	/// SINGAPORE
	#[serde(rename = "SG")]
	Singapore,
	/// SLOVAKIA
	#[serde(rename = "SK")]
	Slovakia,
	/// SLOVENIA
	#[serde(rename = "SI")]
	Slovenia,
	/// SOLOMON ISLANDS
	#[serde(rename = "SB")]
	SolomonIslands,
	/// SOMALIA
	#[serde(rename = "SO")]
	Somalia,
	/// SOUTH AFRICA
	#[serde(rename = "ZA")]
	SouthAfrica,
	/// SOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDS
	#[serde(rename = "GS")]
	SouthGeorgiaAndTheSouthSandwichIslands,
	/// SPAIN
	#[serde(rename = "ES")]
	Spain,
	/// SRI LANKA
	#[serde(rename = "LK")]
	SriLanka,
	/// SUDAN
	#[serde(rename = "SD")]
	Sudan,
	/// SURINAME
	#[serde(rename = "SR")]
	Suriname,
	/// SVALBARD AND JAN MAYEN
	#[serde(rename = "SJ")]
	SvalbardAndJanMayen,
	/// SWAZILAND
	#[serde(rename = "SZ")]
	Swaziland,
	/// SWEDEN
	#[serde(rename = "SE")]
	Sweden,
	/// SWITZERLAND
	#[serde(rename = "CH")]
	Switzerland,
	/// SYRIAN ARAB REPUBLIC
	#[serde(rename = "SY")]
	SyrianArabRepublic,
	/// TAIWAN, PROVINCE OF CHINA
	#[serde(rename = "TW")]
	TaiwanProvinceOfChina,
	/// TAJIKISTAN
	#[serde(rename = "TJ")]
	Tajikistan,
	/// TANZANIA, UNITED REPUBLIC OF
	#[serde(rename = "TZ")]
	TanzaniaUnitedRepublicOf,
	/// THAILAND
	#[serde(rename = "TH")]
	Thailand,
	/// TIMOR-LESTE
	#[serde(rename = "TL")]
	TimorLeste,
	/// TOGO
	#[serde(rename = "TG")]
	Togo,
	/// TOKELAU
	#[serde(rename = "TK")]
	Tokelau,
	/// TONGA
	#[serde(rename = "TO")]
	Tonga,
	/// TRINIDAD AND TOBAGO
	#[serde(rename = "TT")]
	TrinidadAndTobago,
	/// TUNISIA
	#[serde(rename = "TN")]
	Tunisia,
	/// TURKEY
	#[serde(rename = "TR")]
	Turkey,
	/// TURKMENISTAN
	#[serde(rename = "TM")]
	Turkmenistan,
	/// TURKS AND CAICOS ISLANDS
	#[serde(rename = "TC")]
	TurksAndCaicosIslands,
	/// TUVALU
	#[serde(rename = "TV")]
	Tuvalu,
	/// UGANDA
	#[serde(rename = "UG")]
	Uganda,
	/// UKRAINE
	#[serde(rename = "UA")]
	Ukraine,
	/// UNITED ARAB EMIRATES
	#[serde(rename = "AE")]
	UnitedArabEmirates,
	/// UNITED KINGDOM
	#[serde(rename = "GB")]
	UnitedKingdom,
	/// UNITED STATES
	#[serde(rename = "US")]
	UnitedStates,
	/// UNITED STATES MINOR OUTLYING ISLANDS
	#[serde(rename = "UM")]
	UnitedStatesMinorOutlyingIslands,
	/// URUGUAY
	#[serde(rename = "UY")]
	Uruguay,
	/// UZBEKISTAN
	#[serde(rename = "UZ")]
	Uzbekistan,
	/// VANUATU
	#[serde(rename = "VU")]
	Vanuatu,
	/// VENEZUELA
	#[serde(rename = "VE")]
	Venezuela,
	/// VIET NAM
	#[serde(rename = "VN")]
	VietNam,
	/// VIRGIN ISLANDS, BRITISH
	#[serde(rename = "VG")]
	VirginIslandsBritish,
	/// VIRGIN ISLANDS, U.S.
	#[serde(rename = "VI")]
	VirginIslandsUS,
	/// WALLIS AND FUTUNA
	#[serde(rename = "WF")]
	WallisAndFutuna,
	/// WESTERN SAHARA
	#[serde(rename = "EH")]
	WesternSahara,
	/// YEMEN
	#[serde(rename = "YE")]
	Yemen,
	/// YUGOSLAVIA
	#[serde(rename = "YU")]
	Yugoslavia,
	/// ZAMBIA
	#[serde(rename = "ZM")]
	Zambia,
	/// ZIMBABWE
	#[serde(rename = "ZW")]
	Zimbabwe,
}

impl Default for Country {
	fn default() -> Self {
		Country::Afghanistan
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Side {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for IOI and List Order messages only)
	#[serde(rename = "7")]
	Undisclosed,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
	/// Cross short exempt
	#[serde(rename = "A")]
	CrossShortExempt,
	/// "As Defined" (for use with multileg instruments)
	#[serde(rename = "B")]
	AsDefined,
	/// "Opposite" (for use with multileg instruments)
	#[serde(rename = "C")]
	Opposite,
	/// Subscribe (e.g. CIV)
	#[serde(rename = "D")]
	Subscribe,
	/// Redeem (e.g. CIV)
	#[serde(rename = "E")]
	Redeem,
	/// Lend (FINANCING - identifies direction of collateral)
	#[serde(rename = "F")]
	Lend,
	/// Borrow (FINANCING - identifies direction of collateral)
	#[serde(rename = "G")]
	Borrow,
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceType {
	/// Percentage (e.g. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Per unit (i.e. per share or contract)
	#[serde(rename = "2")]
	PerUnit,
	/// Fixed Amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
	/// Discount - percentage points below par
	#[serde(rename = "4")]
	DiscountPercentagePointsBelowPar,
	/// Premium - percentage points over par
	#[serde(rename = "5")]
	PremiumPercentagePointsOverPar,
	/// Spread
	#[serde(rename = "6")]
	Spread,
	/// TED price
	#[serde(rename = "7")]
	TedPrice,
	/// TED yield
	#[serde(rename = "8")]
	TedYield,
	/// Yield
	#[serde(rename = "9")]
	Yield,
	/// Fixed cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "10")]
	FixedCabinetTradePrice,
	/// Variable cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "11")]
	VariableCabinetTradePrice,
	/// Price spread
	#[serde(rename = "12")]
	PriceSpread,
	/// Product ticks in halves
	#[serde(rename = "13")]
	ProductTicksInHalves,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eighths
	#[serde(rename = "15")]
	ProductTicksInEighths,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-fourths
	#[serde(rename = "18")]
	ProductTicksInSixtyFourths,
	/// Product ticks in one-twenty-eighths
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEighths,
	/// Normal rate representation (e.g. FX rate) (represents the number of units of currency 2 that is required to purchase one unit
	/// of currency 1.)
	#[serde(rename = "20")]
	NormalRateRepresentation,
	/// Inverse rate representation (e.g. FX rate) (represents the number of units of 1 that is needed to purchase one unit of currency
	/// 2.)
	#[serde(rename = "21")]
	InverseRateRepresentation,
	/// Basis points (when the price is not spread based)
	#[serde(rename = "22")]
	BasisPoints,
	/// Upfront points (used specifically for CDS pricing)
	#[serde(rename = "23")]
	UpfrontPoints,
	/// Interest rate (Elaboration: When the price is an interest rate. For example, used with benchmark reference rate)
	#[serde(rename = "24")]
	InterestRate,
	/// Percentage of notional
	#[serde(rename = "25")]
	PercentageOfNotional,
}

impl Default for PriceType {
	fn default() -> Self {
		PriceType::Percentage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NetGrossInd {
	/// Net
	#[serde(rename = "1")]
	Net,
	/// Gross
	#[serde(rename = "2")]
	Gross,
}

impl Default for NetGrossInd {
	fn default() -> Self {
		NetGrossInd::Net
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlType {
	/// Regular / FX Spot settlement (T+1 or T+2 depending on currency)
	#[serde(rename = "0")]
	RegularFxSpotSettlement,
	/// Cash (TOD / T+0)
	#[serde(rename = "1")]
	Cash,
	/// Next Day (TOM / T+1)
	#[serde(rename = "2")]
	NextDay,
	/// T+2
	#[serde(rename = "3")]
	T2,
	/// T+3
	#[serde(rename = "4")]
	T3,
	/// T+4
	#[serde(rename = "5")]
	T4,
	/// Future
	#[serde(rename = "6")]
	Future,
	/// When And If Issued
	#[serde(rename = "7")]
	WhenAndIfIssued,
	/// Sellers Option
	#[serde(rename = "8")]
	SellersOption,
	/// T+5
	#[serde(rename = "9")]
	T5,
	/// Broken date - for FX expressing non-standard tenor, <a href="tag_64_SettlDate.html" target="bottom">SettlDate&nbsp;(64)</a> must be specified
	#[serde(rename = "B")]
	BrokenDateForFxExpressingNonStandardTenorAHrefTag64SettlDateHtmlTargetBottomSettlDateNbspAMustBeSpecified,
	/// FX Spot Next settlement (Spot+1, aka next day)
	#[serde(rename = "C")]
	FxSpotNextSettlement,
}

impl Default for SettlType {
	fn default() -> Self {
		SettlType::RegularFxSpotSettlement
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradingSessionID {
	/// Day
	#[serde(rename = "1")]
	Day,
	/// HalfDay
	#[serde(rename = "2")]
	HalfDay,
	/// Morning
	#[serde(rename = "3")]
	Morning,
	/// Afternoon
	#[serde(rename = "4")]
	Afternoon,
	/// Evening
	#[serde(rename = "5")]
	Evening,
	/// After-hours
	#[serde(rename = "6")]
	AfterHours,
	/// Holiday
	#[serde(rename = "7")]
	Holiday,
}

impl Default for TradingSessionID {
	fn default() -> Self {
		TradingSessionID::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradingSessionSubID {
	/// Pre-Trading
	#[serde(rename = "1")]
	PreTrading,
	/// Opening or opening auction
	#[serde(rename = "2")]
	OpeningOrOpeningAuction,
	/// (Continuous) Trading
	#[serde(rename = "3")]
	Trading,
	/// Closing or closing auction
	#[serde(rename = "4")]
	ClosingOrClosingAuction,
	/// Post-Trading
	#[serde(rename = "5")]
	PostTrading,
	/// Intraday Auction
	#[serde(rename = "6")]
	IntradayAuction,
	/// Quiescent
	#[serde(rename = "7")]
	Quiescent,
	/// Any auction
	#[serde(rename = "8")]
	AnyAuction,
	/// Unscheduled intraday auction (Elaboration: An unscheduled intraday auction might be triggered by a circuit breaker)
	#[serde(rename = "9")]
	UnscheduledIntradayAuction,
	/// Out of main session trading (Elaboration: In the context of Market Model Typology "Out of main session trading" refers to
	/// both before and after session, neither auction nor continuous trading)
	#[serde(rename = "10")]
	OutOfMainSessionTrading,
	/// Private auction
	#[serde(rename = "11")]
	PrivateAuction,
	/// Public auction
	#[serde(rename = "12")]
	PublicAuction,
	/// Group auction
	#[serde(rename = "13")]
	GroupAuction,
}

impl Default for TradingSessionSubID {
	fn default() -> Self {
		TradingSessionSubID::PreTrading
	}
}
