
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityTradingRules {
	/// Number of tick rules. This block specifies the rules for determining how a security ticks, i.e. the price increments at which
	/// it can be quoted and traded, depending on the current price of the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1205")]
	pub tick_rules: Option<fix_common::RepeatingValues<TickRule>>,
	/// Number of Lot Types.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1234")]
	pub lot_type_rules: Option<fix_common::RepeatingValues<LotTypeRule>>,
	/// Describes the how the price limits are expressed
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1306")]
	pub price_limit_type: Option<PriceLimitType>,
	/// Allowable low limit price for the trading day. A key parameter in validating order price. Used as the lower band for validating
	/// order prices. Orders submitted with prices below the lower limit will be rejected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1148")]
	pub low_limit_price: Option<f64>,
	/// Allowable high limit price for the trading day. A key parameter in validating order price. Used as the upper band for validating
	/// order prices. Orders submitted with prices above the upper limit will be rejected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1149")]
	pub high_limit_price: Option<f64>,
	/// Reference price for the current trading price range usually representing the mid price between the HighLimitPrice and LowLimitPrice.
	/// The value may be the settlement price or closing price of the prior trading day.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1150")]
	pub trading_reference_price: Option<f64>,
	/// ExpirationCycle
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "827")]
	pub expiration_cycle: Option<ExpirationCycle>,
	/// The minimum order quantity that can be submitted for an order.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "562")]
	pub min_trade_vol: Option<f64>,
	/// The maximum order quantity that can be submitted for a security. For listed derivatives this indicates the minimum quantity
	/// necessary for an order or trade to qualify as a block trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1140")]
	pub max_trade_vol: Option<f64>,
	/// The maximum price variation of an execution from one event to the next for a given security. Expressed in absolute price terms.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1143")]
	pub max_price_variation: Option<f64>,
	/// ImpliedMarketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1144")]
	pub implied_market_indicator: Option<ImpliedMarketIndicator>,
	/// Used when the trading currency can differ from the price currency.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1245")]
	pub trading_currency: Option<TradingCurrency>,
	/// Trading lot size of security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "561")]
	pub round_lot: Option<f64>,
	/// Used for multileg security only. Defines whether the security is pre-defined or user-defined. Not that value = 2 (User-defined,
	/// Non-Securitized, Multileg) does not apply for Securities.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1377")]
	pub multileg_model: Option<MultilegModel>,
	/// Used for multileg security only. Defines the method used when applying the multileg price to the legs.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1378")]
	pub multileg_price_method: Option<MultilegPriceMethod>,
	/// Defines the default Price Type used for trading.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// Allows trading rules to be expressed by trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1309")]
	pub trading_session_rules: Option<fix_common::RepeatingValues<TradingSessionRule>>,
	/// NoNestedInstrAttrib
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1312")]
	pub nested_instr_attrib: Option<fix_common::RepeatingValues<NestedInstrAttri>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TickRule {
	/// Starting price range for specified tick increment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1206")]
	pub start_tick_price_range: Option<f64>,
	/// Ending price range for the specified tick increment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1207")]
	pub end_tick_price_range: Option<f64>,
	/// Tick increment for stated price range. Specifies the valid price increments at which a security can be quoted and traded
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1208")]
	pub tick_increment: Option<f64>,
	/// Specifies the type of tick rule which is being described
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1209")]
	pub tick_rule_type: Option<TickRuleType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LotTypeRule {
	/// Defines the lot type assigned to the order. Use as an alternate to <a href="tag_561_RoundLot.html" target="bottom">RoundLot&nbsp;(561)</a> . To be used with <a href="tag_1231_MinLotSize.html" target="bottom">MinLotSize&nbsp;(1231)</a> . LotType + MinLotSize ( max is next level minus 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1093")]
	pub lot_type: Option<LotType>,
	/// Minimum lot size allowed based on lot type specified in <a href="tag_1093_LotType.html" target="bottom">LotType&nbsp;(1093)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1231")]
	pub min_lot_size: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSessionRule {
	/// Identifier for the trading session. Must be provided if NoTradingSessions &gt; 0. Set to [N/A] if values are not specific to
	/// trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// Identifier for the trading session. Set to [N/A] if values are not specific to trading session sub id.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Number of order types.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1237")]
	pub no_ord_type_rules: Option<usize>,
	/// Indicates order types that are valid for the specified market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// Number of time in force techniques.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1239")]
	pub no_time_in_force_rules: Option<usize>,
	/// Indicates time in force techniques that are valid for the specified market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// Number of execution instructions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1232")]
	pub no_exec_inst_rules: Option<usize>,
	/// Indicates execution instructions that are valid for the specified market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1308")]
	pub exec_inst_value: Option<ExecInstValue>,
	/// Number of match rules.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1235")]
	pub no_match_rules: Option<usize>,
	/// The type of algorithm used to match orders in a specific security on an electronic trading platform. Possible values are FIFO,
	/// Allocation, Pro-rata, Lead Market Maker, Currency Calendar.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1142")]
	pub match_algorithm: Option<String>,
	/// The point in the matching process at which this trade was matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// The number of feed types and corresponding book depths associated with a security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1141")]
	pub no_md_feed_types: Option<usize>,
	/// Describes a class of service for a given data feed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1022")]
	pub md_feed_type: Option<String>,
	/// The depth of book associated with a particular feed type.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "264")]
	pub market_depth: Option<u32>,
	/// Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1021")]
	pub md_book_type: Option<MDBookType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NestedInstrAttri {
	/// Code to represent the type of instrument attribute.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1210")]
	pub nested_instr_attrib_type: Option<NestedInstrAttribType>,
	/// Attribute value appropriate to the NestedInstrAttribType field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1211")]
	pub nested_instr_attrib_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceLimitType {
	/// Price
	#[serde(rename = "0")]
	Price,
	/// Ticks
	#[serde(rename = "1")]
	Ticks,
	/// Percentage
	#[serde(rename = "2")]
	Percentage,
}

impl Default for PriceLimitType {
	fn default() -> Self {
		PriceLimitType::Price
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExpirationCycle {
	/// Expire on trading session close (default)
	#[serde(rename = "0")]
	ExpireOnTradingSessionClose,
	/// Expire on trading session open
	#[serde(rename = "1")]
	ExpireOnTradingSessionOpen,
	/// Expires at specified expiration as the eligibility
	#[serde(rename = "2")]
	ExpiresAtSpecifiedExpirationAsTheEligibility,
}

impl Default for ExpirationCycle {
	fn default() -> Self {
		ExpirationCycle::ExpireOnTradingSessionClose
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ImpliedMarketIndicator {
	/// Not implied
	#[serde(rename = "0")]
	NotImplied,
	/// Implied-in - The existence of a multi-leg instrument is implied by the legs of that instrument
	#[serde(rename = "1")]
	ImpliedInTheExistenceOfAMultiLegInstrumentIsImpliedByTheLegsOfThatInstrument,
	/// Implied-out - The existence of the underlying legs are implied by the multi-leg instrument
	#[serde(rename = "2")]
	ImpliedOutTheExistenceOfTheUnderlyingLegsAreImpliedByTheMultiLegInstrument,
	/// Both Implied-in and Implied-out
	#[serde(rename = "3")]
	BothImpliedInAndImpliedOut,
}

impl Default for ImpliedMarketIndicator {
	fn default() -> Self {
		ImpliedMarketIndicator::NotImplied
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradingCurrency {
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

impl Default for TradingCurrency {
	fn default() -> Self {
		TradingCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MultilegModel {
	/// Predefined Multileg Security
	#[serde(rename = "0")]
	PredefinedMultilegSecurity,
	/// User-defined Multleg Security
	#[serde(rename = "1")]
	UserDefinedMultlegSecurity,
	/// User-defined, Non-Securitized, Multileg
	#[serde(rename = "2")]
	UserDefinedNonSecuritizedMultileg,
}

impl Default for MultilegModel {
	fn default() -> Self {
		MultilegModel::PredefinedMultilegSecurity
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MultilegPriceMethod {
	/// Net Price
	#[serde(rename = "0")]
	NetPrice,
	/// Reversed Net Price
	#[serde(rename = "1")]
	ReversedNetPrice,
	/// Yield Difference
	#[serde(rename = "2")]
	YieldDifference,
	/// Individual
	#[serde(rename = "3")]
	Individual,
	/// Contract Weighted Average Price
	#[serde(rename = "4")]
	ContractWeightedAveragePrice,
	/// Multiplied Price
	#[serde(rename = "5")]
	MultipliedPrice,
}

impl Default for MultilegPriceMethod {
	fn default() -> Self {
		MultilegPriceMethod::NetPrice
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
	/// Product ticks in halfs
	#[serde(rename = "13")]
	ProductTicksInHalfs,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eights
	#[serde(rename = "15")]
	ProductTicksInEights,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-forths
	#[serde(rename = "18")]
	ProductTicksInSixtyForths,
	/// Product ticks in one-twenty-eights
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEights,
}

impl Default for PriceType {
	fn default() -> Self {
		PriceType::Percentage
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TickRuleType {
	/// Regular
	#[serde(rename = "0")]
	Regular,
	/// Variable
	#[serde(rename = "1")]
	Variable,
	/// Fixed
	#[serde(rename = "2")]
	Fixed,
	/// Traded as a spread leg
	#[serde(rename = "3")]
	TradedAsASpreadLeg,
	/// Settled as a spread leg
	#[serde(rename = "4")]
	SettledAsASpreadLeg,
}

impl Default for TickRuleType {
	fn default() -> Self {
		TickRuleType::Regular
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LotType {
	/// Odd Lot
	#[serde(rename = "1")]
	OddLot,
	/// Round Lot
	#[serde(rename = "2")]
	RoundLot,
	/// Block Lot
	#[serde(rename = "3")]
	BlockLot,
}

impl Default for LotType {
	fn default() -> Self {
		LotType::OddLot
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
}

impl Default for TradingSessionSubID {
	fn default() -> Self {
		TradingSessionSubID::PreTrading
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdType {
	/// Market
	#[serde(rename = "1")]
	Market,
	/// Limit
	#[serde(rename = "2")]
	Limit,
	/// Stop / Stop Loss
	#[serde(rename = "3")]
	StopStopLoss,
	/// Stop Limit
	#[serde(rename = "4")]
	StopLimit,
	/// Market On Close (No longer used)
	#[serde(rename = "5")]
	MarketOnClose,
	/// With Or Without
	#[serde(rename = "6")]
	WithOrWithout,
	/// Limit Or Better
	#[serde(rename = "7")]
	LimitOrBetter,
	/// Limit With Or Without
	#[serde(rename = "8")]
	LimitWithOrWithout,
	/// On Basis
	#[serde(rename = "9")]
	OnBasis,
	/// On Close (No longer used)
	#[serde(rename = "A")]
	OnClose,
	/// Limit On Close (No longer used)
	#[serde(rename = "B")]
	LimitOnClose,
	/// Forex Market (No longer used)
	#[serde(rename = "C")]
	ForexMarket,
	/// Previously Quoted
	#[serde(rename = "D")]
	PreviouslyQuoted,
	/// Previously Indicated
	#[serde(rename = "E")]
	PreviouslyIndicated,
	/// Forex Limit (No longer used)
	#[serde(rename = "F")]
	ForexLimit,
	/// Forex Swap
	#[serde(rename = "G")]
	ForexSwap,
	/// Forex Previously Quoted (No longer used)
	#[serde(rename = "H")]
	ForexPreviouslyQuoted,
	/// Funari (Limit day order with unexecuted portion handles as Market On Close. E.g. Japan)
	#[serde(rename = "I")]
	Funari,
	/// Market If Touched (MIT)
	#[serde(rename = "J")]
	MarketIfTouched,
	/// Market With Left Over as Limit (market order with unexecuted quantity becoming limit order at last price)
	#[serde(rename = "K")]
	MarketWithLeftOverAsLimit,
	/// Previous Fund Valuation Point (Historic pricing; for CIV)"
	#[serde(rename = "L")]
	PreviousFundValuationPoint,
	/// Next Fund Valuation Point (Forward pricing; for CIV)"
	#[serde(rename = "M")]
	NextFundValuationPoint,
	/// Pegged
	#[serde(rename = "P")]
	Pegged,
	/// Counter-order selection
	#[serde(rename = "Q")]
	CounterOrderSelection,
}

impl Default for OrdType {
	fn default() -> Self {
		OrdType::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimeInForce {
	/// Day (or session)
	#[serde(rename = "0")]
	Day,
	/// Good Till Cancel (GTC)
	#[serde(rename = "1")]
	GoodTillCancel,
	/// At the Opening (OPG)
	#[serde(rename = "2")]
	AtTheOpening,
	/// Immediate Or Cancel (IOC)
	#[serde(rename = "3")]
	ImmediateOrCancel,
	/// Fill Or Kill (FOK)
	#[serde(rename = "4")]
	FillOrKill,
	/// Good Till Crossing (GTX)
	#[serde(rename = "5")]
	GoodTillCrossing,
	/// Good Till Date (GTD)
	#[serde(rename = "6")]
	GoodTillDate,
	/// At the Close
	#[serde(rename = "7")]
	AtTheClose,
	/// Good Through Crossing
	#[serde(rename = "8")]
	GoodThroughCrossing,
	/// At Crossing
	#[serde(rename = "9")]
	AtCrossing,
}

impl Default for TimeInForce {
	fn default() -> Self {
		TimeInForce::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecInstValue {
	/// Stay on offerside
	#[serde(rename = "0")]
	StayOnOfferside,
	/// Not held
	#[serde(rename = "1")]
	NotHeld,
	/// Work
	#[serde(rename = "2")]
	Work,
	/// Go along
	#[serde(rename = "3")]
	GoAlong,
	/// Over the day
	#[serde(rename = "4")]
	OverTheDay,
	/// Held
	#[serde(rename = "5")]
	Held,
	/// Participate don't initiate
	#[serde(rename = "6")]
	ParticipateDonTInitiate,
	/// Strict scale
	#[serde(rename = "7")]
	StrictScale,
	/// Try to scale
	#[serde(rename = "8")]
	TryToScale,
	/// Stay on bidside
	#[serde(rename = "9")]
	StayOnBidside,
	/// No cross (cross is forbidden)
	#[serde(rename = "A")]
	NoCross,
	/// OK to cross
	#[serde(rename = "B")]
	OkToCross,
	/// Call first
	#[serde(rename = "C")]
	CallFirst,
	/// Percent of volume (indicates that the sender does not want to be all of the volume on the floor vs. a specific percentage)
	#[serde(rename = "D")]
	PercentOfVolume,
	/// Do not increase - DNI
	#[serde(rename = "E")]
	DoNotIncreaseDni,
	/// Do not reduce - DNR
	#[serde(rename = "F")]
	DoNotReduceDnr,
	/// All or none - AON
	#[serde(rename = "G")]
	AllOrNoneAon,
	/// Reinstate on System Failure (mutually exclusive with Q)
	#[serde(rename = "H")]
	ReinstateOnSystemFailure,
	/// Institutions only
	#[serde(rename = "I")]
	InstitutionsOnly,
	/// Reinstate on Trading Halt (mutually exclusive with K)
	#[serde(rename = "J")]
	ReinstateOnTradingHalt,
	/// Cancel on Trading Halt (mutually exclusive with J)
	#[serde(rename = "K")]
	CancelOnTradingHalt,
	/// Last peg (last sale)
	#[serde(rename = "L")]
	LastPeg,
	/// Mid-price peg (midprice of inside quote)
	#[serde(rename = "M")]
	MidPricePeg,
	/// Non-negotiable
	#[serde(rename = "N")]
	NonNegotiable,
	/// Opening peg
	#[serde(rename = "O")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "P")]
	MarketPeg,
	/// Cancel on System Failure (mutually exclusive with H)
	#[serde(rename = "Q")]
	CancelOnSystemFailure,
	/// Primary peg (primary market - buy at bid/sell at offer)
	#[serde(rename = "R")]
	PrimaryPeg,
	/// Suspend
	#[serde(rename = "S")]
	Suspend,
	/// Fixed Peg to Local best bid or offer at time of order
	#[serde(rename = "T")]
	FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
	/// Customer Display Instruction (Rule11Ac1-1/4)
	#[serde(rename = "U")]
	CustomerDisplayInstruction,
	/// Netting (for Forex)
	#[serde(rename = "V")]
	Netting,
	/// Peg to VWAP
	#[serde(rename = "W")]
	PegToVwap,
	/// Trade Along
	#[serde(rename = "X")]
	TradeAlong,
	/// Try to Stop
	#[serde(rename = "Y")]
	TryToStop,
	/// Cancel if Not Best
	#[serde(rename = "Z")]
	CancelIfNotBest,
	/// Trailing Stop Peg
	#[serde(rename = "a")]
	TrailingStopPeg,
	/// Strict Limit (No Price Improvement)
	#[serde(rename = "b")]
	StrictLimit,
	/// Ignore Price Validity Checks
	#[serde(rename = "c")]
	IgnorePriceValidityChecks,
	/// Peg to Limit Price
	#[serde(rename = "d")]
	PegToLimitPrice,
	/// Work to Target Strategy
	#[serde(rename = "e")]
	WorkToTargetStrategy,
	/// Intermarket Sweep
	#[serde(rename = "f")]
	IntermarketSweep,
	/// External Routing Allowed
	#[serde(rename = "g")]
	ExternalRoutingAllowed,
	/// External Routing Not Allowed
	#[serde(rename = "h")]
	ExternalRoutingNotAllowed,
	/// Imbalance Only
	#[serde(rename = "i")]
	ImbalanceOnly,
	/// Single execution requested for block trade
	#[serde(rename = "j")]
	SingleExecutionRequestedForBlockTrade,
	/// Best Execution
	#[serde(rename = "k")]
	BestExecution,
	/// Suspend on system failure (mutually exclusive with H and Q)
	#[serde(rename = "l")]
	SuspendOnSystemFailure,
	/// Suspend on Trading Halt (mutually exclusive with J and K)
	#[serde(rename = "m")]
	SuspendOnTradingHalt,
	/// Reinstate on connection loss (mutually exclusive with o and p)
	#[serde(rename = "n")]
	ReinstateOnConnectionLoss,
	/// Cancel on connection loss (mutually exclusive with n and p)
	#[serde(rename = "o")]
	CancelOnConnectionLoss,
	/// Suspend on connection loss (mutually exclusive with n and o)
	#[serde(rename = "p")]
	SuspendOnConnectionLoss,
	/// Release from suspension (mutually exclusive with S)
	#[serde(rename = "q")]
	ReleaseFromSuspension,
	/// Execute as delta neutral using volatility provided
	#[serde(rename = "r")]
	ExecuteAsDeltaNeutralUsingVolatilityProvided,
	/// Execute as duration neutral
	#[serde(rename = "s")]
	ExecuteAsDurationNeutral,
	/// Execute as FX neutral
	#[serde(rename = "t")]
	ExecuteAsFxNeutral,
}

impl Default for ExecInstValue {
	fn default() -> Self {
		ExecInstValue::StayOnOfferside
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MatchType {
	/// ACT Accepted Trade
	#[serde(rename = "M3")]
	ActAcceptedTrade,
	/// ACT Default Trade
	#[serde(rename = "M4")]
	ActDefaultTrade,
	/// ACT Default After M2
	#[serde(rename = "M5")]
	ActDefaultAfterM2,
	/// ACT M6 Match
	#[serde(rename = "M6")]
	ActM6Match,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges
	#[serde(rename = "A2")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A3")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges
	#[serde(rename = "A4")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus execution time (within
	/// two-minute window)
	#[serde(rename = "A5")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
	/// Compared records resulting from stamped advisories or specialist accepts/pair-offs
	#[serde(rename = "AQ")]
	ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
	/// Summarized Match using A1 exact match criteria except quantity is summarized
	#[serde(rename = "S1")]
	SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A2 exact match criteria except quantity is summarized
	#[serde(rename = "S2")]
	SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A3 exact match criteria except quantity is summarized
	#[serde(rename = "S3")]
	SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A4 exact match criteria except quantity is summarized
	#[serde(rename = "S4")]
	SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A5 exact match criteria except quantity is summarized
	#[serde(rename = "S5")]
	SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Exact Match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator minus badges and times
	#[serde(rename = "M1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimes,
	/// Summarized Match minus badges and times
	#[serde(rename = "M2")]
	SummarizedMatchMinusBadgesAndTimes,
	/// OCS Locked In
	#[serde(rename = "MT")]
	OcsLockedIn,
	/// One-Party Trade Report (privately negotiated trade)
	#[serde(rename = "1")]
	OnePartyTradeReport,
	/// Two-Party Trade Report (privately negotiated trade)
	#[serde(rename = "2")]
	TwoPartyTradeReport,
	/// Confirmed Trade Report (reporting from recognized markets)
	#[serde(rename = "3")]
	ConfirmedTradeReport,
	/// Auto-match
	#[serde(rename = "4")]
	AutoMatch,
	/// Cross Auction
	#[serde(rename = "5")]
	CrossAuction,
	/// Counter-Order Selection
	#[serde(rename = "6")]
	CounterOrderSelection,
	/// Call Auction
	#[serde(rename = "7")]
	CallAuction,
	/// Issuing/Buy Back Auction
	#[serde(rename = "8")]
	IssuingBuyBackAuction,
}

impl Default for MatchType {
	fn default() -> Self {
		MatchType::ActAcceptedTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDBookType {
	/// Top of Book
	#[serde(rename = "1")]
	TopOfBook,
	/// Price Depth
	#[serde(rename = "2")]
	PriceDepth,
	/// Order Depth
	#[serde(rename = "3")]
	OrderDepth,
}

impl Default for MDBookType {
	fn default() -> Self {
		MDBookType::TopOfBook
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NestedInstrAttribType {
	/// Flat (securities pay interest on a current basis but are traded without interest)
	#[serde(rename = "1")]
	Flat,
	/// Zero coupon
	#[serde(rename = "2")]
	ZeroCoupon,
	/// Interest bearing (for Euro commercial paper when not issued at discount)
	#[serde(rename = "3")]
	InterestBearing,
	/// No periodic payments
	#[serde(rename = "4")]
	NoPeriodicPayments,
	/// Variable rate
	#[serde(rename = "5")]
	VariableRate,
	/// Less fee for put
	#[serde(rename = "6")]
	LessFeeForPut,
	/// Stepped coupon
	#[serde(rename = "7")]
	SteppedCoupon,
	/// Coupon period (if not semi-annual). Supply redemption date in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "8")]
	CouponPeriodSupplyRedemptionDateInTheInstrAttribValueField,
	/// When [and if] issued
	#[serde(rename = "9")]
	WhenAndIfIssued,
	/// Original issue discount
	#[serde(rename = "10")]
	OriginalIssueDiscount,
	/// Callable, puttable
	#[serde(rename = "11")]
	CallablePuttable,
	/// Escrowed to Maturity
	#[serde(rename = "12")]
	EscrowedToMaturity,
	/// Escrowed to redemption date - callable. Supply redemption date in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "13")]
	EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrAttribValueField,
	/// Prerefunded
	#[serde(rename = "14")]
	Prerefunded,
	/// In default
	#[serde(rename = "15")]
	InDefault,
	/// Unrated
	#[serde(rename = "16")]
	Unrated,
	/// Taxable
	#[serde(rename = "17")]
	Taxable,
	/// Indexed
	#[serde(rename = "18")]
	Indexed,
	/// Subject to Alternative Minimum Tax
	#[serde(rename = "19")]
	SubjectToAlternativeMinimumTax,
	/// Original issue discount price. Supply price in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "20")]
	OriginalIssueDiscountPriceSupplyPriceInTheInstrAttribValueField,
	/// Callable below maturity value
	#[serde(rename = "21")]
	CallableBelowMaturityValue,
	/// Callable without notice by mail to holder unless registered
	#[serde(rename = "22")]
	CallableWithoutNoticeByMailToHolderUnlessRegistered,
	/// Price tick rules for security.
	#[serde(rename = "23")]
	PriceTickRulesForSecurity,
	/// Trade type eligibility details for security.
	#[serde(rename = "24")]
	TradeTypeEligibilityDetailsForSecurity,
	/// Instrument Denominator
	#[serde(rename = "25")]
	InstrumentDenominator,
	/// Instrument Numerator
	#[serde(rename = "26")]
	InstrumentNumerator,
	/// Instrument Price Precision
	#[serde(rename = "27")]
	InstrumentPricePrecision,
	/// Instrument Strike Price
	#[serde(rename = "28")]
	InstrumentStrikePrice,
	/// Tradeable Indicator
	#[serde(rename = "29")]
	TradeableIndicator,
	/// Text. Supply the text of the attribute or disclaimer in the <a href="tag_872_InstrAttribValue.html" target="bottom">InstrAttribValue&nbsp;(872)</a> field
	#[serde(rename = "99")]
	TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrAttribValueField,
}

impl Default for NestedInstrAttribType {
	fn default() -> Self {
		NestedInstrAttribType::Flat
	}
}
