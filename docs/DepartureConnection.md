# DepartureConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_id** | Option<**i64**> | ID of the connection | [optional]
**label** | Option<**String**> |  | [optional]
**number** | Option<**String**> |  | [optional]
**delay** | Option<**i32**> | Information about the delay in minutes on a given connection | [optional]
**vehicle_category** | Option<[**models::VehicleCategory**](VehicleCategory.md)> |  | [optional]
**free_seat_count** | Option<**i32**> |  | [optional]
**stations** | Option<[**Vec<models::DepartureConnectionStation>**](DepartureConnectionStation.md)> | Individual stations (including metadata) sort according to order which connection builds. | [optional]
**vehicle_standard** | Option<**String**> | Vehicle standard code. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


