# Timetable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timetable_id** | Option<**i64**> | Timetable ID | [optional]
**connection_id** | **i64** | Connection ID | 
**from_city_name** | **String** | Departure city | 
**to_city_name** | **String** | Arrival city | 
**valid_from** | Option<[**String**](string.md)> |  | [optional]
**valid_to** | Option<[**String**](string.md)> |  | [optional]
**direction** | Option<**String**> |  | [optional]
**time_codes** | Option<**Vec<f64>**> | Timecodes used in this timetable | [optional]
**symbols** | Option<**Vec<String>**> | Symbols used in this timetable | [optional]
**connection_code** | **String** | Type and code of the connection | 
**carrier_id** | Option<**i64**> |  | [optional]
**stations** | [**Vec<models::TimetableStation>**](TimetableStation.md) | Individual stations (including metadata) sort according to order which connection builds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


