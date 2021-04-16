
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Currency {
    /// Afghani
    AFA,
    /// Algerian Dinar
    DZD,
    /// Andorran Peseta
    ADP,
    /// Argentine Peso
    ARS,
    /// Armenian Dram
    AMD,
    /// Aruban Guilder
    AWG,
    /// Australian Dollar
    AUD,
    /// Azerbaijanian Manat
    AZM,
    /// Bahamian Dollar
    BSD,
    /// Bahraini Dinar
    BHD,
    /// Baht
    THB,
    /// Balboa
    PAB,
    /// Barbados Dollar
    BBD,
    /// Belarussian Ruble
    BYB,
    /// Belgian Franc
    BEF,
    /// Belize Dollar
    BZD,
    /// Bermudian Dollar
    BMD,
    /// Bolivar
    VEB,
    /// Boliviano
    BOB,
    /// Brazilian Real
    BRL,
    /// Brunei Dollar
    BND,
    /// Burundi Franc
    BIF,
    /// CFA Franc BCEAO+
    XOF,
    /// CFA Franc BEAC#
    XAF,
    /// CFP Franc
    XPF,
    /// Canadian Dollar
    CAD,
    /// Cape Verde Escudo
    CVE,
    /// Cayman Islands Dollar
    KYD,
    /// Cedi
    GHC,
    /// Chilean Peso
    CLP,
    /// Colombian Peso
    COP,
    /// Comoro Franc
    KMF,
    /// Convertible Marks
    BAM,
    /// Cordoba Oro
    NIO,
    /// Costa Rican Colon
    CRC,
    /// Cuban Peso
    CUP,
    /// Cyprus Pound
    CYP,
    /// Czech Koruna
    CZK,
    /// Dalasi
    GMD,
    /// Danish Krone
    DKK,
    /// Denar
    MKD,
    /// Deutsche Mark
    DEM,
    /// Djibouti Franc
    DJF,
    /// Dobra
    STD,
    /// Dominican Peso
    DOP,
    /// Dong
    VND,
    /// Drachma
    GRD,
    /// East Caribbean Dollar
    XCD,
    /// Egyptian Pound
    EGP,
    /// El Salvador Colon
    SVC,
    /// Ethiopian Birr
    ETB,
    /// Euro
    EUR,
    /// Falkland Islands Pound
    FKP,
    /// Fiji Dollar
    FJD,
    /// Forint
    HUF,
    /// Franc Congolais
    CDF,
    /// French Franc
    FRF,
    /// Gibraltar Pound
    GIP,
    /// Gourde
    HTG,
    /// Guarani
    PYG,
    /// Guinea Franc
    GNF,
    /// Guinea-Bissau Peso
    GWP,
    /// Guyana Dollar
    GYD,
    /// Hong Kong Dollar
    HKD,
    /// Hryvnia
    UAH,
    /// Iceland Krona
    ISK,
    /// Indian Rupee
    INR,
    /// Iranian Rial
    IRR,
    /// Iraqi Dinar
    IQD,
    /// Irish Pound
    IEP,
    /// Italian Lira
    ITL,
    /// Jamaican Dollar
    JMD,
    /// Jordanian Dinar
    JOD,
    /// Kenyan Shilling
    KES,
    /// Kina
    PGK,
    /// Kip
    LAK,
    /// Kroon
    EEK,
    /// Kuna
    HRK,
    /// Kuwaiti Dinar
    KWD,
    /// Kwacha
    MWK,
    /// Kwacha
    ZMK,
    /// Kwanza Reajustado
    AOR,
    /// Kyat
    MMK,
    /// Lari
    GEL,
    /// Latvian Lats
    LVL,
    /// Lebanese Pound
    LBP,
    /// Lek
    ALL,
    /// Lempira
    HNL,
    /// Leone
    SLL,
    /// Leu
    ROL,
    /// Lev
    BGL,
    /// Liberian Dollar
    LRD,
    /// Libyan Dinar
    LYD,
    /// Lilangeni
    SZL,
    /// Lithuanian Litas
    LTL,
    /// Loti
    LSL,
    /// Luxembourg Franc
    LUF,
    /// Malagasy Franc
    MGF,
    /// Malaysian Ringgit
    MYR,
    /// Maltese Lira
    MTL,
    /// Manat
    TMM,
    /// Markka
    FIM,
    /// Mauritius Rupee
    MUR,
    /// Metical
    MZM,
    /// Mexican Peso
    MXN,
    /// Mexican Unidad de Inversion (UDI)
    MXV,
    /// Moldovan Leu
    MDL,
    /// Moroccan Dirham
    MAD,
    /// Mvdol
    BOV,
    /// Naira
    NGN,
    /// Nakfa
    ERN,
    /// Namibia Dollar
    NAD,
    /// Nepalese Rupee
    NPR,
    /// Netherlands Antillian Guilder
    ANG,
    /// Netherlands Guilder
    NLG,
    /// New Dinar
    YUM,
    /// New Israeli Sheqel
    ILS,
    /// New Kwanza
    AON,
    /// New Taiwan Dollar
    TWD,
    /// New Zaire
    ZRN,
    /// New Zealand Dollar
    NZD,
    /// Next day
    USN,
    /// Ngultrum
    BTN,
    /// North Korean Won
    KPW,
    /// Norwegian Krone
    NOK,
    /// Nuevo Sol
    PEN,
    /// Ouguiya
    MRO,
    /// Pa'anga
    TOP,
    /// Pakistan Rupee
    PKR,
    /// Pataca
    MOP,
    /// Peso Uruguayo
    UYU,
    /// Philippine Peso
    PHP,
    /// Portuguese Escudo
    PTE,
    /// Pound Sterling
    GBP,
    /// Pula
    BWP,
    /// Qatari Rial
    QAR,
    /// Quetzal
    GTQ,
    /// Rand
    ZAR,
    /// Rial Omani
    OMR,
    /// Riel
    KHR,
    /// Rufiyaa
    MVR,
    /// Rupiah
    IDR,
    /// Russian Ruble
    RUB,
    /// Russian Ruble
    RUR,
    /// Rwanda Franc
    RWF,
    /// SDR
    XDR,
    /// Same day
    USS,
    /// Saudi Riyal
    SAR,
    /// Schilling
    ATS,
    /// Seychelles Rupee
    SCR,
    /// Singapore Dollar
    SGD,
    /// Slovak Koruna
    SKK,
    /// Solomon Islands Dollar
    SBD,
    /// Som
    KGS,
    /// Somali Shilling
    SOS,
    /// Spanish Peseta
    ESP,
    /// Sri Lanka Rupee
    LKR,
    /// St Helena Pound
    SHP,
    /// Sucre
    ECS,
    /// Sudanese Dinar
    SDD,
    /// Surinam Guilder
    SRG,
    /// Swedish Krona
    SEK,
    /// Swiss Franc
    CHF,
    /// Syrian Pound
    SYP,
    /// Tajik Ruble
    TJR,
    /// Taka
    BDT,
    /// Tala
    WST,
    /// Tanzanian Shilling
    TZS,
    /// Tenge
    KZT,
    /// Timor Escudo
    TPE,
    /// Tolar
    SIT,
    /// Trinidad and Tobago Dollar
    TTD,
    /// Tugrik
    MNT,
    /// Tunisian Dinar
    TND,
    /// Turkish Lira
    TRL,
    /// UAE Dirham
    AED,
    /// US Dollar
    USD,
    /// Uganda Shilling
    UGX,
    /// Unidad de Valor Constante (UVC)
    ECV,
    /// Unidades de fomento
    CLF,
    /// Uzbekistan Sum
    UZS,
    /// Vatu
    VUV,
    /// Won
    KRW,
    /// Yemeni Rial
    YER,
    /// Yen
    JPY,
    /// Yuan Renminbi
    CNY,
    /// Zimbabwe Dollar
    ZWD,
    /// Zloty
    PLN,
    /// financial Rand
    ZAL,
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
    XAU,
    /// European Composite Unit (EURCO)
    XBA,
    /// European Monetary Unit (E.M.U.-6)
    XBB,
    /// European Unit of Account 9 (E.U.A.- 9)
    XBC,
    /// European Unit of Account 17 (E.U.A.- 17)
    XBD,
    /// Palladium
    XPD,
    /// Platinum
    XPT,
    /// Silver
    XAG,
    /// UIC-Franc
    XFU,
    /// Gold-Franc
    XFO,
    /// Codes specifically reserved for testing purposes
    XTS,
    /// Codes assigned for transactions where no currency is involved
    XXX,
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