# Inline Type Report

## Overridden shared types (2)

### `Side` ✅ [*, *, 'Buy' | 'Sell'] (12 matches)

### `RfqSide` ✅ [*RFQ*, *, 'buy' | 'sell'] (6 matches)

## Shared signatures without overrides (38, candidates for shared-types.json)

### String literals

#### `'' | 'Maker' | 'Taker' | 'bybit'` (2 definitions)

- rejectParty, MovePositionResultV5, types/response/v5-position.ts
- rejectParty, MovePositionHistoryV5, types/response/v5-position.ts

#### `'0' | '1'` (14 definitions)

- withBonus, SingleAccountCoinBalanceRequestV3, types/request/account-asset.ts
- withBonus, AccountCoinBalancesRequestV3, types/request/account-asset.ts
- direction, AdjustCollateralAmountParamsV5, types/request/v5-crypto-loan.ts
- side, GetP2POnlineAdsParamsV5, types/request/v5-p2p-trading.ts
- side, CreateP2PAdParamsV5, types/request/v5-p2p-trading.ts
- priceType, CreateP2PAdParamsV5, types/request/v5-p2p-trading.ts
- priceType, UpdateP2PAdParamsV5, types/request/v5-p2p-trading.ts
- side, GetP2PPersonalAdsParamsV5, types/request/v5-p2p-trading.ts
- autoRepayMode, SetAutoRepayModeParamsV5, types/request/v5-spot-leverage-token.ts
- innovation, SpotInstrumentInfoV5, types/response/v5-market.ts
- stTag, SpotInstrumentInfoV5, types/response/v5-market.ts
- spotMarginMode, SpotMarginStateV5, types/response/v5-spot-leverage-token.ts
- autoRepayMode, AutoRepayModeItemV5, types/response/v5-spot-leverage-token.ts
- isLeverage, AccountOrderV5, types/response/v5-trade.ts

#### `'0' | '1' | '2'` (2 definitions)

- positionIdx, ContractOrderRequest, types/request/contract.ts
- positionIdx, UMOrderRequest, types/request/unified-margin.ts

#### `'1' | '2'` (2 definitions)

- triggerDirection, ContractOrderRequest, types/request/contract.ts
- status, GetP2PPersonalAdsParamsV5, types/request/v5-p2p-trading.ts

#### `'ACCOUNT_STATUS_NORMAL' | 'ACCOUNT_STATUS_UNSPECIFIED'` (2 definitions)

- status, AssetInfoResponseV3_Spot, types/response/account-asset.ts
- status, AssetInfoV5, types/response/v5-asset.ts

#### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled'` (2 definitions)

- status, CreateRFQResultV5, types/response/v5-rfq.ts
- status, CreateRFQQuoteResultV5, types/response/v5-rfq.ts

#### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` (4 definitions)

- status, GetRFQListParamsV5, types/request/v5-rfq.ts
- status, GetRFQHistoryParamsV5, types/request/v5-rfq.ts
- status, RFQItemV5, types/response/v5-rfq.ts
- status, RFQQuoteItemV5, types/response/v5-rfq.ts

#### `'apy' | 'quantity' | 'term'` (2 definitions)

- orderBy, GetSupplyOrderQuoteFixedParamsV5, types/request/v5-crypto-loan.ts
- orderBy, GetBorrowOrderQuoteFixedParamsV5, types/request/v5-crypto-loan.ts

#### `'baseCoin' | 'quoteCoin'` (3 definitions)

- marketUnit, OrderParamsV5, types/request/v5-trade.ts
- marketUnit, AccountOrderV5, types/response/v5-trade.ts
- marketUnit, WSAccountOrderV5, types/websockets/ws-events.ts

#### `'BULK' | 'ORIGIN'` (2 definitions)

- itemType, CreateP2PAdParamsV5, types/request/v5-p2p-trading.ts
- itemType, UpdateP2PAdParamsV5, types/request/v5-p2p-trading.ts

#### `'Call' | 'Put'` (2 definitions)

- optionType, UMPublicTradesRequest, types/request/unified-margin.ts
- optionType, USDCOptionsRecentTradesRequest, types/request/usdc-options.ts

#### `'CarryTrade' | 'FundingRateArb' | 'FutureSpread' | 'PerpBasis'` (2 definitions)

- contractType, SpreadInstrumentInfoV5, types/response/v5-spreadtrading.ts
- contractType, SpreadOrderHistoryV5, types/response/v5-spreadtrading.ts

#### `'combination' | 'future_leg' | 'spot_leg'` (2 definitions)

