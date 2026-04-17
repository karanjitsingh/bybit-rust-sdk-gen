# Inline Type Report

## Shared inline types (35 types, defined in multiple places)

### `'inverse' | 'linear'` (17 definitions)

- category, GetMarkPriceKlineParamsV5, types/request/v5_market.rs → `GetMarkPriceKlineParamsV5_Category`
- category, GetIndexPriceKlineParamsV5, types/request/v5_market.rs → `GetIndexPriceKlineParamsV5_Category`
- category, GetFundingRateHistoryParamsV5, types/request/v5_market.rs → `GetFundingRateHistoryParamsV5_Category`
- category, GetOpenInterestParamsV5, types/request/v5_market.rs → `GetOpenInterestParamsV5_Category`
- category, GetRiskLimitParamsV5, types/request/v5_market.rs → `GetRiskLimitParamsV5_Category`
- category, GetLongShortRatioParamsV5, types/request/v5_market.rs → `GetLongShortRatioParamsV5_Category`
- category, SetLeverageParamsV5, types/request/v5_position.rs → `SetLeverageParamsV5_Category`
- category, SwitchIsolatedMarginParamsV5, types/request/v5_position.rs → `SwitchIsolatedMarginParamsV5_Category`
- category, SetTPSLModeParamsV5, types/request/v5_position.rs → `SetTPSLModeParamsV5_Category`
- category, SwitchPositionModeParamsV5, types/request/v5_position.rs → `SwitchPositionModeParamsV5_Category`
- category, SetRiskLimitParamsV5, types/request/v5_position.rs → `SetRiskLimitParamsV5_Category`
- category, AddOrReduceMarginParamsV5, types/request/v5_position.rs → `AddOrReduceMarginParamsV5_Category`
- category, ConfirmNewRiskLimitParamsV5, types/request/v5_position.rs → `ConfirmNewRiskLimitParamsV5_Category`
- category, GetPreUpgradeOrderHistoryParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeOrderHistoryParamsV5_Category`
- category, GetPreUpgradeTradeHistoryParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeTradeHistoryParamsV5_Category`
- category, GetPreUpgradeClosedPnlParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeClosedPnlParamsV5_Category`
- category, OpenInterestResponseV5, types/response/v5_market.rs → `OpenInterestResponseV5_Category`

### `'0' | '1'` (14 definitions)

- withBonus, SingleAccountCoinBalanceRequestV3, types/request/account_asset.rs → `SingleAccountCoinBalanceRequestV3_WithBonus`
- withBonus, AccountCoinBalancesRequestV3, types/request/account_asset.rs → `AccountCoinBalancesRequestV3_WithBonus`
- direction, AdjustCollateralAmountParamsV5, types/request/v5_crypto_loan.rs → `AdjustCollateralAmountParamsV5_Direction`
- side, GetP2POnlineAdsParamsV5, types/request/v5_p2p_trading.rs → `GetP2POnlineAdsParamsV5_Side`
- side, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `CreateP2PAdParamsV5_Side`
- priceType, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `CreateP2PAdParamsV5_PriceType`
- priceType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `UpdateP2PAdParamsV5_PriceType`
- side, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs → `GetP2PPersonalAdsParamsV5_Side`
- autoRepayMode, SetAutoRepayModeParamsV5, types/request/v5_spot_leverage_token.rs → `SetAutoRepayModeParamsV5_AutoRepayMode`
- innovation, SpotInstrumentInfoV5, types/response/v5_market.rs → `SpotInstrumentInfoV5_Innovation`
- stTag, SpotInstrumentInfoV5, types/response/v5_market.rs → `SpotInstrumentInfoV5_StTag`
- spotMarginMode, SpotMarginStateV5, types/response/v5_spot_leverage_token.rs → `SpotMarginStateV5_SpotMarginMode`
- autoRepayMode, AutoRepayModeItemV5, types/response/v5_spot_leverage_token.rs → `AutoRepayModeItemV5_AutoRepayMode`
- isLeverage, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_IsLeverage`

### `'Buy' | 'Sell'` (12 definitions)

