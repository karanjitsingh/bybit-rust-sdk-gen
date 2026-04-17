# Inline Type Report

## Overridden shared types (2)

### `Side` ✅ [*, *, 'Buy' | 'Sell'] (12 matches)

- ContractSetAutoAddMarginRequest.side, types/request/contract.ts — ~~`ContractSetAutoAddMarginRequest_Side`~~
- MovePositionParamsV5_List.side, types/request/v5-position.ts — ~~`MovePositionParamsV5_List_Side`~~
- SubmitSpreadOrderParamsV5.side, types/request/v5-spreadtrading.ts — ~~`SubmitSpreadOrderParamsV5_Side`~~
- MovePositionHistoryV5.side, types/response/v5-position.ts — ~~`MovePositionHistoryV5_Side`~~
- ClosedOptionsPositionV5.side, types/response/v5-position.ts — ~~`ClosedOptionsPositionV5_Side`~~
- PreUpgradeOptionsDelivery.side, types/response/v5-preupgrade.ts — ~~`PreUpgradeOptionsDelivery_Side`~~
- PreUpgradeUSDCSessionSettlement.side, types/response/v5-preupgrade.ts — ~~`PreUpgradeUSDCSessionSettlement_Side`~~
- SpreadRecentTradeV5.side, types/response/v5-spreadtrading.ts — ~~`SpreadRecentTradeV5_Side`~~
- SpreadOpenOrderV5.side, types/response/v5-spreadtrading.ts — ~~`SpreadOpenOrderV5_Side`~~
- SpreadOrderHistoryV5.side, types/response/v5-spreadtrading.ts — ~~`SpreadOrderHistoryV5_Side`~~
- SpreadTradeLegV5.side, types/response/v5-spreadtrading.ts — ~~`SpreadTradeLegV5_Side`~~
- SpreadTradeV5.side, types/response/v5-spreadtrading.ts — ~~`SpreadTradeV5_Side`~~

### `RfqSide` ✅ [*RFQ*, *, 'buy' | 'sell'] (6 matches)

- RFQTransactionV5.side, types/request/v5-rfq.ts — ~~`RFQTransactionV5_Side`~~
- ExecuteRFQQuoteParamsV5.quoteSide, types/request/v5-rfq.ts — ~~`ExecuteRFQQuoteParamsV5_QuoteSide`~~
- RFQLegV5.side, types/response/v5-rfq.ts — ~~`RFQLegV5_Side`~~
- RFQTradeLegV5.side, types/response/v5-rfq.ts — ~~`RFQTradeLegV5_Side`~~
- RFQTradeV5.quoteSide, types/response/v5-rfq.ts — ~~`RFQTradeV5_QuoteSide`~~
- RFQPublicTradeLegV5.side, types/response/v5-rfq.ts — ~~`RFQPublicTradeLegV5_Side`~~

## Shared signatures without overrides (38, candidates for shared-types.json)

### String literals

#### `'' | 'Maker' | 'Taker' | 'bybit'` (2 definitions)

- MovePositionResultV5.rejectParty, types/response/v5-position.ts → `MovePositionResultV5_RejectParty`
- MovePositionHistoryV5.rejectParty, types/response/v5-position.ts → `MovePositionHistoryV5_RejectParty`

#### `'0' | '1'` (14 definitions)

