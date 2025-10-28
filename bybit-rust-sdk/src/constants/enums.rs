// Auto-generated from TypeScript definitions
// Source: constants/enum.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use strum_macros::{EnumString, Display};

/// Position idx, used to identify positions in different position modes.
/// Required if you are under One-Way Mode:
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinearPositionIdx {
    #[default]
    OneWayMode = 0,
    BuySide = 1,
    SellSide = 2,
}

