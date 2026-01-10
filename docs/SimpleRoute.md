# SimpleRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | {section0.id},{section1.id}, ...  | 
**departure_station_id** | Option<**i64**> |  | [optional]
**departure_time** | **String** |  | 
**arrival_station_id** | Option<**i64**> |  | [optional]
**arrival_time** | **String** |  | 
**vehicle_types** | [**Vec<models::VehicleType>**](VehicleType.md) | Connection type (for example: train, bus) | 
**transfers_count** | Option<**i32**> | Amount of transfers | [optional]
**free_seats_count** | **i32** | Returns smallest number of available free seats through all sections | 
**price_from** | **f64** |  | 
**price_to** | Option<**f64**> |  | [optional]
**credit_price_from** | **f64** |  | 
**credit_price_to** | Option<**f64**> |  | [optional]
**prices_count** | **i32** | Amount of prices | 
**action_price** | [**models::ActionPrice**](ActionPrice.md) |  | 
**surcharge** | **bool** | TRUE if there is surcharge on this line, otherwise FALSE | 
**notices** | **bool** | Notice of any extraordinarily events on route / traffic limitation | 
**support** | **bool** | TRUE if this line (or its part) have support connection, otherwise FALSE | 
**national_trip** | Option<**bool**> | TRUE => national, FALSE => international | [optional]
**bookable** | **bool** | TRUE if at least one class have enough free seats for reservation, otherwise FALSE | 
**delay** | Option<**String**> | Textual information about the first delay on the route | [optional]
**travel_time** | Option<**String**> | Textual information about the travel time on the route | [optional]
**vehicle_standards** | Option<**Vec<String>**> | Vehicle standards | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


