# TicketBill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number** | **i64** | A bill number. | 
**amount** | **f64** |  | 
**currency** | [**models::Currency**](Currency.md) |  | 
**label** | **String** |  | 
**r#type** | Option<**String**> |  | [optional]
**vat_rate** | Option<**f64**> |  | [optional]
**amount_without_vat** | **f64** | A bill amount minus VAT. | 
**vat_amount** | **f64** | A VAT amount portion of a bill amount. | 
**date_of_chargeable_event** | **String** |  | 
**buyer** | Option<[**models::Buyer**](Buyer.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


