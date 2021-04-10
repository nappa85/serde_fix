
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ThrottleMsgTypeGrp {
	/// NoThrottleMsgType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1618")]
	pub throttle_msg_type: Option<crate::entities::RepeatingValues<ThrottleMsgTyp>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ThrottleMsgTyp {
	/// Required when <a href="tag_1618_NoThrottleMsgType.html" target="bottom">NoThrottleMsgType&nbsp;(1618)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1619")]
	pub throttle_msg_type: Option<ThrottleMsgType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ThrottleMsgType {
	/// Heartbeat
	#[serde(rename = "0")]
	Heartbeat,
	/// Test Request
	#[serde(rename = "1")]
	TestRequest,
	/// Resend Request
	#[serde(rename = "2")]
	ResendRequest,
	/// Reject
	#[serde(rename = "3")]
	Reject,
	/// Sequence Reset
	#[serde(rename = "4")]
	SequenceReset,
	/// Logout
	#[serde(rename = "5")]
	Logout,
	/// Indication of Interest
	#[serde(rename = "6")]
	IndicationOfInterest,
	/// Advertisement
	#[serde(rename = "7")]
	Advertisement,
	/// Execution Report
	#[serde(rename = "8")]
	ExecutionReport,
	/// Order Cancel Reject
	#[serde(rename = "9")]
	OrderCancelReject,
	/// Logon
	#[serde(rename = "A")]
	Logon,
	/// News
	#[serde(rename = "B")]
	News,
	/// Email
	#[serde(rename = "C")]
	Email,
	/// New Order - Single
	#[serde(rename = "D")]
	NewOrderSingle,
	/// New Order - List
	#[serde(rename = "E")]
	NewOrderList,
	/// Order Cancel Request
	#[serde(rename = "F")]
	OrderCancelRequest,
	/// Order Cancel/Replace Request
	#[serde(rename = "G")]
	OrderCancelReplaceRequest,
	/// Order Status Request
	#[serde(rename = "H")]
	OrderStatusRequest,
	/// Allocation Instruction
	#[serde(rename = "J")]
	AllocationInstruction,
	/// List Cancel Request
	#[serde(rename = "K")]
	ListCancelRequest,
	/// List Execute
	#[serde(rename = "L")]
	ListExecute,
	/// List Status Request
	#[serde(rename = "M")]
	ListStatusRequest,
	/// List Status
	#[serde(rename = "N")]
	ListStatus,
	/// Allocation Instruction Ack
	#[serde(rename = "P")]
	AllocationInstructionAck,
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DonTKnowTrade,
	/// Quote Request
	#[serde(rename = "R")]
	QuoteRequest,
	/// Quote
	#[serde(rename = "S")]
	Quote,
	/// Settlement Instructions
	#[serde(rename = "T")]
	SettlementInstructions,
	/// Market Data Request
	#[serde(rename = "V")]
	MarketDataRequest,
	/// Market Data - Snapshot/Full Refresh
	#[serde(rename = "W")]
	MarketDataSnapshotFullRefresh,
	/// Market Data - Incremental Refresh
	#[serde(rename = "X")]
	MarketDataIncrementalRefresh,
	/// Market Data Request Reject
	#[serde(rename = "Y")]
	MarketDataRequestReject,
	/// Quote Cancel
	#[serde(rename = "Z")]
	QuoteCancel,
	/// Quote Status Request
	#[serde(rename = "a")]
	QuoteStatusRequest,
	/// Mass Quote Ack
	#[serde(rename = "b")]
	MassQuoteAck,
	/// Security Definition Request
	#[serde(rename = "c")]
	SecurityDefinitionRequest,
	/// Security Definition
	#[serde(rename = "d")]
	SecurityDefinition,
	/// Security Status Request
	#[serde(rename = "e")]
	SecurityStatusRequest,
	/// Security Status
	#[serde(rename = "f")]
	SecurityStatus,
	/// Trading Session Status Request
	#[serde(rename = "g")]
	TradingSessionStatusRequest,
	/// Trading Session Status
	#[serde(rename = "h")]
	TradingSessionStatus,
	/// Mass Quote
	#[serde(rename = "i")]
	MassQuote,
	/// Business Message Reject
	#[serde(rename = "j")]
	BusinessMessageReject,
	/// Bid Request
	#[serde(rename = "k")]
	BidRequest,
	/// Bid Response
	#[serde(rename = "l")]
	BidResponse,
	/// List Strike Price
	#[serde(rename = "m")]
	ListStrikePrice,
	/// XML message
	#[serde(rename = "n")]
	XmlMessage,
	/// Registration Instructions
	#[serde(rename = "o")]
	RegistrationInstructions,
	/// Registration Instructions Response
	#[serde(rename = "p")]
	RegistrationInstructionsResponse,
	/// Order Mass Cancel Request
	#[serde(rename = "q")]
	OrderMassCancelRequest,
	/// Order Mass Cancel Report
	#[serde(rename = "r")]
	OrderMassCancelReport,
	/// New Order - Cross
	#[serde(rename = "s")]
	NewOrderCross,
	/// Cross Order Cancel/Replace Request
	#[serde(rename = "t")]
	CrossOrderCancelReplaceRequest,
	/// Cross Order Cancel Request
	#[serde(rename = "u")]
	CrossOrderCancelRequest,
	/// Security Type Request
	#[serde(rename = "v")]
	SecurityTypeRequest,
	/// Security Types
	#[serde(rename = "w")]
	SecurityTypes,
	/// Security List Request
	#[serde(rename = "x")]
	SecurityListRequest,
	/// Security List
	#[serde(rename = "y")]
	SecurityList,
	/// Derivative Security List Request
	#[serde(rename = "z")]
	DerivativeSecurityListRequest,
	/// Derivative Security List
	#[serde(rename = "AA")]
	DerivativeSecurityList,
	/// New Order - Multileg
	#[serde(rename = "AB")]
	NewOrderMultileg,
	/// Multileg Order Cancel/Replace
	#[serde(rename = "AC")]
	MultilegOrderCancelReplace,
	/// Trade Capture Report Request
	#[serde(rename = "AD")]
	TradeCaptureReportRequest,
	/// Trade Capture Report
	#[serde(rename = "AE")]
	TradeCaptureReport,
	/// Order Mass Status Request
	#[serde(rename = "AF")]
	OrderMassStatusRequest,
	/// Quote Request Reject
	#[serde(rename = "AG")]
	QuoteRequestReject,
	/// RFQ Request
	#[serde(rename = "AH")]
	RfqRequest,
	/// Quote Status Report
	#[serde(rename = "AI")]
	QuoteStatusReport,
	/// Quote Response
	#[serde(rename = "AJ")]
	QuoteResponse,
	/// Confirmation
	#[serde(rename = "AK")]
	Confirmation,
	/// Position Maintenance Request
	#[serde(rename = "AL")]
	PositionMaintenanceRequest,
	/// Position Maintenance Report
	#[serde(rename = "AM")]
	PositionMaintenanceReport,
	/// Request For Positions
	#[serde(rename = "AN")]
	RequestForPositions,
	/// Request For Positions Ack
	#[serde(rename = "AO")]
	RequestForPositionsAck,
	/// Position Report
	#[serde(rename = "AP")]
	PositionReport,
	/// Trade Capture Report Request Ack
	#[serde(rename = "AQ")]
	TradeCaptureReportRequestAck,
	/// Trade Capture Report Ack
	#[serde(rename = "AR")]
	TradeCaptureReportAck,
	/// Allocation Report
	#[serde(rename = "AS")]
	AllocationReport,
	/// Allocation Report Ack
	#[serde(rename = "AT")]
	AllocationReportAck,
	/// Confirmation Ack
	#[serde(rename = "AU")]
	ConfirmationAck,
	/// Settlement Instruction Request
	#[serde(rename = "AV")]
	SettlementInstructionRequest,
	/// Assignment Report
	#[serde(rename = "AW")]
	AssignmentReport,
	/// Collateral Request
	#[serde(rename = "AX")]
	CollateralRequest,
	/// Collateral Assignment
	#[serde(rename = "AY")]
	CollateralAssignment,
	/// Collateral Response
	#[serde(rename = "AZ")]
	CollateralResponse,
	/// Collateral Report
	#[serde(rename = "BA")]
	CollateralReport,
	/// Collateral Inquiry
	#[serde(rename = "BB")]
	CollateralInquiry,
	/// Network Counterparty System Status Request
	#[serde(rename = "BC")]
	NetworkCounterpartySystemStatusRequest,
	/// Network Counterparty System Status Response
	#[serde(rename = "BD")]
	NetworkCounterpartySystemStatusResponse,
	/// User Request
	#[serde(rename = "BE")]
	UserRequest,
	/// User Response
	#[serde(rename = "BF")]
	UserResponse,
	/// Collateral Inquiry Ack
	#[serde(rename = "BG")]
	CollateralInquiryAck,
	/// Confirmation Request
	#[serde(rename = "BH")]
	ConfirmationRequest,
	/// Trading Session List Request
	#[serde(rename = "BI")]
	TradingSessionListRequest,
	/// Trading Session List
	#[serde(rename = "BJ")]
	TradingSessionList,
	/// Security List Update Report
	#[serde(rename = "BK")]
	SecurityListUpdateReport,
	/// Adjusted Position Report
	#[serde(rename = "BL")]
	AdjustedPositionReport,
	/// Allocation Instruction Alert
	#[serde(rename = "BM")]
	AllocationInstructionAlert,
	/// Execution Acknowledgement
	#[serde(rename = "BN")]
	ExecutionAcknowledgement,
	/// Contrary Intention Report
	#[serde(rename = "BO")]
	ContraryIntentionReport,
	/// Security Definition Update Report
	#[serde(rename = "BP")]
	SecurityDefinitionUpdateReport,
	/// Settlement Obligation Report
	#[serde(rename = "BQ")]
	SettlementObligationReport,
	/// Derivative Security List Update Report
	#[serde(rename = "BR")]
	DerivativeSecurityListUpdateReport,
	/// Trading Session List Update Report
	#[serde(rename = "BS")]
	TradingSessionListUpdateReport,
	/// Market Definition Request
	#[serde(rename = "BT")]
	MarketDefinitionRequest,
	/// Market Definition
	#[serde(rename = "BU")]
	MarketDefinition,
	/// Market Definition Update Report
	#[serde(rename = "BV")]
	MarketDefinitionUpdateReport,
	/// Application Message Request
	#[serde(rename = "BW")]
	ApplicationMessageRequest,
	/// Application Message Request Ack
	#[serde(rename = "BX")]
	ApplicationMessageRequestAck,
	/// Application Message Report
	#[serde(rename = "BY")]
	ApplicationMessageReport,
	/// Order Mass Action Report
	#[serde(rename = "BZ")]
	OrderMassActionReport,
	/// Order Mass Action Request
	#[serde(rename = "CA")]
	OrderMassActionRequest,
	/// User Notification
	#[serde(rename = "CB")]
	UserNotification,
	/// Stream Assignment Request
	#[serde(rename = "CC")]
	StreamAssignmentRequest,
	/// Stream Assignment Report
	#[serde(rename = "CD")]
	StreamAssignmentReport,
	/// Stream Assignment Report ACK
	#[serde(rename = "CE")]
	StreamAssignmentReportAck,
	/// Margin Requirement Inquiry
	#[serde(rename = "CH")]
	MarginRequirementInquiry,
	/// Margin Requirement Inquiry Ack
	#[serde(rename = "CI")]
	MarginRequirementInquiryAck,
	/// Margin Requirement Report
	#[serde(rename = "CJ")]
	MarginRequirementReport,
	/// Party Details List Request
	#[serde(rename = "CF")]
	PartyDetailsListRequest,
	/// Party Details List Report
	#[serde(rename = "CG")]
	PartyDetailsListReport,
	/// Party Details List Update Report
	#[serde(rename = "CK")]
	PartyDetailsListUpdateReport,
	/// Party Risk Limits Request
	#[serde(rename = "CL")]
	PartyRiskLimitsRequest,
	/// Party Risk Limits Report
	#[serde(rename = "CM")]
	PartyRiskLimitsReport,
	/// SecurityMassStatusRequest
	#[serde(rename = "CN")]
	SecurityMassStatusRequest,
	/// SecurityMassStatus
	#[serde(rename = "CO")]
	SecurityMassStatus,
	/// Account Summary Report
	#[serde(rename = "CQ")]
	AccountSummaryReport,
	/// Party Risk Limits Update Report
	#[serde(rename = "CR")]
	PartyRiskLimitsUpdateReport,
	/// Party Risk Limits Definition Request
	#[serde(rename = "CS")]
	PartyRiskLimitsDefinitionRequest,
	/// Party Risk Limits Definition Request Ack
	#[serde(rename = "CT")]
	PartyRiskLimitsDefinitionRequestAck,
	/// Party Entitlements Request
	#[serde(rename = "CU")]
	PartyEntitlementsRequest,
	/// Party Entitlements Report
	#[serde(rename = "CV")]
	PartyEntitlementsReport,
	/// Quote Ack
	#[serde(rename = "CW")]
	QuoteAck,
	/// Party Details Definition Request
	#[serde(rename = "CX")]
	PartyDetailsDefinitionRequest,
	/// Party Details Definition Request Ack
	#[serde(rename = "CY")]
	PartyDetailsDefinitionRequestAck,
	/// Party Entitlements Update Report
	#[serde(rename = "CZ")]
	PartyEntitlementsUpdateReport,
	/// Party Entitlements Definition Request
	#[serde(rename = "DA")]
	PartyEntitlementsDefinitionRequest,
	/// Party Entitlements Definition Request Ack
	#[serde(rename = "DB")]
	PartyEntitlementsDefinitionRequestAck,
	/// Trade Match Report
	#[serde(rename = "DC")]
	TradeMatchReport,
	/// Trade Match Report Ack
	#[serde(rename = "DD")]
	TradeMatchReportAck,
	/// Party Risk Limits Report Ack
	#[serde(rename = "DE")]
	PartyRiskLimitsReportAck,
	/// Party Risk Limit Check Request
	#[serde(rename = "DF")]
	PartyRiskLimitCheckRequest,
	/// Party Risk Limit Check Request Ack
	#[serde(rename = "DG")]
	PartyRiskLimitCheckRequestAck,
	/// Party Action Request
	#[serde(rename = "DH")]
	PartyActionRequest,
	/// Party Action Report
	#[serde(rename = "DI")]
	PartyActionReport,
	/// MassOrder
	#[serde(rename = "DJ")]
	MassOrder,
	/// MassOrderAck
	#[serde(rename = "DK")]
	MassOrderAck,
	/// PositionTransferInstruction
	#[serde(rename = "DL")]
	PositionTransferInstruction,
	/// PositionTransferInstructionAck
	#[serde(rename = "DM")]
	PositionTransferInstructionAck,
	/// PositionTransferReport
	#[serde(rename = "DN")]
	PositionTransferReport,
	/// Market Data Statistics Request
	#[serde(rename = "DO")]
	MarketDataStatisticsRequest,
	/// Market Data Statistics Report
	#[serde(rename = "DP")]
	MarketDataStatisticsReport,
	/// CollateralReportAck
	#[serde(rename = "DQ")]
	CollateralReportAck,
	/// Market Data Report
	#[serde(rename = "DR")]
	MarketDataReport,
	/// AllocationInstructionAlertRequest
	#[serde(rename = "DU")]
	AllocationInstructionAlertRequest,
	/// AllocationInstructionAlertRequestAck
	#[serde(rename = "DV")]
	AllocationInstructionAlertRequestAck,
	/// TradeAggregationRequest
	#[serde(rename = "DW")]
	TradeAggregationRequest,
	/// TradeAggregationReport
	#[serde(rename = "DX")]
	TradeAggregationReport,
}