- side, ContractSetAutoAddMarginRequest, types/request/contract.rs → `ContractSetAutoAddMarginRequest_Side`
- side, MovePositionParamsV5_List, types/request/v5_position.rs → `MovePositionParamsV5_List_Side`
- side, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs → `SubmitSpreadOrderParamsV5_Side`
- side, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_Side`
- side, ClosedOptionsPositionV5, types/response/v5_position.rs → `ClosedOptionsPositionV5_Side`
- side, PreUpgradeOptionsDelivery, types/response/v5_preupgrade.rs → `PreUpgradeOptionsDelivery_Side`
- side, PreUpgradeUSDCSessionSettlement, types/response/v5_preupgrade.rs → `PreUpgradeUSDCSessionSettlement_Side`
- side, SpreadRecentTradeV5, types/response/v5_spreadtrading.rs → `SpreadRecentTradeV5_Side`
- side, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_Side`
- side, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_Side`
- side, SpreadTradeLegV5, types/response/v5_spreadtrading.rs → `SpreadTradeLegV5_Side`
- side, SpreadTradeV5, types/response/v5_spreadtrading.rs → `SpreadTradeV5_Side`

### `'Full' | 'Partial'` (8 definitions)

- tpslMode, ContractOrderRequest, types/request/contract.rs → `ContractOrderRequest_TpslMode`
- tpslMode, ContractSetTPSLRequest, types/request/contract.rs → `ContractSetTPSLRequest_TpslMode`
- tp_sl_mode, InverseSetSlTpPositionModeRequest, types/request/inverse.rs → `InverseSetSlTpPositionModeRequest_Tp_sl_mode`
- tp_sl_mode, LinearSetPositionTpSlModeRequest, types/request/linear.rs → `LinearSetPositionTpSlModeRequest_Tp_sl_mode`
- tpslMode, OrderParamsV5, types/request/v5_trade.rs → `OrderParamsV5_TpslMode`
- tpslMode, AmendOrderParamsV5, types/request/v5_trade.rs → `AmendOrderParamsV5_TpslMode`
- tpslMode, BatchOrderParamsV5, types/request/v5_trade.rs → `BatchOrderParamsV5_TpslMode`
- tpslMode, BatchAmendOrderParamsV5, types/request/v5_trade.rs → `BatchAmendOrderParamsV5_TpslMode`

### `'linear' | 'option' | 'spot'` (7 definitions)

- category, GetMovePositionHistoryParamsV5, types/request/v5_position.rs → `GetMovePositionHistoryParamsV5_Category`
- category, RFQQuoteV5, types/request/v5_rfq.rs → `RFQQuoteV5_Category`
- category, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_Category`
- category, RFQLegV5, types/response/v5_rfq.rs → `RFQLegV5_Category`
- category, QuoteLegV5, types/response/v5_rfq.rs → `QuoteLegV5_Category`
- category, RFQTradeLegV5, types/response/v5_rfq.rs → `RFQTradeLegV5_Category`
- category, RFQPublicTradeLegV5, types/response/v5_rfq.rs → `RFQPublicTradeLegV5_Category`

### `'buy' | 'sell'` (6 definitions)

- side, RFQTransactionV5, types/request/v5_rfq.rs → `RFQTransactionV5_Side`
- quoteSide, ExecuteRFQQuoteParamsV5, types/request/v5_rfq.rs → `ExecuteRFQQuoteParamsV5_QuoteSide`
- side, RFQLegV5, types/response/v5_rfq.rs → `RFQLegV5_Side`
- side, RFQTradeLegV5, types/response/v5_rfq.rs → `RFQTradeLegV5_Side`
- quoteSide, RFQTradeV5, types/response/v5_rfq.rs → `RFQTradeV5_QuoteSide`
- side, RFQPublicTradeLegV5, types/response/v5_rfq.rs → `RFQPublicTradeLegV5_Side`

### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` (4 definitions)

- status, GetRFQListParamsV5, types/request/v5_rfq.rs → `GetRFQListParamsV5_Status`
- status, GetRFQHistoryParamsV5, types/request/v5_rfq.rs → `GetRFQHistoryParamsV5_Status`
- status, RFQItemV5, types/response/v5_rfq.rs → `RFQItemV5_Status`
- status, RFQQuoteItemV5, types/response/v5_rfq.rs → `RFQQuoteItemV5_Status`

### `'inverse' | 'linear' | 'spot'` (3 definitions)

- category, GetAccountInstrumentsInfoParamsV5, types/request/v5_account.rs → `GetAccountInstrumentsInfoParamsV5_Category`
- category, GetKlineParamsV5, types/request/v5_market.rs → `GetKlineParamsV5_Category`
- category, GetRPIOrderbookParamsV5, types/request/v5_market.rs → `GetRPIOrderbookParamsV5_Category`

### `'inverse' | 'linear' | 'option' | 'spot'` (3 definitions)

- category, MovePositionParamsV5_List, types/request/v5_position.rs → `MovePositionParamsV5_List_Category`
- category, RFQTransactionV5, types/request/v5_rfq.rs → `RFQTransactionV5_Category`
- category, WsTopicRequest, util/mod.rs → `WsTopicRequest_Category`

### `'quote' | 'request'` (3 definitions)

- traderType, GetRFQRealtimeParamsV5, types/request/v5_rfq.rs → `GetRFQRealtimeParamsV5_TraderType`
- traderType, GetRFQQuoteRealtimeParamsV5, types/request/v5_rfq.rs → `GetRFQQuoteRealtimeParamsV5_TraderType`
- traderType, GetRFQHistoryParamsV5, types/request/v5_rfq.rs → `GetRFQHistoryParamsV5_TraderType`

