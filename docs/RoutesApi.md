# \RoutesApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_departures**](RoutesApi.md#get_departures) | **GET** /routes/{stationId}/departures | Get arrivals and departures for the given station.
[**get_passengers_data**](RoutesApi.md#get_passengers_data) | **POST** /routes/{routeId}/passengersData | Get mandatory data of  first passenger and others passengers for the given route.
[**get_route_free_seats**](RoutesApi.md#get_route_free_seats) | **POST** /routes/{routeId}/freeSeats | Get route tandem free seats group by vehicle
[**get_simple_route_detail**](RoutesApi.md#get_simple_route_detail) | **GET** /routes/{routeId}/simple | Get detail for the given route.
[**search_routes**](RoutesApi.md#search_routes) | **GET** /routes/search | Get collection of all routes that satisfy specified search criteria for the route.
[**simple_search_routes**](RoutesApi.md#simple_search_routes) | **GET** /routes/search/simple | Get collection of all routes that satisfy specified search criteria for the route.



## get_departures

> models::ArrivalDepartureConnection get_departures(station_id, x_lang, limit)
Get arrivals and departures for the given station.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **i64** | ID of the station | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Maximal count of returned objects. Default value is 19. |  |[default to 19]

### Return type

[**models::ArrivalDepartureConnection**](ArrivalDepartureConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_passengers_data

> models::PassengersDataResponse get_passengers_data(route_id, filter, x_lang, x_currency)
Get mandatory data of  first passenger and others passengers for the given route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**filter** | [**PassengersDataRequest**](PassengersDataRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**models::PassengersDataResponse**](PassengersDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_route_free_seats

> models::RouteSeatsResponse get_route_free_seats(route_id, request, x_lang)
Get route tandem free seats group by vehicle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**request** | [**RouteSeatsRequest**](RouteSeatsRequest.md) | Descriptions of the Route detail | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::RouteSeatsResponse**](RouteSeatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_simple_route_detail

> models::Route get_simple_route_detail(route_id, from_station_id, to_station_id, x_lang, x_currency, tariffs)
Get detail for the given route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**from_station_id** | **i64** |  | [required] |
**to_station_id** | **i64** |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**tariffs** | Option<[**Vec<String>**](String.md)> | Ticket tariff |  |

### Return type

[**models::Route**](Route.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_routes

> models::SearchResult search_routes(from_location_id, from_location_type, to_location_id, to_location_type, x_lang, x_currency, departure_time, tariffs, action_price)
Get collection of all routes that satisfy specified search criteria for the route.

Authorization not required (filtres info bubbles)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_location_id** | **i64** |  | [required] |
**from_location_type** | **String** |  | [required] |
**to_location_id** | **i64** |  | [required] |
**to_location_type** | **String** |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**departure_time** | Option<**String**> | Departure date-time |  |
**tariffs** | Option<[**Vec<String>**](String.md)> | Ticket tariff |  |
**action_price** | Option<**String**> | Code indication of a current marketing action. Filtres searched routes on current marketing action. List of all marketing action for current route included in endpoint /consts/actionPrices. |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## simple_search_routes

> models::SimpleSearchResult simple_search_routes(from_location_id, from_location_type, to_location_id, to_location_type, x_lang, x_currency, x_used_departure_from_date_time, x_used_departure_to_date_time, departure_date, tariffs, action_price, r#move)
Get collection of all routes that satisfy specified search criteria for the route.

Authorization not required (filtres info bubbles)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_location_id** | **i64** |  | [required] |
**from_location_type** | **String** |  | [required] |
**to_location_id** | **i64** |  | [required] |
**to_location_type** | **String** |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_used_departure_from_date_time** | Option<**String**> | Used for repeatable move FORWARD/BACKWARD. Bears FROM date-time value used for previous route search (response returned in header) |  |
**x_used_departure_to_date_time** | Option<**String**> | Used for repeatable move FORWARD/BACKWARD. Bears TO date-time value used for previous route search (response returned in header) |  |
**departure_date** | Option<**String**> | Departure date |  |
**tariffs** | Option<[**Vec<String>**](String.md)> | Ticket tariff |  |
**action_price** | Option<**String**> | Code indication of a current marketing action. Filtres searched routes on current marketing action. List of all marketing action for current route included in endpoint /consts/actionPrices. |  |
**r#move** | Option<**String**> | Move search FORWARD/BACKWARD |  |

### Return type

[**models::SimpleSearchResult**](SimpleSearchResult.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

