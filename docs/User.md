# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> |  | [optional]
**account_code** | Option<**String**> | Used as account ID for login | [optional]
**first_name** | Option<**String**> |  | [optional]
**surname** | Option<**String**> |  | [optional]
**phone_number** | Option<**String**> |  | [optional]
**restrict_phone_numbers** | Option<**bool**> | Restrict phone number for work with sms. | [optional][default to false]
**email** | Option<**String**> |  | [optional]
**mojeid** | Option<**String**> |  | [optional]
**credit** | Option<**f32**> | Account balance | [optional]
**credit_price** | Option<**bool**> | Difference between registered credit account with price advantage and regular unregistered open account | [optional][default to true]
**currency** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]
**default_tariff_key** | Option<**String**> | Set tariff which would be pre-filled after login | [optional]
**notifications** | Option<[**crate::models::Notifications**](Notifications.md)> |  | [optional]
**company_information** | Option<**bool**> |  | [optional][default to false]
**company** | Option<[**crate::models::Company**](Company.md)> |  | [optional]
**conditions_acceptance** | Option<**bool**> | Acceptance of transport conditions and personal data protection | [optional][default to false]
**employee_number** | Option<**i32**> | NULL for normal customer, internal number for Regiojet employee | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