### `'Limit' | 'Market'` (3 definitions)

- orderType, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs → `SubmitSpreadOrderParamsV5_OrderType`
- orderType, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_OrderType`
- orderType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_OrderType`

### `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` (3 definitions)

- timeInForce, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs → `SubmitSpreadOrderParamsV5_TimeInForce`
- timeInForce, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_TimeInForce`
- timeInForce, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_TimeInForce`

### `'baseCoin' | 'quoteCoin'` (3 definitions)

- marketUnit, OrderParamsV5, types/request/v5_trade.rs → `OrderParamsV5_MarketUnit`
- marketUnit, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_MarketUnit`
- marketUnit, WSAccountOrderV5, types/websockets/ws_events.rs → `WSAccountOrderV5_MarketUnit`

### `'Fail' | 'Pending' | 'Success'` (3 definitions)

- status, EarnOrderHistoryV5, types/response/v5_earn.rs → `EarnOrderHistoryV5_Status`
- status, EarnYieldHistoryV5, types/response/v5_earn.rs → `EarnYieldHistoryV5_Status`
- status, EarnHourlyYieldHistoryV5, types/response/v5_earn.rs → `EarnHourlyYieldHistoryV5_Status`

### `'1' | '2'` (2 definitions)

- triggerDirection, ContractOrderRequest, types/request/contract.rs → `ContractOrderRequest_TriggerDirection`
- status, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs → `GetP2PPersonalAdsParamsV5_Status`

### `'0' | '1' | '2'` (2 definitions)

- positionIdx, ContractOrderRequest, types/request/contract.rs → `ContractOrderRequest_PositionIdx`
- positionIdx, UMOrderRequest, types/request/unified_margin.rs → `UMOrderRequest_PositionIdx`

### `'IndexPrice' | 'LastPrice' | 'MarkPrice'` (2 definitions)

- tp_trigger_by, InverseOrderRequest, types/request/inverse.rs → `InverseOrderRequest_Tp_trigger_by`
- sl_trigger_by, InverseOrderRequest, types/request/inverse.rs → `InverseOrderRequest_Sl_trigger_by`

### `'Call' | 'Put'` (2 definitions)

- optionType, UMPublicTradesRequest, types/request/unified_margin.rs → `UMPublicTradesRequest_OptionType`
- optionType, USDCOptionsRecentTradesRequest, types/request/usdc_options.rs → `USDCOptionsRecentTradesRequest_OptionType`

### `'eb_convert_contract' | 'eb_convert_funding' | 'eb_convert_inverse' | 'eb_convert_spot' | 'eb_convert_uta'` (2 definitions)

- accountType, ConvertCoinsParamsV5, types/request/v5_asset.rs → `ConvertCoinsParamsV5_AccountType`
- accountType, RequestConvertQuoteParamsV5, types/request/v5_asset.rs → `RequestConvertQuoteParamsV5_AccountType`

### `'CONVERT' | 'DERIVATIVES' | 'OPTIONS' | 'SPOT'` (2 definitions)

- bizType, GetExchangeBrokerEarningsParamsV5, types/request/v5_broker.rs → `GetExchangeBrokerEarningsParamsV5_BizType`
- bizType, EarningDetailV5, types/response/v5_broker.rs → `EarningDetailV5_BizType`

### `'apy' | 'quantity' | 'term'` (2 definitions)

- orderBy, GetSupplyOrderQuoteFixedParamsV5, types/request/v5_crypto_loan.rs → `GetSupplyOrderQuoteFixedParamsV5_OrderBy`
- orderBy, GetBorrowOrderQuoteFixedParamsV5, types/request/v5_crypto_loan.rs → `GetBorrowOrderQuoteFixedParamsV5_OrderBy`

### `'Redeem' | 'Stake'` (2 definitions)

- orderType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs → `SubmitStakeRedeemParamsV5_OrderType`
- orderType, EarnOrderHistoryV5, types/response/v5_earn.rs → `EarnOrderHistoryV5_OrderType`

### `'FUND' | 'UNIFIED'` (2 definitions)

- accountType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs → `SubmitStakeRedeemParamsV5_AccountType`
- toAccountType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs → `SubmitStakeRedeemParamsV5_ToAccountType`

### `'BULK' | 'ORIGIN'` (2 definitions)

- itemType, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `CreateP2PAdParamsV5_ItemType`
- itemType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `UpdateP2PAdParamsV5_ItemType`

### `'Filled' | 'Processing' | 'Rejected'` (2 definitions)

- status, GetMovePositionHistoryParamsV5, types/request/v5_position.rs → `GetMovePositionHistoryParamsV5_Status`
- status, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_Status`