- SingleAccountCoinBalanceRequestV3.withBonus, types/request/account-asset.ts → `SingleAccountCoinBalanceRequestV3_WithBonus`
- AccountCoinBalancesRequestV3.withBonus, types/request/account-asset.ts → `AccountCoinBalancesRequestV3_WithBonus`
- AdjustCollateralAmountParamsV5.direction, types/request/v5-crypto-loan.ts → `AdjustCollateralAmountParamsV5_Direction`
- GetP2POnlineAdsParamsV5.side, types/request/v5-p2p-trading.ts → `GetP2POnlineAdsParamsV5_Side`
- CreateP2PAdParamsV5.side, types/request/v5-p2p-trading.ts → `CreateP2PAdParamsV5_Side`
- CreateP2PAdParamsV5.priceType, types/request/v5-p2p-trading.ts → `CreateP2PAdParamsV5_PriceType`
- UpdateP2PAdParamsV5.priceType, types/request/v5-p2p-trading.ts → `UpdateP2PAdParamsV5_PriceType`
- GetP2PPersonalAdsParamsV5.side, types/request/v5-p2p-trading.ts → `GetP2PPersonalAdsParamsV5_Side`
- SetAutoRepayModeParamsV5.autoRepayMode, types/request/v5-spot-leverage-token.ts → `SetAutoRepayModeParamsV5_AutoRepayMode`
- SpotInstrumentInfoV5.innovation, types/response/v5-market.ts → `SpotInstrumentInfoV5_Innovation`
- SpotInstrumentInfoV5.stTag, types/response/v5-market.ts → `SpotInstrumentInfoV5_StTag`
- SpotMarginStateV5.spotMarginMode, types/response/v5-spot-leverage-token.ts → `SpotMarginStateV5_SpotMarginMode`
- AutoRepayModeItemV5.autoRepayMode, types/response/v5-spot-leverage-token.ts → `AutoRepayModeItemV5_AutoRepayMode`
- AccountOrderV5.isLeverage, types/response/v5-trade.ts → `AccountOrderV5_Leverage`

#### `'0' | '1' | '2'` (2 definitions)

- ContractOrderRequest.positionIdx, types/request/contract.ts → `ContractOrderRequest_PositionIdx`
- UMOrderRequest.positionIdx, types/request/unified-margin.ts → `UMOrderRequest_PositionIdx`

#### `'1' | '2'` (2 definitions)

- ContractOrderRequest.triggerDirection, types/request/contract.ts → `ContractOrderRequest_TriggerDirection`
- GetP2PPersonalAdsParamsV5.status, types/request/v5-p2p-trading.ts → `GetP2PPersonalAdsParamsV5_Status`

#### `'ACCOUNT_STATUS_NORMAL' | 'ACCOUNT_STATUS_UNSPECIFIED'` (2 definitions)

- AssetInfoResponseV3_Spot.status, types/response/account-asset.ts → `AssetInfoResponseV3_Spot_Status`
- AssetInfoV5.status, types/response/v5-asset.ts → `AssetInfoV5_Status`

#### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled'` (2 definitions)

- CreateRFQResultV5.status, types/response/v5-rfq.ts → `CreateRFQResultV5_Status`
- CreateRFQQuoteResultV5.status, types/response/v5-rfq.ts → `CreateRFQQuoteResultV5_Status`

#### `'Active' | 'Canceled' | 'Expired' | 'Failed' | 'Filled' | 'PendingFill'` (4 definitions)

- GetRFQListParamsV5.status, types/request/v5-rfq.ts → `GetRFQListParamsV5_Status`
- GetRFQHistoryParamsV5.status, types/request/v5-rfq.ts → `GetRFQHistoryParamsV5_Status`
- RFQItemV5.status, types/response/v5-rfq.ts → `RFQItemV5_Status`
- RFQQuoteItemV5.status, types/response/v5-rfq.ts → `RFQQuoteItemV5_Status`

#### `'apy' | 'quantity' | 'term'` (2 definitions)

- GetSupplyOrderQuoteFixedParamsV5.orderBy, types/request/v5-crypto-loan.ts → `GetSupplyOrderQuoteFixedParamsV5_OrderBy`
- GetBorrowOrderQuoteFixedParamsV5.orderBy, types/request/v5-crypto-loan.ts → `GetBorrowOrderQuoteFixedParamsV5_OrderBy`

#### `'baseCoin' | 'quoteCoin'` (3 definitions)

- OrderParamsV5.marketUnit, types/request/v5-trade.ts → `OrderParamsV5_MarketUnit`
- AccountOrderV5.marketUnit, types/response/v5-trade.ts → `AccountOrderV5_MarketUnit`
- WSAccountOrderV5.marketUnit, types/websockets/ws-events.ts → `WSAccountOrderV5_MarketUnit`

#### `'BULK' | 'ORIGIN'` (2 definitions)

- CreateP2PAdParamsV5.itemType, types/request/v5-p2p-trading.ts → `CreateP2PAdParamsV5_ItemType`
- UpdateP2PAdParamsV5.itemType, types/request/v5-p2p-trading.ts → `UpdateP2PAdParamsV5_ItemType`

#### `'Call' | 'Put'` (2 definitions)

