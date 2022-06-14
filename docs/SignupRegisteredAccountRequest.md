# SignupRegisteredAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | **String** |  | 
**surname** | **String** |  | 
**phone_number** | Option<**String**> |  | [optional]
**restrict_phone_numbers** | Option<**bool**> | Restrict phone number for work with sms | [optional][default to false]
**email** | **String** |  | 
**mojeid** | Option<**String**> |  | [optional]
**company_information** | Option<**bool**> |  | [optional][default to false]
**company** | Option<[**crate::models::Company**](Company.md)> |  | [optional]
**default_tariff** | **String** |  | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**password** | **String** |  | 
**notifications** | [**crate::models::Notifications**](Notifications.md) |  | 
**agree_with_terms** | **bool** | Agree with terms | [default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