### `'Filled' | 'Rejected'` (2 definitions)

- status, GetRFQTradeListParamsV5, types/request/v5_rfq.rs → `GetRFQTradeListParamsV5_Status`
- status, RFQTradeV5, types/response/v5_rfq.rs → `RFQTradeV5_Status`

### `'ACCOUNT_STATUS_NORMAL' | 'ACCOUNT_STATUS_UNSPECIFIED'` (2 definitions)

- status, AssetInfoResponseV3_Spot, types/response/account_asset.rs → `AssetInfoResponseV3_Spot_Status`
- status, AssetInfoV5, types/response/v5_asset.rs → `AssetInfoV5_Status`

### `'FA' | 'P' | 'SU'` (2 definitions)

- resultStatus, ManualRepayResultV5, types/response/v5_account.rs → `ManualRepayResultV5_ResultStatus`
- resultStatus, ManualRepayWithoutConversionResultV5, types/response/v5_spot_leverage_token.rs → `ManualRepayWithoutConversionResultV5_ResultStatus`

### `'failure' | 'init' | 'processing' | 'success'` (2 definitions)

- exchangeStatus, ConvertStatusV5, types/response/v5_asset.rs → `ConvertStatusV5_ExchangeStatus`
- exchangeStatus, ConvertHistoryRecordV5, types/response/v5_asset.rs → `ConvertHistoryRecordV5_ExchangeStatus`

### `'Processing' | 'Rejected'` (2 definitions)

- status, MovePositionResultV5, types/response/v5_position.rs → `MovePositionResultV5_Status`
- status, ExecuteRFQQuoteResultV5, types/response/v5_rfq.rs → `ExecuteRFQQuoteResultV5_Status`

### `'' | 'Maker' | 'Taker' | 'bybit'` (2 definitions)

- rejectParty, MovePositionResultV5, types/response/v5_position.rs → `MovePositionResultV5_RejectParty`
- rejectParty, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_RejectParty`

### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled'` (2 definitions)

- status, CreateRFQResultV5, types/response/v5_rfq.rs → `CreateRFQResultV5_Status`
- status, CreateRFQQuoteResultV5, types/response/v5_rfq.rs → `CreateRFQQuoteResultV5_Status`

### `'CarryTrade' | 'FundingRateArb' | 'FutureSpread' | 'PerpBasis'` (2 definitions)

