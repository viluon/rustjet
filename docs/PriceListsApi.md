# \PriceListsApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_time_ticket_prices**](PriceListsApi.md#get_time_ticket_prices) | **GET** /pricelists/timeticket/{routeId}/{fromStationId}/{toStationId}/{timeTicketType}/{validFrom}/{tariff} | Get time ticket prices for connection, tariff, ticket type, stationFrom, stationTo, dateTime and currency.



## get_time_ticket_prices

> Vec<models::TimeTicketPrice> get_time_ticket_prices(route_id, from_station_id, to_station_id, time_ticket_type, valid_from, tariff, x_application_origin, x_currency)
Get time ticket prices for connection, tariff, ticket type, stationFrom, stationTo, dateTime and currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_id** | **i64** | Route ID (connection ID). | [required] |
**from_station_id** | **i64** | Valid station ID (city ID unsupported at this level) | [required] |
**to_station_id** | **i64** | Valid station ID (city ID unsupported at this level) | [required] |
**time_ticket_type** | **String** | TimeTicket type enum.name (FLEX, DAY, WEEK, MONTH) | [required] |
**valid_from** | **String** | Date for validation date and time in TimeTicketPrices | [required] |
**tariff** | **String** | Tariff enum.name | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**Vec<models::TimeTicketPrice>**](TimeTicketPrice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

