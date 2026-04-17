# Inline Type Report

## Shared signatures without overrides (40, candidates for shared-types.json)

### `0 | 1` (32 definitions)

- forceChain, WithdrawCreateRequestV3, types/request/account_asset.rs
- switch, CreateSubMemberRequestV3, types/request/account_asset.rs
- readOnly, CreateSubAPIKeyRequestV3, types/request/account_asset.rs
- autoAddMargin, ContractSetAutoAddMarginRequest, types/request/contract.rs
- tradeMode, ContractSetMarginSwitchRequest, types/request/contract.rs
- withTransferSafeAmount, GetAccountCoinBalanceParamsV5, types/request/v5_asset.rs
- withLtvTransferSafeAmount, GetAccountCoinBalanceParamsV5, types/request/v5_asset.rs
- feeType, WithdrawParamsV5, types/request/v5_asset.rs
- side, GetFiatTradingPairListParamsV5, types/request/v5_asset.rs
- withBonus, GetP2PAccountCoinsBalanceParamsV5, types/request/v5_p2p_trading.rs
- hasUnPostAd, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- isKyc, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- isEmail, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- isMobile, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- hasRegisterTime, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- hasOrderFinishNumberDay30, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- hasCompleteRateDay30, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- hasNationalLimit, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- tradeMode, SwitchIsolatedMarginParamsV5, types/request/v5_position.rs
- autoAddMargin, SetAutoAddMarginParamsV5, types/request/v5_position.rs
- isLeverage, OrderParamsV5, types/request/v5_trade.rs
- isLeverage, BatchOrderParamsV5, types/request/v5_trade.rs
- switch, CreateSubMemberParamsV5, types/request/v5_user.rs
- readOnly, CreateSubApiKeyParamsV5, types/request/v5_user.rs
- readOnly, UpdateApiKeyParamsV5, types/request/v5_user.rs
- switch, CreateSubMemberResponseV3, types/response/account_asset.rs
- isLowestRisk, RiskLimitV5, types/response/v5_market.rs
- autoAddMargin, AddOrReduceMarginResultV5, types/response/v5_position.rs
- readOnly, ApiKeyInfoV5, types/response/v5_user.rs
- uta, ApiKeyInfoV5, types/response/v5_user.rs
- readOnly, UpdateApiKeyResultV5, types/response/v5_user.rs
- readOnly, SubAccountAllApiKeysResultV5_Result, types/response/v5_user.rs

### `'inverse' | 'linear'` (17 definitions)

- category, GetMarkPriceKlineParamsV5, types/request/v5_market.rs
- category, GetIndexPriceKlineParamsV5, types/request/v5_market.rs
- category, GetFundingRateHistoryParamsV5, types/request/v5_market.rs
- category, GetOpenInterestParamsV5, types/request/v5_market.rs
- category, GetRiskLimitParamsV5, types/request/v5_market.rs
- category, GetLongShortRatioParamsV5, types/request/v5_market.rs
- category, SetLeverageParamsV5, types/request/v5_position.rs
- category, SwitchIsolatedMarginParamsV5, types/request/v5_position.rs
- category, SetTPSLModeParamsV5, types/request/v5_position.rs
- category, SwitchPositionModeParamsV5, types/request/v5_position.rs
- category, SetRiskLimitParamsV5, types/request/v5_position.rs
- category, AddOrReduceMarginParamsV5, types/request/v5_position.rs
- category, ConfirmNewRiskLimitParamsV5, types/request/v5_position.rs
- category, GetPreUpgradeOrderHistoryParamsV5, types/request/v5_pre_upgrade.rs
- category, GetPreUpgradeTradeHistoryParamsV5, types/request/v5_pre_upgrade.rs
- category, GetPreUpgradeClosedPnlParamsV5, types/request/v5_pre_upgrade.rs
- category, OpenInterestResponseV5, types/response/v5_market.rs

### `'0' | '1'` (14 definitions)

- withBonus, SingleAccountCoinBalanceRequestV3, types/request/account_asset.rs
- withBonus, AccountCoinBalancesRequestV3, types/request/account_asset.rs
- direction, AdjustCollateralAmountParamsV5, types/request/v5_crypto_loan.rs
- side, GetP2POnlineAdsParamsV5, types/request/v5_p2p_trading.rs
- side, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- priceType, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- priceType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- side, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs
- autoRepayMode, SetAutoRepayModeParamsV5, types/request/v5_spot_leverage_token.rs
- innovation, SpotInstrumentInfoV5, types/response/v5_market.rs
- stTag, SpotInstrumentInfoV5, types/response/v5_market.rs
- spotMarginMode, SpotMarginStateV5, types/response/v5_spot_leverage_token.rs
- autoRepayMode, AutoRepayModeItemV5, types/response/v5_spot_leverage_token.rs
- isLeverage, AccountOrderV5, types/response/v5_trade.rs

### `'Buy' | 'Sell'` (12 definitions)

- side, ContractSetAutoAddMarginRequest, types/request/contract.rs
- side, MovePositionParamsV5_List, types/request/v5_position.rs
- side, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs
- side, MovePositionHistoryV5, types/response/v5_position.rs
- side, ClosedOptionsPositionV5, types/response/v5_position.rs
- side, PreUpgradeOptionsDelivery, types/response/v5_preupgrade.rs
- side, PreUpgradeUSDCSessionSettlement, types/response/v5_preupgrade.rs
- side, SpreadRecentTradeV5, types/response/v5_spreadtrading.rs
- side, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- side, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- side, SpreadTradeLegV5, types/response/v5_spreadtrading.rs
- side, SpreadTradeV5, types/response/v5_spreadtrading.rs