- category, WSSpreadOrderV5, types/websockets/ws-events.ts
- category, WSSpreadExecutionV5, types/websockets/ws-events.ts

#### `'CONVERT' | 'DERIVATIVES' | 'OPTIONS' | 'SPOT'` (2 definitions)

- bizType, GetExchangeBrokerEarningsParamsV5, types/request/v5-broker.ts
- bizType, EarningDetailV5, types/response/v5-broker.ts

#### `'eb_convert_contract' | 'eb_convert_funding' | 'eb_convert_inverse' | 'eb_convert_spot' | 'eb_convert_uta'` (2 definitions)

- accountType, ConvertCoinsParamsV5, types/request/v5-asset.ts
- accountType, RequestConvertQuoteParamsV5, types/request/v5-asset.ts

#### `'FA' | 'P' | 'SU'` (2 definitions)

- resultStatus, ManualRepayResultV5, types/response/v5-account.ts
- resultStatus, ManualRepayWithoutConversionResultV5, types/response/v5-spot-leverage-token.ts

#### `'Fail' | 'Pending' | 'Success'` (3 definitions)

- status, EarnOrderHistoryV5, types/response/v5-earn.ts
- status, EarnYieldHistoryV5, types/response/v5-earn.ts
- status, EarnHourlyYieldHistoryV5, types/response/v5-earn.ts

#### `'failure' | 'init' | 'processing' | 'success'` (2 definitions)

- exchangeStatus, ConvertStatusV5, types/response/v5-asset.ts
- exchangeStatus, ConvertHistoryRecordV5, types/response/v5-asset.ts

#### `'Filled' | 'Processing' | 'Rejected'` (2 definitions)

- status, GetMovePositionHistoryParamsV5, types/request/v5-position.ts
- status, MovePositionHistoryV5, types/response/v5-position.ts

#### `'Filled' | 'Rejected'` (2 definitions)

- status, GetRFQTradeListParamsV5, types/request/v5-rfq.ts
- status, RFQTradeV5, types/response/v5-rfq.ts

#### `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` (3 definitions)

- timeInForce, SubmitSpreadOrderParamsV5, types/request/v5-spreadtrading.ts
- timeInForce, SpreadOpenOrderV5, types/response/v5-spreadtrading.ts
- timeInForce, SpreadOrderHistoryV5, types/response/v5-spreadtrading.ts

#### `'Full' | 'Partial'` (8 definitions)

- tpslMode, ContractOrderRequest, types/request/contract.ts
- tpslMode, ContractSetTPSLRequest, types/request/contract.ts
- tp_sl_mode, InverseSetSlTpPositionModeRequest, types/request/inverse.ts
- tp_sl_mode, LinearSetPositionTpSlModeRequest, types/request/linear.ts
- tpslMode, OrderParamsV5, types/request/v5-trade.ts
- tpslMode, AmendOrderParamsV5, types/request/v5-trade.ts
- tpslMode, BatchOrderParamsV5, types/request/v5-trade.ts
- tpslMode, BatchAmendOrderParamsV5, types/request/v5-trade.ts

#### `'FUND' | 'UNIFIED'` (2 definitions)

- accountType, SubmitStakeRedeemParamsV5, types/request/v5-earn.ts
- toAccountType, SubmitStakeRedeemParamsV5, types/request/v5-earn.ts

#### `'Futures' | 'Spot'` (2 definitions)

- leg1ProdType, SpreadOrderHistoryV5, types/response/v5-spreadtrading.ts
- leg2ProdType, SpreadOrderHistoryV5, types/response/v5-spreadtrading.ts

#### `'IndexPrice' | 'LastPrice' | 'MarkPrice'` (2 definitions)

- tp_trigger_by, InverseOrderRequest, types/request/inverse.ts
- sl_trigger_by, InverseOrderRequest, types/request/inverse.ts

#### `'inverse' | 'linear'` (17 definitions)

