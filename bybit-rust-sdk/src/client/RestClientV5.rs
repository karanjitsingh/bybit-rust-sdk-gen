// Auto-generated from TypeScript definitions
// Source: /Users/karan/github/bybit-rust-sdk-gen/bybit-api/src/rest-client-v5.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::client::BaseRestClient;
use crate::client::ClientResult;
use crate::types::request::v5_account::*;
use crate::types::request::v5_asset::*;
use crate::types::request::v5_broker::{GetBrokerIssuedVoucherParamsV5, GetBrokerSubAccountDepositsV5, GetExchangeBrokerEarningsParamsV5, IssueVoucherParamsV5};
use crate::types::request::v5_crypto_loan::*;
use crate::types::request::v5_earn::{GetEarnOrderHistoryParamsV5, GetEarnPositionParamsV5, SubmitStakeRedeemParamsV5};
use crate::types::request::v5_market::*;
use crate::types::request::v5_p2p_trading::*;
use crate::types::request::v5_position::*;
use crate::types::request::v5_pre_upgrade::*;
use crate::types::request::v5_rfq::*;
use crate::types::request::v5_spot_leverage_token::{GetVIPMarginDataParamsV5};
use crate::types::request::v5_spreadtrading::*;
use crate::types::request::v5_trade::*;
use crate::types::request::v5_user::{CreateSubApiKeyParamsV5, CreateSubMemberParamsV5, DeleteSubMemberParamsV5, GetSubAccountAllApiKeysParamsV5, UpdateApiKeyParamsV5};
use crate::types::response::v5_account::*;
use crate::types::response::v5_asset::*;
use crate::types::response::v5_broker::{BrokerIssuedVoucherV5, BrokerVoucherSpecV5, ExchangeBrokerAccountInfoV5, ExchangeBrokerEarningResultV5};
use crate::types::response::v5_crypto_loan::*;
use crate::types::response::v5_market::*;
use crate::types::response::v5_p2p_trading::*;
use crate::types::response::v5_position::*;
use crate::types::response::v5_preupgrade::{PreUpgradeOptionsDelivery, PreUpgradeUSDCSessionSettlement};
use crate::types::response::v5_rfq::*;
use crate::types::response::v5_spot_leverage_token::{SpotMarginStateV5, VIPMarginDataV5};
use crate::types::response::v5_spreadtrading::{SpreadOrderbookResponseV5};
use crate::types::response::v5_trade::{AccountOrderV5, OrderResultV5, PreCheckOrderResultV5, SpotBorrowCheckResultV5};
use crate::types::response::v5_user::{AffiliateUserInfoV5, ApiKeyInfoV5, CreateSubApiKeyResultV5, CreateSubMemberResultV5, UpdateApiKeyResultV5};
use crate::types::shared_v5::*;


// Generated client: RestClientV5
pub struct RestClientV5<'a> {
    base: &'a BaseRestClient,
}

impl<'a> RestClientV5<'a> {
    /// Create a new instance of RestClientV5
    pub fn new(base: &'a BaseRestClient) -> Self {
        Self { base }
    }

    /// ***** Custom SDK APIs
    /// This method is used to get the latency and time sync between the client and the server.
    /// This is not official API endpoint and is only used for internal testing purposes.
    /// Use this method to check the latency and time sync between the client and the server.
    /// Final values might vary slightly, but it should be within few ms difference.
    /// If you have any suggestions or improvements to this measurement, please create an issue or pull request on GitHub.
    pub async fn fetch_latency_summary(&self) -> ClientResult<serde_json::Value> {
        todo!("Method implementation: fetchLatencySummary")
    }

    /// ***** Misc Bybit APIs
    pub async fn get_system_status(&self, params: Option<GetSystemStatusParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/system/status", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    pub fn get_client_type(&self) -> ClientResult<String> {
        todo!("Method implementation: getClientType")
    }

    pub async fn fetch_server_time(&self) -> ClientResult<f64> {
        todo!("Method implementation: fetchServerTime")
    }