### `'Full' | 'Partial'` (8 definitions)

- tpslMode, ContractOrderRequest, types/request/contract.rs
- tpslMode, ContractSetTPSLRequest, types/request/contract.rs
- tp_sl_mode, InverseSetSlTpPositionModeRequest, types/request/inverse.rs
- tp_sl_mode, LinearSetPositionTpSlModeRequest, types/request/linear.rs
- tpslMode, OrderParamsV5, types/request/v5_trade.rs
- tpslMode, AmendOrderParamsV5, types/request/v5_trade.rs
- tpslMode, BatchOrderParamsV5, types/request/v5_trade.rs
- tpslMode, BatchAmendOrderParamsV5, types/request/v5_trade.rs

### `'linear' | 'option' | 'spot'` (7 definitions)

- category, GetMovePositionHistoryParamsV5, types/request/v5_position.rs
- category, RFQQuoteV5, types/request/v5_rfq.rs
- category, MovePositionHistoryV5, types/response/v5_position.rs
- category, RFQLegV5, types/response/v5_rfq.rs
- category, QuoteLegV5, types/response/v5_rfq.rs
- category, RFQTradeLegV5, types/response/v5_rfq.rs
- category, RFQPublicTradeLegV5, types/response/v5_rfq.rs

### `0 | 1 | 2` (6 definitions)

- positionIdx, ContractSetAutoAddMarginRequest, types/request/contract.rs
- positionIdx, ContractSetTPSLRequest, types/request/contract.rs
- position_idx, LinearSetTradingStopRequest, types/request/linear.rs
- addressType, GetWithdrawalAddressListParamsV5, types/request/v5_asset.rs
- openOnly, GetAccountOrdersParamsV5, types/request/v5_trade.rs
- KycLevel, AffiliateUserInfoV5, types/response/v5_user.rs

### `'buy' | 'sell'` (6 definitions)

- side, RFQTransactionV5, types/request/v5_rfq.rs
- quoteSide, ExecuteRFQQuoteParamsV5, types/request/v5_rfq.rs
- side, RFQLegV5, types/response/v5_rfq.rs
- side, RFQTradeLegV5, types/response/v5_rfq.rs
- quoteSide, RFQTradeV5, types/response/v5_rfq.rs
- side, RFQPublicTradeLegV5, types/response/v5_rfq.rs

### `1 | 2` (6 definitions)

- triggerDirection, OrderParamsV5, types/request/v5_trade.rs
- triggerDirection, BatchOrderParamsV5, types/request/v5_trade.rs
- supportConvert, SmallBalanceCoinV5, types/response/v5_asset.rs
- triggerDirection, AccountOrderV5, types/response/v5_trade.rs
- type, ApiKeyInfoV5, types/response/v5_user.rs
- type, SubAccountAllApiKeysResultV5_Result, types/response/v5_user.rs

### `1 | 6` (4 definitions)

- memberType, CreateSubMemberRequestV3, types/request/account_asset.rs
- memberType, CreateSubMemberParamsV5, types/request/v5_user.rs
- memberType, CreateSubMemberResponseV3, types/response/account_asset.rs
- memberType, SubMemberV3, types/response/account_asset.rs

### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` (4 definitions)

- status, GetRFQListParamsV5, types/request/v5_rfq.rs
- status, GetRFQHistoryParamsV5, types/request/v5_rfq.rs
- status, RFQItemV5, types/response/v5_rfq.rs
- status, RFQQuoteItemV5, types/response/v5_rfq.rs

### `'inverse' | 'linear' | 'spot'` (3 definitions)

- category, GetAccountInstrumentsInfoParamsV5, types/request/v5_account.rs
- category, GetKlineParamsV5, types/request/v5_market.rs
- category, GetRPIOrderbookParamsV5, types/request/v5_market.rs

### `'inverse' | 'linear' | 'option' | 'spot'` (3 definitions)

- category, MovePositionParamsV5_List, types/request/v5_position.rs
- category, RFQTransactionV5, types/request/v5_rfq.rs
- category, WsTopicRequest, util/mod.rs

### `'quote' | 'request'` (3 definitions)

- traderType, GetRFQRealtimeParamsV5, types/request/v5_rfq.rs
- traderType, GetRFQQuoteRealtimeParamsV5, types/request/v5_rfq.rs
- traderType, GetRFQHistoryParamsV5, types/request/v5_rfq.rs

### `'Limit' | 'Market'` (3 definitions)

- orderType, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs
- orderType, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- orderType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs

### `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` (3 definitions)

- timeInForce, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs
- timeInForce, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- timeInForce, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs

### `'baseCoin' | 'quoteCoin'` (3 definitions)

- marketUnit, OrderParamsV5, types/request/v5_trade.rs
- marketUnit, AccountOrderV5, types/response/v5_trade.rs
- marketUnit, WSAccountOrderV5, types/websockets/ws_events.rs

### `'Fail' | 'Pending' | 'Success'` (3 definitions)

- status, EarnOrderHistoryV5, types/response/v5_earn.rs
- status, EarnYieldHistoryV5, types/response/v5_earn.rs
- status, EarnHourlyYieldHistoryV5, types/response/v5_earn.rs

### `'1' | '2'` (2 definitions)