- category, GetMarkPriceKlineParamsV5, types/request/v5-market.ts
- category, GetIndexPriceKlineParamsV5, types/request/v5-market.ts
- category, GetFundingRateHistoryParamsV5, types/request/v5-market.ts
- category, GetOpenInterestParamsV5, types/request/v5-market.ts
- category, GetRiskLimitParamsV5, types/request/v5-market.ts
- category, GetLongShortRatioParamsV5, types/request/v5-market.ts
- category, SetLeverageParamsV5, types/request/v5-position.ts
- category, SwitchIsolatedMarginParamsV5, types/request/v5-position.ts
- category, SetTPSLModeParamsV5, types/request/v5-position.ts
- category, SwitchPositionModeParamsV5, types/request/v5-position.ts
- category, SetRiskLimitParamsV5, types/request/v5-position.ts
- category, AddOrReduceMarginParamsV5, types/request/v5-position.ts
- category, ConfirmNewRiskLimitParamsV5, types/request/v5-position.ts
- category, GetPreUpgradeOrderHistoryParamsV5, types/request/v5-pre-upgrade.ts
- category, GetPreUpgradeTradeHistoryParamsV5, types/request/v5-pre-upgrade.ts
- category, GetPreUpgradeClosedPnlParamsV5, types/request/v5-pre-upgrade.ts
- category, OpenInterestResponseV5, types/response/v5-market.ts

#### `'inverse' | 'linear' | 'option' | 'spot'` (3 definitions)

- category, MovePositionParamsV5_List, types/request/v5-position.ts
- category, RFQTransactionV5, types/request/v5-rfq.ts
- category, WsTopicRequest, util/mod.ts

#### `'inverse' | 'linear' | 'spot'` (3 definitions)

- category, GetAccountInstrumentsInfoParamsV5, types/request/v5-account.ts
- category, GetKlineParamsV5, types/request/v5-market.ts
- category, GetRPIOrderbookParamsV5, types/request/v5-market.ts

#### `'Limit' | 'Market'` (3 definitions)

- orderType, SubmitSpreadOrderParamsV5, types/request/v5-spreadtrading.ts
- orderType, SpreadOpenOrderV5, types/response/v5-spreadtrading.ts
- orderType, SpreadOrderHistoryV5, types/response/v5-spreadtrading.ts

#### `'linear' | 'option' | 'spot'` (7 definitions)

- category, GetMovePositionHistoryParamsV5, types/request/v5-position.ts
- category, RFQQuoteV5, types/request/v5-rfq.ts
- category, MovePositionHistoryV5, types/response/v5-position.ts
- category, RFQLegV5, types/response/v5-rfq.ts
- category, QuoteLegV5, types/response/v5-rfq.ts
- category, RFQTradeLegV5, types/response/v5-rfq.ts
- category, RFQPublicTradeLegV5, types/response/v5-rfq.ts

#### `'Processing' | 'Rejected'` (2 definitions)

- status, MovePositionResultV5, types/response/v5-position.ts
- status, ExecuteRFQQuoteResultV5, types/response/v5-rfq.ts

#### `'quote' | 'request'` (3 definitions)

- traderType, GetRFQRealtimeParamsV5, types/request/v5-rfq.ts
- traderType, GetRFQQuoteRealtimeParamsV5, types/request/v5-rfq.ts
- traderType, GetRFQHistoryParamsV5, types/request/v5-rfq.ts

#### `'Redeem' | 'Stake'` (2 definitions)

- orderType, SubmitStakeRedeemParamsV5, types/request/v5-earn.ts
- orderType, EarnOrderHistoryV5, types/response/v5-earn.ts

### Number literals

#### `0 | 1` (32 definitions)

- forceChain, WithdrawCreateRequestV3, types/request/account-asset.ts
- switch, CreateSubMemberRequestV3, types/request/account-asset.ts
- readOnly, CreateSubAPIKeyRequestV3, types/request/account-asset.ts
- autoAddMargin, ContractSetAutoAddMarginRequest, types/request/contract.ts
- tradeMode, ContractSetMarginSwitchRequest, types/request/contract.ts
- withTransferSafeAmount, GetAccountCoinBalanceParamsV5, types/request/v5-asset.ts
- withLtvTransferSafeAmount, GetAccountCoinBalanceParamsV5, types/request/v5-asset.ts
- feeType, WithdrawParamsV5, types/request/v5-asset.ts
- side, GetFiatTradingPairListParamsV5, types/request/v5-asset.ts
- withBonus, GetP2PAccountCoinsBalanceParamsV5, types/request/v5-p2p-trading.ts
- hasUnPostAd, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- isKyc, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- isEmail, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- isMobile, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- hasRegisterTime, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- hasOrderFinishNumberDay30, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- hasCompleteRateDay30, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- hasNationalLimit, P2PTradingPreferenceSetV5, types/request/v5-p2p-trading.ts
- tradeMode, SwitchIsolatedMarginParamsV5, types/request/v5-position.ts
- autoAddMargin, SetAutoAddMarginParamsV5, types/request/v5-position.ts
- isLeverage, OrderParamsV5, types/request/v5-trade.ts
- isLeverage, BatchOrderParamsV5, types/request/v5-trade.ts
- switch, CreateSubMemberParamsV5, types/request/v5-user.ts
- readOnly, CreateSubApiKeyParamsV5, types/request/v5-user.ts
- readOnly, UpdateApiKeyParamsV5, types/request/v5-user.ts
- switch, CreateSubMemberResponseV3, types/response/account-asset.ts
- isLowestRisk, RiskLimitV5, types/response/v5-market.ts
- autoAddMargin, AddOrReduceMarginResultV5, types/response/v5-position.ts
- readOnly, ApiKeyInfoV5, types/response/v5-user.ts
- uta, ApiKeyInfoV5, types/response/v5-user.ts
- readOnly, UpdateApiKeyResultV5, types/response/v5-user.ts
- readOnly, SubAccountAllApiKeysResultV5_Result, types/response/v5-user.ts