    pub async fn get_server_time(&self) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/market/time", None).await
    }

    /// ***** Demo Account APIs
    pub async fn request_demo_trading_funds(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/account/demo-apply-money", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Create a demo trading account.
    pub async fn create_demo_account(&self) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/user/create-demo-member", None).await
    }

    /// ***** Spread Trading APIs
    /// Get Spread Instruments Info
    pub async fn get_spread_instruments_info(&self, params: Option<GetSpreadInstrumentsInfoParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/spread/instrument", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Spread Orderbook
    pub async fn get_spread_orderbook(&self, params: serde_json::Value) -> ClientResult<SpreadOrderbookResponseV5> {
        self.base.get("/v5/spread/orderbook", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Spread Tickers
    pub async fn get_spread_tickers(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/spread/tickers", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Spread Public Recent Trades
    pub async fn get_spread_recent_trades(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/spread/recent-trade", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Create Spread Order
    pub async fn submit_spread_order(&self, params: SubmitSpreadOrderParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spread/order/create", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Amend Spread Order
    /// You can only modify unfilled or partially filled orders.
    pub async fn amend_spread_order(&self, params: AmendSpreadOrderParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spread/order/amend", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Cancel Spread Order
    pub async fn cancel_spread_order(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spread/order/cancel", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Cancel All Spread Orders
    /// When a symbol is specified, all orders for that symbol will be canceled regardless of the cancelAll field.
    /// When symbol is not specified and cancelAll=true, all orders, regardless of the symbol, will be canceled.
    pub async fn cancel_all_spread_orders(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spread/order/cancel-all", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Spread Open Orders
    /// Query unfilled or partially filled orders in real-time.
    pub async fn get_spread_open_orders(&self, params: Option<GetSpreadOpenOrdersParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spread/order/realtime", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Spread Order History
    /// Note:
    /// - orderId & orderLinkId has a higher priority than startTime & endTime
    /// - Fully canceled orders are stored for up to 24 hours
    /// - Single leg orders can also be found with "createType"=CreateByFutureSpread via Get Order History
    pub async fn get_spread_order_history(&self, params: Option<GetSpreadOrderHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spread/order/history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Spread Trade History
    /// Note:
    /// - In self-trade cases, both the maker and taker single-leg trades will be returned in the same request
    /// - Single leg executions can also be found with "execType"=FutureSpread via Get Trade History
    pub async fn get_spread_trade_history(&self, params: Option<GetSpreadTradeHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spread/execution/list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** Market APIs
    /// Query the kline data. Charts are returned in groups based on the requested interval.
    /// Covers: Spot / Linear contract / Inverse contract
    pub async fn get_kline(&self, params: GetKlineParamsV5) -> ClientResult<CategorySymbolListV5<Vec<OHLCVKlineV5>, String>> {
        self.base.get("/v5/market/kline", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the mark price kline data. Charts are returned in groups based on the requested interval.
    /// Covers: Linear contract / Inverse contract
    pub async fn get_mark_price_kline(&self, params: GetMarkPriceKlineParamsV5) -> ClientResult<CategorySymbolListV5<Vec<OHLCKlineV5>, String>> {
        self.base.get("/v5/market/mark-price-kline", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the index price kline data. Charts are returned in groups based on the requested interval.
    /// Covers: Linear contract / Inverse contract
    pub async fn get_index_price_kline(&self, params: GetIndexPriceKlineParamsV5) -> ClientResult<CategorySymbolListV5<Vec<OHLCKlineV5>, String>> {
        self.base.get("/v5/market/index-price-kline", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Retrieve the premium index price kline data. Charts are returned in groups based on the requested interval.
    /// Covers: Linear contract
    pub async fn get_premium_index_price_kline(&self, params: GetPremiumIndexPriceKlineParamsV5) -> ClientResult<CategorySymbolListV5<Vec<OHLCKlineV5>, String>> {
        self.base.get("/v5/market/premium-index-price-kline", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query a list of instruments of online trading pair.
    /// Covers: Spot / Linear contract / Inverse contract / Option
    /// Note: Spot does not support pagination, so limit & cursor are invalid.
    pub async fn get_instruments_info(&self, params: GetInstrumentsInfoParamsV5) -> ClientResult<InstrumentInfoResponseV5<C>> {
        self.base.get("/v5/market/instruments-info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query orderbook data
    /// Covers: Spot / Linear contract / Inverse contract / Option
    pub async fn get_orderbook(&self, params: GetOrderbookParamsV5) -> ClientResult<OrderbookResponseV5> {
        self.base.get("/v5/market/orderbook", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get RPI Orderbook
    /// Query for orderbook depth data with RPI (Retail Price Improvement) information.
    /// Covers: Spot / USDT contract / USDC contract / Inverse contract
    /// Contract: 50-level of RPI orderbook data
    /// Spot: 50-level of RPI orderbook data
    pub async fn get_rpi_orderbook(&self, params: GetRPIOrderbookParamsV5) -> ClientResult<RPIOrderbookResponseV5> {
        self.base.get("/v5/market/rpi_orderbook", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    /// Covers: Spot / Linear contract / Inverse contract / Option
    pub async fn get_tickers(&self, params: GetTickersParamsV5<CategoryV5>) -> ClientResult<CategoryListV5<Vec<TickerLinearInverseV5>, Vec<serde_json::Value>, Vec<serde_json::Value>, String>> {
        self.base.get("/v5/market/tickers", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query historical funding rate. Each symbol has a different funding interval.
    /// Covers: Linear contract / Inverse perpetual
    pub async fn get_funding_rate_history(&self, params: GetFundingRateHistoryParamsV5) -> ClientResult<CategoryListV5<Vec<FundingRateHistoryResponseV5>, String>> {
        self.base.get("/v5/market/funding/history", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query recent public trading data in Bybit.
    /// Covers: Spot / Linear contract / Inverse contract / Option
    pub async fn get_public_trading_history(&self, params: GetPublicTradingHistoryParamsV5) -> ClientResult<CategoryListV5<Vec<PublicTradeV5>, CategoryV5>> {
        self.base.get("/v5/market/recent-trade", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get open interest of each symbol.
    /// Covers: Linear contract / Inverse contract
    pub async fn get_open_interest(&self, params: GetOpenInterestParamsV5) -> ClientResult<OpenInterestResponseV5> {
        self.base.get("/v5/market/open-interest", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query option historical volatility
    /// Covers: Option
    pub async fn get_historical_volatility(&self, params: GetHistoricalVolatilityParamsV5) -> ClientResult<CategoryListV5<Vec<HistoricalVolatilityV5>, String>> {
        self.base.get("/v5/market/historical-volatility", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query Bybit insurance pool data (BTC/USDT/USDC etc). The data is updated every 24 hours.
    pub async fn get_insurance(&self, params: Option<GetInsuranceParamsV5>) -> ClientResult<InsuranceResponseV5> {
        self.base.get("/v5/market/insurance", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query risk limit of futures
    /// Covers: Linear contract / Inverse contract
    pub async fn get_risk_limit(&self, params: Option<GetRiskLimitParamsV5>) -> ClientResult<CategoryListV5<Vec<RiskLimitV5>, String>> {
        self.base.get("/v5/market/risk-limit", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get the delivery price for option
    /// Covers: Option
    pub async fn get_option_delivery_price(&self, params: GetOptionDeliveryPriceParamsV5) -> ClientResult<CategoryCursorListV5<Vec<OptionDeliveryPriceV5>, CategoryV5>> {
        self.base.get("/v5/market/delivery-price", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get the delivery price of Inverse futures, USDC futures and Options
    /// Covers: USDC futures / Inverse futures / Option
    pub async fn get_delivery_price(&self, params: GetDeliveryPriceParamsV5) -> ClientResult<CategoryCursorListV5<Vec<DeliveryPriceV5>, CategoryV5>> {
        self.base.get("/v5/market/delivery-price", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get New Delivery Price
    /// Get historical option delivery prices.
    /// Covers: Option
    /// INFO
    /// - It is recommended to query this endpoint 1 minute after settlement is completed, because the data returned by this endpoint may be delayed by 1 minute.
    /// - By default, the most recent 50 records are returned in reverse order of "deliveryTime".
    pub async fn get_new_delivery_price(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/market/new-delivery-price", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    pub async fn get_long_short_ratio(&self, params: GetLongShortRatioParamsV5) -> ClientResult<CursorListV5<Vec<LongShortRatioV5>>> {
        self.base.get("/v5/market/account-ratio", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Index Price Components
    /// Query for index price components that contribute to the calculation of an index price.
    pub async fn get_index_price_components(&self, params: GetIndexPriceComponentsParamsV5) -> ClientResult<IndexPriceComponentsResponseV5> {
        self.base.get("/v5/market/index-price-components", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    pub async fn get_order_price_limit(&self, params: serde_json::Value) -> ClientResult<OrderPriceLimitV5> {
        self.base.get("/v5/market/price-limit", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get ADL Alert
    /// Query for ADL (auto-deleveraging mechanism) alerts and insurance pool information.
    /// Covers: USDT Perpetual / USDT Delivery / USDC Perpetual / USDC Delivery / Inverse Contracts
    /// Data update frequency: every 1 minute
    pub async fn get_adl_alert(&self, params: Option<GetADLAlertParamsV5>) -> ClientResult<ADLAlertResponseV5> {
        self.base.get("/v5/market/adlAlert", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Fee Group Structure
    /// Query for the group fee structure and fee rates.
    /// The new grouped fee structure only applies to Pro-level and Market Maker clients.
    /// Covers: USDT Perpetual / USDT Delivery / USDC Perpetual / USDC Delivery / Inverse Contracts
    pub async fn get_fee_group_structure(&self, params: GetFeeGroupStructureParamsV5) -> ClientResult<FeeGroupStructureResponseV5> {
        self.base.get("/v5/market/fee-group-info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** Trade APIs
    pub async fn submit_order(&self, params: OrderParamsV5) -> ClientResult<OrderResultV5> {
        self.base.post_private("/v5/order/create", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    pub async fn amend_order(&self, params: AmendOrderParamsV5) -> ClientResult<OrderResultV5> {
        self.base.post_private("/v5/order/amend", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    pub async fn cancel_order(&self, params: CancelOrderParamsV5) -> ClientResult<OrderResultV5> {
        self.base.post_private("/v5/order/cancel", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query unfilled or partially filled orders in real-time. To query older order records, please use the order history interface.
    pub async fn get_active_orders(&self, params: GetAccountOrdersParamsV5) -> ClientResult<CategoryCursorListV5<Vec<AccountOrderV5>, CategoryV5>> {
        self.base.get_private("/v5/order/realtime", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    pub async fn cancel_all_orders(&self, params: CancelAllOrdersParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/order/cancel-all", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query order history. As order creation/cancellation is asynchronous, the data returned from this endpoint may delay.
    /// If you want to get real-time order information, you could query this endpoint or rely on the websocket stream (recommended).
    pub async fn get_historic_orders(&self, params: GetAccountHistoricOrdersParamsV5) -> ClientResult<CategoryCursorListV5<Vec<AccountOrderV5>, CategoryV5>> {
        self.base.get_private("/v5/order/history", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query users' execution records, sorted by execTime in descending order
    /// Unified account covers: Spot / Linear contract / Options
    /// Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures
    pub async fn get_execution_list(&self, params: GetExecutionListParamsV5) -> ClientResult<CategoryCursorListV5<Vec<ExecutionV5>, CategoryV5>> {
        self.base.get_private("/v5/execution/list", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// This endpoint allows you to place more than one order in a single request.
    /// Covers: Option (UTA, UTA Pro) / USDT Perpetual, UDSC Perpetual, USDC Futures (UTA Pro)
    /// Make sure you have sufficient funds in your account when placing an order.
    /// Once an order is placed, according to the funds required by the order,
    /// the funds in your account will be frozen by the corresponding amount during the life cycle of the order.
    /// A maximum of 20 orders can be placed per request. The returned data list is divided into two lists.
    /// The first list indicates whether or not the order creation was successful and the second list details the created order information.
    /// The structure of the two lists are completely consistent.
    pub async fn batch_submit_orders(&self, category: String, orders: Vec<BatchOrderParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/order/create-batch", Some(serde_json::to_value(category).unwrap_or_default())).await
    }

    /// This endpoint allows you to amend more than one open order in a single request.
    /// Covers: Option (UTA, UTA Pro) / USDT Perpetual, UDSC Perpetual, USDC Futures (UTA Pro)
    /// You can modify unfilled or partially filled orders. Conditional orders are not supported.
    /// A maximum of 20 orders can be amended per request.
    pub async fn batch_amend_orders(&self, category: String, orders: Vec<BatchAmendOrderParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/order/amend-batch", Some(serde_json::to_value(category).unwrap_or_default())).await
    }

    /// This endpoint allows you to cancel more than one open order in a single request.
    /// Covers: Option (UTA, UTA Pro) / USDT Perpetual, UDSC Perpetual, USDC Futures (UTA Pro)
    /// You must specify orderId or orderLinkId. If orderId and orderLinkId is not matched, the system will process orderId first.
    /// You can cancel unfilled or partially filled orders. A maximum of 20 orders can be cancelled per request.
    pub async fn batch_cancel_orders(&self, category: String, orders: Vec<BatchCancelOrderParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/order/cancel-batch", Some(serde_json::to_value(category).unwrap_or_default())).await
    }

    /// Query the qty and amount of borrowable coins in spot account.
    /// Covers: Spot (Unified Account)
    pub async fn get_spot_borrow_check(&self, symbol: String, side: OrderSideV5) -> ClientResult<SpotBorrowCheckResultV5> {
        self.base.get_private("/v5/order/spot-borrow-check", Some(serde_json::to_value(symbol).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// This endpoint allows you to set the disconnection protect time window. Covers: option (unified account).
    /// If you need to turn it on/off, you can contact your client manager for consultation and application.
    /// The default time window is 10 seconds.
    /// Only for institutional clients!
    /// If it doesn't work, use v2!
    pub async fn set_disconnect_cancel_all_window(&self, category: String, timeWindow: f64) -> ClientResult<()> {
        self.base.post_private("/v5/order/disconnected-cancel-all", Some(serde_json::to_value(category).unwrap_or_default())).await
    }

    /// This endpoint allows you to set the disconnection protect time window. Covers: option (unified account).
    /// If you need to turn it on/off, you can contact your client manager for consultation and application.
    /// The default time window is 10 seconds.
    /// Only for institutional clients!
    pub async fn set_disconnect_cancel_all_window_v2(&self, params: serde_json::Value) -> ClientResult<()> {
        self.base.post_private("/v5/order/disconnected-cancel-all", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Pre-check order to calculate changes in IMR and MMR before placing an order
    /// This endpoint supports orders with category = inverse, linear, option.
    /// Only Cross Margin mode and Portfolio Margin mode are supported, isolated margin mode is not supported.
    /// category = inverse is not supported in Cross Margin mode.
    /// Conditional order is not supported.
    pub async fn pre_check_order(&self, params: OrderParamsV5) -> ClientResult<PreCheckOrderResultV5> {
        self.base.post_private("/v5/order/pre-check", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** Position APIs
    /// Query real-time position data, such as position size, cumulative realizedPNL.
    /// 0: cross margin. 1: isolated margin
    /// Unified account covers: Linear contract / Options
    /// Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures
    /// Note: this will give a 404 error if you query the \`option\` category if your account is not unified
    pub async fn get_position_info(&self, params: PositionInfoParamsV5) -> ClientResult<CategoryCursorListV5<Vec<PositionV5>, CategoryV5>> {
        self.base.get_private("/v5/position/list", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Set the leverage
    /// Unified account covers: Linear contract
    /// Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures
    /// Note: Under one-way mode, buyLeverage must be the same as sellLeverage
    pub async fn set_leverage(&self, params: SetLeverageParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/set-leverage", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Select cross margin mode or isolated margin mode.
    /// 0: cross margin. 1: isolated margin
    /// Covers: USDT perpetual (Normal account) / Inverse contract (Normal account).
    /// Switching margin modes will cause orders in progress to be cancelled.
    /// Please make sure that there are no open orders before you switch margin modes.
    pub async fn switch_isolated_margin(&self, params: SwitchIsolatedMarginParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/switch-isolated", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    pub async fn set_tpsl_mode(&self, params: SetTPSLModeParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/set-tpsl-mode", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Switches the position mode for USDT perpetual and Inverse futures.
    /// If you are in one-way Mode, you can only open one position on Buy or Sell side.
    /// If you are in hedge mode, you can open both Buy and Sell side positions simultaneously.
    /// Position mode. 0: Merged Single. 3: Both Sides.
    pub async fn switch_position_mode(&self, params: SwitchPositionModeParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/switch-mode", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    pub async fn set_risk_limit(&self, params: SetRiskLimitParamsV5) -> ClientResult<SetRiskLimitResultV5> {
        self.base.post_private("/v5/position/set-risk-limit", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// This endpoint allows you to set the take profit, stop loss or trailing stop for a position.
    /// Passing these parameters will create conditional orders by the system internally.
    /// The system will cancel these orders if the position is closed, and adjust the qty according to the size of the open position.
    /// Unified account covers: Linear contract.
    /// Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures.
    pub async fn set_trading_stop(&self, params: SetTradingStopParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/trading-stop", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// This endpoint allows you to turn on/off auto-add-margin for an isolated margin position.
    /// Covers: USDT perpetual (Normal Account).
    pub async fn set_auto_add_margin(&self, params: SetAutoAddMarginParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/set-auto-add-margin", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Manually add or reduce margin for isolated margin position
    /// Unified account covers: USDT perpetual / USDC perpetual / USDC futures / Inverse contract
    /// Normal account covers: USDT perpetual / Inverse contract
    pub async fn add_or_reduce_margin(&self, params: AddOrReduceMarginParamsV5) -> ClientResult<AddOrReduceMarginResultV5> {
        self.base.post_private("/v5/position/add-margin", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query user's closed profit and loss records. The results are sorted by createdTime in descending order.
    /// Unified account covers: Linear contract
    /// Normal account covers: USDT perpetual / Inverse perpetual / Inverse futures
    pub async fn get_closed_pn_l(&self, params: GetClosedPnLParamsV5) -> ClientResult<CategoryCursorListV5<Vec<ClosedPnLV5>, CategoryV5>> {
        self.base.get_private("/v5/position/closed-pnl", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Closed Options Positions
    /// Query user's closed options positions, sorted by closeTime in descending order
    /// INFO
    /// Only supports users to query closed options positions in recently 6 months
    /// Fee and price retain 8 decimal places and do not omit the last 0
    pub async fn get_closed_options_positions(&self, params: Option<GetClosedOptionsPositionsParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/position/get-closed-positions", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Move positions between sub-master, master-sub, or sub-sub UIDs.
    /// Unified account covers: USDT perpetual / USDC contract / Spot / Option
    /// INFO
    /// The endpoint can only be called by master UID api key
    /// UIDs must be the same master-sub account relationship
    /// The trades generated from move-position endpoint will not be displayed in the Recent Trade (Rest API & Websocket)
    /// There is no trading fee
    /// fromUid and toUid both should be Unified trading accounts, and they need to be one-way mode when moving the positions
    /// Please note that once executed, you will get execType=MovePosition entry from Get Trade History, Get Closed Pnl, and stream from Execution.
    pub async fn move_position(&self, params: MovePositionParamsV5) -> ClientResult<MovePositionResultV5> {
        self.base.post_private("/v5/position/move-positions", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query moved position data by master UID api key.
    /// Unified account covers: USDT perpetual / USDC contract / Spot / Option
    pub async fn get_move_position_history(&self, params: Option<GetMovePositionHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/position/move-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Confirm new risk limit.
    /// It is only applicable when the user is marked as only reducing positions (please see the isReduceOnly field in the Get Position Info interface).
    /// After the user actively adjusts the risk level, this interface is called to try to calculate the adjusted risk level, and if it passes (retCode=0),
    /// the system will remove the position reduceOnly mark. You are recommended to call Get Position Info to check isReduceOnly field.
    /// Unified account covers: USDT perpetual / USDC contract / Inverse contract
    /// Classic account covers: USDT perpetual / Inverse contract
    pub async fn confirm_new_risk_limit(&self, params: ConfirmNewRiskLimitParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/position/confirm-pending-mmr", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// ***** Pre-upgrade APIs
    /// Get those orders which occurred before you upgrade the account to Unified account.
    /// For now, it only supports to query USDT perpetual, USDC perpetual, Inverse perpetual and futures.
    ///   - can get all status in 7 days
    ///   - can only get filled orders beyond 7 days
    pub async fn get_pre_upgrade_order_history(&self, params: GetPreUpgradeOrderHistoryParamsV5) -> ClientResult<CategoryCursorListV5<Vec<AccountOrderV5>, CategoryV5>> {
        self.base.get_private("/v5/pre-upgrade/order/history", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get users' execution records which occurred before you upgrade the account to Unified account, sorted by execTime in descending order
    /// For now, it only supports to query USDT perpetual, Inverse perpetual and futures.
    ///   - You may have multiple executions in a single order.
    ///   - You can query by symbol, baseCoin, orderId and orderLinkId, and if you pass multiple params,
    ///      the system will process them according to this priority: orderId > orderLinkId > symbol > baseCoin.
    pub async fn get_pre_upgrade_trade_history(&self, params: GetPreUpgradeTradeHistoryParamsV5) -> ClientResult<CategoryCursorListV5<Vec<ExecutionV5>, CategoryV5>> {
        self.base.get_private("/v5/pre-upgrade/execution/list", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query user's closed profit and loss records. The results are sorted by createdTime in descending order.
    /// For now, it only supports to query USDT perpetual, Inverse perpetual and futures.
    pub async fn get_pre_upgrade_closed_pnl(&self, params: GetPreUpgradeClosedPnlParamsV5) -> ClientResult<CategoryCursorListV5<Vec<ClosedPnLV5>, CategoryV5>> {
        self.base.get_private("/v5/pre-upgrade/position/closed-pnl", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query transaction logs which occurred in the USDC Derivatives wallet before the account was upgraded to a Unified account.
    /// You can get USDC Perpetual, Option records.
    /// INFO
    /// USDC Perpeual & Option support the recent 6 months data. Please download older data via GUI
    pub async fn get_pre_upgrade_transactions(&self, params: GetPreUpgradeTransactionLogParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/pre-upgrade/account/transaction-log", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query delivery records of Option before you upgraded the account to a Unified account, sorted by deliveryTime in descending order.
    /// INFO
    /// Supports the recent 6 months data. Please download older data via GUI
    pub async fn get_pre_upgrade_option_delivery_record(&self, params: GetPreUpgradeOptionDeliveryRecordParamsV5) -> ClientResult<CategoryCursorListV5<Vec<PreUpgradeOptionsDelivery>, CategoryV5>> {
        self.base.get_private("/v5/pre-upgrade/asset/delivery-record", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query session settlement records of USDC perpetual before you upgrade the account to Unified account.
    /// INFO
    /// USDC Perpetual support the recent 6 months data. Please download older data via GUI
    pub async fn get_pre_upgrade_usdc_session_settlements(&self, params: GetPreUpgradeUSDCSessionParamsV5) -> ClientResult<CategoryCursorListV5<Vec<PreUpgradeUSDCSessionSettlement>, CategoryV5>> {
        self.base.get_private("/v5/pre-upgrade/asset/settlement-record", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** Account APIs
    /// Obtain wallet balance, query asset information of each currency, and account risk rate information under unified margin mode.
    /// By default, currency information with assets or liabilities of 0 is not returned.
    pub async fn get_wallet_balance(&self, params: GetWalletBalanceParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/wallet-balance", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query the available amount to transfer of a specific coin in the Unified wallet.
    pub async fn get_transferable_amount(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/withdrawal", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Upgrade to unified account.
    /// Banned/OTC loan/Net asset unsatisfying/Express path users cannot upgrade the account to Unified Account for now.
    pub async fn upgrade_to_unified_account(&self) -> ClientResult<UnifiedAccountUpgradeResultV5> {
        self.base.post_private("/v5/account/upgrade-to-uta", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get interest records, sorted in reverse order of creation time.
    /// Unified account
    pub async fn get_borrow_history(&self, params: Option<GetBorrowHistoryParamsV5>) -> ClientResult<CursorListV5<Vec<BorrowHistoryRecordV5>>> {
        self.base.get_private("/v5/account/borrow-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// You can manually repay the liabilities of Unified account
    /// Applicable: Unified Account
    /// Permission: USDC Contracts
    /// - Input the specific coin: repay the liability of this coin in particular
    /// - No coin specified: repay the liability of all coins
    pub async fn repay_liability(&self, params: Option<RepayLiabilityParamsV5>) -> ClientResult<CursorListV5<Vec<RepayLiabilityResultV5>>> {
        self.base.post_private("/v5/account/quick-repayment", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// You can decide whether the assets in the Unified account needs to be collateral coins.
    pub async fn set_collateral_coin(&self, params: SetCollateralCoinParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/account/set-collateral-switch", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    pub async fn batch_set_collateral_coin(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/account/set-collateral-switch-batch", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get the collateral information of the current unified margin account, including loan interest rate,
    /// loanable amount, collateral conversion rate, whether it can be mortgaged as margin, etc.
    pub async fn get_collateral_info(&self, currency: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/collateral-info", currency.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get current account Greeks information
    pub async fn get_coin_greeks(&self, baseCoin: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/coin-greeks", baseCoin.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get the trading fee rate.
    /// Covers: Spot / USDT perpetual / Inverse perpetual / Inverse futures / Options
    pub async fn get_fee_rate(&self, params: GetFeeRateParamsV5) -> ClientResult<CategoryCursorListV5<Vec<FeeRateV5>, CategoryV5>> {
        self.base.get_private("/v5/account/fee-rate", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the margin mode and the upgraded status of account
    pub async fn get_account_info(&self) -> ClientResult<AccountInfoV5> {
        self.base.get_private("/v5/account/info", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the DCP configuration of the account's contracts (USDT perpetual, USDC perpetual and USDC Futures) / spot / options.
    /// Only the configured main / sub account can query information from this API. Calling this API by an account always returns empty.
    /// INFO
    /// support linear contract (USDT, USDC Perp & USDC Futures) / Spot / Options only
    /// Unified account only
    pub async fn get_dcp_info(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/query-dcp-info", None).await
    }

    /// Query transaction logs in Unified account.
    pub async fn get_transaction_log(&self, params: Option<GetTransactionLogParamsV5>) -> ClientResult<CursorListV5<Vec<TransactionLogV5>>> {
        self.base.get_private("/v5/account/transaction-log", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query transaction logs in the derivatives wallet (classic account), and inverse derivatives wallet (upgraded to UTA).
    /// API key permission: "Contract - Position"
    pub async fn get_classic_transaction_logs(&self, params: Option<GetClassicTransactionLogsParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/contract-transaction-log", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Query the SMP group ID of self match prevention.
    pub async fn get_smp_group(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/smp-group", None).await
    }

    /// Default is regular margin mode.
    /// This mode is valid for USDT Perp, USDC Perp and USDC Option.
    pub async fn set_margin_mode(&self, marginMode: AccountMarginModeV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/account/set-margin-mode", Some(serde_json::to_value(marginMode).unwrap_or_default())).await
    }

    /// Turn on/off Spot hedging feature in Portfolio margin for Unified account.
    /// INFO
    /// Only unified account is applicable
    /// Only portfolio margin mode is applicable
    pub async fn set_spot_hedging(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/account/set-hedging-mode", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Set Limit Price Behaviour
    /// You can configure how the system behaves when your limit order price exceeds the highest bid or lowest ask price.
    /// Spot: If the order price exceeds the boundary, the system rejects the request.
    /// Futures: If the order price exceeds the boundary, the system will automatically adjust the price to the nearest allowed boundary.
    pub async fn set_limit_price_action(&self, params: SetLimitPriceActionParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/account/set-limit-px-action", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Limit Price Behaviour
    /// You can get configuration how the system behaves when your limit order price exceeds the highest bid or lowest ask price.
    pub async fn get_limit_price_action(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/user-setting-config", None).await
    }

    /// Configure Market Maker Protection (MMP)
    pub async fn set_mmp(&self, params: MMPModifyParamsV5) -> ClientResult<()> {
        self.base.post_private("/v5/account/mmp-modify", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Once the mmp triggered, you can unfreeze the account via this endpoint
    pub async fn reset_mmp(&self, baseCoin: String) -> ClientResult<()> {
        self.base.post_private("/v5/account/mmp-reset", Some(serde_json::to_value(baseCoin).unwrap_or_default())).await
    }

    /// Get MMP State
    pub async fn get_mmp_state(&self, baseCoin: String) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/account/mmp-state", Some(serde_json::to_value(baseCoin).unwrap_or_default())).await
    }

    /// ***** Asset APIs
    /// Query option delivery records, sorted by deliveryTime in descending order.
    /// Covers: Option
    pub async fn get_delivery_record(&self, params: GetDeliveryRecordParamsV5) -> ClientResult<CategoryCursorListV5<Vec<DeliveryRecordV5>, CategoryV5>> {
        self.base.get_private("/v5/asset/delivery-record", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query session settlement records of USDC perpetual
    /// Covers: Linear contract (USDC Perpetual only, Unified Account)
    pub async fn get_settlement_records(&self, params: GetSettlementRecordParamsV5) -> ClientResult<CategoryCursorListV5<Vec<SettlementRecordV5>, CategoryV5>> {
        self.base.get_private("/v5/asset/settlement-record", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the coin exchange records.
    /// CAUTION: You may experience long delays with this endpoint.
    pub async fn get_coin_exchange_records(&self, params: Option<GetCoinExchangeRecordParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/exchange/order-record", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Query coin information, including chain information, withdraw and deposit status.
    pub async fn get_coin_info(&self, coin: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/coin/query-info", coin.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Query the sub UIDs under a main UID
    /// CAUTION: Can query by the master UID's api key only
    pub async fn get_sub_uid_(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/transfer/query-sub-member-list", None).await
    }

    /// Query asset information.
    /// INFO
    /// For now, it can query SPOT only.
    pub async fn get_asset_info(&self, params: GetAssetInfoParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/transfer/query-asset-info", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query all coin balances of all account types under the master account and sub accounts.
    /// It is not allowed to get the master account coin balance via sub account API key.
    pub async fn get_all_coins_balance(&self, params: GetAllCoinsBalanceParamsV5) -> ClientResult<AllCoinsBalanceV5> {
        self.base.get_private("/v5/asset/transfer/query-account-coins-balance", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query the balance of a specific coin in a specific account type. Supports querying sub UID's balance.
    /// CAUTION: Can query by the master UID's api key only.
    pub async fn get_coin_balance(&self, params: GetAccountCoinBalanceParamsV5) -> ClientResult<AccountCoinBalanceV5> {
        self.base.get_private("/v5/asset/transfer/query-account-coin-balance", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query withdrawable amount.
    pub async fn get_withdrawable_amount(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/withdraw/withdrawable-amount", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query the transferable coin list between each account type.
    pub async fn get_transferable_coin_list(&self, fromAccountType: AccountTypeV5, toAccountType: AccountTypeV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/transfer/query-transfer-coin-list", Some(serde_json::to_value(fromAccountType).unwrap_or_default())).await
    }

    /// Create the internal transfer between different account types under the same UID.
    /// Each account type has its own acceptable coins, e.g, you cannot transfer USDC from SPOT to CONTRACT.
    /// Please refer to the getTransferableCoinList() API to find out more.
    pub async fn create_internal_transfer(&self, transferId: String, coin: String, amount: String, fromAccountType: AccountTypeV5, toAccountType: AccountTypeV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/transfer/inter-transfer", Some(serde_json::to_value(transferId).unwrap_or_default())).await
    }

    /// Query the internal transfer records between different account types under the same UID.
    pub async fn get_internal_transfer_records(&self, params: Option<GetInternalTransferParamsV5>) -> ClientResult<CursorListV5<Vec<InternalTransferRecordV5>>> {
        self.base.get_private("/v5/asset/transfer/query-inter-transfer-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Enable Universal Transfer for Sub UID
    /// Use this endpoint to enable a subaccount to take part in a universal transfer.
    /// It is a one-time switch which, once thrown, enables a subaccount permanently.
    /// If not set, your subaccount cannot use universal transfers.
    pub async fn enable_universal_transfer_for_sub_uid_s(&self, subMemberIds: Vec<String>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/transfer/save-transfer-sub-member", Some(serde_json::to_value(subMemberIds).unwrap_or_default())).await
    }

    /// Transfer between sub-sub or main-sub. Please make sure you have enabled universal transfer on your sub UID in advance.
    pub async fn create_universal_transfer(&self, params: UniversalTransferParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/transfer/universal-transfer", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query universal transfer records
    /// CAUTION
    /// Can query by the master UID's API key only
    pub async fn get_universal_transfer_records(&self, params: Option<GetUniversalTransferRecordsParamsV5>) -> ClientResult<CursorListV5<Vec<UniversalTransferRecordV5>>> {
        self.base.get_private("/v5/asset/transfer/query-universal-transfer-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query allowed deposit coin information.
    /// To find out paired chain of coin, please refer to the coin info api.
    pub async fn get_allowed_deposit_coin_info(&self, params: Option<GetAllowedDepositCoinInfoParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-allowed-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Set auto transfer account after deposit. The same function as the setting for Deposit on web GUI
    pub async fn set_deposit_account(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/deposit/deposit-to-account", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query deposit records.
    /// TIP
    /// endTime - startTime should be less than 30 days. Query last 30 days records by default.
    /// Can use main or sub UID api key to query deposit records respectively.
    pub async fn get_deposit_records(&self, params: Option<GetDepositRecordParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-record", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Query subaccount's deposit records by MAIN UID's API key.
    /// TIP: Query deposit records of SPOT only
    ///      endTime - startTime should be less than 30 days.
    ///      Queries for the last 30 days worth of records by default.
    pub async fn get_sub_account_deposit_records(&self, params: GetSubAccountDepositRecordParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-sub-member-record", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Internal Deposit Records (across Bybit)
    /// Query deposit records through Bybit platform
    /// RULES
    /// The maximum difference between the start time and the end time is 30 days.
    /// Support to get deposit records by Master or Sub Member Api Key
    pub async fn get_internal_deposit_records(&self, params: Option<GetInternalDepositRecordParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-internal-record", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Query the deposit address information of MASTER account.
    pub async fn get_master_deposit_address(&self, coin: String, chainType: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-address", Some(serde_json::to_value(coin).unwrap_or_default())).await
    }

    /// Query the deposit address information of SUB account.
    pub async fn get_sub_deposit_address(&self, coin: String, chainType: String, subMemberId: String) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-sub-member-address", Some(serde_json::to_value(coin).unwrap_or_default())).await
    }

    pub async fn query_sub_member_address(&self, coin: String, chainType: String, subMemberId: String) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/deposit/query-sub-member-address", Some(serde_json::to_value(coin).unwrap_or_default())).await
    }

    /// Query withdrawal records.
    pub async fn get_withdrawal_records(&self, params: Option<GetWithdrawalRecordsParamsV5>) -> ClientResult<CursorRowsV5<Vec<WithdrawalRecordV5>>> {
        self.base.get_private("/v5/asset/withdraw/query-record", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Exchange Entity List.
    /// This endpoint is particularly used for kyc=KOR users. When withdraw funds, you need to fill entity id.
    pub async fn get_exchange_entities(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/withdraw/vasp/list", None).await
    }

    /// Withdraw assets from the SPOT account.
    /// CAUTION: Make sure you have whitelisted your wallet address before calling this endpoint.
    /// You can make an off-chain transfer if the target wallet address is from Bybit. This means that no blockchain fee will be charged.
    pub async fn submit_withdrawal(&self, params: WithdrawParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/withdraw/create", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Cancel the withdrawal
    /// CAUTION: Can query by the master UID's api key only
    pub async fn cancel_withdrawal(&self, id: String) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/withdraw/cancel", Some(serde_json::to_value(id).unwrap_or_default())).await
    }

    /// Query the coin list of convert from (to).
    pub async fn get_convert_coins(&self, params: ConvertCoinsParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/exchange/query-coin-list", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Request a quote for converting coins.
    pub async fn request_convert_quote(&self, params: RequestConvertQuoteParamsV5) -> ClientResult<ConvertQuoteV5> {
        self.base.post_private("/v5/asset/exchange/quote-apply", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Confirm a quote for converting coins.
    pub async fn confirm_convert_quote(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/asset/exchange/convert-execute", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query the exchange result by sending quoteTxId.
    pub async fn get_convert_status(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/exchange/convert-result-query", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query the conversion history.
    pub async fn get_convert_history(&self, params: Option<GetConvertHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/asset/exchange/query-convert-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** User APIs
    /// Create a new sub user id. Use master user's api key only.
    /// The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
    pub async fn create_sub_member(&self, params: CreateSubMemberParamsV5) -> ClientResult<CreateSubMemberResultV5> {
        self.base.post_private("/v5/user/create-sub-member", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// To create new API key for those newly created sub UID. Use master user's api key only.
    /// TIP: The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
    pub async fn create_sub_uid_api_key(&self, params: CreateSubApiKeyParamsV5) -> ClientResult<CreateSubApiKeyResultV5> {
        self.base.post_private("/v5/user/create-sub-api", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// This endpoint allows you to get a list of all sub UID of master account. At most 10k subaccounts.
    pub async fn get_sub_uid_list(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/user/query-sub-members", None).await
    }

    /// This endpoint allows you to get a list of all sub UID of master account. No limit on the number of subaccounts.
    pub async fn get_sub_uid_list_unlimited(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/user/submembers", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Froze sub uid. Use master user's api key only.
    /// TIP: The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
    pub async fn set_sub_uid_frozen_state(&self, subuid: f64, frozen: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/user/frozen-sub-member", Some(serde_json::to_value(subuid).unwrap_or_default())).await
    }

    /// Get the information of the api key. Use the api key pending to be checked to call the endpoint.
    /// Both master and sub user's api key are applicable.
    /// TIP: Any permission can access this endpoint.
    pub async fn get_query_api_key(&self) -> ClientResult<ApiKeyInfoV5> {
        self.base.get_private("/v5/user/query-api", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Query all api keys information of a sub UID.
    pub async fn get_sub_account_all_api_keys(&self, params: GetSubAccountAllApiKeysParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/user/sub-apikeys", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    pub async fn get_uid_wallet_type(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/user/get-member-type", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Modify the settings of a master API key. Use the API key pending to be modified to call the endpoint. Use master user's API key only.
    /// TIP: The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
    pub async fn update_master_api_key(&self, params: UpdateApiKeyParamsV5) -> ClientResult<UpdateApiKeyResultV5> {
        self.base.post_private("/v5/user/update-api", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// This endpoint modifies the settings of a sub API key.
    /// Use the API key pending to be modified to call the endpoint or use master account api key to manage its sub account api key.
    /// The API key must have one of the below permissions in order to call this endpoint
    ///  - sub API key: "Account Transfer", "Sub Member Transfer"
    ///  - master API Key: "Account Transfer", "Sub Member Transfer", "Withdrawal"
    pub async fn update_sub_api_key(&self, params: UpdateApiKeyParamsV5) -> ClientResult<UpdateApiKeyResultV5> {
        self.base.post_private("/v5/user/update-sub-api", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Delete a sub UID. Before deleting the UID, please make sure there are no assets.
    /// TIP:
    /// The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
    pub async fn delete_sub_member(&self, params: DeleteSubMemberParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/user/del-submember", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Delete the api key of master account. Use the api key pending to be delete to call the endpoint. Use master user's api key only.
    /// TIP:
    /// The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
    /// DANGER: BE CAREFUL! The API key used to call this interface will be invalid immediately.
    pub async fn delete_master_api_key(&self) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/user/delete-api", None).await
    }

    /// Delete the api key of sub account. Use the api key pending to be delete to call the endpoint. Use sub user's api key only.
    /// TIP:
    /// The API key must have one of the permissions to be allowed to call the following API endpoint.
    /// - sub API key: "Account Transfer", "Sub Member Transfer"
    /// - master API Key: "Account Transfer", "Sub Member Transfer", "Withdrawal"
    /// DANGER: BE CAREFUL! The sub API key used to call this interface will be invalid immediately.
    pub async fn delete_sub_api_key(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/user/delete-sub-api", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** Affiliate APIs
    /// Get Affiliate User List.
    /// To use this endpoint, you should have an affiliate account and only tick "affiliate" permission while creating the API key.
    /// TIP:
    /// - Use master UID only
    /// - The api key can only have "Affiliate" permission
    pub async fn get_affiliate_user_list(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/affiliate/aff-user-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Affiliate User Info.
    /// This API is used for affiliate to get their users information.
    /// TIP
    /// Use master UID only
    /// The api key can only have "Affiliate" permission
    /// The transaction volume and deposit amount are the total amount of the user done on Bybit, and have nothing to do with commission settlement. Any transaction volume data related to commission settlement is subject to the Affiliate Portal.
    pub async fn get_affiliate_user_info(&self, params: serde_json::Value) -> ClientResult<AffiliateUserInfoV5> {
        self.base.get_private("/v5/user/aff-customer-info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** Spot Margin Trade APIs (UTA)
    /// Get VIP Margin Data.
    /// This margin data is for Unified account in particular.
    /// INFO
    /// Do not need authentication
    pub async fn get_vip_margin_data(&self, params: Option<GetVIPMarginDataParamsV5>) -> ClientResult<VIPMarginDataV5> {
        self.base.get("/v5/spot-margin-trade/data", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Historical Interest Rate
    /// You can query up to six months borrowing interest rate of Margin trading.
    /// INFO: Need authentication, the api key needs "Spot" permission. Only supports Unified account.
    pub async fn get_historical_interest_rate(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-margin-trade/interest-rate-history", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Turn spot margin trade on / off in your UTA account.
    /// CAUTION
    /// Your account needs to turn on spot margin first.
    pub async fn toggle_spot_margin_trade(&self, spotMarginMode: String) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spot-margin-trade/switch-mode", Some(serde_json::to_value(spotMarginMode).unwrap_or_default())).await
    }

    /// Set the user's maximum leverage in spot cross margin.
    /// CAUTION: Your account needs to enable spot margin first; i.e., you must have finished the quiz on web / app.
    pub async fn set_spot_margin_leverage(&self, leverage: String) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spot-margin-trade/set-leverage", Some(serde_json::to_value(leverage).unwrap_or_default())).await
    }

    /// Query the Spot margin status and leverage of Unified account.
    /// Covers: Margin trade (Unified Account)
    pub async fn get_spot_margin_state(&self) -> ClientResult<SpotMarginStateV5> {
        self.base.get_private("/v5/spot-margin-trade/state", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** Spot Margin Trade APIs (Normal)
    /// Get Margin Coin Info
    pub async fn get_spot_margin_coin_info(&self, coin: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-cross-margin-trade/pledge-token", coin.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Borrowable Coin Info
    pub async fn get_spot_margin_borrowable_coin_info(&self, coin: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-cross-margin-trade/borrow-token", coin.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Interest & Quota
    pub async fn get_spot_margin_interest_and_quota(&self, coin: String) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-cross-margin-trade/loan-info", Some(serde_json::to_value(coin).unwrap_or_default())).await
    }

    /// Get Loan Account Info
    pub async fn get_spot_margin_loan_account_info(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-cross-margin-trade/account", None).await
    }

    /// Borrow
    pub async fn spot_margin_borrow(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spot-cross-margin-trade/loan", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Repay
    pub async fn spot_margin_repay(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spot-cross-margin-trade/repay", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Borrow Order Detail
    pub async fn get_spot_margin_borrow_order_detail(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-cross-margin-trade/orders", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Repayment Order Detail
    pub async fn get_spot_margin_repayment_order_detail(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/spot-cross-margin-trade/repay-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Turn spot margin trade on / off in your NORMAL account.
    pub async fn toggle_spot_cross_margin_trade(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/spot-cross-margin-trade/switch", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// ***** Crypto Loan - Legacy
    /// Get Collateral Coins
    /// INFO: Do not need authentication
    pub async fn get_collateral_coins(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/crypto-loan/collateral-data", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Borrowable Coins
    /// INFO: Do not need authentication
    pub async fn get_borrowable_coins(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/crypto-loan/loanable-data", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Account Borrow/Collateral Limit
    /// Query the account borrowable/collateral limit
    /// Permission: "Spot trade"
    pub async fn get_account_borrow_collateral_limit(&self, params: serde_json::Value) -> ClientResult<AccountBorrowCollateralLimitV5> {
        self.base.get_private("/v5/crypto-loan/borrowable-collateralisable-number", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Borrow Crypto Loan
    pub async fn borrow_crypto_loan(&self, params: BorrowCryptoLoanParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan/borrow", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Repay Crypto Loan
    pub async fn repay_crypto_loan(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan/repay", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Unpaid Loan Orders
    /// Query the ongoing loan orders, which are not fully repaid
    pub async fn get_unpaid_loan_orders(&self, params: Option<GetUnpaidLoanOrdersParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan/ongoing-orders", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Repayment Transaction History
    /// Query repaid transaction history
    pub async fn get_repayment_history(&self, params: Option<GetRepaymentHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan/repayment-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Completed Loan Order History
    /// Query the completed loan orders
    pub async fn get_completed_loan_order_history(&self, params: Option<GetCompletedLoanOrderHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan/borrow-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Max. Allowed Reduction Collateral Amount
    /// Query the maximum allowed reduction collateral amount
    pub async fn get_max_allowed_reduction_collateral_amount(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan/max-collateral-amount", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Adjust Collateral Amount
    /// You can increase or reduce collateral amount. When you reduce, please follow the max. allowed reduction amount.
    pub async fn adjust_collateral_amount(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan/adjust-ltv", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Loan LTV Adjustment History
    /// Query the transaction history of collateral amount adjustment
    pub async fn get_loan_ltv_adjustment_history(&self, params: Option<GetLoanLTVAdjustmentHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan/adjustment-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** Crypto Loan New
    /// Get Borrowable Coins New
    pub async fn get_loan_borrowable_coins(&self, params: Option<GetBorrowableCoinsParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/crypto-loan-common/loanable-data", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Collateral Coins New
    pub async fn get_loan_collateral_coins(&self, params: Option<GetCollateralCoinsParamsV5>) -> ClientResult<CollateralDataV5> {
        self.base.get("/v5/crypto-loan-common/collateral-data", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Max. Allowed Collateral Reduction Amount New
    pub async fn get_max_collateral_amount(&self, params: GetMaxCollateralAmountParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-common/max-collateral-amount", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Adjust Collateral Amount New
    /// You can increase or reduce your collateral amount. When you reduce, please obey the Get Max. Allowed Collateral Reduction Amount
    pub async fn update_collateral_amount(&self, params: AdjustCollateralAmountParamsV5) -> ClientResult<AdjustCollateralAmountV5> {
        self.base.post_private("/v5/crypto-loan-common/adjust-ltv", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Collateral Adjustment History New
    /// Query for your LTV adjustment history.
    pub async fn get_collateral_adjustment_history(&self, params: Option<GetCollateralAdjustmentHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-common/adjustment-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Crypto Loan Position New
    pub async fn get_crypto_loan_position(&self) -> ClientResult<CryptoLoanPositionV5> {
        self.base.get_private("/v5/crypto-loan-common/position", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** Crypto Loan New - Flexible Loan
    /// Borrow Flexible Loan
    /// Fully or partially repay a loan. If interest is due, that is paid off first, with the loaned amount being paid off only after due interest.
    pub async fn borrow_flexible(&self, params: BorrowFlexibleParamsV5) -> ClientResult<BorrowFlexibleV5> {
        self.base.post_private("/v5/crypto-loan-flexible/borrow", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Repay Flexible Loan
    /// Fully or partially repay a loan. If interest is due, that is paid off first, with the loaned amount being paid off only after due interest.
    pub async fn repay_flexible(&self, params: RepayFlexibleParamsV5) -> ClientResult<RepayFlexibleV5> {
        self.base.post_private("/v5/crypto-loan-flexible/repay", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Collateral Repayment
    /// Pay interest first, then repay the principal.
    pub async fn repay_collateral_flexible(&self, params: RepayCollateralFlexibleParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan-flexible/repay-collateral", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Flexible Loans
    /// Query for your ongoing loans
    pub async fn get_ongoing_flexible_loans(&self, params: Option<GetOngoingFlexibleLoansParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-flexible/ongoing-coin", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Borrow Orders History
    pub async fn get_borrow_history_flexible(&self, params: Option<GetBorrowHistoryFlexibleParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-flexible/borrow-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Repayment Orders History
    pub async fn get_repayment_history_flexible(&self, params: Option<GetRepaymentHistoryFlexibleParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-flexible/repayment-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** Fixed Loan
    /// Get Supplying Market
    /// If you want to supply, you can use this endpoint to check whether there are any suitable counterparty borrow orders available.
    pub async fn get_supply_order_quote_fixed(&self, params: GetSupplyOrderQuoteFixedParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/crypto-loan-fixed/supply-order-quote", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Borrowing Market
    /// If you want to borrow, you can use this endpoint to check whether there are any suitable counterparty supply orders available.
    pub async fn get_borrow_order_quote_fixed(&self, params: GetBorrowOrderQuoteFixedParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/crypto-loan-fixed/borrow-order-quote", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Create Borrow Order
    /// The loan funds are released to the Funding wallet.
    /// The collateral funds are deducted from the Funding wallet, so make sure you have enough collateral amount in the Funding wallet.
    pub async fn create_borrow_order_fixed(&self, params: CreateBorrowOrderFixedParamsV5) -> ClientResult<CreateBorrowOrderFixedV5> {
        self.base.post_private("/v5/crypto-loan-fixed/borrow", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Create Supply Order
    /// Permission: "Spot trade"
    pub async fn create_supply_order_fixed(&self, params: CreateSupplyOrderFixedParamsV5) -> ClientResult<CreateSupplyOrderFixedV5> {
        self.base.post_private("/v5/crypto-loan-fixed/supply", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Cancel Borrow Order
    pub async fn cancel_borrow_order_fixed(&self, params: CancelBorrowOrderFixedParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan-fixed/borrow-order-cancel", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Cancel Supply Order
    pub async fn cancel_supply_order_fixed(&self, params: CancelSupplyOrderFixedParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan-fixed/supply-order-cancel", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Borrow Contract Info
    pub async fn get_borrow_contract_info_fixed(&self, params: Option<GetBorrowContractInfoFixedParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-fixed/borrow-contract-info", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Supply Contract Info
    pub async fn get_supply_contract_info_fixed(&self, params: Option<GetSupplyContractInfoFixedParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-fixed/supply-contract-info", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Borrow Order Info
    pub async fn get_borrow_order_info_fixed(&self, params: Option<GetBorrowOrderInfoFixedParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-fixed/borrow-order-info", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Supply Order Info
    pub async fn get_supply_order_info_fixed(&self, params: Option<GetSupplyOrderInfoFixedParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-fixed/supply-order-info", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Repay Fixed Loan
    /// Either loanId or loanCurrency needs to be passed
    pub async fn repay_fixed(&self, params: RepayFixedParamsV5) -> ClientResult<RepayFixedV5> {
        self.base.post_private("/v5/crypto-loan-fixed/fully-repay", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Collateral Repayment
    /// Pay interest first, then repay the principal.
    pub async fn repay_collateral_fixed(&self, params: RepayCollateralFixedParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/crypto-loan-flexible/repay-collateral", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Repayment History
    pub async fn get_repayment_history_fixed(&self, params: Option<GetRepaymentHistoryFixedParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/crypto-loan-fixed/repayment-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** Institutional Lending
    /// Get Product Info
    pub async fn get_institutional_lending_product_info(&self, productId: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/ins-loan/product-infos", productId.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Margin Coin Info
    pub async fn get_institutional_lending_margin_coin_info(&self, productId: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/ins-loan/ensure-tokens", productId.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Margin Coin Info With Conversion Rate
    pub async fn get_institutional_lending_margin_coin_info_with_conversion_rate(&self, productId: Option<String>) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/ins-loan/ensure-tokens-convert", productId.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Loan Orders
    pub async fn get_institutional_lending_loan_orders(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/ins-loan/loan-order", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Repay Orders
    pub async fn get_institutional_lending_repay_orders(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/ins-loan/repaid-history", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get LTV
    pub async fn get_institutional_lending_ltv(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/ins-loan/ltv", None).await
    }

    /// Get LTV with Ladder Conversion Rate
    pub async fn get_institutional_lending_ltv_with_ladder_conversion_rate(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/ins-loan/ltv-convert", None).await
    }

    /// Bind or unbind UID for the institutional loan product.
    /// INFO
    /// Risk unit designated UID cannot be unbound
    /// This endpoint can only be called by uids in the risk unit list
    /// The UID must be upgraded to UTA Pro if you try to bind it.
    /// When the API is operated through the API Key of any UID in the risk unit, the UID is bound or unbound in the risk unit.
    pub async fn bind_or_unbind_uid_(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/ins-loan/association-uid", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// ***** Broker
    /// Get Exchange Broker Earning.
    /// INFO
    /// Use exchange broker master account to query
    /// The data can support up to past 1 months until T-1. To extract data from over a month ago, please contact your Relationship Manager
    /// begin & end are either entered at the same time or not entered, and latest 7 days data are returned by default
    /// API rate limit: 10 req / sec
    pub async fn get_exchange_broker_earnings(&self, params: Option<GetExchangeBrokerEarningsParamsV5>) -> ClientResult<ExchangeBrokerEarningResultV5> {
        self.base.get_private("/v5/broker/earnings-info", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Exchange Broker Account Info.
    /// INFO
    /// Use exchange broker master account to query
    /// API rate limit: 10 req / sec
    pub async fn get_exchange_broker_account_info(&self) -> ClientResult<ExchangeBrokerAccountInfoV5> {
        self.base.get_private("/v5/broker/account-info", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Sub Account Deposit Records.
    /// Exchange broker can query subaccount's deposit records by main UID's API key without specifying uid.
    /// API rate limit: 300 req / min
    /// TIP
    /// endTime - startTime should be less than 30 days. Queries for the last 30 days worth of records by default.
    pub async fn get_broker_sub_account_deposits(&self, params: Option<GetBrokerSubAccountDepositsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/broker/asset/query-sub-member-deposit-record", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Query Voucher Spec
    pub async fn get_broker_voucher_spec(&self, params: serde_json::Value) -> ClientResult<BrokerVoucherSpecV5> {
        self.base.post_private("/v5/broker/award/info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Issue a voucher to a user
    /// INFO
    /// Use exchange broker master account to issue
    pub async fn issue_broker_voucher(&self, params: IssueVoucherParamsV5) -> ClientResult<()> {
        self.base.post_private("/v5/broker/award/distribute-award", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query an issued voucher
    /// INFO
    /// Use exchange broker master account to query
    pub async fn get_broker_issued_voucher(&self, params: GetBrokerIssuedVoucherParamsV5) -> ClientResult<BrokerIssuedVoucherV5> {
        self.base.post_private("/v5/broker/award/distribution-record", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** EARN
    /// Get Product Info for Earn products
    /// INFO: Do not need authentication
    pub async fn get_earn_product(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get("/v5/earn/product", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Stake or Redeem Earn products
    /// INFO: API key needs "Earn" permission
    /// NOTE: In times of high demand for loans in the market for a specific cryptocurrency,
    /// the redemption of the principal may encounter delays and is expected to be processed
    /// within 48 hours. Once the redemption request is initiated, it cannot be canceled,
    /// and your principal will continue to earn interest until the process is completed.
    pub async fn submit_stake_redeem(&self, params: SubmitStakeRedeemParamsV5) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/earn/place-order", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Stake/Redeem Order History
    /// INFO: API key needs "Earn" permission
    /// Note: Either orderId or orderLinkId is required. If both are passed,
    /// make sure they're matched, otherwise returning empty result
    pub async fn get_earn_order_history(&self, params: GetEarnOrderHistoryParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/earn/order", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Staked Position
    /// INFO: API key needs "Earn" permission
    /// Note: Fully redeemed position is also returned in the response
    pub async fn get_earn_position(&self, params: GetEarnPositionParamsV5) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/earn/position", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// ***** RFQ APIs
    /// Create RFQ
    /// Create a new request for quote (RFQ) to specified counterparties
    pub async fn create_rfq(&self, params: CreateRFQParamsV5) -> ClientResult<CreateRFQResultV5> {
        self.base.post_private("/v5/rfq/create-rfq", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get RFQ Configuration
    /// Query the information of the quoting party that can participate in the transaction
    /// and your own deskCode and other configuration information
    pub async fn get_rfq_config(&self) -> ClientResult<RFQConfigV5> {
        self.base.get_private("/v5/rfq/config", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Cancel RFQ
    /// Cancel your inquiry
    /// Pass at least one rfqId or rfqLinkId
    pub async fn cancel_rfq(&self, params: CancelRFQParamsV5) -> ClientResult<CancelRFQResultV5> {
        self.base.post_private("/v5/rfq/cancel-rfq", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Cancel All RFQ
    /// Cancel all your inquiry forms
    pub async fn cancel_all_rfq(&self) -> ClientResult<CancelAllRFQResultV5> {
        self.base.post_private("/v5/rfq/cancel-all-rfq", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Create Quote
    /// Allow the quotation party specified in the inquiry form to quote
    /// Pass at least one quoteBuyList and quoteSellList
    pub async fn create_rfq_quote(&self, params: CreateRFQQuoteParamsV5) -> ClientResult<CreateRFQQuoteResultV5> {
        self.base.post_private("/v5/rfq/create-quote", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Execute Quote
    /// Execute quotes, only for the creator of the inquiry form
    /// Need to view the execution result through the /v5/rfq/blocktrade-list interface
    pub async fn execute_rfq_quote(&self, params: ExecuteRFQQuoteParamsV5) -> ClientResult<ExecuteRFQQuoteResultV5> {
        self.base.post_private("/v5/rfq/execute-quote", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Cancel Quote
    /// Cancel a quotation
    /// Pass at least one quoteId, rfqId, and quoteLinkId
    pub async fn cancel_rfq_quote(&self, params: CancelRFQQuoteParamsV5) -> ClientResult<CancelRFQQuoteResultV5> {
        self.base.post_private("/v5/rfq/cancel-quote", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Cancel All Quotes
    /// Cancel all quotations
    pub async fn cancel_all_rfq_quotes(&self) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/rfq/cancel-all-quotes", None).await
    }

    /// Get Real-time RFQ Information
    /// Obtain the inquiry information sent or received by the user, query from rfq-engine without delay
    /// Pass both rfqId and rfqLinkId, rfqId shall prevail
    /// Sort in reverse order according to the creation time of rfq and return it
    pub async fn get_rfq_realtime_info(&self, params: Option<GetRFQRealtimeParamsV5>) -> ClientResult<GetRFQRealtimeResultV5> {
        self.base.get_private("/v5/rfq/rfq-realtime", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Historical RFQ Information
    /// Obtain the information of the inquiry form sent or received by the user, query from the database
    /// Pass both rfqId and rfqLinkId, rfqId shall prevail
    /// Sort in reverse order according to the creation time of rfq and return it
    pub async fn get_rfq_history(&self, params: Option<GetRFQListParamsV5>) -> ClientResult<RFQHistory> {
        self.base.get_private("/v5/rfq/rfq-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get Real-time Quote Information
    /// Obtain quotation information sent or received by users, query from rfq-engine without delay
    /// Pass quoteId and quoteLinkId, quoteId shall prevail
    /// Pass both rfqId and rfqLinkId, rfqId shall prevail
    /// Sort in reverse order according to the creation time of the quotation
    /// Return all non-final quotes
    pub async fn get_rfq_realtime_quote(&self, params: Option<GetRFQQuoteRealtimeParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/rfq/quote-realtime", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Historical Quote Information
    /// Obtain the quotation information sent or received by the user, query from the database
    /// Pass quoteId and quoteLinkId, quoteId shall prevail
    /// Pass both rfqId and rfqLinkId, rfqId shall prevail
    /// Sort in reverse order according to the creation time of the quotation
    pub async fn get_rfq_history_quote(&self, params: Option<GetRFQHistoryParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/rfq/quote-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get Trade Information
    /// Obtain transaction information executed by the user
    pub async fn get_rfq_trades(&self, params: Option<GetRFQTradeListParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/rfq/trade-list", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// Get RFQ Public Transaction Data
    /// Get the recently executed RFQ successfullys
    pub async fn get_rfq_public_trades(&self, params: Option<GetRFQPublicTradesParamsV5>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/rfq/public-trades", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

    /// ***** P2P TRADING
    /// General P2P
    /// Get coin balance of all account types under the master account, and sub account.
    /// Note: this field is mandatory for accountType=UNIFIED, and supports up to 10 coins each request
    pub async fn get_p2_p_account_coins_balance(&self, params: GetP2PAccountCoinsBalanceParamsV5) -> ClientResult<P2PAccountCoinsBalanceV5> {
        self.base.get_private("/v5/asset/transfer/query-account-coins-balance", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Advertisement P2P
    /// Get market online ads list
    pub async fn get_p2_p_online_ads(&self, params: GetP2POnlineAdsParamsV5) -> ClientResult<P2POnlineAdsResponseV5> {
        self.base.post_private("/v5/p2p/item/online", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Post new P2P advertisement
    pub async fn create_p2_p_ad(&self, params: CreateP2PAdParamsV5) -> ClientResult<P2PCreateAdResponseV5> {
        self.base.post_private("/v5/p2p/item/create", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Cancel P2P advertisement
    pub async fn cancel_p2_p_ad(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/p2p/item/cancel", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Update or relist P2P advertisement
    pub async fn update_p2_p_ad(&self, params: UpdateP2PAdParamsV5) -> ClientResult<P2PCreateAdResponseV5> {
        self.base.post_private("/v5/p2p/item/update", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get personal P2P ads list
    pub async fn get_p2_p_personal_ads(&self, params: GetP2PPersonalAdsParamsV5) -> ClientResult<P2PPersonalAdsResponseV5> {
        self.base.post_private("/v5/p2p/item/personal/list", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get P2P ad details
    pub async fn get_p2_p_ad_detail(&self, params: serde_json::Value) -> ClientResult<P2PAdDetailV5> {
        self.base.post_private("/v5/p2p/item/info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Orders P2P
    /// Get all P2P orders
    pub async fn get_p2_p_orders(&self, params: GetP2POrdersParamsV5) -> ClientResult<P2POrdersResponseV5> {
        self.base.post_private("/v5/p2p/order/simplifyList", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get P2P order details
    pub async fn get_p2_p_order_detail(&self, params: serde_json::Value) -> ClientResult<P2POrderDetailV5> {
        self.base.post_private("/v5/p2p/order/info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get pending P2P orders
    pub async fn get_p2_p_pending_orders(&self, params: GetP2PPendingOrdersParamsV5) -> ClientResult<P2POrdersResponseV5> {
        self.base.post_private("/v5/p2p/order/pending/simplifyList", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Mark P2P order as paid
    pub async fn mark_p2_p_order_as_paid(&self, params: MarkP2POrderAsPaidParamsV5) -> ClientResult<()> {
        self.base.post_private("/v5/p2p/order/pay", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Release digital assets in a P2P order
    pub async fn release_p2_p_order(&self, params: serde_json::Value) -> ClientResult<()> {
        self.base.post_private("/v5/p2p/order/finish", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Send chat message in a P2P order
    pub async fn send_p2_p_order_message(&self, params: SendP2POrderMessageParamsV5) -> ClientResult<()> {
        self.base.post_private("/v5/p2p/order/message/send", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Upload chat file for P2P order
    pub async fn upload_p2_p_chat_file(&self, params: serde_json::Value) -> ClientResult<()> {
        self.base.post_private("/v5/p2p/oss/upload_file", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get chat messages in a P2P order
    pub async fn get_p2_p_order_messages(&self, params: GetP2POrderMessagesParamsV5) -> ClientResult<Vec<P2POrderMessageV5>> {
        self.base.post_private("/v5/p2p/order/message/listpage", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// User P2P
    /// Get P2P user account information
    pub async fn get_p2_p_user_info(&self) -> ClientResult<P2PUserInfoV5> {
        self.base.post_private("/v5/p2p/user/personal/info", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get counterparty user information in a P2P order
    pub async fn get_p2_p_counterparty_user_info(&self, params: GetP2PCounterpartyUserInfoParamsV5) -> ClientResult<P2PCounterpartyUserInfoV5> {
        self.base.post_private("/v5/p2p/user/order/personal/info", Some(serde_json::to_value(params).unwrap_or_default())).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// Get user payment information
    pub async fn get_p2_p_user_payments(&self) -> ClientResult<Vec<P2PUserPaymentV5>> {
        self.base.post_private("/v5/p2p/user/payment/list", None).await.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))
    }

    /// ***** API Rate Limit Management APIs
    /// Set API rate limit
    /// API rate limit: 50 req per second
    /// INFO
    /// - If UID requesting this endpoint is a master account, uids in the input parameter must be subaccounts of the master account.
    /// - If UID requesting this endpoint is not a master account, uids in the input parameter must be the UID requesting this endpoint
    /// - UID requesting this endpoint must be an institutional user.
    pub async fn set_api_rate_limit(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.post_private("/v5/apilimit/set", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Query API rate limit
    /// API rate limit: 50 req per second
    /// INFO
    /// - A master account can query api rate limit of its own and subaccounts.
    /// - A subaccount can only query its own api rate limit.
    pub async fn query_api_rate_limit(&self, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/apilimit/query", Some(serde_json::to_value(params).unwrap_or_default())).await
    }

    /// Get Rate Limit Cap
    /// Get your institution's total rate limit usage and cap, across the board.
    /// API rate limit: 50 req per second
    /// Main UIDs or sub UIDs can query this endpoint, but a main UID can only see the rate limits of subs below it.
    pub async fn get_rate_limit_cap(&self) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/apilimit/query-cap", None).await
    }

    /// Get All Rate Limits
    /// Query for all your UID-level rate limits, including all master accounts and subaccounts.
    /// API rate limit: 50 req per second
    pub async fn get_all_rate_limits(&self, params: Option<serde_json::Value>) -> ClientResult<serde_json::Value> {
        self.base.get_private("/v5/apilimit/query-all", params.map(|v| serde_json::to_value(v).unwrap_or_default())).await
    }

}

