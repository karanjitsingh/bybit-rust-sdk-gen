// Auto-generated from TypeScript definitions
// Source: types/response/v5-market.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::GetMarkPriceKlineParamsV5_Category;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::ContractTypeV5;
use crate::types::shared_v5::CopyTradingV5;
use crate::types::shared_v5::InstrumentStatusV5;
use crate::types::shared_v5::MarginTradingV5;
use crate::types::shared_v5::OptionTypeV5;
use crate::types::shared_v5::OrderSideV5;

/// OHLCVT candle used by v5 APIs
/// - list[0]: startTime	string	Start time of the candle (ms)
/// - list[1]: openPrice	string	Open price
/// - list[2]: highPrice	string	Highest price
/// - list[3]: lowPrice	string	Lowest price
/// - list[4]: closePrice	string	Close price. Is the last traded price when the candle is not closed
/// - list[5]: volume	string	Trade volume. Unit of contract: pieces of contract. Unit of spot: quantity of coins
/// - list[6]: turnover	string	Turnover. Unit of figure: quantity of quota coin
pub type OHLCVKlineV5 = (String, String, String, String, String, String, String);

/// OHLC candle used by v5 APIs
/// - list[0]: startTime	string	Start time of the candle (ms)
/// - list[1]: openPrice	string	Open price
/// - list[2]: highPrice	string	Highest price
/// - list[3]: lowPrice	string	Lowest price
/// - list[4]: closePrice	string	Close price. Is the last traded price when the candle is not closed
pub type OHLCKlineV5 = (String, String, String, String, String);

/// [price, size]
pub type OrderbookLevelV5 = (String, String);

/// RPI Orderbook level: [price, nonRpiSize, rpiSize]
pub type RPIOrderbookLevelV5 = (String, String, String);

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InstrumentInfoV5Mapping {
    pub linear: Vec<LinearInverseInstrumentInfoV5>,
    pub inverse: Vec<LinearInverseInstrumentInfoV5>,
    pub option: Vec<OptionInstrumentInfoV5>,
    pub spot: Vec<SpotInstrumentInfoV5>,
}

