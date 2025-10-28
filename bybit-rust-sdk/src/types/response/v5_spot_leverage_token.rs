// Auto-generated from TypeScript definitions
// Source: types/response/v5-spot-leverage-token.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::LTOrderStatusV5;
use crate::types::shared_v5::LTOrderTypeV5;
use crate::types::shared_v5::LeverageTokenStatusV5;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LeverageTokenInfoV5 {
    pub ltCoin: String,
    pub ltName: String,
    pub maxPurchase: String,
    pub minPurchase: String,
    pub maxPurchaseDaily: String,
    pub maxRedeem: String,
    pub minRedeem: String,
    pub maxRedeemDaily: String,
    pub purchaseFeeRate: String,
    pub redeemFeeRate: String,
    pub ltStatus: LeverageTokenStatusV5,
    pub fundFee: String,
    pub fundFeeTime: String,
    pub manageFeeRate: String,
    pub manageFeeTime: String,
    pub value: String,
    pub netValue: String,
    pub total: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LeveragedTokenMarketResultV5 {
    pub ltCoin: String,
    pub nav: String,
    pub navTime: String,
    pub circulation: String,
    pub basket: String,
    pub leverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PurchaseSpotLeveragedTokenResultV5 {
    pub ltCoin: String,
    pub ltOrderStatus: LTOrderStatusV5,
    pub execQty: String,
    pub execAmt: String,
    pub amount: String,
    pub purchaseId: String,
    pub serialNo: String,
    pub valueCoin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RedeemSpotLeveragedTokenResultV5 {
    pub ltCoin: String,
    pub ltOrderStatus: LTOrderStatusV5,
    pub quantity: String,
    pub execQty: String,
    pub execAmt: String,
    pub redeemId: String,
    pub serialNo: String,
    pub valueCoin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotLeveragedTokenOrderHistoryV5 {
    pub ltCoin: String,
    pub orderId: String,
    pub ltOrderType: LTOrderTypeV5,
    pub orderTime: f64,
    pub updateTime: f64,
    pub ltOrderStatus: LTOrderStatusV5,
    pub fee: String,
    pub amount: String,
    pub value: String,
    pub valueCoin: String,
    pub serialNo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VIPMarginDataV5 {
    pub vipCoinList: Vec<VIPMarginDataV5_VipCoinList>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotMarginStateV5 {
    pub spotLeverage: String,
    pub spotMarginMode: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub effectiveLeverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VIPMarginDataV5_VipCoinList_List {
    pub borrowable: bool,
    pub collateralRatio: String,
    pub currency: String,
    pub hourlyBorrowRate: String,
    pub liquidationOrder: String,
    pub marginCollateral: bool,
    pub maxBorrowingAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VIPMarginDataV5_VipCoinList {
    pub list: Vec<VIPMarginDataV5_VipCoinList_List>,
    pub vipLevel: String,
}

