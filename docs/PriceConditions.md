# PriceConditions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Control MD5 hash transformed from conditions content (used for current terms visibility check) | 
**descriptions** | [**crate::models::PriceConditionsDescriptions**](PriceConditionsDescriptions.md) |  | 
**refund_to_original_source_possible** | Option<**bool**> | States that if its possible to refund money to origin money source | [optional]
**cancel_charge** | Option<**f32**> | Total count of all cancel charges in current currency | [optional]
**cancel_charges** | Option<[**Vec<crate::models::CancelCharge>**](CancelCharge.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


