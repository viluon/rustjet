# Discount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**percentage** | **i32** | Percentual discount size | 
**passengers** | **i32** | Maximal number of passengers | 
**from_location** | Option<**String**> | Default country/city/station translation | [optional]
**to_location** | Option<**String**> | Default arrival country/city/station translations | [optional]
**date_from** | Option<**String**> |  | [optional]
**date_to** | Option<**String**> |  | [optional]
**state** | [**models::DiscountState**](DiscountState.md) |  | 
**ticket_id** | Option<**i64**> | If there is used discount there is as well ticket ID which is linked to it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