- UMPublicTradesRequest.optionType, types/request/unified-margin.ts → `UMPublicTradesRequest_OptionType`
- USDCOptionsRecentTradesRequest.optionType, types/request/usdc-options.ts → `USDCOptionsRecentTradesRequest_OptionType`

#### `'CarryTrade' | 'FundingRateArb' | 'FutureSpread' | 'PerpBasis'` (2 definitions)

- SpreadInstrumentInfoV5.contractType, types/response/v5-spreadtrading.ts → `SpreadInstrumentInfoV5_ContractType`
- SpreadOrderHistoryV5.contractType, types/response/v5-spreadtrading.ts → `SpreadOrderHistoryV5_ContractType`

#### `'combination' | 'future_leg' | 'spot_leg'` (2 definitions)

- WSSpreadOrderV5.category, types/websockets/ws-events.ts → `WSSpreadOrderV5_Category`
- WSSpreadExecutionV5.category, types/websockets/ws-events.ts → `WSSpreadExecutionV5_Category`

#### `'CONVERT' | 'DERIVATIVES' | 'OPTIONS' | 'SPOT'` (2 definitions)

- GetExchangeBrokerEarningsParamsV5.bizType, types/request/v5-broker.ts → `GetExchangeBrokerEarningsParamsV5_BizType`
- EarningDetailV5.bizType, types/response/v5-broker.ts → `EarningDetailV5_BizType`

#### `'eb_convert_contract' | 'eb_convert_funding' | 'eb_convert_inverse' | 'eb_convert_spot' | 'eb_convert_uta'` (2 definitions)

- ConvertCoinsParamsV5.accountType, types/request/v5-asset.ts → `ConvertCoinsParamsV5_AccountType`
- RequestConvertQuoteParamsV5.accountType, types/request/v5-asset.ts → `RequestConvertQuoteParamsV5_AccountType`

#### `'FA' | 'P' | 'SU'` (2 definitions)

- ManualRepayResultV5.resultStatus, types/response/v5-account.ts → `ManualRepayResultV5_ResultStatus`
- ManualRepayWithoutConversionResultV5.resultStatus, types/response/v5-spot-leverage-token.ts → `ManualRepayWithoutConversionResultV5_ResultStatus`

#### `'Fail' | 'Pending' | 'Success'` (3 definitions)

- EarnOrderHistoryV5.status, types/response/v5-earn.ts → `EarnOrderHistoryV5_Status`
- EarnYieldHistoryV5.status, types/response/v5-earn.ts → `EarnYieldHistoryV5_Status`
- EarnHourlyYieldHistoryV5.status, types/response/v5-earn.ts → `EarnHourlyYieldHistoryV5_Status`

#### `'failure' | 'init' | 'processing' | 'success'` (2 definitions)

- ConvertStatusV5.exchangeStatus, types/response/v5-asset.ts → `ConvertStatusV5_ExchangeStatus`
- ConvertHistoryRecordV5.exchangeStatus, types/response/v5-asset.ts → `ConvertHistoryRecordV5_ExchangeStatus`

#### `'Filled' | 'Processing' | 'Rejected'` (2 definitions)

- GetMovePositionHistoryParamsV5.status, types/request/v5-position.ts → `GetMovePositionHistoryParamsV5_Status`
- MovePositionHistoryV5.status, types/response/v5-position.ts → `MovePositionHistoryV5_Status`

#### `'Filled' | 'Rejected'` (2 definitions)

- GetRFQTradeListParamsV5.status, types/request/v5-rfq.ts → `GetRFQTradeListParamsV5_Status`
- RFQTradeV5.status, types/response/v5-rfq.ts → `RFQTradeV5_Status`

#### `'FOK' | 'GTC' | 'IOC' | 'PostOnly'` (3 definitions)

- SubmitSpreadOrderParamsV5.timeInForce, types/request/v5-spreadtrading.ts → `SubmitSpreadOrderParamsV5_TimeInForce`
- SpreadOpenOrderV5.timeInForce, types/response/v5-spreadtrading.ts → `SpreadOpenOrderV5_TimeInForce`
- SpreadOrderHistoryV5.timeInForce, types/response/v5-spreadtrading.ts → `SpreadOrderHistoryV5_TimeInForce`

