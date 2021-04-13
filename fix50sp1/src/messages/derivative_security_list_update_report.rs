
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Derivative {
	/// MsgType = BR
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// SecurityReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "320")]
	pub security_req_id: Option<String>,
	/// Identifier for the Derivative Security List message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "322")]
	pub security_response_id: Option<String>,
	/// Result of the Security Request identified by SecurityReqID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "560")]
	pub security_request_result: Option<SecurityRequestResult>,
	/// Updates can be applied to Underlying or option class. If Series information provided, then Series has explicitly changed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "980")]
	pub security_update_action: Option<SecurityUpdateAction>,
	/// Underlying security for which derivatives are being returned.
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Common, "human understood" representation of the security. SecurityID value can be specified if no symbol exists (e.g. non-exchange
	/// traded Collective Investment Vehicles). Use "[N/A]" for products which do not have a symbol.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1214")]
	pub derivative_symbol: Option<String>,
	/// Used in Fixed Income with a value of "WI" to indicate "When Issued" for a security to be reissued under an old CUSIP or ISIN
	/// or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1215")]
	pub derivative_symbol_sfx: Option<String>,
	/// Takes precedence in identifying security to counterparty over SecurityAltID block. Requires DerivativeSecurityIDSource if
	/// specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1216")]
	pub derivative_security_id: Option<String>,
	/// Required if DerivativeSecurityID is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1217")]
	pub derivative_security_id_source: Option<DerivativeSecurityIDSource>,
	/// NoDerivativeSecurityAltID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1218")]
	pub derivative_security_alt_id: Option<fix_common::RepeatingValues<DerivativeSecurityAltI>>,
	/// Indicates the type of product the security is associated with (high-level category).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1246")]
	pub derivative_product: Option<DerivativeProduct>,
	/// Identifies an entire suite of products for a given market. In Futures this may be "interest rates", "agricultural", "equity
	/// indexes", etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1228")]
	pub derivative_product_complex: Option<String>,
	/// Used to indicate if a product or group of product supports the creation of flexible securities.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1243")]
	pub deriv_flex_product_eligibility_indicator: Option<fix_common::Boolean>,
	/// An exchange specific name assigned to a group of related securities which may be concurrently affected by market events and
	/// actions..
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1247")]
	pub derivative_security_group: Option<String>,
	/// Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is
	/// recommended that DerivativeCFICode be used instead of DerivativeSecurityType for non-Fixed Income instruments.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1248")]
	pub derivative_cfi_code: Option<String>,
	/// It is recommended that DerivativeCFICode be used instead of SecurityType for non-Fixed Income instruments. Required for Fixed
	/// Income. Refer to Volume 7 - Fixed Income Futures and Options should be specified using the CFICode[461] field instead of DerivativeSecurityType
	/// (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1249")]
	pub derivative_security_type: Option<DerivativeSecurityType>,
	/// Sub-type qualification/identification of the SecurityType (e.g. for SecurityType=MLEG). If specified, DerivativeSecurityType
	/// is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1250")]
	pub derivative_security_sub_type: Option<String>,
	/// Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced by month
	/// and year (e.g. S and P futures). Note MaturityDate (a full date) can also be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1251")]
	pub derivative_maturity_month_year: Option<fix_common::MonthYear>,
	/// Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month
	/// and year (e.g. S and P futures). May use MaturityMonthYear and or this field. When using MaturityMonthYear, it is recommended
	/// that markets and sell sides report the MaturityDate on all outbound messages as a means of data enrichment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1252")]
	pub derivative_maturity_date: Option<fix_common::LocalMktDate>,
	/// DerivativeMaturityTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1253")]
	pub derivative_maturity_time: Option<fix_common::TZTimeOnly>,
	/// Indicator to determine if Instrument is Settle on Open.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1254")]
	pub derivative_settle_on_open_flag: Option<String>,
	/// DerivativeInstrmtAssignmentMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1255")]
	pub derivative_instrmt_assignment_method: Option<char>,
	/// Gives the current state of the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1256")]
	pub derivative_security_status: Option<DerivativeSecurityStatus>,
	/// Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1276")]
	pub derivative_issue_date: Option<fix_common::LocalMktDate>,
	/// The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded.
	/// Can be used in conjunction with ISIN to address ISIN uniqueness issues.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1257")]
	pub derivative_instr_registry: Option<String>,
	/// ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN
	/// SecurityID (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1258")]
	pub derivative_country_of_issue: Option<String>,
	/// A two-character state or province abbreviation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1259")]
	pub derivative_state_or_province_of_issue: Option<String>,
	/// The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1260")]
	pub derivative_locale_of_issue: Option<String>,
	/// Used for derivatives, such as options and covered warrants.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1261")]
	pub derivative_strike_price: Option<f64>,
	/// Used for derivatives.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1262")]
	pub derivative_strike_currency: Option<String>,
	/// Used for derivatives. Multiplier applied to the strike price for the purpose of calculating the settlement value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1263")]
	pub derivative_strike_multiplier: Option<f64>,
	/// Used for derivatives. The number of shares/units for the financial instrument involved in the option trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1264")]
	pub derivative_strike_value: Option<f64>,
	/// Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate
	/// actions to the underlying. Should not be used to indicate type of option - use the CFICode[461] for this purpose.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1265")]
	pub derivative_opt_attribute: Option<char>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g.
	/// contracts vs. shares) amount.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1266")]
	pub derivative_contract_multiplier: Option<f64>,
	/// Minimum price increment for the instrument. Could also be used to represent tick value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1267")]
	pub derivative_min_price_increment: Option<f64>,
	/// Minimum price increment amount associated with the MinPriceIncrement [969]. For listed derivatives, the value can be calculated
	/// by multiplying MinPriceIncrement by ContractValueFactor [231]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1268")]
	pub derivative_min_price_increment_amount: Option<f64>,
	/// DerivativeUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1269")]
	pub derivative_unit_of_measure: Option<DerivativeUnitOfMeasure>,
	/// DerivativeUnitOfMeasureQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1270")]
	pub derivative_unit_of_measure_qty: Option<f64>,
	/// DerivativePriceUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1315")]
	pub derivative_price_unit_of_measure: Option<DerivativePriceUnitOfMeasure>,
	/// DerivativePriceUnitOfMeasureQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1316")]
	pub derivative_price_unit_of_measure_qty: Option<f64>,
	/// Settlement method for a contract. Can be used as an alternative to CFI Code value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1317")]
	pub derivative_settl_method: Option<DerivativeSettlMethod>,
	/// Method for price quotation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1318")]
	pub derivative_price_quote_method: Option<DerivativePriceQuoteMethod>,
	/// For futures, indicates type of valuation method applied.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1319")]
	pub derivative_futures_valuation_method: Option<DerivativeFuturesValuationMethod>,
	/// Indicates whether strikes are pre-listed only or can also be defined via user request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1320")]
	pub derivative_list_method: Option<DerivativeListMethod>,
	/// Used to express the ceiling price of a capped call.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1321")]
	pub derivative_cap_price: Option<f64>,
	/// Used to express the floor price of a capped put.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1322")]
	pub derivative_floor_price: Option<f64>,
	/// DerivativePutOrCall
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1323")]
	pub derivative_put_or_call: Option<DerivativePutOrCall>,
	/// Type of exercise of a derivatives security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1299")]
	pub derivative_exercise_style: Option<DerivativeExerciseStyle>,
	/// Cash amount indicating the pay out associated with an option. For binary options this is a fixed amount.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1225")]
	pub derivative_opt_pay_amount: Option<f64>,
	/// Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1271")]
	pub derivative_time_unit: Option<DerivativeTimeUnit>,
	/// Can be used to identify the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1272")]
	pub derivative_security_exchange: Option<String>,
	/// Position Limit for the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1273")]
	pub derivative_position_limit: Option<i32>,
	/// Near-term Position Limit for the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1274")]
	pub derivative_nt_position_limit: Option<i32>,
	/// DerivativeIssuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1275")]
	pub derivative_issuer: Option<String>,
	/// Must be set if EncodedIssuer field is specified and must immediately precede it.
	#[serde(rename = "1277")]
	/// Encoded (non-ASCII characters) representation of the Issuer field in the encoded format specified via the MessageEncoding
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1278")]
	pub derivative_encoded_issuer: Option<fix_common::EncodedText<1278>>,
	/// DerivativeSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1279")]
	pub derivative_security_desc: Option<String>,
	/// Must be set if EncodedSecurityDesc field is specified and must immediately precede it.
	#[serde(rename = "1280")]
	/// Encoded (non-ASCII characters) representation of the SecurityDesc field in the encoded format specified via the MessageEncoding
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1281")]
	pub derivative_encoded_security_desc: Option<fix_common::EncodedText<1281>>,
	/// Embedded XML document describing security.
	#[serde(flatten)]
	pub derivative_security_xml: Option<super::super::derivative_security_xml::DerivativeSecurityXML>,
	/// Must be present for MBS or TBA.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1285")]
	pub derivative_contract_settl_month: Option<fix_common::MonthYear>,
	/// NoDerivativeEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1286")]
	pub derivative_events: Option<fix_common::RepeatingValues<DerivativeEvent>>,
	/// Should contain unique combinations of DerivativeInstrumentPartyID, DerivativeInstrumentPartyIDSource, and DerivativeInstrumentPartyRole
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1292")]
	pub derivative_instrument_parties: Option<fix_common::RepeatingValues<DerivativeInstrumentPartie>>,
	/// NoDerivativeInstrAttrib
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1311")]
	pub derivative_instr_attrib: Option<fix_common::RepeatingValues<DerivativeInstrAttri>>,
	/// Number of Market Segments on which a security may trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1310")]
	pub market_segments: Option<fix_common::RepeatingValues<MarketSegment>>,
	/// Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation
	/// is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "393")]
	pub tot_no_related_sym: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// NoRelatedSym
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "146")]
	pub related_sym: Option<fix_common::RepeatingValues<RelatedSy>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeSecurityAltI {
	/// DerivativeSecurityAltID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1219")]
	pub derivative_security_alt_id: Option<String>,
	/// DerivativeSecurityAltIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1220")]
	pub derivative_security_alt_id_source: Option<DerivativeSecurityAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeEvent {
	/// Indicates type of event describing security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1287")]
	pub derivative_event_type: Option<DerivativeEventType>,
	/// DerivativeEventDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1288")]
	pub derivative_event_date: Option<fix_common::LocalMktDate>,
	/// Specific time of event. To be used in combination with DerivativeEventDateEventDate [1288]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1289")]
	pub derivative_event_time: Option<fix_common::UTCTimestamp>,
	/// DerivativeEventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1290")]
	pub derivative_event_px: Option<f64>,
	/// DerivativeEventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1291")]
	pub derivative_event_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeInstrumentPartie {
	/// Used to identify party id related to instrument series.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1293")]
	pub derivative_instrument_party_id: Option<String>,
	/// Used to identify source of instrument series party id.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1294")]
	pub derivative_instrument_party_id_source: Option<DerivativeInstrumentPartyIDSource>,
	/// Used to identify the role of instrument series party id.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1295")]
	pub derivative_instrument_party_role: Option<DerivativeInstrumentPartyRole>,
	/// NoDerivativeInstrumentPartySubIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1296")]
	pub no_derivative_instrument_party_sub_i_ds: Option<usize>,
	/// DerivativeInstrumentPartySubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1297")]
	pub derivative_instrument_party_sub_id: Option<String>,
	/// DerivativeInstrumentPartySubIDType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1298")]
	pub derivative_instrument_party_sub_id_type: Option<DerivativeInstrumentPartySubIDType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeInstrAttri {
	/// DerivativeInstrAttribType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1313")]
	pub derivative_instr_attrib_type: Option<DerivativeInstrAttribType>,
	/// DerivativeInstrAttribValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1314")]
	pub derivative_instr_attrib_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketSegment {
	/// Identifies the market which lists and trades the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Identifies the segment of the market to which the specify trading rules and listing rules apply.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Number of strike rule entries. This block specifies the rules for determining how new strikes should be listed within the
	/// stated price range of the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1201")]
	pub no_strike_rules: Option<usize>,
	/// Allows strike rule to be referenced via an identifier so that rules do not need to be explicitly enumerated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1223")]
	pub strike_rule_id: Option<String>,
	/// Starting price for the range to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1202")]
	pub start_strike_px_range: Option<f64>,
	/// Ending price of the range to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1203")]
	pub end_strike_px_range: Option<f64>,
	/// Value by which strike price should be incremented within the specified price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1204")]
	pub strike_increment: Option<f64>,
	/// Enumeration that represents the exercise style for a class of options. Same values as ExerciseStyle.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1304")]
	pub strike_exercise_style: Option<StrikeExerciseStyle>,
	/// Number of maturity rule entries. This block specifies the rules for determining how new strikes should be listed within the
	/// stated price range of the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1236")]
	pub no_maturity_rules: Option<usize>,
	/// Allows maturity rule to be referenced via an identifier so that rules do not need to be explicitly enumerated.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1222")]
	pub maturity_rule_id: Option<String>,
	/// Format used to generate the MMY for each option contract.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1303")]
	pub maturity_month_year_format: Option<MaturityMonthYearFormat>,
	/// Enumeration specifying the increment unit.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1302")]
	pub maturity_month_year_increment_units: Option<MaturityMonthYearIncrementUnits>,
	/// Starting maturity for the range to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1241")]
	pub start_maturity_month_year: Option<fix_common::MonthYear>,
	/// Ending maturity monthy year to which the StrikeIncrement applies. Price refers to the price of the underlying.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1226")]
	pub end_maturity_month_year: Option<fix_common::MonthYear>,
	/// Value by which maturity month year should be incremented within the specified price range.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1229")]
	pub maturity_month_year_increment: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
	/// If provided, then Instrument occurrence has explicitly changed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1324")]
	pub list_update_action: Option<ListUpdateAction>,
	/// CorporateAction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "292")]
	pub corporate_action: Option<fix_common::SeparatedValues<CorporateAction>>,
	/// SecondaryPriceLimitType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1305")]
	pub secondary_price_limit_type: Option<SecondaryPriceLimitType>,
	/// SecondaryLowLimitPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1221")]
	pub secondary_low_limit_price: Option<f64>,
	/// SecondaryHighLimitPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1230")]
	pub secondary_high_limit_price: Option<f64>,
	/// SecondaryTradingReferencePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1240")]
	pub secondary_trading_reference_price: Option<f64>,
	/// Currency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// NoLegs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "555")]
	pub no_legs: Option<usize>,
	/// Comment, instructions, or other identifying information.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityRequestResult {
	/// Valid request
	#[serde(rename = "0")]
	ValidRequest,
	/// Invalid or unsupported request
	#[serde(rename = "1")]
	InvalidOrUnsupportedRequest,
	/// No instruments found that match selection criteria
	#[serde(rename = "2")]
	NoInstrumentsFoundThatMatchSelectionCriteria,
	/// Not authorized to retrieve instrument data
	#[serde(rename = "3")]
	NotAuthorizedToRetrieveInstrumentData,
	/// Instrument data temporarily unavailable
	#[serde(rename = "4")]
	InstrumentDataTemporarilyUnavailable,
	/// Request for instrument data not supported
	#[serde(rename = "5")]
	RequestForInstrumentDataNotSupported,
}

impl Default for SecurityRequestResult {
	fn default() -> Self {
		SecurityRequestResult::ValidRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityUpdateAction {
	/// Add
	#[serde(rename = "A")]
	Add,
	/// Delete
	#[serde(rename = "D")]
	Delete,
	/// Modify
	#[serde(rename = "M")]
	Modify,
}

impl Default for SecurityUpdateAction {
	fn default() -> Self {
		SecurityUpdateAction::Add
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeSecurityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
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
}

impl Default for DerivativeSecurityIDSource {
	fn default() -> Self {
		DerivativeSecurityIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeProduct {
	/// AGENCY
	#[serde(rename = "1")]
	Agency,
	/// COMMODITY
	#[serde(rename = "2")]
	Commodity,
	/// CORPORATE
	#[serde(rename = "3")]
	Corporate,
	/// CURRENCY
	#[serde(rename = "4")]
	Currency,
	/// EQUITY
	#[serde(rename = "5")]
	Equity,
	/// GOVERNMENT
	#[serde(rename = "6")]
	Government,
	/// INDEX
	#[serde(rename = "7")]
	Index,
	/// LOAN
	#[serde(rename = "8")]
	Loan,
	/// MONEYMARKET
	#[serde(rename = "9")]
	Moneymarket,
	/// MORTGAGE
	#[serde(rename = "10")]
	Mortgage,
	/// MUNICIPAL
	#[serde(rename = "11")]
	Municipal,
	/// OTHER
	#[serde(rename = "12")]
	Other,
	/// FINANCING
	#[serde(rename = "13")]
	Financing,
}

impl Default for DerivativeProduct {
	fn default() -> Self {
		DerivativeProduct::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeSecurityType {
	/// Future
	#[serde(rename = "FUT")]
	Fut,
	/// Option
	#[serde(rename = "OPT")]
	Opt,
	/// US Treasury Note (Deprecated Value Use TNOTE)
	#[serde(rename = "UST")]
	Ust,
	/// US Treasury Bill (Deprecated Value Use TBILL)
	#[serde(rename = "USTB")]
	Ustb,
	/// Euro Supranational Coupons *
	#[serde(rename = "EUSUPRA")]
	Eusupra,
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	Fac,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	Fadn,
	/// Private Export Funding *
	#[serde(rename = "PEF")]
	Pef,
	/// USD Supranational Coupons *
	#[serde(rename = "SUPRA")]
	Supra,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	Corp,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	Cpp,
	/// Convertible Bond
	#[serde(rename = "CB")]
	Cb,
	/// Dual Currency
	#[serde(rename = "DUAL")]
	Dual,
	/// Euro Corporate Bond
	#[serde(rename = "EUCORP")]
	Eucorp,
	/// Indexed Linked
	#[serde(rename = "XLINKD")]
	Xlinkd,
	/// Structured Notes
	#[serde(rename = "STRUCT")]
	Struct,
	/// Yankee Corporate Bond
	#[serde(rename = "YANK")]
	Yank,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	For,
	/// Common Stock
	#[serde(rename = "CS")]
	Cs,
	/// Preferred Stock
	#[serde(rename = "PS")]
	Ps,
	/// Repurchase
	#[serde(rename = "REPO")]
	Repo,
	/// Forward
	#[serde(rename = "FORWARD")]
	Forward,
	/// Buy Sellback
	#[serde(rename = "BUYSELL")]
	Buysell,
	/// Securities Loan
	#[serde(rename = "SECLOAN")]
	Secloan,
	/// Securities Pledge
	#[serde(rename = "SECPLEDGE")]
	Secpledge,
	/// Brady Bond
	#[serde(rename = "BRADY")]
	Brady,
	/// Euro Sovereigns *
	#[serde(rename = "EUSOV")]
	Eusov,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	Tbond,
	/// Interest Strip From Any Bond Or Note
	#[serde(rename = "TINT")]
	Tint,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	Tips,
	/// Principal Strip Of A Callable Bond Or Note
	#[serde(rename = "TCAL")]
	Tcal,
	/// Principal Strip From A Non-Callable Bond Or Note
	#[serde(rename = "TPRN")]
	Tprn,
	/// US Treasury Note
	#[serde(rename = "TNOTE")]
	Tnote,
	/// US Treasury Bill
	#[serde(rename = "TBILL")]
	Tbill,
	/// Term Loan
	#[serde(rename = "TERM")]
	Term,
	/// Revolver Loan
	#[serde(rename = "RVLV")]
	Rvlv,
	/// Revolver/Term Loan
	#[serde(rename = "RVLVTRM")]
	Rvlvtrm,
	/// Bridge Loan
	#[serde(rename = "BRIDGE")]
	Bridge,
	/// Letter Of Credit
	#[serde(rename = "LOFC")]
	Lofc,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	Swing,
	/// Debtor In Possession
	#[serde(rename = "DINP")]
	Dinp,
	/// Defaulted
	#[serde(rename = "DEFLTED")]
	Deflted,
	/// Withdrawn
	#[serde(rename = "WITHDRN")]
	Withdrn,
	/// Replaced
	#[serde(rename = "REPLACD")]
	Replacd,
	/// Matured
	#[serde(rename = "MATURED")]
	Matured,
	/// Amended &amp; Restated
	#[serde(rename = "AMENDED")]
	Amended,
	/// Retired
	#[serde(rename = "RETIRED")]
	Retired,
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	Ba,
	/// Bank Notes
	#[serde(rename = "BN")]
	Bn,
	/// Bill Of Exchanges
	#[serde(rename = "BOX")]
	Box,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	Cd,
	/// Call Loans
	#[serde(rename = "CL")]
	Cl,
	/// Commercial Paper
	#[serde(rename = "CP")]
	Cp,
	/// Deposit Notes
	#[serde(rename = "DN")]
	Dn,
	/// Euro Certificate Of Deposit
	#[serde(rename = "EUCD")]
	Eucd,
	/// Euro Commercial Paper
	#[serde(rename = "EUCP")]
	Eucp,
	/// Liquidity Note
	#[serde(rename = "LQN")]
	Lqn,
	/// Medium Term Notes
	#[serde(rename = "MTN")]
	Mtn,
	/// Overnight
	#[serde(rename = "ONITE")]
	Onite,
	/// Promissory Note
	#[serde(rename = "PN")]
	Pn,
	/// Plazos Fijos
	#[serde(rename = "PZFJ")]
	Pzfj,
	/// Short Term Loan Note
	#[serde(rename = "STN")]
	Stn,
	/// Time Deposit
	#[serde(rename = "TD")]
	Td,
	/// Extended Comm Note
	#[serde(rename = "XCN")]
	Xcn,
	/// Yankee Certificate Of Deposit
	#[serde(rename = "YCD")]
	Ycd,
	/// Asset-backed Securities
	#[serde(rename = "ABS")]
	Abs,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	Cmbs,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	Cmo,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	Iet,
	/// Mortgage-backed Securities
	#[serde(rename = "MBS")]
	Mbs,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	Mio,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	Mpo,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	Mpp,
	/// Miscellaneous Pass-through
	#[serde(rename = "MPT")]
	Mpt,
	/// Pfandbriefe *
	#[serde(rename = "PFAND")]
	Pfand,
	/// To Be Announced
	#[serde(rename = "TBA")]
	Tba,
	/// Other Anticipation Notes (BAN, GAN, etc.)
	#[serde(rename = "AN")]
	An,
	/// Certificate Of Obligation
	#[serde(rename = "COFO")]
	Cofo,
	/// Certificate Of Participation
	#[serde(rename = "COFP")]
	Cofp,
	/// General Obligation Bonds
	#[serde(rename = "GO")]
	Go,
	/// Mandatory Tender
	#[serde(rename = "MT")]
	Mt,
	/// Revenue Anticipation Note
	#[serde(rename = "RAN")]
	Ran,
	/// Revenue Bonds
	#[serde(rename = "REV")]
	Rev,
	/// Special Assessment
	#[serde(rename = "SPCLA")]
	Spcla,
	/// Special Obligation
	#[serde(rename = "SPCLO")]
	Spclo,
	/// Special Tax
	#[serde(rename = "SPCLT")]
	Spclt,
	/// Tax Anticipation Note
	#[serde(rename = "TAN")]
	Tan,
	/// Tax Allocation
	#[serde(rename = "TAXA")]
	Taxa,
	/// Tax Exempt Commercial Paper
	#[serde(rename = "TECP")]
	Tecp,
	/// Tax Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	Tran,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	Vrdn,
	/// Warrant
	#[serde(rename = "WAR")]
	War,
	/// Mutual Fund
	#[serde(rename = "MF")]
	Mf,
	/// Multileg Instrument
	#[serde(rename = "MLEG")]
	Mleg,
	/// No Security Type
	#[serde(rename = "NONE")]
	None,
	/// Options on Futures
	#[serde(rename = "OOF")]
	Oof,
	/// Options on Physical
	#[serde(rename = "OOP")]
	Oop,
	/// Wildcard Entry (was "?" in 4.4, used on Security Definition Request message)
	#[serde(rename = "WLD")]
	Wld,
	/// Cash
	#[serde(rename = "CASH")]
	Cash,
	/// Interest Rate Swap
	#[serde(rename = "IRS")]
	Irs,
	/// Bank Depository Note
	#[serde(rename = "BDN")]
	Bdn,
	/// Canadian Money Markets
	#[serde(rename = "CAMM")]
	Camm,
	/// Canadian Treasury Notes
	#[serde(rename = "CAN")]
	Can,
	/// Canadian Treasury Bills
	#[serde(rename = "CTB")]
	Ctb,
	/// Credit Default Swap
	#[serde(rename = "CDS")]
	Cds,
	/// Canadian Mortgage Bonds
	#[serde(rename = "CMB")]
	Cmb,
	/// Euro Corporate Floating Rate Notes
	#[serde(rename = "EUFRN")]
	Eufrn,
	/// US Corporate Floating Rate Notes
	#[serde(rename = "FRN")]
	Frn,
	/// Canadian Provincial Bonds
	#[serde(rename = "PROV")]
	Prov,
	/// Secured Liquidity Note
	#[serde(rename = "SLQN")]
	Slqn,
	/// Treasury Bill - non US
	#[serde(rename = "TB")]
	Tb,
	/// Term Liquidity Note
	#[serde(rename = "TLQN")]
	Tlqn,
	/// Taxable Municipal CP
	#[serde(rename = "TMCP")]
	Tmcp,
}

impl Default for DerivativeSecurityType {
	fn default() -> Self {
		DerivativeSecurityType::Fut
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeSecurityStatus {
	/// Active
	#[serde(rename = "1")]
	Active,
	/// Inactive
	#[serde(rename = "2")]
	Inactive,
}

impl Default for DerivativeSecurityStatus {
	fn default() -> Self {
		DerivativeSecurityStatus::Active
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeUnitOfMeasure {
	/// Barrels
	#[serde(rename = "Bbl")]
	Barrels,
	/// Billion cubic feet
	#[serde(rename = "Bcf")]
	BillionCubicFeet,
	/// Bushels
	#[serde(rename = "Bu")]
	Bushels,
	/// pounds
	#[serde(rename = "lbs")]
	Pounds,
	/// Gallons
	#[serde(rename = "Gal")]
	Gallons,
	/// Million Barrels (deprecated in FIX 5.0 SP1)
	#[serde(rename = "MMbbl")]
	MillionBarrels,
	/// One Million BTU
	#[serde(rename = "MMBtu")]
	OneMillionBtu,
	/// Megawatt hours
	#[serde(rename = "MWh")]
	MegawattHours,
	/// Troy Ounces
	#[serde(rename = "oz_tr")]
	TroyOunces,
	/// Metric Tons (aka Tonne)
	#[serde(rename = "t")]
	MetricTons,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tons,
	/// US Dollars
	#[serde(rename = "USD")]
	UsDollars,
}

impl Default for DerivativeUnitOfMeasure {
	fn default() -> Self {
		DerivativeUnitOfMeasure::Barrels
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativePriceUnitOfMeasure {
	/// Barrels
	#[serde(rename = "Bbl")]
	Barrels,
	/// Billion cubic feet
	#[serde(rename = "Bcf")]
	BillionCubicFeet,
	/// Bushels
	#[serde(rename = "Bu")]
	Bushels,
	/// pounds
	#[serde(rename = "lbs")]
	Pounds,
	/// Gallons
	#[serde(rename = "Gal")]
	Gallons,
	/// Million Barrels (deprecated in FIX 5.0 SP1)
	#[serde(rename = "MMbbl")]
	MillionBarrels,
	/// One Million BTU
	#[serde(rename = "MMBtu")]
	OneMillionBtu,
	/// Megawatt hours
	#[serde(rename = "MWh")]
	MegawattHours,
	/// Troy Ounces
	#[serde(rename = "oz_tr")]
	TroyOunces,
	/// Metric Tons (aka Tonne)
	#[serde(rename = "t")]
	MetricTons,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tons,
	/// US Dollars
	#[serde(rename = "USD")]
	UsDollars,
}

impl Default for DerivativePriceUnitOfMeasure {
	fn default() -> Self {
		DerivativePriceUnitOfMeasure::Barrels
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeSettlMethod {
	/// Cash settlement required
	#[serde(rename = "C")]
	CashSettlementRequired,
	/// Physical settlement required
	#[serde(rename = "P")]
	PhysicalSettlementRequired,
}

impl Default for DerivativeSettlMethod {
	fn default() -> Self {
		DerivativeSettlMethod::CashSettlementRequired
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativePriceQuoteMethod {
	/// Standard, money per unit of a physical
	#[serde(rename = "STD")]
	StandardMoneyPerUnitOfAPhysical,
	/// Index
	#[serde(rename = "INX")]
	Index,
	/// Interest rate Index
	#[serde(rename = "INT")]
	InterestRateIndex,
}

impl Default for DerivativePriceQuoteMethod {
	fn default() -> Self {
		DerivativePriceQuoteMethod::StandardMoneyPerUnitOfAPhysical
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeFuturesValuationMethod {
	/// premium style
	#[serde(rename = "EQTY")]
	PremiumStyle,
	/// futures style mark-to-market
	#[serde(rename = "FUT")]
	FuturesStyleMarkToMarket,
	/// futures style with an attached cash adjustment
	#[serde(rename = "FUTDA")]
	FuturesStyleWithAnAttachedCashAdjustment,
}

impl Default for DerivativeFuturesValuationMethod {
	fn default() -> Self {
		DerivativeFuturesValuationMethod::PremiumStyle
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeListMethod {
	/// pre-listed only
	#[serde(rename = "0")]
	PreListedOnly,
	/// user requested
	#[serde(rename = "1")]
	UserRequested,
}

impl Default for DerivativeListMethod {
	fn default() -> Self {
		DerivativeListMethod::PreListedOnly
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativePutOrCall {
	/// Put
	#[serde(rename = "0")]
	Put,
	/// Call
	#[serde(rename = "1")]
	Call,
}

impl Default for DerivativePutOrCall {
	fn default() -> Self {
		DerivativePutOrCall::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeExerciseStyle {
	/// European
	#[serde(rename = "0")]
	European,
	/// American
	#[serde(rename = "1")]
	American,
	/// Bermuda
	#[serde(rename = "2")]
	Bermuda,
}

impl Default for DerivativeExerciseStyle {
	fn default() -> Self {
		DerivativeExerciseStyle::European
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeTimeUnit {
	/// Hour
	#[serde(rename = "H")]
	Hour,
	/// Minute
	#[serde(rename = "Min")]
	Minute,
	/// Second
	#[serde(rename = "S")]
	Second,
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

impl Default for DerivativeTimeUnit {
	fn default() -> Self {
		DerivativeTimeUnit::Hour
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

impl Default for LastFragment {
	fn default() -> Self {
		LastFragment::NotLastMessage
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeSecurityAltIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
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
}

impl Default for DerivativeSecurityAltIDSource {
	fn default() -> Self {
		DerivativeSecurityAltIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeEventType {
	/// Put
	#[serde(rename = "1")]
	Put,
	/// Call
	#[serde(rename = "2")]
	Call,
	/// Tender
	#[serde(rename = "3")]
	Tender,
	/// Sinking Fund Call
	#[serde(rename = "4")]
	SinkingFundCall,
	/// Activation
	#[serde(rename = "5")]
	Activation,
	/// Inactiviation
	#[serde(rename = "6")]
	Inactiviation,
	/// Last Eligible Trade Date
	#[serde(rename = "7")]
	LastEligibleTradeDate,
	/// Swap Start Date
	#[serde(rename = "8")]
	SwapStartDate,
	/// Swap End Date
	#[serde(rename = "9")]
	SwapEndDate,
	/// Swap Roll Date
	#[serde(rename = "10")]
	SwapRollDate,
	/// Swap Next Start Date
	#[serde(rename = "11")]
	SwapNextStartDate,
	/// Swap Next Roll Date
	#[serde(rename = "12")]
	SwapNextRollDate,
	/// First Delivery Date
	#[serde(rename = "13")]
	FirstDeliveryDate,
	/// Last Delivery Date
	#[serde(rename = "14")]
	LastDeliveryDate,
	/// Initial Inventory Due Date
	#[serde(rename = "15")]
	InitialInventoryDueDate,
	/// Final Inventory Due Date
	#[serde(rename = "16")]
	FinalInventoryDueDate,
	/// First Intent Date
	#[serde(rename = "17")]
	FirstIntentDate,
	/// Last Intent Date
	#[serde(rename = "18")]
	LastIntentDate,
	/// Position Removal Date
	#[serde(rename = "19")]
	PositionRemovalDate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for DerivativeEventType {
	fn default() -> Self {
		DerivativeEventType::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeInstrumentPartyIDSource {
	/// BIC (Bank Identification Code-Swift managed) code (ISO 9362)
	#[serde(rename = "B")]
	BicCode,
	/// Generally accepted market participant identifier (e.g. NASD mnemonic)
	#[serde(rename = "C")]
	GenerallyAcceptedMarketParticipantIdentifier,
	/// Proprietary/Custom code
	#[serde(rename = "D")]
	ProprietaryCustomCode,
	/// ISO Country Code
	#[serde(rename = "E")]
	IsoCountryCode,
	/// Settlement Entity Location (note if Local Market Settlement use "E = ISO Country Code")
	#[serde(rename = "F")]
	SettlementEntityLocation,
	/// MIC (ISO 10383 - Market Identifier Code) (See " <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="../appendices/appendix_6-c.html" target="_blank">Appendix 6-C</a> " of FIX Specification)
	#[serde(rename = "G")]
	Mic,
	/// CSD participant/member code (e.g. Euroclear, DTC, CREST or Kassenverein number)
	#[serde(rename = "H")]
	CsdParticipantMemberCode,
	/// Korean Investor ID
	#[serde(rename = "1")]
	KoreanInvestorId,
	/// Taiwanese Qualified Foreign Investor ID QFII / FID
	#[serde(rename = "2")]
	TaiwaneseQualifiedForeignInvestorIdQfiiFid,
	/// Taiwanese Trading Account
	#[serde(rename = "3")]
	TaiwaneseTradingAccount,
	/// Malaysian Central Depository (MCD) number
	#[serde(rename = "4")]
	MalaysianCentralDepositoryNumber,
	/// Chinese B Share (Shezhen and Shanghai)
	#[serde(rename = "5")]
	ChineseBShare,
	/// UK National Insurance or Pension Number
	#[serde(rename = "6")]
	UkNationalInsuranceOrPensionNumber,
	/// US Social Security Number
	#[serde(rename = "7")]
	UsSocialSecurityNumber,
	/// US Employer Identification Number
	#[serde(rename = "8")]
	UsEmployerIdentificationNumber,
	/// Australian Business Number
	#[serde(rename = "9")]
	AustralianBusinessNumber,
	/// Australian Tax File Number
	#[serde(rename = "A")]
	AustralianTaxFileNumber,
	/// Directed broker three character acronym as defined in ISITC 'ETC Best Practice' guidelines document
	#[serde(rename = "I")]
	DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument,
}

impl Default for DerivativeInstrumentPartyIDSource {
	fn default() -> Self {
		DerivativeInstrumentPartyIDSource::BicCode
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeInstrumentPartyRole {
	/// Executing Firm (formerly FIX 4.2 ExecBroker)
	#[serde(rename = "1")]
	ExecutingFirm,
	/// Broker of Credit (formerly FIX 4.2 BrokerOfCredit)
	#[serde(rename = "2")]
	BrokerOfCredit,
	/// Client ID (formerly FIX 4.2 ClientID)
	#[serde(rename = "3")]
	ClientId,
	/// Clearing Firm (formerly FIX 4.2 ClearingFirm)
	#[serde(rename = "4")]
	ClearingFirm,
	/// Investor ID
	#[serde(rename = "5")]
	InvestorId,
	/// Introducing Firm
	#[serde(rename = "6")]
	IntroducingFirm,
	/// Entering Firm
	#[serde(rename = "7")]
	EnteringFirm,
	/// Locate/Lending Firm (for short-sales)
	#[serde(rename = "8")]
	LocateLendingFirm,
	/// Fund manager Client ID (for CIV)
	#[serde(rename = "9")]
	FundManagerClientId,
	/// Settlement Location (formerly FIX 4.2 SettlLocation)
	#[serde(rename = "10")]
	SettlementLocation,
	/// Order Origination Trader (associated with Order Origination Firm - e.g. trader who initiates/submits the order)
	#[serde(rename = "11")]
	OrderOriginationTrader,
	/// Executing Trader (associated with Executing Firm - actually executes)
	#[serde(rename = "12")]
	ExecutingTrader,
	/// Order Origination Firm (e.g. buyside firm)
	#[serde(rename = "13")]
	OrderOriginationFirm,
	/// Giveup Clearing Firm (firm to which trade is given up)
	#[serde(rename = "14")]
	GiveupClearingFirm,
	/// Correspondant Clearing Firm
	#[serde(rename = "15")]
	CorrespondantClearingFirm,
	/// Executing System
	#[serde(rename = "16")]
	ExecutingSystem,
	/// Contra Firm
	#[serde(rename = "17")]
	ContraFirm,
	/// Contra Clearing Firm
	#[serde(rename = "18")]
	ContraClearingFirm,
	/// Sponsoring Firm
	#[serde(rename = "19")]
	SponsoringFirm,
	/// Underlying Contra Firm
	#[serde(rename = "20")]
	UnderlyingContraFirm,
	/// Clearing Organization
	#[serde(rename = "21")]
	ClearingOrganization,
	/// Exchange
	#[serde(rename = "22")]
	Exchange,
	/// Customer Account
	#[serde(rename = "24")]
	CustomerAccount,
	/// Correspondent Clearing Organization
	#[serde(rename = "25")]
	CorrespondentClearingOrganization,
	/// Correspondent Broker
	#[serde(rename = "26")]
	CorrespondentBroker,
	/// Buyer/Seller (Receiver/Deliverer)
	#[serde(rename = "27")]
	BuyerSeller,
	/// Custodian
	#[serde(rename = "28")]
	Custodian,
	/// Intermediary
	#[serde(rename = "29")]
	Intermediary,
	/// Agent
	#[serde(rename = "30")]
	Agent,
	/// Sub-custodian
	#[serde(rename = "31")]
	SubCustodian,
	/// Beneficiary
	#[serde(rename = "32")]
	Beneficiary,
	/// Interested party
	#[serde(rename = "33")]
	InterestedParty,
	/// Regulatory body
	#[serde(rename = "34")]
	RegulatoryBody,
	/// Liquidity provider
	#[serde(rename = "35")]
	LiquidityProvider,
	/// Entering trader
	#[serde(rename = "36")]
	EnteringTrader,
	/// Contra trader
	#[serde(rename = "37")]
	ContraTrader,
	/// Position account
	#[serde(rename = "38")]
	PositionAccount,
	/// Contra Investor ID
	#[serde(rename = "39")]
	ContraInvestorId,
	/// Transfer to Firm
	#[serde(rename = "40")]
	TransferToFirm,
	/// Contra Position Account
	#[serde(rename = "41")]
	ContraPositionAccount,
	/// Contra Exchange
	#[serde(rename = "42")]
	ContraExchange,
	/// Internal Carry Account
	#[serde(rename = "43")]
	InternalCarryAccount,
	/// Order Entry Operator ID
	#[serde(rename = "44")]
	OrderEntryOperatorId,
	/// Secondary Account Number
	#[serde(rename = "45")]
	SecondaryAccountNumber,
	/// Foriegn Firm
	#[serde(rename = "46")]
	ForiegnFirm,
	/// Third Party Allocation Firm
	#[serde(rename = "47")]
	ThirdPartyAllocationFirm,
	/// Claiming Account
	#[serde(rename = "48")]
	ClaimingAccount,
	/// Asset Manager
	#[serde(rename = "49")]
	AssetManager,
	/// Pledgor Account
	#[serde(rename = "50")]
	PledgorAccount,
	/// Pledgee Account
	#[serde(rename = "51")]
	PledgeeAccount,
	/// Large Trader Reportable Account
	#[serde(rename = "52")]
	LargeTraderReportableAccount,
	/// Trader mnemonic
	#[serde(rename = "53")]
	TraderMnemonic,
	/// Sender Location
	#[serde(rename = "54")]
	SenderLocation,
	/// Session ID
	#[serde(rename = "55")]
	SessionId,
	/// Acceptable Counterparty
	#[serde(rename = "56")]
	AcceptableCounterparty,
	/// Unacceptable Counterparty
	#[serde(rename = "57")]
	UnacceptableCounterparty,
	/// Entering Unit
	#[serde(rename = "58")]
	EnteringUnit,
	/// Executing Unit
	#[serde(rename = "59")]
	ExecutingUnit,
	/// Introducing Broker
	#[serde(rename = "60")]
	IntroducingBroker,
	/// Quote originator
	#[serde(rename = "61")]
	QuoteOriginator,
	/// Report originator
	#[serde(rename = "62")]
	ReportOriginator,
	/// Systematic internaliser (SI)
	#[serde(rename = "63")]
	SystematicInternaliser,
	/// Multilateral Trading Facility (MTF)
	#[serde(rename = "64")]
	MultilateralTradingFacility,
	/// Regulated Market (RM)
	#[serde(rename = "65")]
	RegulatedMarket,
	/// Market Maker
	#[serde(rename = "66")]
	MarketMaker,
	/// Investment Firm
	#[serde(rename = "67")]
	InvestmentFirm,
	/// Host Competent Authority (Host CA)
	#[serde(rename = "68")]
	HostCompetentAuthority,
	/// Home Competent Authority (Home CA)
	#[serde(rename = "69")]
	HomeCompetentAuthority,
	/// Competent Authority of the most relevant market in terms of liquidity (CAL)
	#[serde(rename = "70")]
	CompetentAuthorityOfTheMostRelevantMarketInTermsOfLiquidity,
	/// Competent Authority of the Transaction (Execution) Venue (CATV)
	#[serde(rename = "71")]
	CompetentAuthorityOfTheTransactionVenue,
	/// Reporting intermediary (medium/vendor via which report has been published)
	#[serde(rename = "72")]
	ReportingIntermediary,
	/// Execution Venue
	#[serde(rename = "73")]
	ExecutionVenue,
	/// Market data entry originator
	#[serde(rename = "74")]
	MarketDataEntryOriginator,
	/// Location ID
	#[serde(rename = "75")]
	LocationId,
	/// Prime Broker providing General Trade Services
	#[serde(rename = "79")]
	PrimeBrokerProvidingGeneralTradeServices,
	/// Step-Out Firm (Prime Broker)
	#[serde(rename = "80")]
	StepOutFirm,
	/// BrokerClearingID
	#[serde(rename = "81")]
	BrokerClearingId,
}

impl Default for DerivativeInstrumentPartyRole {
	fn default() -> Self {
		DerivativeInstrumentPartyRole::ExecutingFirm
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeInstrumentPartySubIDType {
	/// Firm
	#[serde(rename = "1")]
	N1,
	/// Person
	#[serde(rename = "2")]
	N2,
	/// System
	#[serde(rename = "3")]
	N3,
	/// Application
	#[serde(rename = "4")]
	N4,
	/// Full legal name of firm
	#[serde(rename = "5")]
	N5,
	/// Postal address
	#[serde(rename = "6")]
	N6,
	/// Phone number
	#[serde(rename = "7")]
	N7,
	/// Email address
	#[serde(rename = "8")]
	N8,
	/// Contact name
	#[serde(rename = "9")]
	N9,
	/// Securities account number (for settlement instructions)
	#[serde(rename = "10")]
	N10,
	/// Registration number (for settlement instructions and confirmations)
	#[serde(rename = "11")]
	N11,
	/// Registered address (for confirmation purposes)
	#[serde(rename = "12")]
	N12,
	/// Regulatory status (for confirmation purposes)
	#[serde(rename = "13")]
	N13,
	/// Registration name (for settlement instructions)
	#[serde(rename = "14")]
	N14,
	/// Cash account number (for settlement instructions)
	#[serde(rename = "15")]
	N15,
	/// BIC
	#[serde(rename = "16")]
	N16,
	/// CSD participant member code
	#[serde(rename = "17")]
	N17,
	/// Registered address
	#[serde(rename = "18")]
	N18,
	/// Fund account name
	#[serde(rename = "19")]
	N19,
	/// Telex number
	#[serde(rename = "20")]
	N20,
	/// Fax number
	#[serde(rename = "21")]
	N21,
	/// Securities account name
	#[serde(rename = "22")]
	N22,
	/// Cash account name
	#[serde(rename = "23")]
	N23,
	/// Department
	#[serde(rename = "24")]
	N24,
	/// Location desk
	#[serde(rename = "25")]
	N25,
	/// Position account type
	#[serde(rename = "26")]
	N26,
	/// Security locate ID
	#[serde(rename = "27")]
	N27,
	/// Market maker
	#[serde(rename = "28")]
	N28,
	/// Eligible counterparty
	#[serde(rename = "29")]
	N29,
	/// Professional client
	#[serde(rename = "30")]
	N30,
	/// Location
	#[serde(rename = "31")]
	N31,
	/// Execution venue
	#[serde(rename = "32")]
	N32,
	/// Currency delivery identifier
	#[serde(rename = "33")]
	N33,
}

impl Default for DerivativeInstrumentPartySubIDType {
	fn default() -> Self {
		DerivativeInstrumentPartySubIDType::N1
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DerivativeInstrAttribType {
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
	CouponPeriodSupplyRedemptionDateInTheAHrefTag872InstrAttribValueHtmlTargetBottomInstrAttribValueNbspAField,
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
	EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheAHrefTag872InstrAttribValueHtmlTargetBottomInstrAttribValueNbspAField,
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
	OriginalIssueDiscountPriceSupplyPriceInTheAHrefTag872InstrAttribValueHtmlTargetBottomInstrAttribValueNbspAField,
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
	TextSupplyTheTextOfTheAttributeOrDisclaimerInTheAHrefTag872InstrAttribValueHtmlTargetBottomInstrAttribValueNbspAField,
}

impl Default for DerivativeInstrAttribType {
	fn default() -> Self {
		DerivativeInstrAttribType::Flat
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrikeExerciseStyle {
	/// European
	#[serde(rename = "0")]
	European,
	/// American
	#[serde(rename = "1")]
	American,
	/// Bermuda
	#[serde(rename = "2")]
	Bermuda,
}

impl Default for StrikeExerciseStyle {
	fn default() -> Self {
		StrikeExerciseStyle::European
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MaturityMonthYearFormat {
	/// YearMonth Only (default)
	#[serde(rename = "0")]
	YearMonthOnly,
	/// YearMonthDay
	#[serde(rename = "1")]
	YearMonthDay,
	/// YearMonthWeek
	#[serde(rename = "2")]
	YearMonthWeek,
}

impl Default for MaturityMonthYearFormat {
	fn default() -> Self {
		MaturityMonthYearFormat::YearMonthOnly
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MaturityMonthYearIncrementUnits {
	/// Months
	#[serde(rename = "0")]
	Months,
	/// Days
	#[serde(rename = "1")]
	Days,
	/// Weeks
	#[serde(rename = "2")]
	Weeks,
	/// Years
	#[serde(rename = "3")]
	Years,
}

impl Default for MaturityMonthYearIncrementUnits {
	fn default() -> Self {
		MaturityMonthYearIncrementUnits::Months
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListUpdateAction {
	/// Add
	#[serde(rename = "A")]
	Add,
	/// Delete
	#[serde(rename = "D")]
	Delete,
	/// Modify
	#[serde(rename = "M")]
	Modify,
}

impl Default for ListUpdateAction {
	fn default() -> Self {
		ListUpdateAction::Add
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CorporateAction {
	/// Ex-Dividend
	#[serde(rename = "A")]
	ExDividend,
	/// Ex-Distribution
	#[serde(rename = "B")]
	ExDistribution,
	/// Ex-Rights
	#[serde(rename = "C")]
	ExRights,
	/// New
	#[serde(rename = "D")]
	New,
	/// Ex-Interest
	#[serde(rename = "E")]
	ExInterest,
	/// Cash Dividend
	#[serde(rename = "F")]
	CashDividend,
	/// Stock Dividend
	#[serde(rename = "G")]
	StockDividend,
	/// Non-Integer Stock Split
	#[serde(rename = "H")]
	NonIntegerStockSplit,
	/// Reverse Stock Split
	#[serde(rename = "I")]
	ReverseStockSplit,
	/// Standard-Integer Stock Split
	#[serde(rename = "J")]
	StandardIntegerStockSplit,
	/// Position Consolidation
	#[serde(rename = "K")]
	PositionConsolidation,
	/// Liquidation Reorganization
	#[serde(rename = "L")]
	LiquidationReorganization,
	/// Merger Reorganization
	#[serde(rename = "M")]
	MergerReorganization,
	/// Rights Offering
	#[serde(rename = "N")]
	RightsOffering,
	/// Shareholder Meeting
	#[serde(rename = "O")]
	ShareholderMeeting,
	/// Spinoff
	#[serde(rename = "P")]
	Spinoff,
	/// Tender Offer
	#[serde(rename = "Q")]
	TenderOffer,
	/// Warrant
	#[serde(rename = "R")]
	Warrant,
	/// Special Action
	#[serde(rename = "S")]
	SpecialAction,
	/// Symbol Conversion
	#[serde(rename = "T")]
	SymbolConversion,
	/// CUSIP / Name Change
	#[serde(rename = "U")]
	CusipNameChange,
	/// Leap Rollover
	#[serde(rename = "V")]
	LeapRollover,
	/// Succession Event
	#[serde(rename = "W")]
	SuccessionEvent,
}

impl Default for CorporateAction {
	fn default() -> Self {
		CorporateAction::ExDividend
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecondaryPriceLimitType {
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

impl Default for SecondaryPriceLimitType {
	fn default() -> Self {
		SecondaryPriceLimitType::Price
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Currency {
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

impl Default for Currency {
	fn default() -> Self {
		Currency::Afa
	}
}
