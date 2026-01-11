# Station

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Station identifier | 
**name** | **String** |  | 
**fullname** | **String** |  | 
**aliases** | Option<**Vec<String>**> | City nickname or aliases (Bratislava --> Blava) | [optional]
**address** | Option<**String**> |  | [optional]
**stations_types** | Option<**Vec<String>**> |  | [optional]
**iata_code** | Option<**String**> |  | [optional]
**station_url** | Option<**String**> |  | [optional]
**station_time_zone_code** | Option<**String**> |  | [optional]
**wheel_chair_platform** | Option<**String**> |  | [optional]
**significance** | **i32** | Station significancion (for example: for city Brno, there is Grand station more significant than Bohunice station) | 
**longitude** | Option<**f64**> | GPS - longitude | [optional]
**latitude** | Option<**f64**> | GPS - latitude | [optional]
**image_url** | Option<**String**> | Station picture URL (for example: may contain detail information about transfer) | [optional]
**cis_number** | Option<**i64**> | International ID of BusStation | [optional]
**sr70_number** | Option<**i64**> | International ID of RailwayStation | [optional]
**available** | **bool** | Defines if the station is currently available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


