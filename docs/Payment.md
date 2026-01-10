# Payment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_id** | **i64** |  | 
**ticket_id** | Option<**i64**> |  | [optional]
**ticket_type** | Option<[**models::PaymentTicketType**](PaymentTicketType.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**amount** | **f64** |  | 
**currency** | [**models::Currency**](Currency.md) |  | 
**method** | [**models::TransactionMethod**](TransactionMethod.md) |  | 
**date_transaction** | **String** |  | 
**is_receipt_available** | **bool** |  | 
**is_invoice_available** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


