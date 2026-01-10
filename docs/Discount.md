# Discount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**percentage** | **i32** | Percentual discount size | 
**passengers** | **i32** | Maximal number of passengers | 
**from_country** | Option<**String**> | Country code value (see endpoint GET /consts/locations) | [optional]
**from_city_id** | Option<**i64**> | ID of the City from fromCountry (see endpoint GET /consts/locations) | [optional]
**from_station_id** | Option<**i64**> | ID of the Station from fromCity (see endpoint GET /consts/locations) | [optional]
**to_country** | Option<**String**> | ID of the City from toCountry (see endpoint GET /consts/locations) | [optional]
**to_city_id** | Option<**i64**> | City ID (see endpoint GET /consts/locations) | [optional]
**to_station_id** | Option<**i64**> | ID of the Station from toCity (see endpoint GET /consts/locations) | [optional]
**date_from** | Option<**String**> |  | [optional]
**date_to** | Option<**String**> |  | [optional]
**state** | [**models::DiscountState**](DiscountState.md) |  | 
**ticket_id** | Option<**i64**> | If there is used discount there is as well ticket ID which is linked to it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