#### `0 | 1 | 2` (6 definitions)

- positionIdx, ContractSetAutoAddMarginRequest, types/request/contract.ts
- positionIdx, ContractSetTPSLRequest, types/request/contract.ts
- position_idx, LinearSetTradingStopRequest, types/request/linear.ts
- addressType, GetWithdrawalAddressListParamsV5, types/request/v5-asset.ts
- openOnly, GetAccountOrdersParamsV5, types/request/v5-trade.ts
- KycLevel, AffiliateUserInfoV5, types/response/v5-user.ts

#### `0 | 3` (2 definitions)

- mode, ContractSetPositionModeRequest, types/request/contract.ts
- mode, SwitchPositionModeParamsV5, types/request/v5-position.ts

#### `1 | 2` (6 definitions)

- triggerDirection, OrderParamsV5, types/request/v5-trade.ts
- triggerDirection, BatchOrderParamsV5, types/request/v5-trade.ts
- supportConvert, SmallBalanceCoinV5, types/response/v5-asset.ts
- triggerDirection, AccountOrderV5, types/response/v5-trade.ts
- type, ApiKeyInfoV5, types/response/v5-user.ts
- type, SubAccountAllApiKeysResultV5_Result, types/response/v5-user.ts

#### `1 | 6` (4 definitions)

- memberType, CreateSubMemberRequestV3, types/request/account-asset.ts
- memberType, CreateSubMemberParamsV5, types/request/v5-user.ts
- memberType, CreateSubMemberResponseV3, types/response/account-asset.ts
- memberType, SubMemberV3, types/response/account-asset.ts

## Unique inline types (36, single definition)

