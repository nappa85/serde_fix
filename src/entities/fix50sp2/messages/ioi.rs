
use serde::{Serialize, Deserialize};

// use import_fields::import_fields;

// use crate::entities::{ApplVerID, Boolean, FixDateTime, fixt11::{header::*, trailer::Signature}};
use crate::entities::fix50sp2::{application_sequence_control::ApplicationSequenceControl, instrument::Instrument};

/// MsgType = 6
// #[import_fields("src/entities/fixt11/header.rs::Header", "src/entities/fixt11/trailer.rs::Trailer", "src/entities/fix50sp2/application_sequence_control.rs::ApplicationSequenceControl")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IoI {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    #[serde(flatten)]
    pub application_sequence_control: ApplicationSequenceControl,
    /// IOIID
    #[serde(rename = "23")]
    pub ioi_id: String,
    /// IOITransType
    #[serde(rename = "28")]
    pub ioi_trans_type: IOITransType,
    /// Required for Cancel and Replace IOITransType (28) messages
    #[serde(rename = "26")]
    pub ioi_ref_id: Option<String>,
    /// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
    #[serde(flatten)]
    pub instrument: Instrument,
/*<Parties> 	N 	Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
<FinancingDetails> 	N 	Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages".
<UndInstrmtGrp> 	N 	Number of underlyings
///  	Side of Indication. Valid subset of values: 1 = Buy, 2 = Sell, 7 = Undisclosed, B = As Defined (for multilegs), C = Opposite (for multilegs)
#[serde(rename = "54")]
pub Side: Y,
///  	
<OrderQtyData> 	N 	Insert here the set of "Instrument" fields defined in "Common Components of Application Messages". The value zero is used if NoLegs (555) repeating group is used. Applicable if needed to express CashOrderQty (tag 152) (152) .
#[serde(rename = "854")]
pub QtyType: N,
///  	The value zero is used if NoLegs (555) repeating group is used
#[serde(rename = "27")]
pub IOIQty: Y,
///  	
<Stipulations> 	N 	Insert here the set of "Stipulations" fields defined in "Common Components of Application Messages".
#[serde(rename = "15")]
pub Currency: N,
<InstrmtLegIOIGrp> 	N 	Required for multileg IOIs.
///  	
44 	Price 	@Px 	N 	
#[serde(rename = "423")]
pub PriceType: N,
///  	
25 	IOIQltyInd 	@QltyInd 	N 	
#[serde(rename = "62")]
pub ValidUntilTime: N,
///  	
<IOIQualGrp> 	N 	Required if any IOIQualifiers are specified. Indicates the number of repeating IOIQualifiers.
#[serde(rename = "130")]
pub IOINaturalFlag: N,
///  	
354 	EncodedTextLen 	@EncTxtLen 	C 	Must be set if EncodedText (355) field is specified and must immediately precede it.
#[serde(rename = "58")]
pub Text: N,
///  	Encoded (non-ASCII characters) representation of the Text (58) field in the encoded format specified via the MessageEncoding (347) field.
#[serde(rename = "355")]
pub EncodedText: C,
///  	
149 	URLLink 	@URL 	N 	A URL (Uniform Resource Locator) link to additional information (i.e. http://www.XYZ.com/research.html) .
#[serde(rename = "60")]
pub TransactTime: N,
<RoutingGrp> 	N 	Required if any RoutingType (216) and RoutingIDs (217) are specified. Indicates the number within repeating group.
<SpreadOrBenchmarkCurveData> 	N 	Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components of Application Messages".
<YieldData> 	N 	
<RelativeValueGrp> 	N 	
<InstrumentExtension> 	N 	
<RelatedInstrumentGrp> 	N 	
<PriceQualifierGrp> 	N 	 */
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum IOITransType {
    #[serde(rename = "N")]
    New,
    #[serde(rename = "C")]
    Cancel,
    #[serde(rename = "R")]
    Replace,
}
/*
#[cfg(test)]
mod test {
    #[test]
    fn ioi() {
        let msg = "1180=1\u{1}1181=1\u{1}1350=1\u{1}23=1\u{1}28=N\u{1}";
        dbg!(crate::from_str::<super::IoI>(msg)).unwrap();
    }
}
*/