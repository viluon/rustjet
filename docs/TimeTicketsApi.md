# \TimeTicketsApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_time_ticket**](TimeTicketsApi.md#cancel_time_ticket) | **PUT** /tickets/timetickets/{timeTicketId}/cancel | Delete time ticket by ID
[**get_time_ticket_by_id**](TimeTicketsApi.md#get_time_ticket_by_id) | **GET** /tickets/timetickets/{timeTicketId} | Get time ticket by ID
[**time_ticket_check_in**](TimeTicketsApi.md#time_ticket_check_in) | **PUT** /tickets/timetickets/checkin/{timeTicketId} | Verify time ticket for connection.



## cancel_time_ticket

> models::SuccessResponse cancel_time_ticket(time_ticket_id, request, x_lang)
Delete time ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**request** | [**CancelTicketRequest**](CancelTicketRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_ticket_by_id

> models::TimeTicket get_time_ticket_by_id(time_ticket_id, x_lang)
Get time ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::TimeTicket**](TimeTicket.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_ticket_check_in

> models::TimeTicketCheckinInfo time_ticket_check_in(time_ticket_id, body, x_lang)
Verify time ticket for connection.

Verify time ticket by timeTicketID, connectionID, stationId and time . If everithing is OK return 200. If ticket doesnt exists return 404, else return 400.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**body** | [**TimeTicketsCheckinRequest**](TimeTicketsCheckinRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::TimeTicketCheckinInfo**](TimeTicketCheckinInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

