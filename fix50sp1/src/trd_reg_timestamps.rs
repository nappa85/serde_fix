
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegTimestamps {
	/// NoTrdRegTimestamps
	#[serde(rename = "768")]
	pub trd_reg_timestamps: fix_common::RepeatingValues<TrdRegTimestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegTimestamp {
	/// Required if <a href="tag_768_NoTrdRegTimestamps.html" target="bottom">NoTrdRegTimestamps&nbsp;(768)</a> &gt; 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "769")]
	pub trd_reg_timestamp: Option<fix_common::UTCTimestamp>,
	/// Required if <a href="tag_768_NoTrdRegTimestamps.html" target="bottom">NoTrdRegTimestamps&nbsp;(768)</a> &gt; 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "770")]
	pub trd_reg_timestamp_type: Option<TrdRegTimestampType>,
	/// TrdRegTimestampOrigin
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "771")]
	pub trd_reg_timestamp_origin: Option<String>,
	/// Type of Trading desk
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1033")]
	pub desk_type: Option<DeskType>,
	/// DeskTypeSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1034")]
	pub desk_type_source: Option<DeskTypeSource>,
	/// DeskOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1035")]
	pub desk_order_handling_inst: Option<fix_common::SeparatedValues<DeskOrderHandlingInst>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdRegTimestampType {
	/// Execution Time
	#[serde(rename = "1")]
	ExecutionTime,
	/// Time In
	#[serde(rename = "2")]
	TimeIn,
	/// Time Out
	#[serde(rename = "3")]
	TimeOut,
	/// Broker Receipt
	#[serde(rename = "4")]
	BrokerReceipt,
	/// Broker Execution
	#[serde(rename = "5")]
	BrokerExecution,
	/// Desk Receipt
	#[serde(rename = "6")]
	DeskReceipt,
}

impl Default for TrdRegTimestampType {
	fn default() -> Self {
		TrdRegTimestampType::ExecutionTime
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeskType {
	/// Agency
	#[serde(rename = "A")]
	Agency,
	/// Arbitrage
	#[serde(rename = "AR")]
	Arbitrage,
	/// Derivatives
	#[serde(rename = "D")]
	Derivatives,
	/// International
	#[serde(rename = "IN")]
	International,
	/// Institutional
	#[serde(rename = "IS")]
	Institutional,
	/// Other
	#[serde(rename = "O")]
	Other,
	/// Preferred Trading
	#[serde(rename = "PF")]
	PreferredTrading,
	/// Proprietary
	#[serde(rename = "PR")]
	Proprietary,
	/// Program Trading
	#[serde(rename = "PT")]
	ProgramTrading,
	/// Sales
	#[serde(rename = "S")]
	Sales,
	/// Trading
	#[serde(rename = "T")]
	Trading,
}

impl Default for DeskType {
	fn default() -> Self {
		DeskType::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeskTypeSource {
	/// NASD OATS
	#[serde(rename = "1")]
	NasdOats,
}

impl Default for DeskTypeSource {
	fn default() -> Self {
		DeskTypeSource::NasdOats
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeskOrderHandlingInst {
	/// Add-on Order
	#[serde(rename = "ADD")]
	AddOnOrder,
	/// All or None
	#[serde(rename = "AON")]
	AllOrNone,
	/// Cash Not Held
	#[serde(rename = "CNH")]
	CashNotHeld,
	/// Directed Order
	#[serde(rename = "DIR")]
	DirectedOrder,
	/// Exchange for Physical Transaction
	#[serde(rename = "E.W")]
	ExchangeForPhysicalTransaction,
	/// Fill or Kill
	#[serde(rename = "FOK")]
	FillOrKill,
	/// Imbalance Only
	#[serde(rename = "IO")]
	ImbalanceOnly,
	/// Immediate or Cancel
	#[serde(rename = "IOC")]
	ImmediateOrCancel,
	/// Limit On Open
	#[serde(rename = "LOO")]
	LimitOnOpen,
	/// Limit on Close
	#[serde(rename = "LOC")]
	LimitOnClose,
	/// Market at Open
	#[serde(rename = "MAO")]
	MarketAtOpen,
	/// Market at Close
	#[serde(rename = "MAC")]
	MarketAtClose,
	/// Market on Open
	#[serde(rename = "MOO")]
	MarketOnOpen,
	/// Market On Close
	#[serde(rename = "MOC")]
	MarketOnClose,
	/// Minimum Quantity
	#[serde(rename = "MQT")]
	MinimumQuantity,
	/// Not Held
	#[serde(rename = "NH")]
	NotHeld,
	/// Over the Day
	#[serde(rename = "OVD")]
	OverTheDay,
	/// Pegged
	#[serde(rename = "PEG")]
	Pegged,
	/// Reserve Size Order
	#[serde(rename = "RSV")]
	ReserveSizeOrder,
	/// Stop Stock Transaction
	#[serde(rename = "S.W")]
	StopStockTransaction,
	/// Scale
	#[serde(rename = "SCL")]
	Scale,
	/// Time Order
	#[serde(rename = "TMO")]
	TimeOrder,
	/// Trailing Stop
	#[serde(rename = "TS")]
	TrailingStop,
	/// Work
	#[serde(rename = "WRK")]
	Work,
}

impl Default for DeskOrderHandlingInst {
	fn default() -> Self {
		DeskOrderHandlingInst::AddOnOrder
	}
}
