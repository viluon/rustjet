# \SeatReservationOnlyTicketsApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_sro_ticket**](SeatReservationOnlyTicketsApi.md#cancel_sro_ticket) | **DELETE** /tickets/RJ_SRO/{sroTicketId} | Cancel seat reservation ticket.
[**get_all_sro_tickets**](SeatReservationOnlyTicketsApi.md#get_all_sro_tickets) | **GET** /tickets/RJ_SRO | Get all seat reservation only tickets of the user
[**get_sro_ticket_by_id**](SeatReservationOnlyTicketsApi.md#get_sro_ticket_by_id) | **GET** /tickets/RJ_SRO/{sroTicketId} | Get seat reservation only ticket by ID



## cancel_sro_ticket

> models::SuccessResponse cancel_sro_ticket(sro_ticket_id, body, x_application_origin, x_lang, x_tx_token)
Cancel seat reservation ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sro_ticket_id** | **i64** | ID of the seat reservation only ticket with which to perform the given action (get, print, cancel, etc.) | [required] |
**body** | [**CancelTicketRequest**](CancelTicketRequest.md) | Object with control hash and flag, which indicates whether money should be refunded to original payment channel. | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_tx_token** | Option<**String**> | Token (hash) identified transaction. |  |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_sro_tickets

> Vec<models::SroTicket> get_all_sro_tickets(x_application_origin, x_lang, limit, offset, departure_from, departure_to, arrival_from, arrival_to, ticket_states, sort_property, sort_direction)
Get all seat reservation only tickets of the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Page size limit (states how many records should return). |  |[default to 10]
**offset** | Option<**i32**> | Page number (value must be dividable by limit). |  |[default to 0]
**departure_from** | Option<**String**> | Filtres sro tickets with departure date higher than |  |
**departure_to** | Option<**String**> | Filtres sro tickets with departure date lower than |  |
**arrival_from** | Option<**String**> | Filtres sro tickets with arrival date higher than |  |
**arrival_to** | Option<**String**> | Filtres sro tickets with arrival date lower than |  |
**ticket_states** | Option<[**Vec<String>**](String.md)> | Filtres sro tickets by state (UNPAID, VALID, USED) |  |
**sort_property** | Option<**String**> | Sort property name |  |[default to DEPARTURE]
**sort_direction** | Option<**String**> | Sorting direction |  |[default to DESC]

### Return type

[**Vec<models::SroTicket>**](SroTicket.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sro_ticket_by_id

> models::SroTicket get_sro_ticket_by_id(sro_ticket_id, x_application_origin, x_lang)
Get seat reservation only ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sro_ticket_id** | **i64** | ID of the seat reservation only ticket with which to perform the given action (get, print, cancel, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::SroTicket**](SroTicket.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