- contractType, SpreadInstrumentInfoV5, types/response/v5_spreadtrading.rs → `SpreadInstrumentInfoV5_ContractType`
- contractType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_ContractType`

### `'Futures' | 'Spot'` (2 definitions)

- leg1ProdType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_Leg1ProdType`
- leg2ProdType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_Leg2ProdType`

### `'combination' | 'future_leg' | 'spot_leg'` (2 definitions)

- category, WSSpreadOrderV5, types/websockets/ws_events.rs → `WSSpreadOrderV5_Category`
- category, WSSpreadExecutionV5, types/websockets/ws_events.rs → `WSSpreadExecutionV5_Category`

## Unique inline types (132, single definition)

- `'0' | '1'` → withBonus, AccountCoinBalancesRequestV3, types/request/account_asset.rs → `AccountCoinBalancesRequestV3_WithBonus`
- `'1' | '6'` → memberType, CreateSubMemberRequestV3, types/request/account_asset.rs → `CreateSubMemberRequestV3_MemberType`
- `'0' | '3'` → mode, ContractSetPositionModeRequest, types/request/contract.rs → `ContractSetPositionModeRequest_Mode`
- `'Full' | 'Partial'` → tpslMode, ContractSetTPSLRequest, types/request/contract.rs → `ContractSetTPSLRequest_TpslMode`
- `'AdlTrade' | 'BustTrade' | 'Funding' | 'Trade'` → execType, ContractUserExecutionHistoryRequest, types/request/contract.rs → `ContractUserExecutionHistoryRequest_ExecType`
- `'IndexPrice' | 'LastPrice' | 'MarkPrice'` → sl_trigger_by, InverseOrderRequest, types/request/inverse.rs → `InverseOrderRequest_SlTriggerBy`
- `'Full' | 'Partial'` → tp_sl_mode, InverseSetSlTpPositionModeRequest, types/request/inverse.rs → `InverseSetSlTpPositionModeRequest_TpSlMode`
- `'BothSide' | 'MergedSingle'` → mode, LinearSetPositionModeRequest, types/request/linear.rs → `LinearSetPositionModeRequest_Mode`
- `'Full' | 'Partial'` → tp_sl_mode, LinearSetPositionTpSlModeRequest, types/request/linear.rs → `LinearSetPositionTpSlModeRequest_TpSlMode`
- `'15min' | '1d' | '1h' | '30min' | '4h' | '5min'` → interval, UMOpenInterestRequest, types/request/unified_margin.rs → `UMOpenInterestRequest_Interval`
- `'0' | '1' | '2'` → positionIdx, UMOrderRequest, types/request/unified_margin.rs → `UMOrderRequest_PositionIdx`
- `'DELIVERING' | 'OFFLINE' | 'ONLINE' | 'WAITING_ONLINE'` → status, USDCOptionsContractInfoRequest, types/request/usdc_options.rs → `USDCOptionsContractInfoRequest_Status`
- `'Call' | 'Put'` → optionType, USDCOptionsRecentTradesRequest, types/request/usdc_options.rs → `USDCOptionsRecentTradesRequest_OptionType`
- `'OFF' | 'ON'` → collateralSwitch, SetCollateralCoinParamsV5, types/request/v5_account.rs → `SetCollateralCoinParamsV5_CollateralSwitch`
- `'FUND' | 'SPOT'` → accountType, WithdrawParamsV5, types/request/v5_asset.rs → `WithdrawParamsV5_AccountType`
- `'eb_convert_contract' | 'eb_convert_funding' | 'eb_convert_inverse' | 'eb_convert_spot' | 'eb_convert_uta'` → accountType, RequestConvertQuoteParamsV5, types/request/v5_asset.rs → `RequestConvertQuoteParamsV5_AccountType`
- `'0' | '1'` → direction, AdjustCollateralAmountParamsV5, types/request/v5_crypto_loan.rs → `AdjustCollateralAmountParamsV5_Direction`
- `'apy' | 'quantity' | 'term'` → orderBy, GetBorrowOrderQuoteFixedParamsV5, types/request/v5_crypto_loan.rs → `GetBorrowOrderQuoteFixedParamsV5_OrderBy`
- `'FUND' | 'UNIFIED'` → toAccountType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs → `SubmitStakeRedeemParamsV5_ToAccountType`
- `'inverse' | 'linear' | 'spot'` → category, GetKlineParamsV5, types/request/v5_market.rs → `GetKlineParamsV5_Category`
- `'inverse' | 'linear'` → category, GetIndexPriceKlineParamsV5, types/request/v5_market.rs → `GetIndexPriceKlineParamsV5_Category`
- `'inverse' | 'linear' | 'spot'` → category, GetRPIOrderbookParamsV5, types/request/v5_market.rs → `GetRPIOrderbookParamsV5_Category`
- `'inverse' | 'linear'` → category, GetFundingRateHistoryParamsV5, types/request/v5_market.rs → `GetFundingRateHistoryParamsV5_Category`
- `'inverse' | 'linear'` → category, GetOpenInterestParamsV5, types/request/v5_market.rs → `GetOpenInterestParamsV5_Category`
- `'14' | '180' | '21' | '270' | '30' | '60' | '7' | '90'` → period, GetHistoricalVolatilityParamsV5, types/request/v5_market.rs → `GetHistoricalVolatilityParamsV5_Period`
- `'inverse' | 'linear'` → category, GetRiskLimitParamsV5, types/request/v5_market.rs → `GetRiskLimitParamsV5_Category`
- `'inverse' | 'linear' | 'option'` → category, GetDeliveryPriceParamsV5, types/request/v5_market.rs → `GetDeliveryPriceParamsV5_Category`
- `'inverse' | 'linear'` → category, GetLongShortRatioParamsV5, types/request/v5_market.rs → `GetLongShortRatioParamsV5_Category`
- `'0' | '1'` → side, GetP2POnlineAdsParamsV5, types/request/v5_p2p_trading.rs → `GetP2POnlineAdsParamsV5_Side`
- `'0' | '1'` → side, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `CreateP2PAdParamsV5_Side`
- `'0' | '1'` → priceType, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `CreateP2PAdParamsV5_PriceType`
- `'0' | '1'` → priceType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `UpdateP2PAdParamsV5_PriceType`
- `'ACTIVE' | 'MODIFY'` → actionType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `UpdateP2PAdParamsV5_ActionType`
- `'BULK' | 'ORIGIN'` → itemType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs → `UpdateP2PAdParamsV5_ItemType`
- `'1' | '2'` → status, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs → `GetP2PPersonalAdsParamsV5_Status`
- `'0' | '1'` → side, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs → `GetP2PPersonalAdsParamsV5_Side`
- `'inverse' | 'linear'` → category, SetLeverageParamsV5, types/request/v5_position.rs → `SetLeverageParamsV5_Category`
- `'inverse' | 'linear'` → category, SwitchIsolatedMarginParamsV5, types/request/v5_position.rs → `SwitchIsolatedMarginParamsV5_Category`
- `'inverse' | 'linear'` → category, SetTPSLModeParamsV5, types/request/v5_position.rs → `SetTPSLModeParamsV5_Category`
- `'inverse' | 'linear'` → category, SwitchPositionModeParamsV5, types/request/v5_position.rs → `SwitchPositionModeParamsV5_Category`
- `'inverse' | 'linear'` → category, SetRiskLimitParamsV5, types/request/v5_position.rs → `SetRiskLimitParamsV5_Category`
- `'inverse' | 'linear'` → category, AddOrReduceMarginParamsV5, types/request/v5_position.rs → `AddOrReduceMarginParamsV5_Category`
- `'Buy' | 'Sell'` → side, MovePositionParamsV5_List, types/request/v5_position.rs → `MovePositionParamsV5_List_Side`
- `'inverse' | 'linear'` → category, ConfirmNewRiskLimitParamsV5, types/request/v5_position.rs → `ConfirmNewRiskLimitParamsV5_Category`
- `'inverse' | 'linear'` → category, GetPreUpgradeOrderHistoryParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeOrderHistoryParamsV5_Category`
- `'Order' | 'StopOrder'` → orderFilter, GetPreUpgradeOrderHistoryParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeOrderHistoryParamsV5_OrderFilter`
- `'inverse' | 'linear'` → category, GetPreUpgradeTradeHistoryParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeTradeHistoryParamsV5_Category`
- `'inverse' | 'linear'` → category, GetPreUpgradeClosedPnlParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeClosedPnlParamsV5_Category`
- `'linear' | 'option'` → category, GetPreUpgradeTransactionLogParamsV5, types/request/v5_pre_upgrade.rs → `GetPreUpgradeTransactionLogParamsV5_Category`
- `'inverse' | 'linear' | 'option' | 'spot'` → category, RFQTransactionV5, types/request/v5_rfq.rs → `RFQTransactionV5_Category`
- `'linear' | 'option' | 'spot'` → category, RFQQuoteV5, types/request/v5_rfq.rs → `RFQQuoteV5_Category`
- `'buy' | 'sell'` → quoteSide, ExecuteRFQQuoteParamsV5, types/request/v5_rfq.rs → `ExecuteRFQQuoteParamsV5_QuoteSide`
- `'quoter' | 'request'` → traderType, GetRFQListParamsV5, types/request/v5_rfq.rs → `GetRFQListParamsV5_TraderType`
- `'quote' | 'request'` → traderType, GetRFQQuoteRealtimeParamsV5, types/request/v5_rfq.rs → `GetRFQQuoteRealtimeParamsV5_TraderType`
- `'quote' | 'request'` → traderType, GetRFQHistoryParamsV5, types/request/v5_rfq.rs → `GetRFQHistoryParamsV5_TraderType`
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` → status, GetRFQHistoryParamsV5, types/request/v5_rfq.rs → `GetRFQHistoryParamsV5_Status`
- `'0' | '1'` → autoRepayMode, SetAutoRepayModeParamsV5, types/request/v5_spot_leverage_token.rs → `SetAutoRepayModeParamsV5_AutoRepayMode`
- `'Buy' | 'Sell'` → side, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs → `SubmitSpreadOrderParamsV5_Side`
- `'Full' | 'Partial'` → tpslMode, OrderParamsV5, types/request/v5_trade.rs → `OrderParamsV5_TpslMode`
- `'Counterparty' | 'Queue'` → bboSideType, OrderParamsV5, types/request/v5_trade.rs → `OrderParamsV5_BboSideType`
- `'1' | '2' | '3' | '4' | '5'` → bboLevel, OrderParamsV5, types/request/v5_trade.rs → `OrderParamsV5_BboLevel`
- `'Full' | 'Partial'` → tpslMode, AmendOrderParamsV5, types/request/v5_trade.rs → `AmendOrderParamsV5_TpslMode`
- `'Full' | 'Partial'` → tpslMode, BatchOrderParamsV5, types/request/v5_trade.rs → `BatchOrderParamsV5_TpslMode`
- `'Full' | 'Partial'` → tpslMode, BatchAmendOrderParamsV5, types/request/v5_trade.rs → `BatchAmendOrderParamsV5_TpslMode`
- `'IN' | 'OUT'` → type, SubAccountTransferResponseV3_List, types/response/account_asset.rs → `SubAccountTransferResponseV3_List_Type`
- `'1' | '2' | '4'` → status, SubMemberV3, types/response/account_asset.rs → `SubMemberV3_Status`
- `'DERIVATIVES' | 'OPTIONS' | 'SPOT'` → product, DCPInfoV5, types/response/v5_account.rs → `DCPInfoV5_Product`
- `'ACCOUNT_STATUS_NORMAL' | 'ACCOUNT_STATUS_UNSPECIFIED'` → status, AssetInfoV5, types/response/v5_asset.rs → `AssetInfoV5_Status`
- `'1' | '2' | '3'` → status, InternalDepositRecordV5, types/response/v5_asset.rs → `InternalDepositRecordV5_Status`
- `'failure' | 'init' | 'processing' | 'success'` → exchangeStatus, ConvertHistoryRecordV5, types/response/v5_asset.rs → `ConvertHistoryRecordV5_ExchangeStatus`
- `'CONVERT' | 'DERIVATIVES' | 'OPTIONS' | 'SPOT'` → bizType, EarningDetailV5, types/response/v5_broker.rs → `EarningDetailV5_BizType`
- `'AWARD_AMOUNT_UNIT_COIN' | 'AWARD_AMOUNT_UNIT_USD'` → amountUnit, BrokerVoucherSpecV5, types/response/v5_broker.rs → `BrokerVoucherSpecV5_AmountUnit`
- `'Available' | 'NotAvailable'` → status, EarnProductV5, types/response/v5_earn.rs → `EarnProductV5_Status`
- `'Redeem' | 'Stake'` → orderType, EarnOrderHistoryV5, types/response/v5_earn.rs → `EarnOrderHistoryV5_OrderType`
- `'Fail' | 'Pending' | 'Success'` → status, EarnYieldHistoryV5, types/response/v5_earn.rs → `EarnYieldHistoryV5_Status`
- `'Fail' | 'Pending' | 'Success'` → status, EarnHourlyYieldHistoryV5, types/response/v5_earn.rs → `EarnHourlyYieldHistoryV5_Status`
- `'0' | '1'` → innovation, SpotInstrumentInfoV5, types/response/v5_market.rs → `SpotInstrumentInfoV5_Innovation`
- `'0' | '1'` → stTag, SpotInstrumentInfoV5, types/response/v5_market.rs → `SpotInstrumentInfoV5_StTag`
- `'inverse' | 'linear'` → category, OpenInterestResponseV5, types/response/v5_market.rs → `OpenInterestResponseV5_Category`
- `'linear' | 'option' | 'spot'` → category, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_Category`
- `'Buy' | 'Sell'` → side, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_Side`
- `'Filled' | 'Processing' | 'Rejected'` → status, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_Status`
- `'' | 'Maker' | 'Taker' | 'bybit'` → rejectParty, MovePositionHistoryV5, types/response/v5_position.rs → `MovePositionHistoryV5_RejectParty`
- `'Buy' | 'Sell'` → side, ClosedOptionsPositionV5, types/response/v5_position.rs → `ClosedOptionsPositionV5_Side`
- `'Buy' | 'None' | 'Sell'` → side, PreUpgradeTransaction, types/response/v5_preupgrade.rs → `PreUpgradeTransaction_Side`
- `'BTC' | 'ETH' | 'USDC' | 'USDT'` → currency, PreUpgradeTransaction, types/response/v5_preupgrade.rs → `PreUpgradeTransaction_Currency`
- `'Buy' | 'Sell'` → side, PreUpgradeOptionsDelivery, types/response/v5_preupgrade.rs → `PreUpgradeOptionsDelivery_Side`
- `'Buy' | 'Sell'` → side, PreUpgradeUSDCSessionSettlement, types/response/v5_preupgrade.rs → `PreUpgradeUSDCSessionSettlement_Side`
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled'` → status, CreateRFQQuoteResultV5, types/response/v5_rfq.rs → `CreateRFQQuoteResultV5_Status`
- `'Processing' | 'Rejected'` → status, ExecuteRFQQuoteResultV5, types/response/v5_rfq.rs → `ExecuteRFQQuoteResultV5_Status`
- `'linear' | 'option' | 'spot'` → category, RFQLegV5, types/response/v5_rfq.rs → `RFQLegV5_Category`
- `'buy' | 'sell'` → side, RFQLegV5, types/response/v5_rfq.rs → `RFQLegV5_Side`
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` → status, RFQItemV5, types/response/v5_rfq.rs → `RFQItemV5_Status`
- `'linear' | 'option' | 'spot'` → category, QuoteLegV5, types/response/v5_rfq.rs → `QuoteLegV5_Category`
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` → status, RFQQuoteItemV5, types/response/v5_rfq.rs → `RFQQuoteItemV5_Status`
- `'linear' | 'option' | 'spot'` → category, RFQTradeLegV5, types/response/v5_rfq.rs → `RFQTradeLegV5_Category`
- `'buy' | 'sell'` → side, RFQTradeLegV5, types/response/v5_rfq.rs → `RFQTradeLegV5_Side`
- `'buy' | 'sell'` → quoteSide, RFQTradeV5, types/response/v5_rfq.rs → `RFQTradeV5_QuoteSide`
- `'Filled' | 'Rejected'` → status, RFQTradeV5, types/response/v5_rfq.rs → `RFQTradeV5_Status`
- `'linear' | 'option' | 'spot'` → category, RFQPublicTradeLegV5, types/response/v5_rfq.rs → `RFQPublicTradeLegV5_Category`
- `'buy' | 'sell'` → side, RFQPublicTradeLegV5, types/response/v5_rfq.rs → `RFQPublicTradeLegV5_Side`
- `'0' | '1'` → spotMarginMode, SpotMarginStateV5, types/response/v5_spot_leverage_token.rs → `SpotMarginStateV5_SpotMarginMode`
- `'FA' | 'P' | 'SU'` → resultStatus, ManualRepayWithoutConversionResultV5, types/response/v5_spot_leverage_token.rs → `ManualRepayWithoutConversionResultV5_ResultStatus`
- `'0' | '1'` → autoRepayMode, AutoRepayModeItemV5, types/response/v5_spot_leverage_token.rs → `AutoRepayModeItemV5_AutoRepayMode`
- `'Settling' | 'Trading'` → status, SpreadInstrumentInfoV5, types/response/v5_spreadtrading.rs → `SpreadInstrumentInfoV5_Status`
- `'LinearFutures' | 'LinearPerpetual' | 'Spot'` → contractType, SpreadInstrumentInfoV5_Legs, types/response/v5_spreadtrading.rs → `SpreadInstrumentInfoV5_Legs_ContractType`
- `'Buy' | 'Sell'` → side, SpreadRecentTradeV5, types/response/v5_spreadtrading.rs → `SpreadRecentTradeV5_Side`
- `'Limit' | 'Market'` → orderType, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_OrderType`
- `'Buy' | 'Sell'` → side, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_Side`
- `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` → timeInForce, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_TimeInForce`
- `'New' | 'PartiallyFilled'` → orderStatus, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs → `SpreadOpenOrderV5_OrderStatus`
- `'Limit' | 'Market'` → orderType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_OrderType`
- `'CarryTrade' | 'FundingRateArb' | 'FutureSpread' | 'PerpBasis'` → contractType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_ContractType`
- `'Cancelled' | 'Filled' | 'Rejected'` → orderStatus, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_OrderStatus`
- `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` → timeInForce, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_TimeInForce`
- `'Buy' | 'Sell'` → side, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_Side`
- `'Futures' | 'Spot'` → leg2ProdType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs → `SpreadOrderHistoryV5_Leg2ProdType`
- `'Buy' | 'Sell'` → side, SpreadTradeLegV5, types/response/v5_spreadtrading.rs → `SpreadTradeLegV5_Side`
- `'linear' | 'spot'` → category, SpreadTradeLegV5, types/response/v5_spreadtrading.rs → `SpreadTradeLegV5_Category`
- `'Buy' | 'Sell'` → side, SpreadTradeV5, types/response/v5_spreadtrading.rs → `SpreadTradeV5_Side`
- `'0' | '1'` → isLeverage, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_Leverage`
- `'baseCoin' | 'quoteCoin'` → marketUnit, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_MarketUnit`
- `'' | 'Full' | 'Partial'` → tpslMode, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_TpslMode`
- `'' | 'OcoTriggerBySl' | 'OcoTriggerByUnknown' | 'OcoTriggerTp'` → ocoTriggerType, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_OcoTriggerType`
- `'' | 'iv' | 'price'` → placeType, AccountOrderV5, types/response/v5_trade.rs → `AccountOrderV5_PlaceType`
- `'LEVEL_1' | 'LEVEL_2' | 'LEVEL_DEFAULT'` → kycLevel, ApiKeyInfoV5, types/response/v5_user.rs → `ApiKeyInfoV5_KycLevel`
- `'1' | '2' | '3' | '4'` → totalWalletBalance, AffiliateUserInfoV5, types/response/v5_user.rs → `AffiliateUserInfoV5_TotalWalletBalance`
- `'TWSTopic' | 'number' | 'string'` → args, WsRequestOperationBybit, types/websockets/ws_api.rs → `WsRequestOperationBybit_Args`
- `'baseCoin' | 'quoteCoin'` → marketUnit, WSAccountOrderV5, types/websockets/ws_events.rs → `WSAccountOrderV5_MarketUnit`
- `'combination' | 'future_leg' | 'spot_leg'` → category, WSSpreadExecutionV5, types/websockets/ws_events.rs → `WSSpreadExecutionV5_Category`
- `'inverse' | 'linear' | 'option' | 'spot'` → category, WsTopicRequest, util/mod.rs → `WsTopicRequest_Category`
- `'TEventType' | 'connectionReady' | 'connectionReadyForAuth' | 'pong'` → eventType, EmittableEvent, util/mod.rs → `EmittableEvent_EventType`