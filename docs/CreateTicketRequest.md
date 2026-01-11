# CreateTicketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**route** | [**models::CreateTicketRouteRequest**](CreateTicketRouteRequest.md) |  | 
**selected_addons** | Option<[**Vec<models::SelectedAddon>**](SelectedAddon.md)> |  | [optional]
**passengers** | [**Vec<models::Passenger>**](Passenger.md) |  | 
**code_discount** | Option<**String**> | Flat rate discount from fare price (does not apply on addons and charges). Applies first (before percentual discount) | [optional]
**percentual_discount_ids** | Option<**Vec<i64>**> | Percentual discount from fare price (does not apply on addons and charges). Applies as seconds (after flat rate discount) | [optional]
**carbon_offset** | Option<[**models::AmountCurrency**](AmountCurrency.md)> |  | [optional]
**company** | Option<[**models::Company**](Company.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