#### `'Full' | 'Partial'` (8 definitions)

- ContractOrderRequest.tpslMode, types/request/contract.ts → `ContractOrderRequest_TpslMode`
- ContractSetTPSLRequest.tpslMode, types/request/contract.ts → `ContractSetTPSLRequest_TpslMode`
- InverseSetSlTpPositionModeRequest.tp_sl_mode, types/request/inverse.ts → `InverseSetSlTpPositionModeRequest_TpSlMode`
- LinearSetPositionTpSlModeRequest.tp_sl_mode, types/request/linear.ts → `LinearSetPositionTpSlModeRequest_TpSlMode`
- OrderParamsV5.tpslMode, types/request/v5-trade.ts → `OrderParamsV5_TpslMode`
- AmendOrderParamsV5.tpslMode, types/request/v5-trade.ts → `AmendOrderParamsV5_TpslMode`
- BatchOrderParamsV5.tpslMode, types/request/v5-trade.ts → `BatchOrderParamsV5_TpslMode`
- BatchAmendOrderParamsV5.tpslMode, types/request/v5-trade.ts → `BatchAmendOrderParamsV5_TpslMode`

#### `'FUND' | 'UNIFIED'` (2 definitions)

- SubmitStakeRedeemParamsV5.accountType, types/request/v5-earn.ts → `SubmitStakeRedeemParamsV5_AccountType`
- SubmitStakeRedeemParamsV5.toAccountType, types/request/v5-earn.ts → `SubmitStakeRedeemParamsV5_ToAccountType`

#### `'Futures' | 'Spot'` (2 definitions)

- SpreadOrderHistoryV5.leg1ProdType, types/response/v5-spreadtrading.ts → `SpreadOrderHistoryV5_Leg1ProdType`
- SpreadOrderHistoryV5.leg2ProdType, types/response/v5-spreadtrading.ts → `SpreadOrderHistoryV5_Leg2ProdType`

#### `'IndexPrice' | 'LastPrice' | 'MarkPrice'` (2 definitions)

- InverseOrderRequest.tp_trigger_by, types/request/inverse.ts → `InverseOrderRequest_TpTriggerBy`
- InverseOrderRequest.sl_trigger_by, types/request/inverse.ts → `InverseOrderRequest_SlTriggerBy`

#### `'inverse' | 'linear'` (17 definitions)

- GetMarkPriceKlineParamsV5.category, types/request/v5-market.ts → `GetMarkPriceKlineParamsV5_Category`
- GetIndexPriceKlineParamsV5.category, types/request/v5-market.ts → `GetIndexPriceKlineParamsV5_Category`
- GetFundingRateHistoryParamsV5.category, types/request/v5-market.ts → `GetFundingRateHistoryParamsV5_Category`
- GetOpenInterestParamsV5.category, types/request/v5-market.ts → `GetOpenInterestParamsV5_Category`
- GetRiskLimitParamsV5.category, types/request/v5-market.ts → `GetRiskLimitParamsV5_Category`
- GetLongShortRatioParamsV5.category, types/request/v5-market.ts → `GetLongShortRatioParamsV5_Category`
- SetLeverageParamsV5.category, types/request/v5-position.ts → `SetLeverageParamsV5_Category`
- SwitchIsolatedMarginParamsV5.category, types/request/v5-position.ts → `SwitchIsolatedMarginParamsV5_Category`
- SetTPSLModeParamsV5.category, types/request/v5-position.ts → `SetTPSLModeParamsV5_Category`
- SwitchPositionModeParamsV5.category, types/request/v5-position.ts → `SwitchPositionModeParamsV5_Category`
- SetRiskLimitParamsV5.category, types/request/v5-position.ts → `SetRiskLimitParamsV5_Category`
- AddOrReduceMarginParamsV5.category, types/request/v5-position.ts → `AddOrReduceMarginParamsV5_Category`
- ConfirmNewRiskLimitParamsV5.category, types/request/v5-position.ts → `ConfirmNewRiskLimitParamsV5_Category`
- GetPreUpgradeOrderHistoryParamsV5.category, types/request/v5-pre-upgrade.ts → `GetPreUpgradeOrderHistoryParamsV5_Category`
- GetPreUpgradeTradeHistoryParamsV5.category, types/request/v5-pre-upgrade.ts → `GetPreUpgradeTradeHistoryParamsV5_Category`
- GetPreUpgradeClosedPnlParamsV5.category, types/request/v5-pre-upgrade.ts → `GetPreUpgradeClosedPnlParamsV5_Category`
- OpenInterestResponseV5.category, types/response/v5-market.ts → `OpenInterestResponseV5_Category`

