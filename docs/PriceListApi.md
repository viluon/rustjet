# \PriceListApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_time_ticket_price**](PriceListApi.md#get_time_ticket_price) | **POST** /pricelists/timeticket/{fromStationId}/{toStationId}/{timeTicketType}/{validFrom}/{tariff} | Get timeTicket prices for tariff, ticket type, stationFrom, stationTo, dateTime and currency.



## get_time_ticket_price

> Vec<models::TimeTicketPrice> get_time_ticket_price(from_station_id, to_station_id, time_ticket_type, valid_from, tariff, x_currency)
Get timeTicket prices for tariff, ticket type, stationFrom, stationTo, dateTime and currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_station_id** | **i64** | Valid station ID (city ID unsupported at this level) | [required] |
**to_station_id** | **i64** | Valid station ID (city ID unsupported at this level) | [required] |
**time_ticket_type** | **String** | TimeTicket type enum.name (FLEX, DAY, WEEK, MONTH) | [required] |
**valid_from** | **String** | Date for validation date and time in TimeTicketPrices | [required] |
**tariff** | **String** | Tariff enum.name | [required] |
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**Vec<models::TimeTicketPrice>**](TimeTicketPrice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

