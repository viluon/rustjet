# Vehicle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vehicle_id** | **i64** |  | 
**code** | Option<**String**> | Vehicle code tag (BUS => SPZ, VAGON => code) | [optional]
**layout_url** | Option<**String**> |  | [optional]
**horizontal_layout_url** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::VehicleType**](VehicleType.md)> |  | [optional]
**vehicle_standard_key** | **String** | Vehicle standard code tag | 
**services** | Option<**Vec<String>**> | Supported services icons (wifi, etc.) | [optional]
**vehicle_number** | **i32** |  | 
**seat_classes** | **Vec<String>** | Available classes in this vehicle | 
**notifications** | Option<**Vec<String>**> | Additional informations relating to whole vehicle. These informations are visible, but wont requiring confirmation. | [optional]
**free_seats** | [**Vec<models::Seat>**](Seat.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


