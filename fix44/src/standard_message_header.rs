
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageHeader {
	/// FIX.4.4 (Always unencrypted, must be first field in message)
	#[serde(rename = "8")]
	pub begin_string: Option<fix_common::FixVersion>,
	/// (Always unencrypted, must be second field in message)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "9")]
	pub body_length: usize,
	/// (Always unencrypted, must be third field in message)
	#[serde(rename = "35")]
	pub msg_type: MsgType,
	/// (Always unencrypted)
	#[serde(rename = "49")]
	pub sender_comp_id: String,
	/// (Always unencrypted)
	#[serde(rename = "56")]
	pub target_comp_id: String,
	/// Trading partner company ID used when sending messages via a third party (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "115")]
	pub on_behalf_of_comp_id: Option<String>,
	/// Trading partner company ID used when sending messages via a third party (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "128")]
	pub deliver_to_comp_id: Option<String>,
	/// Required to identify length of encrypted section of message. (Always unencrypted)
	#[serde(rename = "90")]
	/// Required when message body is encrypted. Always immediately follows <a href="tag_90_SecureDataLen.html" target="bottom">SecureDataLen&nbsp;(90)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "91")]
	pub secure_data: Option<fix_common::EncodedText<91>>,
	/// (Can be embedded within encrypted data section.)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "34")]
	pub msg_seq_num: usize,
	/// (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "50")]
	pub sender_sub_id: Option<String>,
	/// Sender's LocationID (i.e. geographic location and/or desk) (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "142")]
	pub sender_location_id: Option<String>,
	/// "ADMIN" reserved for administrative messages not intended for a specific user. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "57")]
	pub target_sub_id: Option<String>,
	/// Trading partner LocationID (i.e. geographic location and/or desk) (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "143")]
	pub target_location_id: Option<String>,
	/// Trading partner SubID used when delivering messages via a third party. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "116")]
	pub on_behalf_of_sub_id: Option<String>,
	/// Trading partner LocationID (i.e. geographic location and/or desk) used when delivering messages via a third party. (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "144")]
	pub on_behalf_of_location_id: Option<String>,
	/// Trading partner SubID used when delivering messages via a third party. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "129")]
	pub deliver_to_sub_id: Option<String>,
	/// Trading partner LocationID (i.e. geographic location and/or desk) used when delivering messages via a third party. (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "145")]
	pub deliver_to_location_id: Option<String>,
	/// Always required for retransmitted messages, whether prompted by the sending system or as the result of a resend request. (Can
	/// be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43")]
	pub poss_dup_flag: Option<PossDupFlag>,
	/// Required when message may be duplicate of another message sent under a different sequence number. (Can be embedded within
	/// encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "97")]
	pub poss_resend: Option<PossResend>,
	/// (Can be embedded within encrypted data section.)
	#[serde(rename = "52")]
	pub sending_time: fix_common::UTCTimestamp,
	/// Required for message resent as a result of a <a href="message_Resend_Request_2.html" target="main">Resend Request&nbsp;(2)</a> . If data is not available set to same value as <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "122")]
	pub orig_sending_time: Option<fix_common::UTCTimestamp>,
	/// Required when specifying <a href="tag_213_XmlData.html" target="bottom">XmlData&nbsp;(213)</a> to identify the length of a XmlData message block. (Can be embedded within encrypted data section.)
	#[serde(rename = "212")]
	/// Can contain a XML formatted message block (e.g. FIXML). Always immediately follows <a href="tag_212_XmlDataLen.html" target="bottom">XmlDataLen&nbsp;(212)</a> field. (Can be embedded within encrypted data section.) <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="https://www.fixtrading.org/standards/" target="_blank">See Volume 1: FIXML Support of FIX Specification</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "213")]
	pub xml_data: Option<fix_common::EncodedText<213>>,
	/// Type of message encoding (non-ASCII characters) used in a message's "Encoded" fields. Required if any "Encoding" fields are
	/// used.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "347")]
	pub message_encoding: Option<MessageEncoding>,
	/// The last <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> value received and processed. Can be specified on every message sent. Useful for detecting a backlog with a counterparty.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "369")]
	pub last_msg_seq_num_processed: Option<usize>,
	/// Number of repeating groups of historical "hop" information. Only applicable if <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> is used, however, its use is optional. Note that some market regulations or counterparties may require tracking of message
	/// hops.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "627")]
	pub hops: Option<fix_common::RepeatingValues<Hop>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Hop {
	/// Third party firm which delivered a specific message either from the firm which originated the message or from another third
	/// party (if multiple "hops" are performed). It is recommended that this value be the <a href="tag_49_SenderCompID.html" target="bottom">SenderCompID&nbsp;(49)</a> of the third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "628")]
	pub hop_comp_id: Option<String>,
	/// Time that <a href="tag_628_HopCompID.html" target="bottom">HopCompID&nbsp;(628)</a> sent the message. It is recommended that this value be the <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> of the message sent by the third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "629")]
	pub hop_sending_time: Option<fix_common::UTCTimestamp>,
	/// Reference identifier assigned by <a href="tag_628_HopCompID.html" target="bottom">HopCompID&nbsp;(628)</a> associated with the message sent. It is recommended that this value be the <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> of the message sent by the third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "630")]
	pub hop_ref_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MsgType {
	/// <a href="message_Heartbeat_0.html" target="main">Heartbeat&nbsp;(0)</a>
	#[serde(rename = "0")]
	AHrefMessageHeartbeat0HtmlTargetMainHeartbeatNbspA,
	/// <a href="message_Test_Request_1.html" target="main">Test Request&nbsp;(1)</a>
	#[serde(rename = "1")]
	AHrefMessageTestRequest1HtmlTargetMainTestRequestNbspA,
	/// <a href="message_Resend_Request_2.html" target="main">Resend Request&nbsp;(2)</a>
	#[serde(rename = "2")]
	AHrefMessageResendRequest2HtmlTargetMainResendRequestNbspA,
	/// <a href="message_Reject_3.html" target="main">Reject&nbsp;(3)</a>
	#[serde(rename = "3")]
	AHrefMessageReject3HtmlTargetMainRejectNbspA,
	/// <a href="message_Sequence_Reset_4.html" target="main">Sequence Reset&nbsp;(4)</a>
	#[serde(rename = "4")]
	AHrefMessageSequenceReset4HtmlTargetMainSequenceResetNbspA,
	/// <a href="message_Logout_5.html" target="main">Logout&nbsp;(5)</a>
	#[serde(rename = "5")]
	AHrefMessageLogout5HtmlTargetMainLogoutNbspA,
	/// <a href="message_Indication_of_Interest_6.html" target="main">Indication of Interest&nbsp;(6)</a>
	#[serde(rename = "6")]
	AHrefMessageIndicationOfInterest6HtmlTargetMainIndicationOfInterestNbspA,
	/// <a href="message_Advertisement_7.html" target="main">Advertisement&nbsp;(7)</a>
	#[serde(rename = "7")]
	AHrefMessageAdvertisement7HtmlTargetMainAdvertisementNbspA,
	/// <a href="message_Execution_Report_8.html" target="main">Execution Report&nbsp;(8)</a>
	#[serde(rename = "8")]
	AHrefMessageExecutionReport8HtmlTargetMainExecutionReportNbspA,
	/// <a href="message_Order_Cancel_Reject_9.html" target="main">Order Cancel Reject&nbsp;(9)</a>
	#[serde(rename = "9")]
	AHrefMessageOrderCancelReject9HtmlTargetMainOrderCancelRejectNbspA,
	/// <a href="message_Logon_A.html" target="main">Logon&nbsp;(A)</a>
	#[serde(rename = "A")]
	AHrefMessageLogonAHtmlTargetMainLogonNbspA,
	/// <a href="message_News_B.html" target="main">News&nbsp;(B)</a>
	#[serde(rename = "B")]
	AHrefMessageNewsBHtmlTargetMainNewsNbspA,
	/// <a href="message_Email_C.html" target="main">Email&nbsp;(C)</a>
	#[serde(rename = "C")]
	AHrefMessageEmailCHtmlTargetMainEmailNbspA,
	/// <a href="message_New_Order_Single_D.html" target="main">New Order - Single&nbsp;(D)</a>
	#[serde(rename = "D")]
	AHrefMessageNewOrderSingleDHtmlTargetMainNewOrderSingleNbspA,
	/// <a href="message_New_Order_List_E.html" target="main">New Order - List&nbsp;(E)</a>
	#[serde(rename = "E")]
	AHrefMessageNewOrderListEHtmlTargetMainNewOrderListNbspA,
	/// <a href="message_Order_Cancel_Request_F.html" target="main">Order Cancel Request&nbsp;(F)</a>
	#[serde(rename = "F")]
	AHrefMessageOrderCancelRequestFHtmlTargetMainOrderCancelRequestNbspA,
	/// <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a>
	#[serde(rename = "G")]
	AHrefMessageOrderCancelReplaceRequestGHtmlTargetMainOrderCancelReplaceRequestNbspA,
	/// <a href="message_Order_Status_Request_H.html" target="main">Order Status Request&nbsp;(H)</a>
	#[serde(rename = "H")]
	AHrefMessageOrderStatusRequestHHtmlTargetMainOrderStatusRequestNbspA,
	/// <a href="message_Allocation_Instruction_J.html" target="main">Allocation&nbsp;(J)</a>
	#[serde(rename = "J")]
	AHrefMessageAllocationInstructionJHtmlTargetMainAllocationNbspA,
	/// <a href="message_List_Cancel_Request_K.html" target="main">List Cancel Request&nbsp;(K)</a>
	#[serde(rename = "K")]
	AHrefMessageListCancelRequestKHtmlTargetMainListCancelRequestNbspA,
	/// <a href="message_List_Execute_L.html" target="main">List Execute&nbsp;(L)</a>
	#[serde(rename = "L")]
	AHrefMessageListExecuteLHtmlTargetMainListExecuteNbspA,
	/// <a href="message_List_Status_Request_M.html" target="main">List Status Request&nbsp;(M)</a>
	#[serde(rename = "M")]
	AHrefMessageListStatusRequestMHtmlTargetMainListStatusRequestNbspA,
	/// <a href="message_List_Status_N.html" target="main">List Status&nbsp;(N)</a>
	#[serde(rename = "N")]
	AHrefMessageListStatusNHtmlTargetMainListStatusNbspA,
	/// <a href="message_Allocation_Instruction_Ack_P.html" target="main">Allocation ACK&nbsp;(P)</a>
	#[serde(rename = "P")]
	AHrefMessageAllocationInstructionAckPHtmlTargetMainAllocationAckNbspA,
	/// <a href="message_Dont_Know_Trade_Q.html" target="main">Don't Know Trade&nbsp;(Q)</a>
	#[serde(rename = "Q")]
	AHrefMessageDontKnowTradeQHtmlTargetMainDonTKnowTradeNbspA,
	/// <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a>
	#[serde(rename = "R")]
	AHrefMessageQuoteRequestRHtmlTargetMainQuoteRequestNbspA,
	/// <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a>
	#[serde(rename = "S")]
	AHrefMessageQuoteSHtmlTargetMainQuoteNbspA,
	/// <a href="message_Settlement_Instructions_T.html" target="main">Settlement Instructions&nbsp;(T)</a>
	#[serde(rename = "T")]
	AHrefMessageSettlementInstructionsTHtmlTargetMainSettlementInstructionsNbspA,
	/// <a href="message_Market_Data_Request_V.html" target="main">Market Data Request&nbsp;(V)</a>
	#[serde(rename = "V")]
	AHrefMessageMarketDataRequestVHtmlTargetMainMarketDataRequestNbspA,
	/// <a href="message_Market_Data_Snapshot_Full_Refresh_W.html" target="main">Market Data - Snapshot/Full Refresh&nbsp;(W)</a>
	#[serde(rename = "W")]
	AHrefMessageMarketDataSnapshotFullRefreshWHtmlTargetMainMarketDataSnapshotFullRefreshNbspA,
	/// <a href="message_Market_Data_Incremental_Refresh_X.html" target="main">Market Data - Incremental Refresh&nbsp;(X)</a>
	#[serde(rename = "X")]
	AHrefMessageMarketDataIncrementalRefreshXHtmlTargetMainMarketDataIncrementalRefreshNbspA,
	/// <a href="message_Market_Data_Request_Reject_Y.html" target="main">Market Data Request Reject&nbsp;(Y)</a>
	#[serde(rename = "Y")]
	AHrefMessageMarketDataRequestRejectYHtmlTargetMainMarketDataRequestRejectNbspA,
	/// <a href="message_Quote_Cancel_Z.html" target="main">Quote Cancel&nbsp;(Z)</a>
	#[serde(rename = "Z")]
	AHrefMessageQuoteCancelZHtmlTargetMainQuoteCancelNbspA,
	/// <a href="message_Quote_Status_Request_a.html" target="main">Quote Status Request&nbsp;(a)</a>
	#[serde(rename = "a")]
	AHrefMessageQuoteStatusRequestAHtmlTargetMainQuoteStatusRequestNbspA,
	/// <a href="message_Mass_Quote_Acknowledgement_b.html" target="main">Quote Acknowledgement&nbsp;(b)</a>
	#[serde(rename = "b")]
	AHrefMessageMassQuoteAcknowledgementBHtmlTargetMainQuoteAcknowledgementNbspA,
	/// <a href="message_Security_Definition_Request_c.html" target="main">Security Definition Request&nbsp;(c)</a>
	#[serde(rename = "c")]
	AHrefMessageSecurityDefinitionRequestCHtmlTargetMainSecurityDefinitionRequestNbspA,
	/// <a href="message_Security_Definition_d.html" target="main">Security Definition&nbsp;(d)</a>
	#[serde(rename = "d")]
	AHrefMessageSecurityDefinitionDHtmlTargetMainSecurityDefinitionNbspA,
	/// <a href="message_Security_Status_Request_e.html" target="main">Security Status Request&nbsp;(e)</a>
	#[serde(rename = "e")]
	AHrefMessageSecurityStatusRequestEHtmlTargetMainSecurityStatusRequestNbspA,
	/// <a href="message_Security_Status_f.html" target="main">Security Status&nbsp;(f)</a>
	#[serde(rename = "f")]
	AHrefMessageSecurityStatusFHtmlTargetMainSecurityStatusNbspA,
	/// <a href="message_Trading_Session_Status_Request_g.html" target="main">Trading Session Status Request&nbsp;(g)</a>
	#[serde(rename = "g")]
	AHrefMessageTradingSessionStatusRequestGHtmlTargetMainTradingSessionStatusRequestNbspA,
	/// <a href="message_Trading_Session_Status_h.html" target="main">Trading Session Status&nbsp;(h)</a>
	#[serde(rename = "h")]
	AHrefMessageTradingSessionStatusHHtmlTargetMainTradingSessionStatusNbspA,
	/// <a href="message_Mass_Quote_i.html" target="main">Mass Quote&nbsp;(i)</a>
	#[serde(rename = "i")]
	AHrefMessageMassQuoteIHtmlTargetMainMassQuoteNbspA,
	/// <a href="message_Business_Message_Reject_j.html" target="main">Business Message Reject&nbsp;(j)</a>
	#[serde(rename = "j")]
	AHrefMessageBusinessMessageRejectJHtmlTargetMainBusinessMessageRejectNbspA,
	/// <a href="message_Bid_Request_k.html" target="main">Bid Request&nbsp;(k)</a>
	#[serde(rename = "k")]
	AHrefMessageBidRequestKHtmlTargetMainBidRequestNbspA,
	/// <a href="message_Bid_Response_l.html" target="main">Bid Response&nbsp;(l)</a>
	#[serde(rename = "l")]
	AHrefMessageBidResponseLHtmlTargetMainBidResponseNbspA,
	/// <a href="message_List_Strike_Price_m.html" target="main">List Strike Price&nbsp;(m)</a>
	#[serde(rename = "m")]
	AHrefMessageListStrikePriceMHtmlTargetMainListStrikePriceNbspA,
	/// <a href="message_XML_message_n.html" target="main">XML message&nbsp;(n)</a>
	#[serde(rename = "n")]
	AHrefMessageXmlMessageNHtmlTargetMainXmlMessageNbspA,
	/// <a href="message_Registration_Instructions_o.html" target="main">Registration Instructions&nbsp;(o)</a>
	#[serde(rename = "o")]
	AHrefMessageRegistrationInstructionsOHtmlTargetMainRegistrationInstructionsNbspA,
	/// <a href="message_Registration_Instructions_Response_p.html" target="main">Registration Instructions Response&nbsp;(p)</a>
	#[serde(rename = "p")]
	AHrefMessageRegistrationInstructionsResponsePHtmlTargetMainRegistrationInstructionsResponseNbspA,
	/// <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a>
	#[serde(rename = "q")]
	AHrefMessageOrderMassCancelRequestQHtmlTargetMainOrderMassCancelRequestNbspA,
	/// <a href="message_Order_Mass_Cancel_Report_r.html" target="main">Order Mass Cancel Report&nbsp;(r)</a>
	#[serde(rename = "r")]
	AHrefMessageOrderMassCancelReportRHtmlTargetMainOrderMassCancelReportNbspA,
	/// <a href="message_New_Order_Cross_s.html" target="main">New Order - Cross&nbsp;(s)</a>
	#[serde(rename = "s")]
	AHrefMessageNewOrderCrossSHtmlTargetMainNewOrderCrossNbspA,
	/// <a href="message_Cross_Order_Cancel_Replace_Request_t.html" target="main">Cross Order Cancel/Replace Request&nbsp;(t)</a>
	#[serde(rename = "t")]
	AHrefMessageCrossOrderCancelReplaceRequestTHtmlTargetMainCrossOrderCancelReplaceRequestNbspA,
	/// <a href="message_Cross_Order_Cancel_Request_u.html" target="main">Cross Order Cancel Request&nbsp;(u)</a>
	#[serde(rename = "u")]
	AHrefMessageCrossOrderCancelRequestUHtmlTargetMainCrossOrderCancelRequestNbspA,
	/// <a href="message_Security_Type_Request_v.html" target="main">Security Type Request&nbsp;(v)</a>
	#[serde(rename = "v")]
	AHrefMessageSecurityTypeRequestVHtmlTargetMainSecurityTypeRequestNbspA,
	/// <a href="message_Security_Types_w.html" target="main">Security Types&nbsp;(w)</a>
	#[serde(rename = "w")]
	AHrefMessageSecurityTypesWHtmlTargetMainSecurityTypesNbspA,
	/// <a href="message_Security_List_Request_x.html" target="main">Security List Request&nbsp;(x)</a>
	#[serde(rename = "x")]
	AHrefMessageSecurityListRequestXHtmlTargetMainSecurityListRequestNbspA,
	/// <a href="message_Security_List_y.html" target="main">Security List&nbsp;(y)</a>
	#[serde(rename = "y")]
	AHrefMessageSecurityListYHtmlTargetMainSecurityListNbspA,
	/// <a href="message_Derivative_Security_List_Request_z.html" target="main">Derivative Security List Request&nbsp;(z)</a>
	#[serde(rename = "z")]
	AHrefMessageDerivativeSecurityListRequestZHtmlTargetMainDerivativeSecurityListRequestNbspA,
	/// <a href="message_Derivative_Security_List_AA.html" target="main">Derivative Security List&nbsp;(AA)</a>
	#[serde(rename = "AA")]
	AHrefMessageDerivativeSecurityListAaHtmlTargetMainDerivativeSecurityListNbspA,
	/// <a href="message_New_Order_Multileg_AB.html" target="main">New Order - Multileg&nbsp;(AB)</a>
	#[serde(rename = "AB")]
	AHrefMessageNewOrderMultilegAbHtmlTargetMainNewOrderMultilegNbspA,
	/// <a href="message_Multileg_Order_Cancel_Replace_Request_AC.html" target="main">Multileg Order Cancel/Replace Request&nbsp;(AC)</a>
	#[serde(rename = "AC")]
	AHrefMessageMultilegOrderCancelReplaceRequestAcHtmlTargetMainMultilegOrderCancelReplaceRequestNbspA,
	/// <a href="message_Trade_Capture_Report_Request_AD.html" target="main">Trade Capture Report Request&nbsp;(AD)</a>
	#[serde(rename = "AD")]
	AHrefMessageTradeCaptureReportRequestAdHtmlTargetMainTradeCaptureReportRequestNbspA,
	/// <a href="message_Trade_Capture_Report_AE.html" target="main">Trade Capture Report&nbsp;(AE)</a>
	#[serde(rename = "AE")]
	AHrefMessageTradeCaptureReportAeHtmlTargetMainTradeCaptureReportNbspA,
	/// <a href="message_Order_Mass_Status_Request_AF.html" target="main">Order Mass Status Request&nbsp;(AF)</a>
	#[serde(rename = "AF")]
	AHrefMessageOrderMassStatusRequestAfHtmlTargetMainOrderMassStatusRequestNbspA,
	/// <a href="message_Quote_Request_Reject_AG.html" target="main">Quote Request Reject&nbsp;(AG)</a>
	#[serde(rename = "AG")]
	AHrefMessageQuoteRequestRejectAgHtmlTargetMainQuoteRequestRejectNbspA,
	/// <a href="message_RFQ_Request_AH.html" target="main">RFQ Request&nbsp;(AH)</a>
	#[serde(rename = "AH")]
	AHrefMessageRfqRequestAhHtmlTargetMainRfqRequestNbspA,
	/// <a href="message_Quote_Status_Report_AI.html" target="main">Quote Status Report&nbsp;(AI)</a>
	#[serde(rename = "AI")]
	AHrefMessageQuoteStatusReportAiHtmlTargetMainQuoteStatusReportNbspA,
	/// <a href="message_Quote_Response_AJ.html" target="main">Quote Response&nbsp;(AJ)</a>
	#[serde(rename = "AJ")]
	AHrefMessageQuoteResponseAjHtmlTargetMainQuoteResponseNbspA,
	/// <a href="message_Confirmation_AK.html" target="main">Confirmation&nbsp;(AK)</a>
	#[serde(rename = "AK")]
	AHrefMessageConfirmationAkHtmlTargetMainConfirmationNbspA,
	/// <a href="message_Position_Maintenance_Request_AL.html" target="main">Position Maintenance Request&nbsp;(AL)</a>
	#[serde(rename = "AL")]
	AHrefMessagePositionMaintenanceRequestAlHtmlTargetMainPositionMaintenanceRequestNbspA,
	/// <a href="message_Position_Maintenance_Report_AM.html" target="main">Position Maintenance Report&nbsp;(AM)</a>
	#[serde(rename = "AM")]
	AHrefMessagePositionMaintenanceReportAmHtmlTargetMainPositionMaintenanceReportNbspA,
	/// <a href="message_Request_for_Positions_AN.html" target="main">Request for Positions&nbsp;(AN)</a>
	#[serde(rename = "AN")]
	AHrefMessageRequestForPositionsAnHtmlTargetMainRequestForPositionsNbspA,
	/// <a href="message_Request_for_Positions_Ack_AO.html" target="main">Request for Positions Ack&nbsp;(AO)</a>
	#[serde(rename = "AO")]
	AHrefMessageRequestForPositionsAckAoHtmlTargetMainRequestForPositionsAckNbspA,
	/// <a href="message_Position_Report_AP.html" target="main">Position Report&nbsp;(AP)</a>
	#[serde(rename = "AP")]
	AHrefMessagePositionReportApHtmlTargetMainPositionReportNbspA,
	/// <a href="message_Trade_Capture_Report_Request_Ack_AQ.html" target="main">Trade Capture Report Request Ack&nbsp;(AQ)</a>
	#[serde(rename = "AQ")]
	AHrefMessageTradeCaptureReportRequestAckAqHtmlTargetMainTradeCaptureReportRequestAckNbspA,
	/// <a href="message_Trade_Capture_Report_Ack_AR.html" target="main">Trade Capture Report Ack&nbsp;(AR)</a>
	#[serde(rename = "AR")]
	AHrefMessageTradeCaptureReportAckArHtmlTargetMainTradeCaptureReportAckNbspA,
	/// <a href="message_Allocation_Report_AS.html" target="main">Allocation Report&nbsp;(AS)</a>
	#[serde(rename = "AS")]
	AHrefMessageAllocationReportAsHtmlTargetMainAllocationReportNbspA,
	/// <a href="message_Allocation_Report_Ack_AT.html" target="main">Allocation Report Ack&nbsp;(AT)</a>
	#[serde(rename = "AT")]
	AHrefMessageAllocationReportAckAtHtmlTargetMainAllocationReportAckNbspA,
	/// <a href="message_Confirmation_Ack_AU.html" target="main">Confirmation Ack&nbsp;(AU)</a>
	#[serde(rename = "AU")]
	AHrefMessageConfirmationAckAuHtmlTargetMainConfirmationAckNbspA,
	/// <a href="message_Settlement_Instruction_Request_AV.html" target="main">Settlement Instruction Request&nbsp;(AV)</a>
	#[serde(rename = "AV")]
	AHrefMessageSettlementInstructionRequestAvHtmlTargetMainSettlementInstructionRequestNbspA,
	/// <a href="message_Assignment_Report_AW.html" target="main">Assignment Report&nbsp;(AW)</a>
	#[serde(rename = "AW")]
	AHrefMessageAssignmentReportAwHtmlTargetMainAssignmentReportNbspA,
	/// <a href="message_Collateral_Request_AX.html" target="main">Collateral Request&nbsp;(AX)</a>
	#[serde(rename = "AX")]
	AHrefMessageCollateralRequestAxHtmlTargetMainCollateralRequestNbspA,
	/// <a href="message_Collateral_Assignment_AY.html" target="main">Collateral Assignment&nbsp;(AY)</a>
	#[serde(rename = "AY")]
	AHrefMessageCollateralAssignmentAyHtmlTargetMainCollateralAssignmentNbspA,
	/// <a href="message_Collateral_Response_AZ.html" target="main">Collateral Response&nbsp;(AZ)</a>
	#[serde(rename = "AZ")]
	AHrefMessageCollateralResponseAzHtmlTargetMainCollateralResponseNbspA,
	/// <a href="message_Collateral_Report_BA.html" target="main">Collateral Report&nbsp;(BA)</a>
	#[serde(rename = "BA")]
	AHrefMessageCollateralReportBaHtmlTargetMainCollateralReportNbspA,
	/// <a href="message_Collateral_Inquiry_BB.html" target="main">Collateral Inquiry&nbsp;(BB)</a>
	#[serde(rename = "BB")]
	AHrefMessageCollateralInquiryBbHtmlTargetMainCollateralInquiryNbspA,
	/// <a href="message_Network_Counterparty_System_Status_Request_BC.html" target="main">Network Status Request&nbsp;(BC)</a>
	#[serde(rename = "BC")]
	AHrefMessageNetworkCounterpartySystemStatusRequestBcHtmlTargetMainNetworkStatusRequestNbspA,
	/// <a href="message_Network_Counterparty_System_Status_Response_BD.html" target="main">Network Status Response&nbsp;(BD)</a>
	#[serde(rename = "BD")]
	AHrefMessageNetworkCounterpartySystemStatusResponseBdHtmlTargetMainNetworkStatusResponseNbspA,
	/// <a href="message_User_Request_BE.html" target="main">User Request&nbsp;(BE)</a>
	#[serde(rename = "BE")]
	AHrefMessageUserRequestBeHtmlTargetMainUserRequestNbspA,
	/// <a href="message_User_Response_BF.html" target="main">User Response&nbsp;(BF)</a>
	#[serde(rename = "BF")]
	AHrefMessageUserResponseBfHtmlTargetMainUserResponseNbspA,
	/// <a href="message_Collateral_Inquiry_Ack_BG.html" target="main">Collateral Inquiry Ack&nbsp;(BG)</a>
	#[serde(rename = "BG")]
	AHrefMessageCollateralInquiryAckBgHtmlTargetMainCollateralInquiryAckNbspA,
	/// <a href="message_Confirmation_Request_BH.html" target="main">Confirmation Request&nbsp;(BH)</a>
	#[serde(rename = "BH")]
	AHrefMessageConfirmationRequestBhHtmlTargetMainConfirmationRequestNbspA,
}

impl Default for MsgType {
	fn default() -> Self {
		MsgType::AHrefMessageHeartbeat0HtmlTargetMainHeartbeatNbspA
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PossDupFlag {
	/// Possible duplicate
	#[serde(rename = "Y")]
	PossibleDuplicate,
	/// Original transmission
	#[serde(rename = "N")]
	OriginalTransmission,
}

impl Default for PossDupFlag {
	fn default() -> Self {
		PossDupFlag::PossibleDuplicate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PossResend {
	/// Possible resend
	#[serde(rename = "Y")]
	PossibleResend,
	/// Original transmission
	#[serde(rename = "N")]
	OriginalTransmission,
}

impl Default for PossResend {
	fn default() -> Self {
		PossResend::PossibleResend
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MessageEncoding {
	/// JIS
	#[serde(rename = "ISO-2022-JP")]
	Jis,
	/// EUC
	#[serde(rename = "EUC-JP")]
	Euc,
	/// SJIS
	#[serde(rename = "Shift_JIS")]
	Sjis,
	/// Unicode
	#[serde(rename = "UTF-8")]
	Unicode,
}

impl Default for MessageEncoding {
	fn default() -> Self {
		MessageEncoding::Jis
	}
}