- `'' | 'Full' | 'Partial'` → `AccountOrderV5_TpslMode` — tpslMode, AccountOrderV5, types/response/v5-trade.ts
- `'' | 'iv' | 'price'` → `AccountOrderV5_PlaceType` — placeType, AccountOrderV5, types/response/v5-trade.ts
- `'' | 'OcoTriggerBySl' | 'OcoTriggerByUnknown' | 'OcoTriggerTp'` → `AccountOrderV5_OcoTriggerType` — ocoTriggerType, AccountOrderV5, types/response/v5-trade.ts
- `'1' | '2' | '3' | '4'` → `AffiliateUserInfoV5_TotalWalletBalance` — totalWalletBalance, AffiliateUserInfoV5, types/response/v5-user.ts
- `'1' | '2' | '3' | '4' | '5'` → `OrderParamsV5_BboLevel` — bboLevel, OrderParamsV5, types/request/v5-trade.ts
- `'15min' | '1d' | '1h' | '30min' | '4h' | '5min'` → `UMOpenInterestRequest_Interval` — interval, UMOpenInterestRequest, types/request/unified-margin.ts
- `'ACTIVE' | 'MODIFY'` → `UpdateP2PAdParamsV5_ActionType` — actionType, UpdateP2PAdParamsV5, types/request/v5-p2p-trading.ts
- `'AdlTrade' | 'BustTrade' | 'Funding' | 'Trade'` → `ContractUserExecutionHistoryRequest_ExecType` — execType, ContractUserExecutionHistoryRequest, types/request/contract.ts
- `'Available' | 'NotAvailable'` → `EarnProductV5_Status` — status, EarnProductV5, types/response/v5-earn.ts
- `'AWARD_AMOUNT_UNIT_COIN' | 'AWARD_AMOUNT_UNIT_USD'` → `BrokerVoucherSpecV5_AmountUnit` — amountUnit, BrokerVoucherSpecV5, types/response/v5-broker.ts
- `'BothSide' | 'MergedSingle'` → `LinearSetPositionModeRequest_Mode` — mode, LinearSetPositionModeRequest, types/request/linear.ts
- `'BTC' | 'ETH' | 'USDC' | 'USDT'` → `PreUpgradeTransaction_Currency` — currency, PreUpgradeTransaction, types/response/v5-preupgrade.ts
- `'Buy' | 'None' | 'Sell'` → `PreUpgradeTransaction_Side` — side, PreUpgradeTransaction, types/response/v5-preupgrade.ts
- `'buy' | 'sell'` → `RfqSide` — side, RFQTransactionV5, types/shared.ts
- `'Buy' | 'Sell'` → `Side` — side, ContractSetAutoAddMarginRequest, types/shared.ts
- `'Cancelled' | 'Filled' | 'Rejected'` → `SpreadOrderHistoryV5_OrderStatus` — orderStatus, SpreadOrderHistoryV5, types/response/v5-spreadtrading.ts
- `'connectionReady' | 'connectionReadyForAuth' | 'pong' | TEventType` → `EmittableEvent_EventType` — eventType, EmittableEvent, util/mod.ts
- `'Counterparty' | 'Queue'` → `OrderParamsV5_BboSideType` — bboSideType, OrderParamsV5, types/request/v5-trade.ts
- `'DELIVERING' | 'OFFLINE' | 'ONLINE' | 'WAITING_ONLINE'` → `USDCOptionsContractInfoRequest_Status` — status, USDCOptionsContractInfoRequest, types/request/usdc-options.ts
- `'DERIVATIVES' | 'OPTIONS' | 'SPOT'` → `DCPInfoV5_Product` — product, DCPInfoV5, types/response/v5-account.ts
- `'FUND' | 'SPOT'` → `WithdrawParamsV5_AccountType` — accountType, WithdrawParamsV5, types/request/v5-asset.ts
- `'IN' | 'OUT'` → `SubAccountTransferResponseV3_List_Type` — type, SubAccountTransferResponseV3_List, types/response/account-asset.ts
- `'inverse' | 'linear' | 'option'` → `GetDeliveryPriceParamsV5_Category` — category, GetDeliveryPriceParamsV5, types/request/v5-market.ts
- `'LEVEL_1' | 'LEVEL_2' | 'LEVEL_DEFAULT'` → `ApiKeyInfoV5_KycLevel` — kycLevel, ApiKeyInfoV5, types/response/v5-user.ts
- `'linear' | 'option'` → `GetPreUpgradeTransactionLogParamsV5_Category` — category, GetPreUpgradeTransactionLogParamsV5, types/request/v5-pre-upgrade.ts
- `'linear' | 'spot'` → `SpreadTradeLegV5_Category` — category, SpreadTradeLegV5, types/response/v5-spreadtrading.ts
- `'LinearFutures' | 'LinearPerpetual' | 'Spot'` → `SpreadInstrumentInfoV5_Legs_ContractType` — contractType, SpreadInstrumentInfoV5_Legs, types/response/v5-spreadtrading.ts
- `'New' | 'PartiallyFilled'` → `SpreadOpenOrderV5_OrderStatus` — orderStatus, SpreadOpenOrderV5, types/response/v5-spreadtrading.ts
- `'OFF' | 'ON'` → `SetCollateralCoinParamsV5_CollateralSwitch` — collateralSwitch, SetCollateralCoinParamsV5, types/request/v5-account.ts
- `'Order' | 'StopOrder'` → `GetPreUpgradeOrderHistoryParamsV5_OrderFilter` — orderFilter, GetPreUpgradeOrderHistoryParamsV5, types/request/v5-pre-upgrade.ts
- `'quoter' | 'request'` → `GetRFQListParamsV5_TraderType` — traderType, GetRFQListParamsV5, types/request/v5-rfq.ts
- `'Settling' | 'Trading'` → `SpreadInstrumentInfoV5_Status` — status, SpreadInstrumentInfoV5, types/response/v5-spreadtrading.ts
- `1 | 2 | 3` → `InternalDepositRecordV5_Status` — status, InternalDepositRecordV5, types/response/v5-asset.ts
- `1 | 2 | 4` → `SubMemberV3_Status` — status, SubMemberV3, types/response/account-asset.ts
- `14 | 180 | 21 | 270 | 30 | 60 | 7 | 90` → `GetHistoricalVolatilityParamsV5_Period` — period, GetHistoricalVolatilityParamsV5, types/request/v5-market.ts
- `TWSTopic | number | string` → `WsRequestOperationBybit_Args` — args, WsRequestOperationBybit, types/websockets/ws-api.ts