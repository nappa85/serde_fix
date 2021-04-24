
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeCapLegUnderlyingsGrp {
	/// Number of legs for the underlying instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1342")]
	pub of_leg_underlyings: Option<fix_common::RepeatingValues<OfLegUnderlying>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OfLegUnderlying {
    #[serde(flatten)]
    pub underlying_leg_instrument: Option<super::underlying_leg_instrument::UnderlyingLegInstrument>,
}
