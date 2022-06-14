# PassengersInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**passengers** | [**Vec<crate::models::Passenger>**](Passenger.md) |  | 
**first_passenger_data** | [**Vec<crate::models::PersonalDataType>**](PersonalDataType.md) | Required fields for 1st passenger (returns only if tickets is editable) | 
**other_passengers_data** | [**Vec<crate::models::PersonalDataType>**](PersonalDataType.md) | Required fields for all others passengers (returns only if tickets is editable) | 
**change_charge** | **f32** | Administrative charge for user information change (in ticket currency) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


