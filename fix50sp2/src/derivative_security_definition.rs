
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeSecurityDefinition {
	/// Optional block which can be used to to summarize common attributes shared across a set of option instruments which belong
	/// to the same series.
	#[serde(flatten)]
	pub derivative_instrument: Option<super::derivative_instrument::DerivativeInstrument>,
	/// Additional attribution for the instrument series
	#[serde(flatten)]
	pub derivative_instrument_attribute: Option<super::derivative_instrument_attribute::DerivativeInstrumentAttribute>,
	/// Security trading and listing attributes for the series level
	#[serde(flatten)]
	pub market_segment_grp: Option<super::market_segment_grp::MarketSegmentGrp>,
	/// SecurityClassificationGrp
	#[serde(flatten)]
	pub security_classification_grp: Option<super::security_classification_grp::SecurityClassificationGrp>,
}
