# PaymentMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_method_code** | **String** | Payment method code identifier | 
**payment_type** | [**crate::models::PaymentType**](PaymentType.md) |  | 
**active** | **bool** | Checks if payment method is active | 
**description** | Option<**String**> | Payment method status (for example: why its not available) | [optional]
**deadline** | Option<**String**> | Payment method availability expiration date-time | [optional]
**deadline_at** | Option<**i32**> | Minutes left to end of payment method availability | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


