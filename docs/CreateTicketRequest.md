# CreateTicketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**route** | [**crate::models::CreateTicketRouteRequest**](CreateTicketRouteRequest.md) |  | 
**selected_addons** | Option<[**Vec<crate::models::SelectedAddon>**](SelectedAddon.md)> |  | [optional]
**passengers** | [**Vec<crate::models::Passenger>**](Passenger.md) |  | 
**code_discount** | Option<**String**> | Flat rate discount from fare price (does not apply on addons, insurance and charges). Applies first (before percentual discount) | [optional]
**percentual_discount_ids** | Option<**Vec<i64>**> | Percentual discount from fare price (does not apply on addons, insurance and charges). Applies as seconds (after flat rate discount) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