- triggerDirection, ContractOrderRequest, types/request/contract.rs
- status, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs

### `'0' | '1' | '2'` (2 definitions)

- positionIdx, ContractOrderRequest, types/request/contract.rs
- positionIdx, UMOrderRequest, types/request/unified_margin.rs

### `0 | 3` (2 definitions)

- mode, ContractSetPositionModeRequest, types/request/contract.rs
- mode, SwitchPositionModeParamsV5, types/request/v5_position.rs

### `'IndexPrice' | 'LastPrice' | 'MarkPrice'` (2 definitions)

- tp_trigger_by, InverseOrderRequest, types/request/inverse.rs
- sl_trigger_by, InverseOrderRequest, types/request/inverse.rs

### `'Call' | 'Put'` (2 definitions)

- optionType, UMPublicTradesRequest, types/request/unified_margin.rs
- optionType, USDCOptionsRecentTradesRequest, types/request/usdc_options.rs

### `'eb_convert_contract' | 'eb_convert_funding' | 'eb_convert_inverse' | 'eb_convert_spot' | 'eb_convert_uta'` (2 definitions)

- accountType, ConvertCoinsParamsV5, types/request/v5_asset.rs
- accountType, RequestConvertQuoteParamsV5, types/request/v5_asset.rs

### `'CONVERT' | 'DERIVATIVES' | 'OPTIONS' | 'SPOT'` (2 definitions)

- bizType, GetExchangeBrokerEarningsParamsV5, types/request/v5_broker.rs
- bizType, EarningDetailV5, types/response/v5_broker.rs

### `'apy' | 'quantity' | 'term'` (2 definitions)

- orderBy, GetSupplyOrderQuoteFixedParamsV5, types/request/v5_crypto_loan.rs
- orderBy, GetBorrowOrderQuoteFixedParamsV5, types/request/v5_crypto_loan.rs

### `'Redeem' | 'Stake'` (2 definitions)

- orderType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs
- orderType, EarnOrderHistoryV5, types/response/v5_earn.rs

### `'FUND' | 'UNIFIED'` (2 definitions)

- accountType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs
- toAccountType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs

### `'BULK' | 'ORIGIN'` (2 definitions)

- itemType, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- itemType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs

### `'Filled' | 'Processing' | 'Rejected'` (2 definitions)

- status, GetMovePositionHistoryParamsV5, types/request/v5_position.rs
- status, MovePositionHistoryV5, types/response/v5_position.rs

### `'Filled' | 'Rejected'` (2 definitions)

- status, GetRFQTradeListParamsV5, types/request/v5_rfq.rs
- status, RFQTradeV5, types/response/v5_rfq.rs

### `'ACCOUNT_STATUS_NORMAL' | 'ACCOUNT_STATUS_UNSPECIFIED'` (2 definitions)

- status, AssetInfoResponseV3_Spot, types/response/account_asset.rs
- status, AssetInfoV5, types/response/v5_asset.rs

### `'FA' | 'P' | 'SU'` (2 definitions)

- resultStatus, ManualRepayResultV5, types/response/v5_account.rs
- resultStatus, ManualRepayWithoutConversionResultV5, types/response/v5_spot_leverage_token.rs

### `'failure' | 'init' | 'processing' | 'success'` (2 definitions)

- exchangeStatus, ConvertStatusV5, types/response/v5_asset.rs
- exchangeStatus, ConvertHistoryRecordV5, types/response/v5_asset.rs

### `'Processing' | 'Rejected'` (2 definitions)

- status, MovePositionResultV5, types/response/v5_position.rs
- status, ExecuteRFQQuoteResultV5, types/response/v5_rfq.rs

### `'' | 'Maker' | 'Taker' | 'bybit'` (2 definitions)

- rejectParty, MovePositionResultV5, types/response/v5_position.rs
- rejectParty, MovePositionHistoryV5, types/response/v5_position.rs

### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled'` (2 definitions)

- status, CreateRFQResultV5, types/response/v5_rfq.rs
- status, CreateRFQQuoteResultV5, types/response/v5_rfq.rs

### `'CarryTrade' | 'FundingRateArb' | 'FutureSpread' | 'PerpBasis'` (2 definitions)

- contractType, SpreadInstrumentInfoV5, types/response/v5_spreadtrading.rs
- contractType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs

### `'Futures' | 'Spot'` (2 definitions)

- leg1ProdType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- leg2ProdType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs

### `'combination' | 'future_leg' | 'spot_leg'` (2 definitions)

- category, WSSpreadOrderV5, types/websockets/ws_events.rs
- category, WSSpreadExecutionV5, types/websockets/ws_events.rs

## Unique inline types (175, single definition)

