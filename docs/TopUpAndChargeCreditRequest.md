# TopUpAndChargeCreditRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f64** |  | 
**currency** | [**models::Currency**](Currency.md) |  | 
**payment_method_code** | **String** |  | 
**correlation_id** | **String** |  | 
**remember_card** | Option<**bool**> |  | [optional][default to false]
**form_fields** | [**Vec<models::PaymentFormFieldValue>**](PaymentFormFieldValue.md) |  | 
**tickets** | Option<[**Vec<models::PaymentTicketRequest>**](PaymentTicketRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


