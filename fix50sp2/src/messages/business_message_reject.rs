
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Business {
	/// MsgType = j
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// MsgSeqNum of rejected message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "45")]
	pub ref_seq_num: Option<usize>,
	/// The <a href="tag_35_MsgType.html" target="bottom">MsgType&nbsp;(35)</a> of the FIX message being referenced.
	#[serde(rename = "372")]
	pub ref_msg_type: RefMsgType,
	/// Recommended when rejecting an application message that does not explicitly provide <a href="tag_1128_ApplVerID.html" target="bottom">ApplVerID(1128)&nbsp;(1128)</a> on the message being rejected. In this case the value from the <a href="tag_1137_DefaultApplVerID.html" target="bottom">DefaultApplVerID(1137)&nbsp;(1137)</a> or the default value specified in the NoMsgTypes repeating group on the logon message should be provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1130")]
	pub ref_appl_ver_id: Option<String>,
	/// Recommended when rejecting an application message that does not explicitly provide <a href="tag_1156_ApplExtID.html" target="bottom">ApplExtID(1156)&nbsp;(1156)</a> on the rejected message. In this case the value from the <a href="tag_1407_DefaultApplExtID.html" target="bottom">DefaultApplExtID(1407)&nbsp;(1407)</a> or the default value specified in the <a href="tag_384_NoMsgTypes.html" target="bottom">NoMsgTypes&nbsp;(384)</a> repeating group on the logon message should be provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1406")]
	pub ref_appl_ext_id: Option<i32>,
	/// Recommended when rejecting an application message that does not explicitly provide <a href="tag_1129_CstmApplVerID.html" target="bottom">CstmApplVerID(1129)&nbsp;(1129)</a> on the message being rejected. In this case the value from the <a href="tag_1408_DefaultCstmApplVerID.html" target="bottom">DefaultCstmApplVerID(1408)&nbsp;(1408)</a> or the default value specified in the <a href="tag_384_NoMsgTypes.html" target="bottom">NoMsgTypes&nbsp;(384)</a> repeating group on the logon message should be provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1131")]
	pub ref_cstm_appl_ver_id: Option<String>,
	/// The value of the business-level ID field on the message being referenced. Required unless the corresponding ID field (see
	/// list above) was not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "379")]
	pub business_reject_ref_id: Option<String>,
	/// Code to identify reason for a <a href="message_Business_Message_Reject_j.html" target="main">Business Message Reject&nbsp;(j)</a> message.
	#[serde(rename = "380")]
	pub business_reject_reason: BusinessRejectReason,
	/// Where possible, message to explain reason for rejection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RefMsgType {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BusinessRejectReason {
	/// Other
	#[serde(rename = "0")]
	Other,
	/// Unknown ID
	#[serde(rename = "1")]
	UnknownId,
	/// Unknown Security
	#[serde(rename = "2")]
	UnknownSecurity,
	/// Unsupported message type
	#[serde(rename = "3")]
	UnsupportedMessageType,
	/// Application not available
	#[serde(rename = "4")]
	ApplicationNotAvailable,
	/// Conditionally required field missing
	#[serde(rename = "5")]
	ConditionallyRequiredFieldMissing,
	/// Not Authorized
	#[serde(rename = "6")]
	NotAuthorized,
	/// DeliverTo firm not available at this time
	#[serde(rename = "7")]
	DeliverToFirmNotAvailableAtThisTime,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Throttle limit exceeded
	#[serde(rename = "8")]
	ThrottleLimitExceeded,
	/// Throttle limit exceeded, session will be disconnected
	#[serde(rename = "9")]
	ThrottleLimitExceededSessionWillBeDisconnected,
	/// Throttled messages rejected on request
	#[serde(rename = "10")]
	ThrottledMessagesRejectedOnRequest,
}