- `'0' | '1'` → `AccountCoinBalancesRequestV3_WithBonus` — withBonus, AccountCoinBalancesRequestV3, types/request/account_asset.rs
- `0 | 1` → `CreateSubMemberRequestV3_Switch` — switch, CreateSubMemberRequestV3, types/request/account_asset.rs
- `0 | 1` → `CreateSubAPIKeyRequestV3_ReadOnly` — readOnly, CreateSubAPIKeyRequestV3, types/request/account_asset.rs
- `0 | 1` → `ContractSetAutoAddMarginRequest_AutoAddMargin` — autoAddMargin, ContractSetAutoAddMarginRequest, types/request/contract.rs
- `0 | 1` → `ContractSetMarginSwitchRequest_TradeMode` — tradeMode, ContractSetMarginSwitchRequest, types/request/contract.rs
- `'Full' | 'Partial'` → `ContractSetTPSLRequest_TpslMode` — tpslMode, ContractSetTPSLRequest, types/request/contract.rs
- `0 | 1 | 2` → `ContractSetTPSLRequest_PositionIdx` — positionIdx, ContractSetTPSLRequest, types/request/contract.rs
- `'AdlTrade' | 'BustTrade' | 'Funding' | 'Trade'` → `ContractUserExecutionHistoryRequest_ExecType` — execType, ContractUserExecutionHistoryRequest, types/request/contract.rs
- `'IndexPrice' | 'LastPrice' | 'MarkPrice'` → `InverseOrderRequest_SlTriggerBy` — sl_trigger_by, InverseOrderRequest, types/request/inverse.rs
- `'Full' | 'Partial'` → `InverseSetSlTpPositionModeRequest_TpSlMode` — tp_sl_mode, InverseSetSlTpPositionModeRequest, types/request/inverse.rs
- `'BothSide' | 'MergedSingle'` → `LinearSetPositionModeRequest_Mode` — mode, LinearSetPositionModeRequest, types/request/linear.rs
- `'Full' | 'Partial'` → `LinearSetPositionTpSlModeRequest_TpSlMode` — tp_sl_mode, LinearSetPositionTpSlModeRequest, types/request/linear.rs
- `0 | 1 | 2` → `LinearSetTradingStopRequest_PositionIdx` — position_idx, LinearSetTradingStopRequest, types/request/linear.rs
- `'15min' | '1d' | '1h' | '30min' | '4h' | '5min'` → `UMOpenInterestRequest_Interval` — interval, UMOpenInterestRequest, types/request/unified_margin.rs
- `'0' | '1' | '2'` → `UMOrderRequest_PositionIdx` — positionIdx, UMOrderRequest, types/request/unified_margin.rs
- `'DELIVERING' | 'OFFLINE' | 'ONLINE' | 'WAITING_ONLINE'` → `USDCOptionsContractInfoRequest_Status` — status, USDCOptionsContractInfoRequest, types/request/usdc_options.rs
- `'Call' | 'Put'` → `USDCOptionsRecentTradesRequest_OptionType` — optionType, USDCOptionsRecentTradesRequest, types/request/usdc_options.rs
- `'OFF' | 'ON'` → `SetCollateralCoinParamsV5_CollateralSwitch` — collateralSwitch, SetCollateralCoinParamsV5, types/request/v5_account.rs
- `0 | 1` → `GetAccountCoinBalanceParamsV5_WithTransferSafeAmount` — withTransferSafeAmount, GetAccountCoinBalanceParamsV5, types/request/v5_asset.rs
- `0 | 1` → `GetAccountCoinBalanceParamsV5_WithLtvTransferSafeAmount` — withLtvTransferSafeAmount, GetAccountCoinBalanceParamsV5, types/request/v5_asset.rs
- `0 | 1 | 2` → `GetWithdrawalAddressListParamsV5_AddressType` — addressType, GetWithdrawalAddressListParamsV5, types/request/v5_asset.rs
- `'FUND' | 'SPOT'` → `WithdrawParamsV5_AccountType` — accountType, WithdrawParamsV5, types/request/v5_asset.rs
- `0 | 1` → `WithdrawParamsV5_FeeType` — feeType, WithdrawParamsV5, types/request/v5_asset.rs
- `'eb_convert_contract' | 'eb_convert_funding' | 'eb_convert_inverse' | 'eb_convert_spot' | 'eb_convert_uta'` → `RequestConvertQuoteParamsV5_AccountType` — accountType, RequestConvertQuoteParamsV5, types/request/v5_asset.rs
- `0 | 1` → `GetFiatTradingPairListParamsV5_Side` — side, GetFiatTradingPairListParamsV5, types/request/v5_asset.rs
- `'0' | '1'` → `AdjustCollateralAmountParamsV5_Direction` — direction, AdjustCollateralAmountParamsV5, types/request/v5_crypto_loan.rs
- `'apy' | 'quantity' | 'term'` → `GetBorrowOrderQuoteFixedParamsV5_OrderBy` — orderBy, GetBorrowOrderQuoteFixedParamsV5, types/request/v5_crypto_loan.rs
- `'FUND' | 'UNIFIED'` → `SubmitStakeRedeemParamsV5_ToAccountType` — toAccountType, SubmitStakeRedeemParamsV5, types/request/v5_earn.rs
- `'inverse' | 'linear' | 'spot'` → `GetKlineParamsV5_Category` — category, GetKlineParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear'` → `GetIndexPriceKlineParamsV5_Category` — category, GetIndexPriceKlineParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear' | 'spot'` → `GetRPIOrderbookParamsV5_Category` — category, GetRPIOrderbookParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear'` → `GetFundingRateHistoryParamsV5_Category` — category, GetFundingRateHistoryParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear'` → `GetOpenInterestParamsV5_Category` — category, GetOpenInterestParamsV5, types/request/v5_market.rs
- `14 | 180 | 21 | 270 | 30 | 60 | 7 | 90` → `GetHistoricalVolatilityParamsV5_Period` — period, GetHistoricalVolatilityParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear'` → `GetRiskLimitParamsV5_Category` — category, GetRiskLimitParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear' | 'option'` → `GetDeliveryPriceParamsV5_Category` — category, GetDeliveryPriceParamsV5, types/request/v5_market.rs
- `'inverse' | 'linear'` → `GetLongShortRatioParamsV5_Category` — category, GetLongShortRatioParamsV5, types/request/v5_market.rs
- `0 | 1` → `GetP2PAccountCoinsBalanceParamsV5_WithBonus` — withBonus, GetP2PAccountCoinsBalanceParamsV5, types/request/v5_p2p_trading.rs
- `'0' | '1'` → `GetP2POnlineAdsParamsV5_Side` — side, GetP2POnlineAdsParamsV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_UnPostAd` — hasUnPostAd, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_Kyc` — isKyc, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_Email` — isEmail, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_Mobile` — isMobile, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_RegisterTime` — hasRegisterTime, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_OrderFinishNumberDay30` — hasOrderFinishNumberDay30, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_CompleteRateDay30` — hasCompleteRateDay30, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `0 | 1` → `P2PTradingPreferenceSetV5_NationalLimit` — hasNationalLimit, P2PTradingPreferenceSetV5, types/request/v5_p2p_trading.rs
- `'0' | '1'` → `CreateP2PAdParamsV5_Side` — side, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- `'0' | '1'` → `CreateP2PAdParamsV5_PriceType` — priceType, CreateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- `'0' | '1'` → `UpdateP2PAdParamsV5_PriceType` — priceType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- `'ACTIVE' | 'MODIFY'` → `UpdateP2PAdParamsV5_ActionType` — actionType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- `'BULK' | 'ORIGIN'` → `UpdateP2PAdParamsV5_ItemType` — itemType, UpdateP2PAdParamsV5, types/request/v5_p2p_trading.rs
- `'1' | '2'` → `GetP2PPersonalAdsParamsV5_Status` — status, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs
- `'0' | '1'` → `GetP2PPersonalAdsParamsV5_Side` — side, GetP2PPersonalAdsParamsV5, types/request/v5_p2p_trading.rs
- `'inverse' | 'linear'` → `SetLeverageParamsV5_Category` — category, SetLeverageParamsV5, types/request/v5_position.rs
- `'inverse' | 'linear'` → `SwitchIsolatedMarginParamsV5_Category` — category, SwitchIsolatedMarginParamsV5, types/request/v5_position.rs
- `0 | 1` → `SwitchIsolatedMarginParamsV5_TradeMode` — tradeMode, SwitchIsolatedMarginParamsV5, types/request/v5_position.rs
- `'inverse' | 'linear'` → `SetTPSLModeParamsV5_Category` — category, SetTPSLModeParamsV5, types/request/v5_position.rs
- `'inverse' | 'linear'` → `SwitchPositionModeParamsV5_Category` — category, SwitchPositionModeParamsV5, types/request/v5_position.rs
- `0 | 3` → `SwitchPositionModeParamsV5_Mode` — mode, SwitchPositionModeParamsV5, types/request/v5_position.rs
- `'inverse' | 'linear'` → `SetRiskLimitParamsV5_Category` — category, SetRiskLimitParamsV5, types/request/v5_position.rs
- `0 | 1` → `SetAutoAddMarginParamsV5_AutoAddMargin` — autoAddMargin, SetAutoAddMarginParamsV5, types/request/v5_position.rs
- `'inverse' | 'linear'` → `AddOrReduceMarginParamsV5_Category` — category, AddOrReduceMarginParamsV5, types/request/v5_position.rs
- `'Buy' | 'Sell'` → `MovePositionParamsV5_List_Side` — side, MovePositionParamsV5_List, types/request/v5_position.rs
- `'inverse' | 'linear'` → `ConfirmNewRiskLimitParamsV5_Category` — category, ConfirmNewRiskLimitParamsV5, types/request/v5_position.rs
- `'inverse' | 'linear'` → `GetPreUpgradeOrderHistoryParamsV5_Category` — category, GetPreUpgradeOrderHistoryParamsV5, types/request/v5_pre_upgrade.rs
- `'Order' | 'StopOrder'` → `GetPreUpgradeOrderHistoryParamsV5_OrderFilter` — orderFilter, GetPreUpgradeOrderHistoryParamsV5, types/request/v5_pre_upgrade.rs
- `'inverse' | 'linear'` → `GetPreUpgradeTradeHistoryParamsV5_Category` — category, GetPreUpgradeTradeHistoryParamsV5, types/request/v5_pre_upgrade.rs
- `'inverse' | 'linear'` → `GetPreUpgradeClosedPnlParamsV5_Category` — category, GetPreUpgradeClosedPnlParamsV5, types/request/v5_pre_upgrade.rs
- `'linear' | 'option'` → `GetPreUpgradeTransactionLogParamsV5_Category` — category, GetPreUpgradeTransactionLogParamsV5, types/request/v5_pre_upgrade.rs
- `'inverse' | 'linear' | 'option' | 'spot'` → `RFQTransactionV5_Category` — category, RFQTransactionV5, types/request/v5_rfq.rs
- `'linear' | 'option' | 'spot'` → `RFQQuoteV5_Category` — category, RFQQuoteV5, types/request/v5_rfq.rs
- `'buy' | 'sell'` → `ExecuteRFQQuoteParamsV5_QuoteSide` — quoteSide, ExecuteRFQQuoteParamsV5, types/request/v5_rfq.rs
- `'quoter' | 'request'` → `GetRFQListParamsV5_TraderType` — traderType, GetRFQListParamsV5, types/request/v5_rfq.rs
- `'quote' | 'request'` → `GetRFQQuoteRealtimeParamsV5_TraderType` — traderType, GetRFQQuoteRealtimeParamsV5, types/request/v5_rfq.rs
- `'quote' | 'request'` → `GetRFQHistoryParamsV5_TraderType` — traderType, GetRFQHistoryParamsV5, types/request/v5_rfq.rs
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` → `GetRFQHistoryParamsV5_Status` — status, GetRFQHistoryParamsV5, types/request/v5_rfq.rs
- `'0' | '1'` → `SetAutoRepayModeParamsV5_AutoRepayMode` — autoRepayMode, SetAutoRepayModeParamsV5, types/request/v5_spot_leverage_token.rs
- `'Buy' | 'Sell'` → `SubmitSpreadOrderParamsV5_Side` — side, SubmitSpreadOrderParamsV5, types/request/v5_spreadtrading.rs
- `0 | 1` → `OrderParamsV5_Leverage` — isLeverage, OrderParamsV5, types/request/v5_trade.rs
- `'Full' | 'Partial'` → `OrderParamsV5_TpslMode` — tpslMode, OrderParamsV5, types/request/v5_trade.rs
- `'Counterparty' | 'Queue'` → `OrderParamsV5_BboSideType` — bboSideType, OrderParamsV5, types/request/v5_trade.rs
- `'1' | '2' | '3' | '4' | '5'` → `OrderParamsV5_BboLevel` — bboLevel, OrderParamsV5, types/request/v5_trade.rs
- `'Full' | 'Partial'` → `AmendOrderParamsV5_TpslMode` — tpslMode, AmendOrderParamsV5, types/request/v5_trade.rs
- `0 | 1 | 2` → `GetAccountOrdersParamsV5_OpenOnly` — openOnly, GetAccountOrdersParamsV5, types/request/v5_trade.rs
- `0 | 1` → `BatchOrderParamsV5_Leverage` — isLeverage, BatchOrderParamsV5, types/request/v5_trade.rs
- `1 | 2` → `BatchOrderParamsV5_TriggerDirection` — triggerDirection, BatchOrderParamsV5, types/request/v5_trade.rs
- `'Full' | 'Partial'` → `BatchOrderParamsV5_TpslMode` — tpslMode, BatchOrderParamsV5, types/request/v5_trade.rs
- `'Full' | 'Partial'` → `BatchAmendOrderParamsV5_TpslMode` — tpslMode, BatchAmendOrderParamsV5, types/request/v5_trade.rs
- `1 | 6` → `CreateSubMemberParamsV5_MemberType` — memberType, CreateSubMemberParamsV5, types/request/v5_user.rs
- `0 | 1` → `CreateSubMemberParamsV5_Switch` — switch, CreateSubMemberParamsV5, types/request/v5_user.rs
- `0 | 1` → `CreateSubApiKeyParamsV5_ReadOnly` — readOnly, CreateSubApiKeyParamsV5, types/request/v5_user.rs
- `0 | 1` → `UpdateApiKeyParamsV5_ReadOnly` — readOnly, UpdateApiKeyParamsV5, types/request/v5_user.rs
- `'IN' | 'OUT'` → `SubAccountTransferResponseV3_List_Type` — type, SubAccountTransferResponseV3_List, types/response/account_asset.rs
- `1 | 6` → `CreateSubMemberResponseV3_MemberType` — memberType, CreateSubMemberResponseV3, types/response/account_asset.rs
- `0 | 1` → `CreateSubMemberResponseV3_Switch` — switch, CreateSubMemberResponseV3, types/response/account_asset.rs
- `1 | 6` → `SubMemberV3_MemberType` — memberType, SubMemberV3, types/response/account_asset.rs
- `1 | 2 | 4` → `SubMemberV3_Status` — status, SubMemberV3, types/response/account_asset.rs
- `'DERIVATIVES' | 'OPTIONS' | 'SPOT'` → `DCPInfoV5_Product` — product, DCPInfoV5, types/response/v5_account.rs
- `'ACCOUNT_STATUS_NORMAL' | 'ACCOUNT_STATUS_UNSPECIFIED'` → `AssetInfoV5_Status` — status, AssetInfoV5, types/response/v5_asset.rs
- `1 | 2 | 3` → `InternalDepositRecordV5_Status` — status, InternalDepositRecordV5, types/response/v5_asset.rs
- `'failure' | 'init' | 'processing' | 'success'` → `ConvertHistoryRecordV5_ExchangeStatus` — exchangeStatus, ConvertHistoryRecordV5, types/response/v5_asset.rs
- `1 | 2` → `SmallBalanceCoinV5_SupportConvert` — supportConvert, SmallBalanceCoinV5, types/response/v5_asset.rs
- `'CONVERT' | 'DERIVATIVES' | 'OPTIONS' | 'SPOT'` → `EarningDetailV5_BizType` — bizType, EarningDetailV5, types/response/v5_broker.rs
- `'AWARD_AMOUNT_UNIT_COIN' | 'AWARD_AMOUNT_UNIT_USD'` → `BrokerVoucherSpecV5_AmountUnit` — amountUnit, BrokerVoucherSpecV5, types/response/v5_broker.rs
- `'Available' | 'NotAvailable'` → `EarnProductV5_Status` — status, EarnProductV5, types/response/v5_earn.rs
- `'Redeem' | 'Stake'` → `EarnOrderHistoryV5_OrderType` — orderType, EarnOrderHistoryV5, types/response/v5_earn.rs
- `'Fail' | 'Pending' | 'Success'` → `EarnYieldHistoryV5_Status` — status, EarnYieldHistoryV5, types/response/v5_earn.rs
- `'Fail' | 'Pending' | 'Success'` → `EarnHourlyYieldHistoryV5_Status` — status, EarnHourlyYieldHistoryV5, types/response/v5_earn.rs
- `'0' | '1'` → `SpotInstrumentInfoV5_Innovation` — innovation, SpotInstrumentInfoV5, types/response/v5_market.rs
- `'0' | '1'` → `SpotInstrumentInfoV5_StTag` — stTag, SpotInstrumentInfoV5, types/response/v5_market.rs
- `'inverse' | 'linear'` → `OpenInterestResponseV5_Category` — category, OpenInterestResponseV5, types/response/v5_market.rs
- `0 | 1` → `RiskLimitV5_LowestRisk` — isLowestRisk, RiskLimitV5, types/response/v5_market.rs
- `0 | 1` → `AddOrReduceMarginResultV5_AutoAddMargin` — autoAddMargin, AddOrReduceMarginResultV5, types/response/v5_position.rs
- `'linear' | 'option' | 'spot'` → `MovePositionHistoryV5_Category` — category, MovePositionHistoryV5, types/response/v5_position.rs
- `'Buy' | 'Sell'` → `MovePositionHistoryV5_Side` — side, MovePositionHistoryV5, types/response/v5_position.rs
- `'Filled' | 'Processing' | 'Rejected'` → `MovePositionHistoryV5_Status` — status, MovePositionHistoryV5, types/response/v5_position.rs
- `'' | 'Maker' | 'Taker' | 'bybit'` → `MovePositionHistoryV5_RejectParty` — rejectParty, MovePositionHistoryV5, types/response/v5_position.rs
- `'Buy' | 'Sell'` → `ClosedOptionsPositionV5_Side` — side, ClosedOptionsPositionV5, types/response/v5_position.rs
- `'Buy' | 'None' | 'Sell'` → `PreUpgradeTransaction_Side` — side, PreUpgradeTransaction, types/response/v5_preupgrade.rs
- `'BTC' | 'ETH' | 'USDC' | 'USDT'` → `PreUpgradeTransaction_Currency` — currency, PreUpgradeTransaction, types/response/v5_preupgrade.rs
- `'Buy' | 'Sell'` → `PreUpgradeOptionsDelivery_Side` — side, PreUpgradeOptionsDelivery, types/response/v5_preupgrade.rs
- `'Buy' | 'Sell'` → `PreUpgradeUSDCSessionSettlement_Side` — side, PreUpgradeUSDCSessionSettlement, types/response/v5_preupgrade.rs
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled'` → `CreateRFQQuoteResultV5_Status` — status, CreateRFQQuoteResultV5, types/response/v5_rfq.rs
- `'Processing' | 'Rejected'` → `ExecuteRFQQuoteResultV5_Status` — status, ExecuteRFQQuoteResultV5, types/response/v5_rfq.rs
- `'linear' | 'option' | 'spot'` → `RFQLegV5_Category` — category, RFQLegV5, types/response/v5_rfq.rs
- `'buy' | 'sell'` → `RFQLegV5_Side` — side, RFQLegV5, types/response/v5_rfq.rs
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` → `RFQItemV5_Status` — status, RFQItemV5, types/response/v5_rfq.rs
- `'linear' | 'option' | 'spot'` → `QuoteLegV5_Category` — category, QuoteLegV5, types/response/v5_rfq.rs
- `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` → `RFQQuoteItemV5_Status` — status, RFQQuoteItemV5, types/response/v5_rfq.rs
- `'linear' | 'option' | 'spot'` → `RFQTradeLegV5_Category` — category, RFQTradeLegV5, types/response/v5_rfq.rs
- `'buy' | 'sell'` → `RFQTradeLegV5_Side` — side, RFQTradeLegV5, types/response/v5_rfq.rs
- `'buy' | 'sell'` → `RFQTradeV5_QuoteSide` — quoteSide, RFQTradeV5, types/response/v5_rfq.rs
- `'Filled' | 'Rejected'` → `RFQTradeV5_Status` — status, RFQTradeV5, types/response/v5_rfq.rs
- `'linear' | 'option' | 'spot'` → `RFQPublicTradeLegV5_Category` — category, RFQPublicTradeLegV5, types/response/v5_rfq.rs
- `'buy' | 'sell'` → `RFQPublicTradeLegV5_Side` — side, RFQPublicTradeLegV5, types/response/v5_rfq.rs
- `'0' | '1'` → `SpotMarginStateV5_SpotMarginMode` — spotMarginMode, SpotMarginStateV5, types/response/v5_spot_leverage_token.rs
- `'FA' | 'P' | 'SU'` → `ManualRepayWithoutConversionResultV5_ResultStatus` — resultStatus, ManualRepayWithoutConversionResultV5, types/response/v5_spot_leverage_token.rs
- `'0' | '1'` → `AutoRepayModeItemV5_AutoRepayMode` — autoRepayMode, AutoRepayModeItemV5, types/response/v5_spot_leverage_token.rs
- `'Settling' | 'Trading'` → `SpreadInstrumentInfoV5_Status` — status, SpreadInstrumentInfoV5, types/response/v5_spreadtrading.rs
- `'LinearFutures' | 'LinearPerpetual' | 'Spot'` → `SpreadInstrumentInfoV5_Legs_ContractType` — contractType, SpreadInstrumentInfoV5_Legs, types/response/v5_spreadtrading.rs
- `'Buy' | 'Sell'` → `SpreadRecentTradeV5_Side` — side, SpreadRecentTradeV5, types/response/v5_spreadtrading.rs
- `'Limit' | 'Market'` → `SpreadOpenOrderV5_OrderType` — orderType, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- `'Buy' | 'Sell'` → `SpreadOpenOrderV5_Side` — side, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` → `SpreadOpenOrderV5_TimeInForce` — timeInForce, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- `'New' | 'PartiallyFilled'` → `SpreadOpenOrderV5_OrderStatus` — orderStatus, SpreadOpenOrderV5, types/response/v5_spreadtrading.rs
- `'Limit' | 'Market'` → `SpreadOrderHistoryV5_OrderType` — orderType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- `'CarryTrade' | 'FundingRateArb' | 'FutureSpread' | 'PerpBasis'` → `SpreadOrderHistoryV5_ContractType` — contractType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- `'Cancelled' | 'Filled' | 'Rejected'` → `SpreadOrderHistoryV5_OrderStatus` — orderStatus, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` → `SpreadOrderHistoryV5_TimeInForce` — timeInForce, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- `'Buy' | 'Sell'` → `SpreadOrderHistoryV5_Side` — side, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- `'Futures' | 'Spot'` → `SpreadOrderHistoryV5_Leg2ProdType` — leg2ProdType, SpreadOrderHistoryV5, types/response/v5_spreadtrading.rs
- `'Buy' | 'Sell'` → `SpreadTradeLegV5_Side` — side, SpreadTradeLegV5, types/response/v5_spreadtrading.rs
- `'linear' | 'spot'` → `SpreadTradeLegV5_Category` — category, SpreadTradeLegV5, types/response/v5_spreadtrading.rs
- `'Buy' | 'Sell'` → `SpreadTradeV5_Side` — side, SpreadTradeV5, types/response/v5_spreadtrading.rs
- `'0' | '1'` → `AccountOrderV5_Leverage` — isLeverage, AccountOrderV5, types/response/v5_trade.rs
- `'baseCoin' | 'quoteCoin'` → `AccountOrderV5_MarketUnit` — marketUnit, AccountOrderV5, types/response/v5_trade.rs
- `'' | 'Full' | 'Partial'` → `AccountOrderV5_TpslMode` — tpslMode, AccountOrderV5, types/response/v5_trade.rs
- `'' | 'OcoTriggerBySl' | 'OcoTriggerByUnknown' | 'OcoTriggerTp'` → `AccountOrderV5_OcoTriggerType` — ocoTriggerType, AccountOrderV5, types/response/v5_trade.rs
- `1 | 2` → `AccountOrderV5_TriggerDirection` — triggerDirection, AccountOrderV5, types/response/v5_trade.rs
- `'' | 'iv' | 'price'` → `AccountOrderV5_PlaceType` — placeType, AccountOrderV5, types/response/v5_trade.rs
- `0 | 1` → `ApiKeyInfoV5_ReadOnly` — readOnly, ApiKeyInfoV5, types/response/v5_user.rs
- `1 | 2` → `ApiKeyInfoV5_Type` — type, ApiKeyInfoV5, types/response/v5_user.rs
- `0 | 1` → `ApiKeyInfoV5_Uta` — uta, ApiKeyInfoV5, types/response/v5_user.rs
- `'LEVEL_1' | 'LEVEL_2' | 'LEVEL_DEFAULT'` → `ApiKeyInfoV5_KycLevel` — kycLevel, ApiKeyInfoV5, types/response/v5_user.rs
- `0 | 1` → `UpdateApiKeyResultV5_ReadOnly` — readOnly, UpdateApiKeyResultV5, types/response/v5_user.rs
- `1 | 2` → `SubAccountAllApiKeysResultV5_Result_Type` — type, SubAccountAllApiKeysResultV5_Result, types/response/v5_user.rs
- `0 | 1` → `SubAccountAllApiKeysResultV5_Result_ReadOnly` — readOnly, SubAccountAllApiKeysResultV5_Result, types/response/v5_user.rs
- `'1' | '2' | '3' | '4'` → `AffiliateUserInfoV5_TotalWalletBalance` — totalWalletBalance, AffiliateUserInfoV5, types/response/v5_user.rs
- `0 | 1 | 2` → `AffiliateUserInfoV5_KycLevel` — KycLevel, AffiliateUserInfoV5, types/response/v5_user.rs
- `TWSTopic | number | string` → `WsRequestOperationBybit_Args` — args, WsRequestOperationBybit, types/websockets/ws_api.rs
- `'baseCoin' | 'quoteCoin'` → `WSAccountOrderV5_MarketUnit` — marketUnit, WSAccountOrderV5, types/websockets/ws_events.rs
- `'combination' | 'future_leg' | 'spot_leg'` → `WSSpreadExecutionV5_Category` — category, WSSpreadExecutionV5, types/websockets/ws_events.rs
- `'inverse' | 'linear' | 'option' | 'spot'` → `WsTopicRequest_Category` — category, WsTopicRequest, util/mod.rs
- `'connectionReady' | 'connectionReadyForAuth' | 'pong' | TEventType` → `EmittableEvent_EventType` — eventType, EmittableEvent, util/mod.rs