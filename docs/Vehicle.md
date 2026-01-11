# Vehicle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**code** | Option<**String**> | Vehicle code tag (BUS => SPZ, VAGON => code) | [optional]
**r#type** | Option<[**models::VehicleType**](VehicleType.md)> |  | [optional]
**number** | **i32** |  | 
**seat_classes** | [**Vec<models::VehicleSeatClass>**](VehicleSeatClass.md) | Available services in this vehicle | 
**standard** | **String** | Vehicle standard code tag | 
**notifications** | Option<**Vec<String>**> | Additional informations relating to whole vehicle. These informations are visible, but wont requiring confirmation. | [optional]
**catering_enabled** | Option<**bool**> | Indicates if catering is enabled in current vehicle (i.e. it is possible for a customer to place catering order). | [optional]
**decks** | [**Vec<models::VehicleDeck>**](VehicleDeck.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


