
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct News {
	/// MsgType = B
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Unique identifer for News message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1472")]
	pub news_id: Option<NewsID>,
	/// News items referenced by this News message.
	#[serde(flatten)]
	pub news_ref_grp: Option<super::super::news_ref_grp::NewsRefGrp>,
	/// NewsCategory
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1473")]
	pub news_category: Option<NewsCategory>,
	/// Used to optionally specify the national language used for the News item.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1474")]
	pub language_code: Option<LanguageCode>,
	/// OrigTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42")]
	pub orig_time: Option<fix_common::UTCTimestamp>,
	/// Urgency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "61")]
	pub urgency: Option<Urgency>,
	/// Specifies the headline text
	#[serde(rename = "148")]
	pub headline: String,
	/// Must be set if <a href="tag_359_EncodedHeadline.html" target="bottom">EncodedHeadline&nbsp;(359)</a> field is specified and must immediately precede it.
	#[serde(rename = "358")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_148_Headline.html" target="bottom">Headline&nbsp;(148)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "359")]
	pub encoded_headline: Option<fix_common::EncodedText<359>>,
	/// Required if any <a href="tag_216_RoutingType.html" target="bottom">RoutingType&nbsp;(216)</a> and <a href="tag_217_RoutingID.html" target="bottom">RoutingIDs&nbsp;(217)</a> are specified. Indicates the number within repeating group.
	#[serde(flatten)]
	pub routing_grp: Option<super::super::routing_grp::RoutingGrp>,
	/// Used to optionally specify the market to which this News applies.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Used to optionally specify the market segment to which this News applies.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Specifies the number of repeating symbols (instruments) specified.
	#[serde(flatten)]
	pub instrmt_grp: Option<super::super::instrmt_grp::InstrmtGrp>,
	/// InstrmtLegGrp
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// Number of underlyings.
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// Specifies the number of repeating lines of text specified.
	#[serde(flatten)]
	pub lines_of_text_grp: super::super::lines_of_text_grp::LinesOfTextGrp,
	/// A URL (Uniform Resource Locator) link to additional information (i.e. http://www.XYZ.com/research.html)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "149")]
	pub url_link: Option<String>,
	/// RawDataLength
	#[serde(rename = "95")]
	/// RawData
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NewsID {
	/// ICB (Industry Classification Benchmark) published by Dow Jones and FTSE - <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.icbenchmark.com" target="_blank">www.icbenchmark.com</a> .
	#[serde(rename = "1")]
	IcbPublishedByDowJonesAndFtseAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwIcbenchmarkComTargetBlankWwwIcbenchmarkComA,
	/// NAICS (North American Industry Classification System). Replaced SIC (Standard Industry Classification) naics <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.census.gov/naics" target="_blank">www.census.gov/naics</a> or <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="http://www.naics.com" target="_blank">www.naics.com</a> .
	#[serde(rename = "2")]
	NaicsReplacedSicNaicsAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwCensusGovNaicsTargetBlankWwwCensusGovNaicsAOrAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwNaicsComTargetBlankWwwNaicsComA,
	/// GICS (Global Industry Classification Standard) published by Standards and Poor.
	#[serde(rename = "3")]
	GicsPublishedByStandardsAndPoor,
}

impl Default for NewsID {
	fn default() -> Self {
		NewsID::IcbPublishedByDowJonesAndFtseAXmlnsHttpWwwB2BitsComFixProtocolXmlnsXsiHttpWwwW3Org2001XmlSchemaInstanceHrefHttpWwwIcbenchmarkComTargetBlankWwwIcbenchmarkComA
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NewsCategory {
	/// Company News
	#[serde(rename = "0")]
	CompanyNews,
	/// Marketplace News
	#[serde(rename = "1")]
	MarketplaceNews,
	/// Financial Market News
	#[serde(rename = "2")]
	FinancialMarketNews,
	/// Technical News
	#[serde(rename = "3")]
	TechnicalNews,
	/// Other News
	#[serde(rename = "99")]
	OtherNews,
}

impl Default for NewsCategory {
	fn default() -> Self {
		NewsCategory::CompanyNews
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LanguageCode {
	/// Abkhaz
	#[serde(rename = "ab")]
	Abkhaz,
	/// Afar
	#[serde(rename = "aa")]
	Afar,
	/// Afrikaans
	#[serde(rename = "af")]
	Afrikaans,
	/// Akan
	#[serde(rename = "ak")]
	Akan,
	/// Albanian
	#[serde(rename = "sq")]
	Albanian,
	/// Amharic
	#[serde(rename = "am")]
	Amharic,
	/// Arabic
	#[serde(rename = "ar")]
	Arabic,
	/// Aragonese
	#[serde(rename = "an")]
	Aragonese,
	/// Armenian
	#[serde(rename = "hy")]
	Armenian,
	/// Assamese
	#[serde(rename = "as")]
	Assamese,
	/// Avaric
	#[serde(rename = "av")]
	Avaric,
	/// Avestan
	#[serde(rename = "ae")]
	Avestan,
	/// Aymara
	#[serde(rename = "ay")]
	Aymara,
	/// Azerbaijani
	#[serde(rename = "az")]
	Azerbaijani,
	/// Bambara
	#[serde(rename = "bm")]
	Bambara,
	/// Bashkir
	#[serde(rename = "ba")]
	Bashkir,
	/// Basque
	#[serde(rename = "eu")]
	Basque,
	/// Belarusian
	#[serde(rename = "be")]
	Belarusian,
	/// Bengali
	#[serde(rename = "bn")]
	Bengali,
	/// Bihari
	#[serde(rename = "bh")]
	Bihari,
	/// Bislama
	#[serde(rename = "bi")]
	Bislama,
	/// Bosnian
	#[serde(rename = "bs")]
	Bosnian,
	/// Breton
	#[serde(rename = "br")]
	Breton,
	/// Bulgarian
	#[serde(rename = "bg")]
	Bulgarian,
	/// Burmese
	#[serde(rename = "my")]
	Burmese,
	/// Catalan; Valencian
	#[serde(rename = "ca")]
	CatalanValencian,
	/// Chamorro
	#[serde(rename = "ch")]
	Chamorro,
	/// Chechen
	#[serde(rename = "ce")]
	Chechen,
	/// Chichewa; Chewa; Nyanja
	#[serde(rename = "ny")]
	ChichewaChewaNyanja,
	/// Chinese
	#[serde(rename = "zh")]
	Chinese,
	/// Chuvash
	#[serde(rename = "cv")]
	Chuvash,
	/// Cornish
	#[serde(rename = "kw")]
	Cornish,
	/// Corsican
	#[serde(rename = "co")]
	Corsican,
	/// Cree
	#[serde(rename = "cr")]
	Cree,
	/// Croatian
	#[serde(rename = "hr")]
	Croatian,
	/// Czech
	#[serde(rename = "cs")]
	Czech,
	/// Danish
	#[serde(rename = "da")]
	Danish,
	/// Divehi; Dhivehi; Maldivian;
	#[serde(rename = "dv")]
	DivehiDhivehiMaldivian,
	/// Dutch
	#[serde(rename = "nl")]
	Dutch,
	/// Dzongkha
	#[serde(rename = "dz")]
	Dzongkha,
	/// English
	#[serde(rename = "en")]
	English,
	/// Esperanto
	#[serde(rename = "eo")]
	Esperanto,
	/// Estonian
	#[serde(rename = "et")]
	Estonian,
	/// Ewe
	#[serde(rename = "ee")]
	Ewe,
	/// Faroese
	#[serde(rename = "fo")]
	Faroese,
	/// Fijian
	#[serde(rename = "fj")]
	Fijian,
	/// Finnish
	#[serde(rename = "fi")]
	Finnish,
	/// French
	#[serde(rename = "fr")]
	French,
	/// Fula; Fulah; Pulaar; Pular
	#[serde(rename = "ff")]
	FulaFulahPulaarPular,
	/// Galician
	#[serde(rename = "gl")]
	Galician,
	/// Georgian
	#[serde(rename = "ka")]
	Georgian,
	/// German
	#[serde(rename = "de")]
	German,
	/// Greek, Modern
	#[serde(rename = "el")]
	GreekModern,
	/// Guarani'
	#[serde(rename = "gn")]
	Guarani,
	/// Gujarati
	#[serde(rename = "gu")]
	Gujarati,
	/// Haitian; Haitian Creole
	#[serde(rename = "ht")]
	HaitianHaitianCreole,
	/// Hausa
	#[serde(rename = "ha")]
	Hausa,
	/// Hebrew (modern)
	#[serde(rename = "he")]
	Hebrew,
	/// Herero
	#[serde(rename = "hz")]
	Herero,
	/// Hindi
	#[serde(rename = "hi")]
	Hindi,
	/// Hiri Motu
	#[serde(rename = "ho")]
	HiriMotu,
	/// Hungarian
	#[serde(rename = "hu")]
	Hungarian,
	/// Interlingua
	#[serde(rename = "ia")]
	Interlingua,
	/// Indonesian
	#[serde(rename = "id")]
	Indonesian,
	/// Interlingue
	#[serde(rename = "ie")]
	Interlingue,
	/// Irish
	#[serde(rename = "ga")]
	Irish,
	/// Igbo
	#[serde(rename = "ig")]
	Igbo,
	/// Inupiaq
	#[serde(rename = "ik")]
	Inupiaq,
	/// Ido
	#[serde(rename = "io")]
	Ido,
	/// Icelandic
	#[serde(rename = "is")]
	Icelandic,
	/// Italian
	#[serde(rename = "it")]
	Italian,
	/// Inuktitut
	#[serde(rename = "iu")]
	Inuktitut,
	/// Japanese
	#[serde(rename = "ja")]
	Japanese,
	/// Javanese
	#[serde(rename = "jv")]
	Javanese,
	/// Kalaallisut, Greenlandic
	#[serde(rename = "kl")]
	KalaallisutGreenlandic,
	/// Kannada
	#[serde(rename = "kn")]
	Kannada,
	/// Kanuri
	#[serde(rename = "kr")]
	Kanuri,
	/// Kashmiri
	#[serde(rename = "ks")]
	Kashmiri,
	/// Kazakh
	#[serde(rename = "kk")]
	Kazakh,
	/// Khmer
	#[serde(rename = "km")]
	Khmer,
	/// Kikuyu, Gikuyu
	#[serde(rename = "ki")]
	KikuyuGikuyu,
	/// Kinyarwanda
	#[serde(rename = "rw")]
	Kinyarwanda,
	/// Kirghiz, Kyrgyz
	#[serde(rename = "ky")]
	KirghizKyrgyz,
	/// Komi
	#[serde(rename = "kv")]
	Komi,
	/// Kongo
	#[serde(rename = "kg")]
	Kongo,
	/// Korean
	#[serde(rename = "ko")]
	Korean,
	/// Kurdish
	#[serde(rename = "ku")]
	Kurdish,
	/// Kwanyama, Kuanyama
	#[serde(rename = "kj")]
	KwanyamaKuanyama,
	/// Latin
	#[serde(rename = "la")]
	Latin,
	/// Luxembourgish, Letzeburgesch
	#[serde(rename = "lb")]
	LuxembourgishLetzeburgesch,
	/// Luganda
	#[serde(rename = "lg")]
	Luganda,
	/// Limburgish, Limburgan, Limburger
	#[serde(rename = "li")]
	LimburgishLimburganLimburger,
	/// Lingala
	#[serde(rename = "ln")]
	Lingala,
	/// Lao
	#[serde(rename = "lo")]
	Lao,
	/// Lithuanian
	#[serde(rename = "lt")]
	Lithuanian,
	/// Luba-Katanga
	#[serde(rename = "lu")]
	LubaKatanga,
	/// Latvian
	#[serde(rename = "lv")]
	Latvian,
	/// Manx
	#[serde(rename = "gv")]
	Manx,
	/// Macedonian
	#[serde(rename = "mk")]
	Macedonian,
	/// Malagasy
	#[serde(rename = "mg")]
	Malagasy,
	/// Malay
	#[serde(rename = "ms")]
	Malay,
	/// Malayalam
	#[serde(rename = "ml")]
	Malayalam,
	/// Maltese
	#[serde(rename = "mt")]
	Maltese,
	/// Ma-ori
	#[serde(rename = "mi")]
	MaOri,
	/// Marathi (Mara-t.hi-)
	#[serde(rename = "mr")]
	Marathi,
	/// Marshallese
	#[serde(rename = "mh")]
	Marshallese,
	/// Mongolian
	#[serde(rename = "mn")]
	Mongolian,
	/// Nauru
	#[serde(rename = "na")]
	Nauru,
	/// Navajo, Navaho
	#[serde(rename = "nv")]
	NavajoNavaho,
	/// Norwegian Bokma*l
	#[serde(rename = "nb")]
	NorwegianBokmaL,
	/// North Ndebele
	#[serde(rename = "nd")]
	NorthNdebele,
	/// Nepali
	#[serde(rename = "ne")]
	Nepali,
	/// Ndonga
	#[serde(rename = "ng")]
	Ndonga,
	/// Norwegian Nynorsk
	#[serde(rename = "nn")]
	NorwegianNynorsk,
	/// Norwegian
	#[serde(rename = "no")]
	Norwegian,
	/// Nuosu
	#[serde(rename = "ii")]
	Nuosu,
	/// South Ndebele
	#[serde(rename = "nr")]
	SouthNdebele,
	/// Occitan
	#[serde(rename = "oc")]
	Occitan,
	/// Ojibwe, Ojibwa
	#[serde(rename = "oj")]
	OjibweOjibwa,
	/// Old Church Slavonic, Church Slavic, Church Slavonic, Old Bulgarian, Old Slavonic
	#[serde(rename = "cu")]
	OldChurchSlavonicChurchSlavicChurchSlavonicOldBulgarianOldSlavonic,
	/// Oromo
	#[serde(rename = "om")]
	Oromo,
	/// Oriya
	#[serde(rename = "or")]
	Oriya,
	/// Ossetian, Ossetic
	#[serde(rename = "os")]
	OssetianOssetic,
	/// Panjabi, Punjabi
	#[serde(rename = "pa")]
	PanjabiPunjabi,
	/// Pa-li
	#[serde(rename = "pi")]
	PaLi,
	/// Persian
	#[serde(rename = "fa")]
	Persian,
	/// Polish
	#[serde(rename = "pl")]
	Polish,
	/// Pashto, Pushto
	#[serde(rename = "ps")]
	PashtoPushto,
	/// Portuguese
	#[serde(rename = "pt")]
	Portuguese,
	/// Quechua
	#[serde(rename = "qu")]
	Quechua,
	/// Romansh
	#[serde(rename = "rm")]
	Romansh,
	/// Kirundi
	#[serde(rename = "rn")]
	Kirundi,
	/// Romanian, Moldavian, Moldovan
	#[serde(rename = "ro")]
	RomanianMoldavianMoldovan,
	/// Russian
	#[serde(rename = "ru")]
	Russian,
	/// Sanskrit (Sam.skr.ta)
	#[serde(rename = "sa")]
	Sanskrit,
	/// Sardinian
	#[serde(rename = "sc")]
	Sardinian,
	/// Sindhi
	#[serde(rename = "sd")]
	Sindhi,
	/// Northern Sami
	#[serde(rename = "se")]
	NorthernSami,
	/// Samoan
	#[serde(rename = "sm")]
	Samoan,
	/// Sango
	#[serde(rename = "sg")]
	Sango,
	/// Serbian
	#[serde(rename = "sr")]
	Serbian,
	/// Scottish Gaelic; Gaelic
	#[serde(rename = "gd")]
	ScottishGaelicGaelic,
	/// Shona
	#[serde(rename = "sn")]
	Shona,
	/// Sinhala, Sinhalese
	#[serde(rename = "si")]
	SinhalaSinhalese,
	/// Slovak
	#[serde(rename = "sk")]
	Slovak,
	/// Slovene
	#[serde(rename = "sl")]
	Slovene,
	/// Somali
	#[serde(rename = "so")]
	Somali,
	/// Southern Sotho
	#[serde(rename = "st")]
	SouthernSotho,
	/// Spanish; Castilian
	#[serde(rename = "es")]
	SpanishCastilian,
	/// Sundanese
	#[serde(rename = "su")]
	Sundanese,
	/// Swahili
	#[serde(rename = "sw")]
	Swahili,
	/// Swati
	#[serde(rename = "ss")]
	Swati,
	/// Swedish
	#[serde(rename = "sv")]
	Swedish,
	/// Tamil
	#[serde(rename = "ta")]
	Tamil,
	/// Telugu
	#[serde(rename = "te")]
	Telugu,
	/// Tajik
	#[serde(rename = "tg")]
	Tajik,
	/// Thai
	#[serde(rename = "th")]
	Thai,
	/// Tigrinya
	#[serde(rename = "ti")]
	Tigrinya,
	/// Tibetan Standard, Tibetan, Central
	#[serde(rename = "bo")]
	TibetanStandardTibetanCentral,
	/// Turkmen
	#[serde(rename = "tk")]
	Turkmen,
	/// Tagalog
	#[serde(rename = "tl")]
	Tagalog,
	/// Tswana
	#[serde(rename = "tn")]
	Tswana,
	/// Tonga (Tonga Islands)
	#[serde(rename = "to")]
	Tonga,
	/// Turkish
	#[serde(rename = "tr")]
	Turkish,
	/// Tsonga
	#[serde(rename = "ts")]
	Tsonga,
	/// Tatar
	#[serde(rename = "tt")]
	Tatar,
	/// Twi
	#[serde(rename = "tw")]
	Twi,
	/// Tahitian
	#[serde(rename = "ty")]
	Tahitian,
	/// Uighur, Uyghur
	#[serde(rename = "ug")]
	UighurUyghur,
	/// Ukrainian
	#[serde(rename = "uk")]
	Ukrainian,
	/// Urdu
	#[serde(rename = "ur")]
	Urdu,
	/// Uzbek
	#[serde(rename = "uz")]
	Uzbek,
	/// Venda
	#[serde(rename = "ve")]
	Venda,
	/// Vietnamese
	#[serde(rename = "vi")]
	Vietnamese,
	/// Volapuk
	#[serde(rename = "vo")]
	Volapuk,
	/// Walloon
	#[serde(rename = "wa")]
	Walloon,
	/// Welsh
	#[serde(rename = "cy")]
	Welsh,
	/// Wolof
	#[serde(rename = "wo")]
	Wolof,
	/// Western Frisian
	#[serde(rename = "fy")]
	WesternFrisian,
	/// Xhosa
	#[serde(rename = "xh")]
	Xhosa,
	/// Yiddish
	#[serde(rename = "yi")]
	Yiddish,
	/// Yoruba
	#[serde(rename = "yo")]
	Yoruba,
	/// Zhuang, Chuang
	#[serde(rename = "za")]
	ZhuangChuang,
	/// Zulu
	#[serde(rename = "zu")]
	Zulu,
}

impl Default for LanguageCode {
	fn default() -> Self {
		LanguageCode::Abkhaz
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Urgency {
	/// Normal
	#[serde(rename = "0")]
	Normal,
	/// Flash
	#[serde(rename = "1")]
	Flash,
	/// Background
	#[serde(rename = "2")]
	Background,
}

impl Default for Urgency {
	fn default() -> Self {
		Urgency::Normal
	}
}
