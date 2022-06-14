# \ConstsApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**consts_translations_language_get**](ConstsApi.md#consts_translations_language_get) | **GET** /consts/translations/{language} | Get the used dictionary of the selected language
[**get_action_prices**](ConstsApi.md#get_action_prices) | **GET** /consts/actionPrices | Returns list of Action Prices
[**get_carriers**](ConstsApi.md#get_carriers) | **GET** /consts/carriers | Returns List of Carriers
[**get_city_pairs**](ConstsApi.md#get_city_pairs) | **GET** /consts/cityPairs | Returns List of possible CityPairs
[**get_locations**](ConstsApi.md#get_locations) | **GET** /consts/locations | List all possible locations served by RegioJet transportation services
[**get_online_paymenth_methods**](ConstsApi.md#get_online_paymenth_methods) | **GET** /consts/paymentMethods | Get the list of all payment methods
[**get_seat_classes**](ConstsApi.md#get_seat_classes) | **GET** /consts/seatClasses | Get all possible seat classes
[**get_tariffs**](ConstsApi.md#get_tariffs) | **GET** /consts/tariffs | Get all possible tariffs
[**get_timetables**](ConstsApi.md#get_timetables) | **GET** /consts/timetables | Get timetables of all connections or of a single one
[**get_vehicle_standards**](ConstsApi.md#get_vehicle_standards) | **GET** /consts/vehicleStandards | Get all possible vehicle standards



## consts_translations_language_get

> ::std::collections::HashMap<String, String> consts_translations_language_get(language)
Get the used dictionary of the selected language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | Very similar to X-Lang parameter of other endpoints, but it is taken from the path instead of a header field. | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_action_prices

> Vec<crate::models::ActionPrice> get_action_prices(x_lang, active_only)
Returns list of Action Prices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**active_only** | Option<**bool**> | Get only active of action prices |  |[default to true]

### Return type

[**Vec<crate::models::ActionPrice>**](ActionPrice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_carriers

> Vec<crate::models::Carrier> get_carriers(x_lang)
Returns List of Carriers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::Carrier>**](Carrier.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_city_pairs

> Vec<crate::models::CityPair> get_city_pairs(x_lang, x_currency, from_city_id, to_city_id, number_of_transfers)
Returns List of possible CityPairs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**from_city_id** | Option<**i64**> | from City Id |  |
**to_city_id** | Option<**i64**> | to City Id |  |
**number_of_transfers** | Option<**i32**> | number of Transfers |  |[default to 0]

### Return type

[**Vec<crate::models::CityPair>**](CityPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locations

> Vec<crate::models::Country> get_locations(x_lang, station_type)
List all possible locations served by RegioJet transportation services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**station_type** | Option<**String**> | * `BUS_STATION` - bus station * `TRAIN_STATION` - train station  |  |

### Return type

[**Vec<crate::models::Country>**](Country.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_online_paymenth_methods

> Vec<crate::models::PaymentMethod> get_online_paymenth_methods(x_lang, x_currency)
Get the list of all payment methods

Methods are marked as active if they are available and accept payments in the currency passed in X-Currency parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**Vec<crate::models::PaymentMethod>**](PaymentMethod.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_seat_classes

> Vec<crate::models::SeatClass> get_seat_classes(x_lang)
Get all possible seat classes

Note the difference between tariffs, seat classes and vehicle standards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::SeatClass>**](SeatClass.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tariffs

> Vec<crate::models::Tariff> get_tariffs(x_lang)
Get all possible tariffs

Note the difference between tariffs, seat classes and vehicle standards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::Tariff>**](Tariff.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timetables

> Vec<crate::models::Timetable> get_timetables(x_lang, connection_id)
Get timetables of all connections or of a single one

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**connection_id** | Option<**i64**> | ID of the connection for which we want the timetables. Timetables of all connections will be returned if the parameter is omitted. You can get connection IDs in responses of **_/routes/search**, in **Route.Section.Line.id** part of of the result. |  |

### Return type

[**Vec<crate::models::Timetable>**](Timetable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vehicle_standards

> Vec<crate::models::VehicleStandard> get_vehicle_standards(x_lang)
Get all possible vehicle standards

Note the difference between tariffs, seat classes and vehicle standards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::VehicleStandard>**](VehicleStandard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

