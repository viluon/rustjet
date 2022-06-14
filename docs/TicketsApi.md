# \TicketsApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_ticket**](TicketsApi.md#cancel_ticket) | **PUT** /tickets/{ticketId}/cancel | Delete ticket by ID
[**cancel_ticket_by_affiliate**](TicketsApi.md#cancel_ticket_by_affiliate) | **PUT** /tickets/{accountCode}/{ticketId}/cancel/affiliate | Delete ticket by ID for affiliate partners
[**create_registered_tickets**](TicketsApi.md#create_registered_tickets) | **POST** /tickets/create/registered | Create new ticket(s) for an account
[**create_tickets_by_affiliate**](TicketsApi.md#create_tickets_by_affiliate) | **POST** /tickets/create/affiliate | Create new ticket(s) for new account as affiliate partner
[**create_unregistered_tickets**](TicketsApi.md#create_unregistered_tickets) | **POST** /tickets/create/unregistered | Create new ticket(s) for new account
[**delete_passenger**](TicketsApi.md#delete_passenger) | **DELETE** /tickets/{ticketId}/passengers/{passengerId} | Delete passenger from the ticket
[**get_all_tickets**](TicketsApi.md#get_all_tickets) | **GET** /tickets | Get all tickets of the user
[**get_all_tickets_for_affiliate**](TicketsApi.md#get_all_tickets_for_affiliate) | **GET** /tickets/{accountCode}/affiliate | Get all tickets of the user for affiliate partner
[**get_ticket_by_id**](TicketsApi.md#get_ticket_by_id) | **GET** /tickets/{ticketId} | Get ticket by ID
[**get_ticket_by_idfor_affiliate**](TicketsApi.md#get_ticket_by_idfor_affiliate) | **GET** /tickets/{accountCode}/{ticketId}/affiliate | Get ticket by ID for affiliate partner
[**get_ticket_detail_rating**](TicketsApi.md#get_ticket_detail_rating) | **GET** /tickets/{ticketId}/rating | Get ticket rating questions
[**get_ticket_qr_code**](TicketsApi.md#get_ticket_qr_code) | **GET** /tickets/{ticketId}/qrcode | Get GR code for ticket
[**get_ticket_qr_code_png**](TicketsApi.md#get_ticket_qr_code_png) | **GET** /tickets/{ticketId}/qrcode/png | GR code for ticket
[**get_unpaid_tickets**](TicketsApi.md#get_unpaid_tickets) | **GET** /tickets/unpaid | Get unpaid tickets (and tickets with remaining items to pay) of the user.
[**print_ticket**](TicketsApi.md#print_ticket) | **GET** /tickets/{ticketId}/print | Print ticket
[**put_ticket_detail_rating**](TicketsApi.md#put_ticket_detail_rating) | **PUT** /tickets/{ticketId}/rating | Update ticket rating
[**send_ticket_by_email**](TicketsApi.md#send_ticket_by_email) | **POST** /tickets/{ticketId}/sendByEmail | Send ticket to email
[**update_passenger**](TicketsApi.md#update_passenger) | **PUT** /tickets/{ticketId}/passengers/{passengerId} | Update passenger on the ticket



## cancel_ticket

> crate::models::SuccessResponse cancel_ticket(ticket_id, request, x_lang)
Delete ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**request** | [**CancelTicketRequest**](CancelTicketRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_ticket_by_affiliate

> crate::models::SuccessResponse cancel_ticket_by_affiliate(account_code, ticket_id, request, x_lang)
Delete ticket by ID for affiliate partners

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_code** | **String** | Text code identifying a customer's account. | [required] |
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**request** | [**CancelTicketRequest**](CancelTicketRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_registered_tickets

> crate::models::CreateTicketResponseRegistered create_registered_tickets(tickets_request, x_lang, x_currency)
Create new ticket(s) for an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickets_request** | [**CreateRegisteredTicketRequest**](CreateRegisteredTicketRequest.md) | Ticket(s) to be created | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**crate::models::CreateTicketResponseRegistered**](CreateTicketResponseRegistered.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/1.1.0+json
- **Accept**: application/1.1.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tickets_by_affiliate

> crate::models::CreateTicketResponseUnregistered create_tickets_by_affiliate(tickets_request, x_lang, x_currency)
Create new ticket(s) for new account as affiliate partner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickets_request** | [**CreateUnregisteredTicketRequest**](CreateUnregisteredTicketRequest.md) | Ticket(s) to be created | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**crate::models::CreateTicketResponseUnregistered**](CreateTicketResponseUnregistered.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_unregistered_tickets

> crate::models::CreateTicketResponseUnregistered create_unregistered_tickets(tickets_request, x_lang, x_currency, x_body_hash, x_re_captcha_token)
Create new ticket(s) for new account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickets_request** | [**CreateUnregisteredTicketRequest**](CreateUnregisteredTicketRequest.md) | Ticket(s) to be created | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_body_hash** | Option<**String**> | Hash of the request body HMAC(SHA-3-512(body), secretKey). |  |
**x_re_captcha_token** | Option<**String**> | Token (hash) provided Google ReCaptcha v3. |  |

### Return type

[**crate::models::CreateTicketResponseUnregistered**](CreateTicketResponseUnregistered.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/1.1.0+json
- **Accept**: application/1.1.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_passenger

> crate::models::SuccessResponse delete_passenger(ticket_id, passenger_id, request, x_lang)
Delete passenger from the ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**passenger_id** | **i64** | ID of the passenger | [required] |
**request** | [**DeletePassengerRequest**](DeletePassengerRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tickets

> Vec<crate::models::Ticket> get_all_tickets(x_lang, limit, offset, departure_from, departure_to, arrival_from, arrival_to, ticket_states, sort_property, sort_direction, simple)
Get all tickets of the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Page size limit (states how many records should return). |  |[default to 10]
**offset** | Option<**i32**> | Page number (value must be dividable by limit). |  |[default to 0]
**departure_from** | Option<**String**> | Filtres tickets with departure date higher than |  |
**departure_to** | Option<**String**> | Filtres tickets with departure date lower than |  |
**arrival_from** | Option<**String**> | Filtres tickets with arrival date higher than |  |
**arrival_to** | Option<**String**> | Filtres tickets with arrival date lower than |  |
**ticket_states** | Option<[**Vec<String>**](String.md)> | Filtres tickets by state (UNPAID, VALID, USED) |  |
**sort_property** | Option<**String**> | Sort property name |  |[default to DEPARTURE]
**sort_direction** | Option<**String**> | Sorting direction |  |[default to DESC]
**simple** | Option<**bool**> | Get simplification of the tickets |  |[default to false]

### Return type

[**Vec<crate::models::Ticket>**](Ticket.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tickets_for_affiliate

> Vec<crate::models::Ticket> get_all_tickets_for_affiliate(account_code, x_lang, limit, offset, departure_from, departure_to, arrival_from, arrival_to, ticket_states, sort_property, sort_direction)
Get all tickets of the user for affiliate partner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_code** | **String** | Text code identifying a customer's account. | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Page size limit (states how many records should return). |  |[default to 10]
**offset** | Option<**i32**> | Page number (value must be dividable by limit). |  |[default to 0]
**departure_from** | Option<**String**> | Filtres tickets with departure date lower than |  |
**departure_to** | Option<**String**> | Filtres tickets with departure date lower than |  |
**arrival_from** | Option<**String**> | Filtres tickets with arrival date higher than |  |
**arrival_to** | Option<**String**> | Filtres tickets with arrival date lower than |  |
**ticket_states** | Option<[**Vec<String>**](String.md)> | Filtres tickets by state (UNPAID, VALID, USED) |  |
**sort_property** | Option<**String**> | Sort property name |  |[default to DEPARTURE]
**sort_direction** | Option<**String**> | Sorting direction |  |[default to DESC]

### Return type

[**Vec<crate::models::Ticket>**](Ticket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_by_id

> crate::models::Ticket get_ticket_by_id(ticket_id, x_lang)
Get ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::Ticket**](Ticket.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_by_idfor_affiliate

> crate::models::Ticket get_ticket_by_idfor_affiliate(ticket_id, account_code, x_lang)
Get ticket by ID for affiliate partner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**account_code** | **String** | Text code identifying a customer's account. | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::Ticket**](Ticket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_detail_rating

> Vec<crate::models::RatingFormData> get_ticket_detail_rating(ticket_id, x_lang)
Get ticket rating questions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::RatingFormData>**](RatingFormData.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_qr_code

> crate::models::QrCodeTicket get_ticket_qr_code(ticket_id, x_lang)
Get GR code for ticket

Get QR code content in JSON format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::QrCodeTicket**](QrCodeTicket.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_qr_code_png

> crate::models::QrCodeTicket get_ticket_qr_code_png(ticket_id, x_lang)
GR code for ticket

Get QR code as a PNG image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::QrCodeTicket**](QrCodeTicket.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unpaid_tickets

> Vec<crate::models::Ticket> get_unpaid_tickets(x_lang, sort_property, sort_direction)
Get unpaid tickets (and tickets with remaining items to pay) of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**sort_property** | Option<**String**> | Sort property name |  |[default to DEPARTURE]
**sort_direction** | Option<**String**> | Sorting direction |  |[default to DESC]

### Return type

[**Vec<crate::models::Ticket>**](Ticket.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_ticket

> String print_ticket(ticket_id, x_lang)
Print ticket

Note the unusual HTML output, not JSON like most other methods!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

**String**

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_ticket_detail_rating

> crate::models::SuccessResponse put_ticket_detail_rating(ticket_id, rating, x_lang)
Update ticket rating

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**rating** | [**PutRatingRequest**](PutRatingRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_ticket_by_email

> crate::models::SuccessResponse send_ticket_by_email(ticket_id, email, x_lang)
Send ticket to email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**email** | [**TicketEmailRequest**](TicketEmailRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_passenger

> crate::models::PassengerChangeResponse update_passenger(ticket_id, passenger_id, passenger, x_lang)
Update passenger on the ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**passenger_id** | **i64** | ID of the passenger | [required] |
**passenger** | [**PassengerRequest**](PassengerRequest.md) | Updated Passenger object | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::PassengerChangeResponse**](PassengerChangeResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