#### `'inverse' | 'linear' | 'option' | 'spot'` (3 definitions)

- MovePositionParamsV5_List.category, types/request/v5-position.ts → `MovePositionParamsV5_List_Category`
- RFQTransactionV5.category, types/request/v5-rfq.ts → `RFQTransactionV5_Category`
- WsTopicRequest.category, util/mod.ts → `WsTopicRequest_Category`

#### `'inverse' | 'linear' | 'spot'` (3 definitions)

- GetAccountInstrumentsInfoParamsV5.category, types/request/v5-account.ts → `GetAccountInstrumentsInfoParamsV5_Category`
- GetKlineParamsV5.category, types/request/v5-market.ts → `GetKlineParamsV5_Category`
- GetRPIOrderbookParamsV5.category, types/request/v5-market.ts → `GetRPIOrderbookParamsV5_Category`

#### `'Limit' | 'Market'` (3 definitions)

- SubmitSpreadOrderParamsV5.orderType, types/request/v5-spreadtrading.ts → `SubmitSpreadOrderParamsV5_OrderType`
- SpreadOpenOrderV5.orderType, types/response/v5-spreadtrading.ts → `SpreadOpenOrderV5_OrderType`
- SpreadOrderHistoryV5.orderType, types/response/v5-spreadtrading.ts → `SpreadOrderHistoryV5_OrderType`

#### `'linear' | 'option' | 'spot'` (7 definitions)

- GetMovePositionHistoryParamsV5.category, types/request/v5-position.ts → `GetMovePositionHistoryParamsV5_Category`
- RFQQuoteV5.category, types/request/v5-rfq.ts → `RFQQuoteV5_Category`
- MovePositionHistoryV5.category, types/response/v5-position.ts → `MovePositionHistoryV5_Category`
- RFQLegV5.category, types/response/v5-rfq.ts → `RFQLegV5_Category`
- QuoteLegV5.category, types/response/v5-rfq.ts → `QuoteLegV5_Category`
- RFQTradeLegV5.category, types/response/v5-rfq.ts → `RFQTradeLegV5_Category`
- RFQPublicTradeLegV5.category, types/response/v5-rfq.ts → `RFQPublicTradeLegV5_Category`

#### `'Processing' | 'Rejected'` (2 definitions)

- MovePositionResultV5.status, types/response/v5-position.ts → `MovePositionResultV5_Status`
- ExecuteRFQQuoteResultV5.status, types/response/v5-rfq.ts → `ExecuteRFQQuoteResultV5_Status`

#### `'quote' | 'request'` (3 definitions)

- GetRFQRealtimeParamsV5.traderType, types/request/v5-rfq.ts → `GetRFQRealtimeParamsV5_TraderType`
- GetRFQQuoteRealtimeParamsV5.traderType, types/request/v5-rfq.ts → `GetRFQQuoteRealtimeParamsV5_TraderType`
- GetRFQHistoryParamsV5.traderType, types/request/v5-rfq.ts → `GetRFQHistoryParamsV5_TraderType`

#### `'Redeem' | 'Stake'` (2 definitions)

- SubmitStakeRedeemParamsV5.orderType, types/request/v5-earn.ts → `SubmitStakeRedeemParamsV5_OrderType`
- EarnOrderHistoryV5.orderType, types/response/v5-earn.ts → `EarnOrderHistoryV5_OrderType`

### Number literals

#### `0 | 1` (32 definitions)

