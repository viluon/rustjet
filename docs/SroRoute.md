# SroRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | {section0.id},{section1.id}, ... | 
**departure_station_id** | **i64** |  | 
**departure_time** | **String** |  | 
**arrival_station_id** | Option<**i64**> |  | [optional]
**arrival_time** | **String** |  | 
**vehicle_types** | [**Vec<models::VehicleType>**](VehicleType.md) | Vehicle types (for example: train, bus) | 
**transfers_count** | Option<**i32**> | Number of transfers | [optional]
**price_from** | **f64** | Lowest found price. | 
**price_to** | **f64** | Highest found price. | 
**prices_count** | Option<**i32**> | Overall number of found prices. | [optional]
**notices** | **bool** | Notice of any extraordinarily events on route / traffic limitation | 
**support** | **bool** | TRUE if this line (or its part) have support connection, otherwise FALSE | 
**national_trip** | **bool** | TRUE => national, FALSE => international | 
**delay** | **String** | Textual information about the first delay on the route | 
**travel_time** | **String** | Textual information about the travel time on the route | 
**vehicle_standard** | **String** | Vehicle standard code. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


