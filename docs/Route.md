# Route

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | {section0.id},{section1.id}, ...  | 
**main_section_id** | **i64** | Major part of the route. Only for this section can customer select Class or Standard. | 
**departure_station_id** | **i64** |  | 
**departure_station_name** | Option<**String**> |  | [optional]
**departure_city_id** | **i64** |  | 
**departure_city_name** | Option<**String**> |  | [optional]
**departure_time** | **String** |  | 
**arrival_station_id** | **i64** |  | 
**arrival_station_name** | Option<**String**> |  | [optional]
**arrival_city_id** | **i64** |  | 
**arrival_city_name** | Option<**String**> |  | [optional]
**arrival_time** | **String** |  | 
**free_seats_count** | **i32** | Returns smallest number of available free seats through all sections | 
**price_from** | Option<**f64**> |  | [optional]
**price_to** | Option<**f64**> |  | [optional]
**credit_price_from** | Option<**f64**> |  | [optional]
**credit_price_to** | Option<**f64**> |  | [optional]
**vehicle_types** | Option<[**Vec<models::VehicleType>**](VehicleType.md)> |  | [optional]
**price_classes** | Option<[**Vec<models::PriceClass>**](PriceClass.md)> |  | [optional]
**surcharge** | Option<[**models::Surcharge**](Surcharge.md)> |  | [optional]
**sections** | [**Vec<models::Section>**](Section.md) |  | 
**notices** | Option<**bool**> | Notice of any extraordinarily events on route / traffic limitation | [optional]
**transfers_info** | Option<[**models::TransfersInfo**](TransfersInfo.md)> |  | [optional]
**national_trip** | Option<**bool**> | TRUE => national, FALSE => international | [optional]
**bookable** | **bool** | TRUE if at least one class have enough free seats for reservation, otherwise FALSE | 
**delay** | Option<**String**> | Textual information about the first delay on the route | [optional]
**travel_time** | Option<**String**> | Textual information about the travel time on the route | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