- WithdrawCreateRequestV3.forceChain, types/request/account-asset.ts → `WithdrawCreateRequestV3_ForceChain`
- CreateSubMemberRequestV3.switch, types/request/account-asset.ts → `CreateSubMemberRequestV3_Switch`
- CreateSubAPIKeyRequestV3.readOnly, types/request/account-asset.ts → `CreateSubAPIKeyRequestV3_ReadOnly`
- ContractSetAutoAddMarginRequest.autoAddMargin, types/request/contract.ts → `ContractSetAutoAddMarginRequest_AutoAddMargin`
- ContractSetMarginSwitchRequest.tradeMode, types/request/contract.ts → `ContractSetMarginSwitchRequest_TradeMode`
- GetAccountCoinBalanceParamsV5.withTransferSafeAmount, types/request/v5-asset.ts → `GetAccountCoinBalanceParamsV5_WithTransferSafeAmount`
- GetAccountCoinBalanceParamsV5.withLtvTransferSafeAmount, types/request/v5-asset.ts → `GetAccountCoinBalanceParamsV5_WithLtvTransferSafeAmount`
- WithdrawParamsV5.feeType, types/request/v5-asset.ts → `WithdrawParamsV5_FeeType`
- GetFiatTradingPairListParamsV5.side, types/request/v5-asset.ts → `GetFiatTradingPairListParamsV5_Side`
- GetP2PAccountCoinsBalanceParamsV5.withBonus, types/request/v5-p2p-trading.ts → `GetP2PAccountCoinsBalanceParamsV5_WithBonus`
- P2PTradingPreferenceSetV5.hasUnPostAd, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_UnPostAd`
- P2PTradingPreferenceSetV5.isKyc, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_Kyc`
- P2PTradingPreferenceSetV5.isEmail, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_Email`
- P2PTradingPreferenceSetV5.isMobile, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_Mobile`
- P2PTradingPreferenceSetV5.hasRegisterTime, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_RegisterTime`
- P2PTradingPreferenceSetV5.hasOrderFinishNumberDay30, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_OrderFinishNumberDay30`
- P2PTradingPreferenceSetV5.hasCompleteRateDay30, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_CompleteRateDay30`
- P2PTradingPreferenceSetV5.hasNationalLimit, types/request/v5-p2p-trading.ts → `P2PTradingPreferenceSetV5_NationalLimit`
- SwitchIsolatedMarginParamsV5.tradeMode, types/request/v5-position.ts → `SwitchIsolatedMarginParamsV5_TradeMode`
- SetAutoAddMarginParamsV5.autoAddMargin, types/request/v5-position.ts → `SetAutoAddMarginParamsV5_AutoAddMargin`
- OrderParamsV5.isLeverage, types/request/v5-trade.ts → `OrderParamsV5_Leverage`
- BatchOrderParamsV5.isLeverage, types/request/v5-trade.ts → `BatchOrderParamsV5_Leverage`
- CreateSubMemberParamsV5.switch, types/request/v5-user.ts → `CreateSubMemberParamsV5_Switch`
- CreateSubApiKeyParamsV5.readOnly, types/request/v5-user.ts → `CreateSubApiKeyParamsV5_ReadOnly`
- UpdateApiKeyParamsV5.readOnly, types/request/v5-user.ts → `UpdateApiKeyParamsV5_ReadOnly`
- CreateSubMemberResponseV3.switch, types/response/account-asset.ts → `CreateSubMemberResponseV3_Switch`
- RiskLimitV5.isLowestRisk, types/response/v5-market.ts → `RiskLimitV5_LowestRisk`
- AddOrReduceMarginResultV5.autoAddMargin, types/response/v5-position.ts → `AddOrReduceMarginResultV5_AutoAddMargin`
- ApiKeyInfoV5.readOnly, types/response/v5-user.ts → `ApiKeyInfoV5_ReadOnly`
- ApiKeyInfoV5.uta, types/response/v5-user.ts → `ApiKeyInfoV5_Uta`
- UpdateApiKeyResultV5.readOnly, types/response/v5-user.ts → `UpdateApiKeyResultV5_ReadOnly`
- SubAccountAllApiKeysResultV5_Result.readOnly, types/response/v5-user.ts → `SubAccountAllApiKeysResultV5_Result_ReadOnly`

#### `0 | 1 | 2` (6 definitions)

- ContractSetAutoAddMarginRequest.positionIdx, types/request/contract.ts → `ContractSetAutoAddMarginRequest_PositionIdx`
- ContractSetTPSLRequest.positionIdx, types/request/contract.ts → `ContractSetTPSLRequest_PositionIdx`
- LinearSetTradingStopRequest.position_idx, types/request/linear.ts → `LinearSetTradingStopRequest_PositionIdx`
- GetWithdrawalAddressListParamsV5.addressType, types/request/v5-asset.ts → `GetWithdrawalAddressListParamsV5_AddressType`
- GetAccountOrdersParamsV5.openOnly, types/request/v5-trade.ts → `GetAccountOrdersParamsV5_OpenOnly`
- AffiliateUserInfoV5.KycLevel, types/response/v5-user.ts → `AffiliateUserInfoV5_KycLevel`

#### `0 | 3` (2 definitions)

- ContractSetPositionModeRequest.mode, types/request/contract.ts → `ContractSetPositionModeRequest_Mode`
- SwitchPositionModeParamsV5.mode, types/request/v5-position.ts → `SwitchPositionModeParamsV5_Mode`

#### `1 | 2` (6 definitions)

- OrderParamsV5.triggerDirection, types/request/v5-trade.ts → `OrderParamsV5_TriggerDirection`
- BatchOrderParamsV5.triggerDirection, types/request/v5-trade.ts → `BatchOrderParamsV5_TriggerDirection`
- SmallBalanceCoinV5.supportConvert, types/response/v5-asset.ts → `SmallBalanceCoinV5_SupportConvert`
- AccountOrderV5.triggerDirection, types/response/v5-trade.ts → `AccountOrderV5_TriggerDirection`
- ApiKeyInfoV5.type, types/response/v5-user.ts → `ApiKeyInfoV5_Type`
- SubAccountAllApiKeysResultV5_Result.type, types/response/v5-user.ts → `SubAccountAllApiKeysResultV5_Result_Type`

#### `1 | 6` (4 definitions)

- CreateSubMemberRequestV3.memberType, types/request/account-asset.ts → `CreateSubMemberRequestV3_MemberType`
- CreateSubMemberParamsV5.memberType, types/request/v5-user.ts → `CreateSubMemberParamsV5_MemberType`
- CreateSubMemberResponseV3.memberType, types/response/account-asset.ts → `CreateSubMemberResponseV3_MemberType`
- SubMemberV3.memberType, types/response/account-asset.ts → `SubMemberV3_MemberType`

## Unique inline types (34, single definition)

- `'' | 'Full' | 'Partial'` → `AccountOrderV5_TpslMode` — AccountOrderV5.tpslMode, types/response/v5-trade.ts
- `'' | 'iv' | 'price'` → `AccountOrderV5_PlaceType` — AccountOrderV5.placeType, types/response/v5-trade.ts
- `'' | 'OcoTriggerBySl' | 'OcoTriggerByUnknown' | 'OcoTriggerTp'` → `AccountOrderV5_OcoTriggerType` — AccountOrderV5.ocoTriggerType, types/response/v5-trade.ts
- `'1' | '2' | '3' | '4'` → `AffiliateUserInfoV5_TotalWalletBalance` — AffiliateUserInfoV5.totalWalletBalance, types/response/v5-user.ts
- `'1' | '2' | '3' | '4' | '5'` → `OrderParamsV5_BboLevel` — OrderParamsV5.bboLevel, types/request/v5-trade.ts
- `'15min' | '1d' | '1h' | '30min' | '4h' | '5min'` → `UMOpenInterestRequest_Interval` — UMOpenInterestRequest.interval, types/request/unified-margin.ts
- `'ACTIVE' | 'MODIFY'` → `UpdateP2PAdParamsV5_ActionType` — UpdateP2PAdParamsV5.actionType, types/request/v5-p2p-trading.ts
- `'AdlTrade' | 'BustTrade' | 'Funding' | 'Trade'` → `ContractUserExecutionHistoryRequest_ExecType` — ContractUserExecutionHistoryRequest.execType, types/request/contract.ts
- `'Available' | 'NotAvailable'` → `EarnProductV5_Status` — EarnProductV5.status, types/response/v5-earn.ts
- `'AWARD_AMOUNT_UNIT_COIN' | 'AWARD_AMOUNT_UNIT_USD'` → `BrokerVoucherSpecV5_AmountUnit` — BrokerVoucherSpecV5.amountUnit, types/response/v5-broker.ts
- `'BothSide' | 'MergedSingle'` → `LinearSetPositionModeRequest_Mode` — LinearSetPositionModeRequest.mode, types/request/linear.ts
- `'BTC' | 'ETH' | 'USDC' | 'USDT'` → `PreUpgradeTransaction_Currency` — PreUpgradeTransaction.currency, types/response/v5-preupgrade.ts
- `'Buy' | 'None' | 'Sell'` → `PreUpgradeTransaction_Side` — PreUpgradeTransaction.side, types/response/v5-preupgrade.ts
- `'Cancelled' | 'Filled' | 'Rejected'` → `SpreadOrderHistoryV5_OrderStatus` — SpreadOrderHistoryV5.orderStatus, types/response/v5-spreadtrading.ts
- `'connectionReady' | 'connectionReadyForAuth' | 'pong' | TEventType` → `EmittableEvent_EventType` — EmittableEvent.eventType, util/mod.ts
- `'Counterparty' | 'Queue'` → `OrderParamsV5_BboSideType` — OrderParamsV5.bboSideType, types/request/v5-trade.ts
- `'DELIVERING' | 'OFFLINE' | 'ONLINE' | 'WAITING_ONLINE'` → `USDCOptionsContractInfoRequest_Status` — USDCOptionsContractInfoRequest.status, types/request/usdc-options.ts
- `'DERIVATIVES' | 'OPTIONS' | 'SPOT'` → `DCPInfoV5_Product` — DCPInfoV5.product, types/response/v5-account.ts
- `'FUND' | 'SPOT'` → `WithdrawParamsV5_AccountType` — WithdrawParamsV5.accountType, types/request/v5-asset.ts
- `'IN' | 'OUT'` → `SubAccountTransferResponseV3_List_Type` — SubAccountTransferResponseV3_List.type, types/response/account-asset.ts
- `'inverse' | 'linear' | 'option'` → `GetDeliveryPriceParamsV5_Category` — GetDeliveryPriceParamsV5.category, types/request/v5-market.ts
- `'LEVEL_1' | 'LEVEL_2' | 'LEVEL_DEFAULT'` → `ApiKeyInfoV5_KycLevel` — ApiKeyInfoV5.kycLevel, types/response/v5-user.ts
- `'linear' | 'option'` → `GetPreUpgradeTransactionLogParamsV5_Category` — GetPreUpgradeTransactionLogParamsV5.category, types/request/v5-pre-upgrade.ts
- `'linear' | 'spot'` → `SpreadTradeLegV5_Category` — SpreadTradeLegV5.category, types/response/v5-spreadtrading.ts
- `'LinearFutures' | 'LinearPerpetual' | 'Spot'` → `SpreadInstrumentInfoV5_Legs_ContractType` — SpreadInstrumentInfoV5_Legs.contractType, types/response/v5-spreadtrading.ts
- `'New' | 'PartiallyFilled'` → `SpreadOpenOrderV5_OrderStatus` — SpreadOpenOrderV5.orderStatus, types/response/v5-spreadtrading.ts
- `'OFF' | 'ON'` → `SetCollateralCoinParamsV5_CollateralSwitch` — SetCollateralCoinParamsV5.collateralSwitch, types/request/v5-account.ts
- `'Order' | 'StopOrder'` → `GetPreUpgradeOrderHistoryParamsV5_OrderFilter` — GetPreUpgradeOrderHistoryParamsV5.orderFilter, types/request/v5-pre-upgrade.ts
- `'quoter' | 'request'` → `GetRFQListParamsV5_TraderType` — GetRFQListParamsV5.traderType, types/request/v5-rfq.ts
- `'Settling' | 'Trading'` → `SpreadInstrumentInfoV5_Status` — SpreadInstrumentInfoV5.status, types/response/v5-spreadtrading.ts
- `1 | 2 | 3` → `InternalDepositRecordV5_Status` — InternalDepositRecordV5.status, types/response/v5-asset.ts
- `1 | 2 | 4` → `SubMemberV3_Status` — SubMemberV3.status, types/response/account-asset.ts
- `14 | 180 | 21 | 270 | 30 | 60 | 7 | 90` → `GetHistoricalVolatilityParamsV5_Period` — GetHistoricalVolatilityParamsV5.period, types/request/v5-market.ts
- `TWSTopic | number | string` → `WsRequestOperationBybit_Args` — WsRequestOperationBybit.args, types/websockets/ws-api.ts