/// - openInterest	string	Open interest
/// - timestamp	string	The timestamp (ms)
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OpenInterestV5 {
    pub openInterest: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5 {
    pub symbol: String,
    pub contractType: ContractTypeV5,
    pub status: InstrumentStatusV5,
    pub baseCoin: String,
    pub quoteCoin: String,
    /// The region to which the trading pair belongs
    pub symbolType: String,
    pub launchTime: String,
    pub deliveryTime: Option<String>,
    pub deliveryFeeRate: Option<String>,
    pub priceScale: String,
    pub leverageFilter: LinearInverseInstrumentInfoV5_LeverageFilter,
    pub priceFilter: LinearInverseInstrumentInfoV5_PriceFilter,
    pub lotSizeFilter: LinearInverseInstrumentInfoV5_LotSizeFilter,
    pub unifiedMarginTrade: bool,
    pub fundingInterval: f64,
    pub settleCoin: String,
    pub copyTrading: CopyTradingV5,
    pub upperFundingRate: String,
    pub lowerFundingRate: String,
    pub riskParameters: LinearInverseInstrumentInfoV5_RiskParameters,
    pub isPreListing: bool,
    pub preListingInfo: Option<LinearInverseInstrumentInfoV5_PreListingInfo>,
    pub displayName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OptionInstrumentInfoV5 {
    pub symbol: String,
    pub optionsType: OptionTypeV5,
    pub status: InstrumentStatusV5,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub settleCoin: String,
    /// The region to which the trading pair belongs
    pub symbolType: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub deliveryFeeRate: String,
    pub priceFilter: OptionInstrumentInfoV5_PriceFilter,
    pub lotSizeFilter: OptionInstrumentInfoV5_LotSizeFilter,
    pub displayName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotInstrumentInfoV5 {
    pub symbol: String,
    pub baseCoin: String,
    pub quoteCoin: String,
    /// The region to which the trading pair belongs
    pub symbolType: String,
    /// Deprecated, always 0
    pub innovation: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub status: InstrumentStatusV5,
    pub marginTrading: MarginTradingV5,
    pub stTag: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub lotSizeFilter: SpotInstrumentInfoV5_LotSizeFilter,
    pub priceFilter: SpotInstrumentInfoV5_PriceFilter,
    pub riskParameters: SpotInstrumentInfoV5_RiskParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OrderbookResponseV5 {
    pub s: String,
    pub b: Vec<(String, String)>,
    pub a: Vec<(String, String)>,
    pub ts: f64,
    pub u: f64,
    pub seq: f64,
    pub cts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RPIOrderbookResponseV5 {
    /// Symbol name
    pub s: String,
    /// Bids. Sorted by price in descending order
    pub b: Vec<(String, String, String)>,
    /// Asks. Sorted by price in ascending order
    pub a: Vec<(String, String, String)>,
    /// The timestamp (ms) that the system generates the data
    pub ts: f64,
    /// Update ID, is always in sequence corresponds to u in the 50-level WebSocket RPI orderbook stream
    pub u: f64,
    /// Cross sequence
    pub seq: f64,
    /// The timestamp from the matching engine when this orderbook data is produced
    pub cts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TickerLinearInverseV5 {
    pub symbol: String,
    pub lastPrice: String,
    pub indexPrice: String,
    pub markPrice: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub openInterest: String,
    pub openInterestValue: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub fundingRate: String,
    pub nextFundingTime: String,
    pub predictedDeliveryPrice: String,
    pub basisRate: String,
    pub deliveryFeeRate: String,
    pub deliveryTime: String,
    pub ask1Size: String,
    pub bid1Price: String,
    pub ask1Price: String,
    pub bid1Size: String,
    pub preOpenPrice: String,
    pub preQty: String,
    pub curPreListingPhase: String,
    pub basis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TickerOptionV5 {
    pub symbol: String,
    pub bid1Price: String,
    pub bid1Size: String,
    pub bid1Iv: String,
    pub ask1Price: String,
    pub ask1Size: String,
    pub ask1Iv: String,
    pub lastPrice: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub markPrice: String,
    pub indexPrice: String,
    pub markIv: String,
    pub underlyingPrice: String,
    pub openInterest: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub totalVolume: String,
    pub totalTurnover: String,
    pub delta: String,
    pub gamma: String,
    pub vega: String,
    pub theta: String,
    pub predictedDeliveryPrice: String,
    pub change24h: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TickerSpotV5 {
    pub symbol: String,
    pub bid1Price: String,
    pub bid1Size: String,
    pub ask1Price: String,
    pub ask1Size: String,
    pub lastPrice: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub usdIndexPrice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FundingRateHistoryResponseV5 {
    pub symbol: String,
    pub fundingRate: String,
    pub fundingRateTimestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PublicTradeV5 {
    pub execId: String,
    pub symbol: String,
    pub price: String,
    pub size: String,
    pub side: OrderSideV5,
    pub time: String,
    pub isBlockTrade: bool,
    pub isRPITrade: bool,
    pub mP: Option<String>,
    pub iP: Option<String>,
    pub mIv: Option<String>,
    pub iv: Option<String>,
    pub seq: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OpenInterestResponseV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub list: Vec<OpenInterestResponseV5_List>,
    pub nextPageCursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct HistoricalVolatilityV5 {
    pub period: f64,
    pub value: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InsuranceDataV5 {
    pub coin: String,
    pub symbols: String,
    pub balance: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InsuranceResponseV5 {
    pub updatedTime: String,
    pub list: Vec<InsuranceDataV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RiskLimitV5 {
    pub id: f64,
    pub symbol: String,
    pub riskLimitValue: String,
    pub maintenanceMargin: f64,
    pub initialMargin: f64,
    pub section: serde_json::Value,
    pub isLowestRisk: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub maxLeverage: String,
    pub mmDeduction: String,
    pub nextPageCursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OptionDeliveryPriceV5 {
    pub symbol: String,
    pub deliveryPrice: String,
    pub deliveryTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DeliveryPriceV5 {
    pub symbol: String,
    pub deliveryPrice: String,
    pub deliveryTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LongShortRatioV5 {
    pub symbol: String,
    pub buyRatio: String,
    pub sellRatio: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OrderPriceLimitV5 {
    pub symbol: String,
    pub buyLmt: String,
    pub sellLmt: String,
    pub ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct IndexPriceComponentV5 {
    /// Name of the exchange
    pub exchange: String,
    /// Spot trading pair on the exchange (e.g., BTCUSDT)
    pub spotPair: String,
    /// Equivalent price
    pub equivalentPrice: String,
    /// Multiplier used for the component price
    pub multiplier: String,
    /// Actual price
    pub price: String,
    /// Weight in the index calculation
    pub weight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct IndexPriceComponentsResponseV5 {
    /// Name of the index (e.g., BTCUSDT)
    pub indexName: String,
    /// Last price of the index
    pub lastPrice: String,
    /// Timestamp of the last update in milliseconds
    pub updateTime: String,
    /// List of components contributing to the index price
    pub components: Vec<IndexPriceComponentV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ADLAlertItemV5 {
    /// Token of the insurance pool
    pub coin: String,
    /// Trading pair name
    pub symbol: String,
    /// Balance of the insurance fund. Used to determine if ADL is triggered
    pub balance: String,
    /// Maximum balance of the insurance pool in the last 8 hours
    pub maxBalance: String,
    /// PnL ratio threshold for triggering contract PnL drawdown ADL
    pub insurancePnlRatio: String,
    /// Symbol's PnL drawdown ratio in the last 8 hours. Used to determine whether ADL is triggered or stopped
    pub pnlRatio: String,
    /// Trigger threshold for contract PnL drawdown ADL
    pub adlTriggerThreshold: String,
    /// Stop ratio threshold for contract PnL drawdown ADL
    pub adlStopRatio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ADLAlertResponseV5 {
    /// Latest data update timestamp (ms)
    pub updateTime: String,
    /// List of ADL alert items
    pub list: Vec<ADLAlertItemV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FeeGroupLevelV5 {
    /// Pro level name or Market Maker level name
    pub level: String,
    /// Taker fee rate
    pub takerFeeRate: String,
    /// Maker fee rate
    pub makerFeeRate: String,
    /// Maker rebate fee rate
    pub makerRebate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FeeGroupRatesV5 {
    /// Pro-level fee structures
    pub pro: Vec<FeeGroupLevelV5>,
    /// Market Maker-level fee structures
    pub marketMaker: Vec<FeeGroupLevelV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FeeGroupItemV5 {
    /// Fee group name
    pub groupName: String,
    /// Group weighting factor
    pub weightingFactor: f64,
    /// Symbols number
    pub symbolsNumbers: f64,
    /// Symbol names
    pub symbols: Vec<String>,
    /// Fee rate details for different categories
    pub feeRates: FeeGroupRatesV5,
    /// Latest data update timestamp (ms)
    pub updateTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FeeGroupStructureResponseV5 {
    /// List of fee group objects
    pub list: Vec<FeeGroupItemV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_LeverageFilter {
    pub minLeverage: String,
    pub maxLeverage: String,
    pub leverageStep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_PriceFilter {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_LotSizeFilter {
    pub maxOrderQty: String,
    pub maxMktOrderQty: String,
    pub minOrderQty: String,
    pub qtyStep: String,
    pub postOnlyMaxOrderQty: Option<String>,
    pub minNotionalValue: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_RiskParameters {
    pub priceLimitRatioX: String,
    pub priceLimitRatioY: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_PreListingInfo_Phases {
    pub phase: String,
    pub startTime: String,
    pub endTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_PreListingInfo_AuctionFeeInfo {
    pub auctionFeeRate: String,
    pub takerFeeRate: String,
    pub makerFeeRate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearInverseInstrumentInfoV5_PreListingInfo {
    pub curAuctionPhase: String,
    pub phases: Vec<LinearInverseInstrumentInfoV5_PreListingInfo_Phases>,
    pub auctionFeeInfo: LinearInverseInstrumentInfoV5_PreListingInfo_AuctionFeeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OptionInstrumentInfoV5_PriceFilter {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OptionInstrumentInfoV5_LotSizeFilter {
    pub maxOrderQty: String,
    pub minOrderQty: String,
    pub qtyStep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotInstrumentInfoV5_LotSizeFilter {
    pub basePrecision: String,
    pub quotePrecision: String,
    pub minOrderQty: String,
    pub maxOrderQty: String,
    pub minOrderAmt: String,
    pub maxOrderAmt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotInstrumentInfoV5_PriceFilter {
    pub tickSize: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotInstrumentInfoV5_RiskParameters {
    pub priceLimitRatioX: String,
    pub priceLimitRatioY: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OpenInterestResponseV5_List {
    pub openInterest: String,
    pub timestamp: String,
}

