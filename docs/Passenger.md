# Passenger

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> |  | [optional]
**first_name** | Option<**String**> | First name | [optional]
**surname** | Option<**String**> | Surname | [optional]
**phone** | Option<**String**> |  | [optional]
**email** | Option<**String**> | Email are always required after first passenger if not pre-filled in user account (or customer isnt logged in) | [optional]
**date_of_birth** | Option<[**String**](string.md)> |  | [optional]
**tariff** | **String** |  | 
**amount** | Option<**f32**> | Basic price (ticket purchase) for tariff set on ticket with ticket currency (doesnt count with discounts) | [optional]
**money_back** | Option<**f32**> | Final amount for passenger which will be refunded if canceled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


