# \RoutesApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_arrivals**](RoutesApi.md#get_arrivals) | **GET** /routes/{stationId}/arrivals | Get arrivals for the given station.
[**get_departures**](RoutesApi.md#get_departures) | **GET** /routes/{stationId}/departures | Get departures for the given station.
[**get_passengers_data**](RoutesApi.md#get_passengers_data) | **POST** /routes/{routeId}/passengersData | Get mandatory data of  first passenger and others passengers for the given route.
[**get_route_free_seats**](RoutesApi.md#get_route_free_seats) | **POST** /routes/freeSeats | Get route tandem free seats grouped by vehicles.
[**get_route_free_seats_by_route_id120**](RoutesApi.md#get_route_free_seats_by_route_id120) | **POST** /routes/{routeId}/freeSeats | Get route tandem free seats group by vehicle
[**get_simple_route_detail**](RoutesApi.md#get_simple_route_detail) | **GET** /routes/{routeId}/simple | Get detail for the given route.
[**get_sro_passengers_data**](RoutesApi.md#get_sro_passengers_data) | **GET** /routes/{routeId}/passengersData/{ticketType} | Get mandatory data for first passenger and other passengers for given route.
[**get_sro_route_detail**](RoutesApi.md#get_sro_route_detail) | **GET** /routes/{routeId}/RJ_SRO | Get details for the given route.
[**search_routes**](RoutesApi.md#search_routes) | **GET** /routes/search | Get collection of all routes that satisfy specified search criteria for the route.
[**search_sro_routes**](RoutesApi.md#search_sro_routes) | **GET** /routes/search/RJ_SRO | Search routes available for SRO tickets.
[**simple_search_routes**](RoutesApi.md#simple_search_routes) | **GET** /routes/search/simple | Get collection of all routes that satisfy specified search criteria for the route.



## get_arrivals

> models::ArrivalConnection get_arrivals(station_id, x_application_origin, x_lang, limit)
Get arrivals for the given station.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **i64** | ID of the station | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Maximal count of returned objects. Default value is 19. |  |[default to 19]

### Return type

[**models::ArrivalConnection**](ArrivalConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_departures

> models::DepartureConnection get_departures(station_id, x_application_origin, x_lang, limit)
Get departures for the given station.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **i64** | ID of the station | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Maximal count of returned objects. Default value is 19. |  |[default to 19]

### Return type

[**models::DepartureConnection**](DepartureConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/1.2.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_passengers_data

> models::PassengersDataResponse get_passengers_data(route_id, filter, x_application_origin, x_lang, x_currency)
Get mandatory data of  first passenger and others passengers for the given route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**filter** | [**PassengersDataRequest**](PassengersDataRequest.md) |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
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

> models::RouteSeatsResponse get_route_free_seats(request, x_application_origin, x_lang, x_currency, x_occupied)
Get route tandem free seats grouped by vehicles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**RouteSeatsRequest**](RouteSeatsRequest.md) | Descriptions of the Route detail | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_occupied** | Option<**bool**> | Return occupied seats. To turn off have to be deactivated at administrative app and X-occupied have to be false. Default value is false. |  |[default to false]

### Return type

[**models::RouteSeatsResponse**](RouteSeatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/1.1.0+json
- **Accept**: application/1.1.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_route_free_seats_by_route_id120

> models::RouteSeatsResponse120 get_route_free_seats_by_route_id120(route_id, request, x_application_origin, x_lang, x_occupied)
Get route tandem free seats group by vehicle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**request** | [**RouteSeatsRequest100**](RouteSeatsRequest100.md) | Descriptions of the Route detail | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_occupied** | Option<**bool**> | Return occupied seats. To turn off have to be deactivated at administrative app and X-occupied have to be false. Default value is false. |  |[default to false]

### Return type

[**models::RouteSeatsResponse120**](RouteSeatsResponse_1_2_0.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_simple_route_detail

> models::SimpleRouteDetail get_simple_route_detail(route_id, from_station_id, to_station_id, x_application_origin, x_lang, x_currency, tariffs)
Get detail for the given route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**from_station_id** | **i64** |  | [required] |
**to_station_id** | **i64** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**tariffs** | Option<[**Vec<String>**](String.md)> | Ticket tariff |  |

### Return type

[**models::SimpleRouteDetail**](SimpleRouteDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sro_passengers_data

> models::PassengersDataResponse get_sro_passengers_data(route_id, ticket_type, from_station_id, to_station_id, departure_date, seat_class, number_of_passengers, x_application_origin, x_lang, x_currency)
Get mandatory data for first passenger and other passengers for given route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**ticket_type** | **String** | Ticket type | [required] |
**from_station_id** | **i64** |  | [required] |
**to_station_id** | **i64** |  | [required] |
**departure_date** | **String** | Date and time of departure. Date is required. Time is optional. | [required] |
**seat_class** | **i32** | Seat class. | [required] |[default to 2]
**number_of_passengers** | **i32** | Number of passengers. | [required] |[default to 1]
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**models::PassengersDataResponse**](PassengersDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sro_route_detail

> models::SroRouteDetail get_sro_route_detail(route_id, from_station_id, to_station_id, departure_date, x_application_origin, x_lang, x_currency, seat_class, number_of_passengers)
Get details for the given route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **String** | ID of the route that contains IDs of sections separed by , (example: section0.id,section1.id, ... sectionx.id) | [required] |
**from_station_id** | **i64** |  | [required] |
**to_station_id** | **i64** |  | [required] |
**departure_date** | **String** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**seat_class** | Option<**i32**> | Seat class. |  |
**number_of_passengers** | Option<**i32**> | Number of passengers. |  |

### Return type

[**models::SroRouteDetail**](SroRouteDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_routes

> models::SearchResult search_routes(from_location_id, from_location_type, to_location_id, to_location_type, x_application_origin, x_lang, x_currency, departure_time, tariffs, action_price)
Get collection of all routes that satisfy specified search criteria for the route.

Authorization not required (filtres info bubbles)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_location_id** | **i64** |  | [required] |
**from_location_type** | **String** |  | [required] |
**to_location_id** | **i64** |  | [required] |
**to_location_type** | **String** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
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


## search_sro_routes

> Vec<models::SroRoute> search_sro_routes(from_location, to_location, x_application_origin, x_currency, x_lang, from_location_type, to_location_type, departure_date, seat_class, number_of_passengers)
Search routes available for SRO tickets.

Returns a list of all available routes for SRO tickets that satisfy specified search criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_location** | **String** |  | [required] |
**to_location** | **String** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**from_location_type** | Option<**String**> |  |  |[default to STATION]
**to_location_type** | Option<**String**> |  |  |[default to STATION]
**departure_date** | Option<**String**> | Date and time of departure. Date is required. Time is optional. |  |
**seat_class** | Option<**i32**> | Seat class. |  |[default to 2]
**number_of_passengers** | Option<**i32**> | Number of passengers. |  |[default to 1]

### Return type

[**Vec<models::SroRoute>**](SroRoute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## simple_search_routes

> models::SimpleSearchResult simple_search_routes(from_location_id, from_location_type, to_location_id, to_location_type, x_application_origin, x_lang, x_currency, x_used_departure_from_date_time, x_used_departure_to_date_time, departure_date, tariffs, action_price, r#move, affiliate)
Get collection of all routes that satisfy specified search criteria for the route.

Authorization not required (filtres info bubbles)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_location_id** | **String** |  | [required] |
**from_location_type** | **String** |  | [required] |
**to_location_id** | **String** |  | [required] |
**to_location_type** | **String** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_used_departure_from_date_time** | Option<**String**> | Used for repeatable move FORWARD/BACKWARD. Bears FROM date-time value used for previous route search (response returned in header) |  |
**x_used_departure_to_date_time** | Option<**String**> | Used for repeatable move FORWARD/BACKWARD. Bears TO date-time value used for previous route search (response returned in header) |  |
**departure_date** | Option<**String**> | Departure date |  |
**tariffs** | Option<[**Vec<String>**](String.md)> | Ticket tariff |  |
**action_price** | Option<**String**> | Code indication of a current marketing action. Filtres searched routes on current marketing action. List of all marketing action for current route included in endpoint /consts/actionPrices. |  |
**r#move** | Option<**String**> | Move search FORWARD/BACKWARD |  |
**affiliate** | Option<**String**> | Affiliate code |  |

### Return type

[**models::SimpleSearchResult**](SimpleSearchResult.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/1.2.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

