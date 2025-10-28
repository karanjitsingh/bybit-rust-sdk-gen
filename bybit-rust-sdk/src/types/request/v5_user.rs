// Auto-generated from TypeScript definitions
// Source: types/request/v5-user.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::account_asset::inline::CreateSubMemberRequestV3_MemberType;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::PermissionsV5;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubMemberParamsV5 {
    pub username: String,
    pub password: Option<String>,
    /// 1: normal, 6: custodial
    pub memberType: CreateSubMemberRequestV3_MemberType,
    /// 0: quick login disabled (default), 1: quick login enabled
    pub switch: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub isUta: Option<bool>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubApiKeyParamsV5 {
    pub subuid: f64,
    pub note: Option<String>,
    pub readOnly: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub ips: Option<String>,
    pub permissions: PermissionsV5,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateApiKeyParamsV5 {
    pub apikey: Option<String>,
    pub readOnly: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub ips: Option<Vec<String>>,
    pub permissions: PermissionsV5,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateSubApiKeyUpdateParamsV5 {
    pub readOnly: Option<f64>,
    pub ips: Option<Vec<String>>,
    pub permissions: PermissionsV5,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteSubMemberParamsV5 {
    pub subMemberId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSubAccountAllApiKeysParamsV5 {
    pub subMemberId: String,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

