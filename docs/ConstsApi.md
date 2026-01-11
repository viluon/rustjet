# \ConstsApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_action_prices**](ConstsApi.md#get_action_prices) | **GET** /consts/actionPrices | Returns list of Action Prices
[**get_carriers**](ConstsApi.md#get_carriers) | **GET** /consts/carriers | Returns List of Carriers
[**get_city_pairs**](ConstsApi.md#get_city_pairs) | **GET** /consts/cityPairs | Returns List of possible CityPairs
[**get_integrated_transport_systems**](ConstsApi.md#get_integrated_transport_systems) | **GET** /consts/integrated-transport-systems | Get description of integrated transport system with superzones and zones.
[**get_line_symbols**](ConstsApi.md#get_line_symbols) | **GET** /consts/timetables/{timetableId}/symbols | Get symbols by given timetable id
[**get_line_symbols_by_line_group_code_line_number**](ConstsApi.md#get_line_symbols_by_line_group_code_line_number) | **GET** /consts/timetables/{code}/{number}/symbols | Get symbols by given line
[**get_line_time_codes**](ConstsApi.md#get_line_time_codes) | **GET** /consts/timetables/{timetableId}/time-codes | Get time codes by given timetable id
[**get_line_time_codes_by_line_group_code_line_number**](ConstsApi.md#get_line_time_codes_by_line_group_code_line_number) | **GET** /consts/timetables/{code}/{number}/time-codes | Get time code by given line
[**get_line_timetables**](ConstsApi.md#get_line_timetables) | **GET** /consts/timetables/{code}/{number} | Get lines for departure and arrival station
[**get_lines**](ConstsApi.md#get_lines) | **GET** /consts/lines/{fromStationId}/{toStationId} | Get lines for departure and arrival station
[**get_lines_by_departure_station**](ConstsApi.md#get_lines_by_departure_station) | **GET** /consts/lines/{stationId} | Get lines for departure station
[**get_locations**](ConstsApi.md#get_locations) | **GET** /consts/locations | List all possible locations served by RegioJet transportation services
[**get_online_paymenth_methods**](ConstsApi.md#get_online_paymenth_methods) | **GET** /consts/paymentMethods | Get the list of all payment methods
[**get_payment_definitions**](ConstsApi.md#get_payment_definitions) | **GET** /consts/paymentDefinitions | 
[**get_points_of_sales**](ConstsApi.md#get_points_of_sales) | **GET** /consts/points-of-sales | 
[**get_seat_classes**](ConstsApi.md#get_seat_classes) | **GET** /consts/seatClasses | Get all possible seat classes
[**get_symbols**](ConstsApi.md#get_symbols) | **GET** /consts/symbols | Get all symbols
[**get_tariffs**](ConstsApi.md#get_tariffs) | **GET** /consts/tariffs | Get all possible tariffs
[**get_timetable_by_bus_connection_id**](ConstsApi.md#get_timetable_by_bus_connection_id) | **GET** /consts/timetables/{busConnectionId} | Get timetable for busConnection
[**get_timetables**](ConstsApi.md#get_timetables) | **GET** /consts/timetables | Get timetables of all connections or of a single one
[**get_translations**](ConstsApi.md#get_translations) | **GET** /consts/translations/{language} | Get the used dictionary of the selected language
[**get_vehicle_standards**](ConstsApi.md#get_vehicle_standards) | **GET** /consts/vehicleStandards | Get all possible vehicle standards



## get_action_prices

> Vec<models::ActionPrice> get_action_prices(x_application_origin, x_lang, active_only)
Returns list of Action Prices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**active_only** | Option<**bool**> | Get only active of action prices |  |[default to true]

### Return type

[**Vec<models::ActionPrice>**](ActionPrice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_carriers

> Vec<models::Carrier> get_carriers(x_application_origin, x_lang)
Returns List of Carriers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::Carrier>**](Carrier.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_city_pairs

> Vec<models::CityPair> get_city_pairs(x_application_origin, x_lang, x_currency, from_city_id, to_city_id, number_of_transfers)
Returns List of possible CityPairs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**from_city_id** | Option<**i64**> | from City Id |  |
**to_city_id** | Option<**i64**> | to City Id |  |
**number_of_transfers** | Option<**i32**> | number of Transfers |  |[default to 0]

### Return type

[**Vec<models::CityPair>**](CityPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrated_transport_systems

> Vec<models::IntegratedTransportSystem> get_integrated_transport_systems(x_application_origin)
Get description of integrated transport system with superzones and zones.

Returns list as tree of ITS, superzones and zones.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]

### Return type

[**Vec<models::IntegratedTransportSystem>**](IntegratedTransportSystem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_line_symbols

> Vec<models::Symbol> get_line_symbols(timetable_id, x_application_origin, x_lang)
Get symbols by given timetable id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timetable_id** | **i64** | ID of the timetable | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::Symbol>**](Symbol.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_line_symbols_by_line_group_code_line_number

> Vec<models::Symbol> get_line_symbols_by_line_group_code_line_number(code, number, x_application_origin, x_lang)
Get symbols by given line

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**number** | **i32** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::Symbol>**](Symbol.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_line_time_codes

> Vec<models::TimeCode> get_line_time_codes(timetable_id, x_application_origin, x_lang)
Get time codes by given timetable id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timetable_id** | **i64** | ID of the timetable | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::TimeCode>**](TimeCode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_line_time_codes_by_line_group_code_line_number

> Vec<models::TimeCode> get_line_time_codes_by_line_group_code_line_number(code, number, x_application_origin, x_lang)
Get time code by given line

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**number** | **i32** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::TimeCode>**](TimeCode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_line_timetables

> Vec<models::Timetable> get_line_timetables(code, number, x_application_origin, x_lang, direction, timetable)
Get lines for departure and arrival station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**number** | **i32** |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**direction** | Option<**String**> |  |  |
**timetable** | Option<**i64**> |  |  |

### Return type

[**Vec<models::Timetable>**](Timetable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lines

> Vec<models::TimeTableLine> get_lines(from_station_id, to_station_id, x_application_origin, x_lang)
Get lines for departure and arrival station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_station_id** | **i64** | ID of the departure station | [required] |
**to_station_id** | **i64** | ID of the arrival station | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::TimeTableLine>**](TimeTableLine.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lines_by_departure_station

> Vec<models::TimeTableLine> get_lines_by_departure_station(station_id, x_application_origin, x_lang, code)
Get lines for departure station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **i64** | ID of the station | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**code** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::TimeTableLine>**](TimeTableLine.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locations

> Vec<models::Country> get_locations(x_application_origin, x_lang, station_type, include_unavailable)
List all possible locations served by RegioJet transportation services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**station_type** | Option<**String**> | * `BUS_STATION` - bus station * `TRAIN_STATION` - train station  |  |
**include_unavailable** | Option<**bool**> | If set to true the unavailable stations will be included in the response. |  |[default to false]

### Return type

[**Vec<models::Country>**](Country.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_online_paymenth_methods

> Vec<models::PaymentMethod> get_online_paymenth_methods(x_application_origin, x_lang, x_currency)
Get the list of all payment methods

Methods are marked as active if they are available and accept payments in the currency passed in X-Currency parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**Vec<models::PaymentMethod>**](PaymentMethod.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_definitions

> Vec<models::AllowedPaymentDefinition> get_payment_definitions(x_lang)


Gets list of available payment definitions with allowed currency for each channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::AllowedPaymentDefinition>**](AllowedPaymentDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_points_of_sales

> Vec<models::PointOfSale> get_points_of_sales(x_lang)


Get a list of points of sales.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::PointOfSale>**](PointOfSale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_seat_classes

> Vec<models::SeatClass> get_seat_classes(x_application_origin, x_lang)
Get all possible seat classes

Note the difference between tariffs, seat classes and vehicle standards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::SeatClass>**](SeatClass.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_symbols

> Vec<models::Symbol> get_symbols(x_application_origin, x_lang)
Get all symbols

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::Symbol>**](Symbol.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tariffs

> Vec<models::Tariff> get_tariffs(x_application_origin, x_lang)
Get all possible tariffs

Note the difference between tariffs, seat classes and vehicle standards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::Tariff>**](Tariff.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timetable_by_bus_connection_id

> models::Timetable get_timetable_by_bus_connection_id(bus_connection_id, x_application_origin, x_lang)
Get timetable for busConnection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bus_connection_id** | **i64** | ID of this particular connection | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::Timetable**](Timetable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timetables

> Vec<models::Timetable> get_timetables(x_application_origin, x_lang, connection_id)
Get timetables of all connections or of a single one

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**connection_id** | Option<**i64**> | ID of the connection for which we want the timetables. Timetables of all connections will be returned if the parameter is omitted. You can get connection IDs in responses of **_/routes/search**, in **Route.Section.Line.id** part of of the result. |  |

### Return type

[**Vec<models::Timetable>**](Timetable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translations

> std::collections::HashMap<String, String> get_translations(language, x_application_origin)
Get the used dictionary of the selected language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Very similar to X-Lang parameter of other endpoints, but it is taken from the path instead of a header field. | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vehicle_standards

> Vec<models::VehicleStandard> get_vehicle_standards(x_application_origin, x_lang)
Get all possible vehicle standards

Note the difference between tariffs, seat classes and vehicle standards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::VehicleStandard>**](VehicleStandard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

