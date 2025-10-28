// Auto-generated from TypeScript definitions
// Source: types/response/v5-p2p-trading.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::v5_p2p_trading::P2PTradingPreferenceSetV5;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PCoinBalanceV5 {
    pub coin: String,
    pub transferBalance: String,
    pub walletBalance: String,
    pub bonus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAccountCoinsBalanceV5 {
    pub memberId: String,
    pub accountType: String,
    pub balance: Vec<P2PCoinBalanceV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POnlineAdV5 {
    pub id: String,
    pub nickName: String,
    pub tokenId: String,
    pub currencyId: String,
    pub side: String,
    pub price: String,
    pub lastQuantity: String,
    pub minAmount: String,
    pub maxAmount: String,
    pub payments: Vec<String>,
    pub recentOrderNum: f64,
    pub recentExecuteRate: f64,
    pub isOnline: bool,
    pub authTag: Vec<String>,
    pub paymentPeriod: f64,
    pub accountId: f64,
    pub userId: f64,
    pub priceType: f64,
    pub premium: String,
    pub quantity: String,
    pub frozenQuantity: String,
    pub executedQuantity: String,
    pub remark: String,
    pub status: f64,
    pub createDate: String,
    pub orderNum: String,
    pub finishNum: String,
    pub fee: String,
    pub lastLogoutTime: String,
    pub blocked: String,
    pub makerContact: bool,
    pub symbolInfo: P2POnlineAdV5_SymbolInfo,
    pub tradingPreferenceSet: P2POnlineAdV5_TradingPreferenceSet,
    pub version: f64,
    pub authStatus: f64,
    pub recommend: bool,
    pub recommendTag: String,
    pub userType: String,
    pub itemType: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POnlineAdsResponseV5 {
    pub count: f64,
    pub items: Vec<P2POnlineAdV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PCreateAdResponseV5 {
    pub itemId: String,
    pub securityRiskToken: String,
    pub riskTokenType: String,
    pub riskVersion: String,
    pub needSecurityRisk: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PPaymentTermV5 {
    pub id: String,
    pub realName: String,
    pub paymentType: f64,
    pub bankName: String,
    pub branchName: String,
    pub accountNo: String,
    pub qrcode: String,
    pub visible: f64,
    pub payMessage: String,
    pub firstName: String,
    pub lastName: String,
    pub secondLastName: String,
    pub clabe: String,
    pub debitCardNumber: String,
    pub mobile: String,
    pub businessName: String,
    pub concept: String,
    pub paymentExt1: String,
    pub paymentExt2: String,
    pub paymentExt3: String,
    pub paymentExt4: String,
    pub paymentExt5: String,
    pub paymentExt6: String,
    pub paymentTemplateVersion: f64,
    pub paymentConfig: P2PPaymentTermV5_PaymentConfig,
    pub realNameVerified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAdDetailV5 {
    pub id: String,
    pub accountId: String,
    pub userId: String,
    pub nickName: String,
    pub tokenId: String,
    pub tokenName: String,
    pub currencyId: String,
    pub side: f64,
    pub priceType: f64,
    pub price: String,
    pub premium: String,
    pub lastQuantity: String,
    pub quantity: String,
    pub frozenQuantity: String,
    pub executedQuantity: String,
    pub minAmount: String,
    pub maxAmount: String,
    pub remark: String,
    pub status: f64,
    pub createDate: String,
    pub payments: Vec<String>,
    pub orderNum: f64,
    pub finishNum: f64,
    pub recentOrderNum: f64,
    pub recentExecuteRate: f64,
    pub fee: String,
    pub isOnline: bool,
    pub lastLogoutTime: String,
    pub symbolInfo: P2PAdDetailV5_SymbolInfo,
    pub tradingPreferenceSet: P2PTradingPreferenceSetV5,
    pub paymentTerms: Vec<P2PPaymentTermV5>,
    pub version: f64,
    pub updateDate: String,
    pub feeRate: String,
    pub paymentPeriod: f64,
    pub itemType: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PPersonalAdsResponseV5 {
    pub count: f64,
    pub items: Vec<P2PAdDetailV5>,
    pub hiddenFlag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POrderExtensionV5 {
    pub isDelayWithdraw: bool,
    pub delayTime: String,
    pub startTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POrderV5 {
    pub id: String,
    pub side: f64,
    pub tokenId: String,
    pub orderType: String,
    pub amount: String,
    pub currencyId: String,
    pub price: String,
    pub notifyTokenQuantity: Option<String>,
    pub notifyTokenId: Option<String>,
    pub fee: String,
    pub targetNickName: String,
    pub targetUserId: String,
    pub status: f64,
    pub selfUnreadMsgCount: String,
    pub createDate: String,
    pub transferLastSeconds: String,
    pub appealLastSeconds: String,
    pub userId: String,
    pub sellerRealName: String,
    pub buyerRealName: String,
    pub judgeInfo: P2POrderV5_JudgeInfo,
    pub unreadMsgCount: String,
    pub extension: P2POrderExtensionV5,
    pub bulkOrderFlag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POrdersResponseV5 {
    pub count: f64,
    pub items: Vec<P2POrderV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PPaymentConfigItemV5 {
    pub view: bool,
    pub name: String,
    pub label: String,
    pub placeholder: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub maxLength: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PPaymentConfigV5 {
    pub paymentType: String,
    pub checkType: f64,
    pub sort: f64,
    pub paymentName: String,
    pub addTips: String,
    pub itemTips: String,
    pub online: f64,
    pub items: Vec<P2PPaymentConfigItemV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PPaymentTermDetailV5 {
    pub id: String,
    pub realName: String,
    pub paymentType: f64,
    pub bankName: String,
    pub branchName: String,
    pub accountNo: String,
    pub qrcode: String,
    pub visible: f64,
    pub payMessage: String,
    pub firstName: String,
    pub lastName: String,
    pub secondLastName: String,
    pub clabe: String,
    pub debitCardNumber: String,
    pub mobile: String,
    pub businessName: String,
    pub concept: String,
    pub online: String,
    pub paymentExt1: String,
    pub paymentExt2: String,
    pub paymentExt3: String,
    pub paymentExt4: String,
    pub paymentExt5: String,
    pub paymentExt6: String,
    pub paymentTemplateVersion: f64,
    pub paymentConfigVo: P2PPaymentConfigV5,
    pub ruPaymentPrompt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAppraiseInfoV5 {
    pub anonymous: String,
    pub appraiseContent: String,
    pub appraiseId: String,
    pub appraiseType: String,
    pub modifyFlag: String,
    pub updateDate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PJudgeInfoV5 {
    pub autoJudgeUnlockTime: String,
    pub dissentResult: String,
    pub preDissent: String,
    pub postDissent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POrderDetailV5 {
    pub id: String,
    pub side: f64,
    pub itemId: String,
    pub accountId: String,
    pub userId: String,
    pub nickName: String,
    pub makerUserId: String,
    pub targetAccountId: String,
    pub targetUserId: String,
    pub targetNickName: String,
    pub targetFirstName: String,
    pub targetSecondName: String,
    pub targetUserAuthStatus: f64,
    pub targetConnectInformation: String,
    pub payerRealName: String,
    pub sellerRealName: String,
    pub buyerRealName: String,
    pub tokenId: String,
    pub tokenName: String,
    pub currencyId: String,
    pub price: String,
    pub quantity: String,
    pub amount: String,
    pub payCode: String,
    pub paymentType: f64,
    pub transferDate: String,
    pub status: f64,
    pub createDate: String,
    pub paymentTermList: Vec<P2PPaymentTermDetailV5>,
    pub remark: String,
    pub transferLastSeconds: String,
    pub recentOrderNum: f64,
    pub recentExecuteRate: f64,
    pub appealLastSeconds: String,
    pub appealContent: String,
    pub appealType: f64,
    pub appealNickName: String,
    pub canAppeal: String,
    pub totalAppealTimes: String,
    pub appealedTimes: String,
    pub paymentTermResult: P2PPaymentTermDetailV5,
    pub orderFinishMinute: f64,
    pub confirmedPayTerm: P2PPaymentTermDetailV5,
    pub makerFee: String,
    pub takerFee: String,
    pub fee: String,
    pub showContact: bool,
    pub tokenBalance: String,
    pub fiatBalance: String,
    pub unreadMsgCount: String,
    pub updateDate: String,
    pub extension: P2POrderExtensionV5,
    pub selfUnreadMsgCount: String,
    pub judgeType: String,
    pub canReport: bool,
    pub canReportDisagree: bool,
    pub canReportType: Vec<String>,
    pub canReportDisagreeType: Vec<String>,
    pub appraiseStatus: String,
    pub appraiseInfo: P2PAppraiseInfoV5,
    pub canReportDisagreeTypes: Vec<String>,
    pub canReportTypes: Vec<String>,
    pub orderType: String,
    pub middleToken: String,
    pub beforePrice: String,
    pub beforeQuantity: String,
    pub beforeToken: String,
    pub alternative: String,
    pub appealUserId: String,
    pub notifyTokenId: String,
    pub notifyTokenQuantity: String,
    pub cancelResponsible: String,
    pub chainType: String,
    pub chainAddress: String,
    pub tradeHashCode: String,
    pub estimatedGasFee: String,
    pub gasFeeTokenId: String,
    pub tradingFeeTokenId: String,
    pub onChainInfo: String,
    pub transactionId: String,
    pub displayRefund: String,
    pub chainWithdrawLastSeconds: String,
    pub chainTransferLastSeconds: String,
    pub orderSource: String,
    pub cancelReason: String,
    pub sellerCancelExamineRemainTime: String,
    pub needSellerExamineCancel: bool,
    pub couponCurrencyAmount: String,
    pub totalCurrencyAmount: String,
    pub usedCoupon: bool,
    pub couponTokenId: String,
    pub couponQuantity: String,
    pub completedOrderAppealCount: f64,
    pub totalCompletedOrderAppealCount: f64,
    pub realOrderStatus: f64,
    pub appealVersion: f64,
    pub judgeInfo: P2PJudgeInfoV5,
    pub helpType: String,
    pub appealFlowStatus: String,
    pub appealSubStatus: String,
    pub bulkOrderFlag: bool,
    pub targetUserType: String,
    pub targetUserDisplays: Vec<String>,
    pub appealProcessChangeFlag: bool,
    pub appealNegotiationNode: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POrderMessageV5 {
    pub id: String,
    pub message: String,
    pub userId: String,
    pub msgType: f64,
    pub msgCode: f64,
    pub createDate: String,
    pub contentType: String,
    pub orderId: String,
    pub msgUuid: String,
    pub nickName: String,
    pub fileName: String,
    pub accountId: String,
    pub isRead: f64,
    pub read: f64,
    pub roleType: String,
    pub onlyForCustomer: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PUserInfoV5 {
    pub nickName: String,
    pub defaultNickName: bool,
    pub isOnline: bool,
    pub kycLevel: String,
    pub email: String,
    pub mobile: String,
    pub lastLogoutTime: String,
    pub recentRate: String,
    pub totalFinishCount: f64,
    pub totalFinishSellCount: f64,
    pub totalFinishBuyCount: f64,
    pub recentFinishCount: f64,
    pub averageReleaseTime: String,
    pub averageTransferTime: String,
    pub accountCreateDays: f64,
    pub firstTradeDays: f64,
    pub realName: String,
    pub recentTradeAmount: String,
    pub totalTradeAmount: String,
    pub registerTime: String,
    pub authStatus: f64,
    pub kycCountryCode: String,
    pub blocked: String,
    pub goodAppraiseRate: String,
    pub goodAppraiseCount: f64,
    pub badAppraiseCount: f64,
    pub accountId: f64,
    pub paymentCount: f64,
    pub contactCount: f64,
    pub vipLevel: f64,
    pub userCancelCountLimit: f64,
    pub paymentRealNameUneditable: bool,
    pub userId: String,
    pub realNameEn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PCounterpartyUserInfoV5 {
    pub nickName: String,
    pub defaultNickName: bool,
    pub whiteFlag: f64,
    pub contactConfig: bool,
    pub isOnline: bool,
    pub email: String,
    pub mobile: String,
    pub kycLevel: f64,
    pub lastLogoutTime: String,
    pub recentRate: f64,
    pub totalFinishCount: f64,
    pub totalFinishSellCount: f64,
    pub totalFinishBuyCount: f64,
    pub recentFinishCount: f64,
    pub averageReleaseTime: String,
    pub averageTransferTime: String,
    pub accountCreateDays: f64,
    pub firstTradeDays: f64,
    pub realName: String,
    pub recentTradeAmount: String,
    pub totalTradeAmount: String,
    pub executeNum: f64,
    pub orderNum: f64,
    pub hasUnPostAd: f64,
    pub registerTime: String,
    pub authStatus: f64,
    pub kycCountryCode: String,
    pub blocked: String,
    pub goodAppraiseRate: String,
    pub goodAppraiseCount: f64,
    pub badAppraiseCount: f64,
    pub accountId: String,
    pub paymentCount: f64,
    pub contactCount: f64,
    pub realNameMask: String,
    pub vipLevel: f64,
    pub vipProfit: Vec<serde_json::Value>,
    pub userTag: Vec<serde_json::Value>,
    pub userCancelCountLimit: f64,
    pub paymentRealNameUneditable: bool,
    pub lostRoleAffected: bool,
    pub userCurPrivilege: Vec<String>,
    pub userType: String,
    pub userId: String,
    pub realNameEn: String,
    pub canSubOnline: bool,
    pub curPrivilegeInfo: Vec<serde_json::Value>,
    pub openApiSwitch: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PUserPaymentV5 {
    pub id: String,
    pub realName: String,
    pub paymentType: String,
    pub bankName: String,
    pub branchName: String,
    pub accountNo: String,
    pub qrcode: String,
    pub visible: f64,
    pub payMessage: String,
    pub firstName: String,
    pub lastName: String,
    pub secondLastName: String,
    pub clabe: String,
    pub debitCardNumber: String,
    pub mobile: String,
    pub businessName: String,
    pub concept: String,
    pub online: String,
    pub countNo: String,
    pub paymentExt1: String,
    pub paymentExt2: String,
    pub paymentExt3: String,
    pub paymentExt4: String,
    pub paymentExt5: String,
    pub paymentExt6: String,
    pub paymentTemplateVersion: f64,
    pub hasPaymentTemplateChanged: bool,
    pub paymentConfigVo: P2PPaymentConfigV5,
    pub realNameVerified: bool,
    pub channel: String,
    pub currencyBalance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POnlineAdV5_SymbolInfo_Currency {
    pub id: String,
    pub exchangeId: String,
    pub orgId: String,
    pub currencyId: String,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POnlineAdV5_SymbolInfo_Token {
    pub id: String,
    pub exchangeId: String,
    pub orgId: String,
    pub tokenId: String,
    pub scale: f64,
    pub sequence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POnlineAdV5_SymbolInfo {
    pub id: String,
    pub exchangeId: String,
    pub orgId: String,
    pub tokenId: String,
    pub currencyId: String,
    pub status: String,
    pub lowerLimitAlarm: f64,
    pub upperLimitAlarm: f64,
    pub itemDownRange: String,
    pub itemUpRange: String,
    pub currencyMinQuote: String,
    pub currencyMaxQuote: String,
    pub currencyLowerMaxQuote: String,
    pub tokenMinQuote: String,
    pub tokenMaxQuote: String,
    pub kycCurrencyLimit: String,
    pub itemSideLimit: String,
    pub buyFeeRate: String,
    pub sellFeeRate: String,
    pub orderAutoCancelMinute: f64,
    pub orderFinishMinute: f64,
    pub tradeSide: f64,
    pub currency: P2POnlineAdV5_SymbolInfo_Currency,
    pub token: P2POnlineAdV5_SymbolInfo_Token,
    pub buyAd: f64,
    pub sellAd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POnlineAdV5_TradingPreferenceSet {
    pub hasUnPostAd: f64,
    pub isKyc: f64,
    pub isEmail: f64,
    pub isMobile: f64,
    pub hasRegisterTime: f64,
    pub registerTimeThreshold: f64,
    pub orderFinishNumberDay30: f64,
    pub completeRateDay30: f64,
    pub nationalLimit: f64,
    pub hasOrderFinishNumberDay30: f64,
    pub hasCompleteRateDay30: f64,
    pub hasNationalLimit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PPaymentTermV5_PaymentConfig {
    pub paymentType: f64,
    pub paymentName: String,
    pub paymentDialect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAdDetailV5_SymbolInfo_Currency {
    pub id: String,
    pub exchangeId: String,
    pub orgId: String,
    pub currencyId: String,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAdDetailV5_SymbolInfo_Token {
    pub id: String,
    pub exchangeId: String,
    pub orgId: String,
    pub tokenId: String,
    pub scale: f64,
    pub sequence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAdDetailV5_SymbolInfo_BuyAd {
    pub paymentPeriods: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAdDetailV5_SymbolInfo_SellAd {
    pub paymentPeriods: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PAdDetailV5_SymbolInfo {
    pub id: String,
    pub exchangeId: String,
    pub orgId: String,
    pub tokenId: String,
    pub currencyId: String,
    pub status: f64,
    pub lowerLimitAlarm: f64,
    pub upperLimitAlarm: f64,
    pub itemDownRange: String,
    pub itemUpRange: String,
    pub currencyMinQuote: String,
    pub currencyMaxQuote: String,
    pub currencyLowerMaxQuote: String,
    pub tokenMinQuote: String,
    pub tokenMaxQuote: String,
    pub kycCurrencyLimit: String,
    pub itemSideLimit: f64,
    pub buyFeeRate: String,
    pub sellFeeRate: String,
    pub orderAutoCancelMinute: f64,
    pub orderFinishMinute: f64,
    pub tradeSide: f64,
    pub currency: P2PAdDetailV5_SymbolInfo_Currency,
    pub token: P2PAdDetailV5_SymbolInfo_Token,
    pub buyAd: P2PAdDetailV5_SymbolInfo_BuyAd,
    pub sellAd: P2PAdDetailV5_SymbolInfo_SellAd,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2POrderV5_JudgeInfo {
    pub autoJudgeUnlockTime: String,
    pub dissentResult: String,
    pub preDissent: String,
    pub postDissent: String,
}

