# \TimeTicketsApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_time_ticket**](TimeTicketsApi.md#cancel_time_ticket) | **DELETE** /tickets/timeticket/{timeTicketId} | Storno time ticket by id
[**create_time_ticket**](TimeTicketsApi.md#create_time_ticket) | **POST** /tickets/timeticket | Creates time ticket.
[**create_unregistered_time_ticket**](TimeTicketsApi.md#create_unregistered_time_ticket) | **POST** /tickets/timetickets/unregistered | Create unregistered time ticket.
[**get_all_time_tickets**](TimeTicketsApi.md#get_all_time_tickets) | **GET** /tickets/RJ_TIME | Get IDs of time tickets and time tickets of a user.
[**get_all_valid_time_tickets**](TimeTicketsApi.md#get_all_valid_time_tickets) | **GET** /tickets/timetickets | Get IDs of time tickets and time tickets of a user.
[**get_time_ticket_by_id**](TimeTicketsApi.md#get_time_ticket_by_id) | **GET** /tickets/timeTickets/{ticketId} | Get ticket by ID
[**get_time_ticket_qr_code**](TimeTicketsApi.md#get_time_ticket_qr_code) | **GET** /tickets/timeticket/{ticketId}/qrcode | Get QR code for time ticket
[**get_time_ticket_qr_code_png**](TimeTicketsApi.md#get_time_ticket_qr_code_png) | **GET** /tickets/timeticket/{ticketId}/qrcode/png | GR code for time ticket
[**get_unpaid_time_tickets**](TimeTicketsApi.md#get_unpaid_time_tickets) | **GET** /tickets/timetickets/unpaid | Get unpaid time tickets of the user.
[**print_time_ticket**](TimeTicketsApi.md#print_time_ticket) | **GET** /tickets/timeticket/{timeTicketId}/print | Print time ticket
[**print_time_ticket_pdf**](TimeTicketsApi.md#print_time_ticket_pdf) | **GET** /tickets/timeticket/{timeTicketId}/pdf | Print time ticket in pdf
[**send_time_ticket_by_email**](TimeTicketsApi.md#send_time_ticket_by_email) | **POST** /tickets/timeticket/{timeTicketId}/sendByEmail | Send time ticket to email



## cancel_time_ticket

> models::SuccessResponse cancel_time_ticket(time_ticket_id, x_application_origin, x_lang)
Storno time ticket by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (get, print, cancel, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_time_ticket

> i64 create_time_ticket(body, x_application_origin, x_lang, x_currency, x_tx_token)
Creates time ticket.

Creates time ticket flexi or line

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateRegisteredTimeTicketsRequest**](CreateRegisteredTimeTicketsRequest.md) |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_tx_token** | Option<**String**> | Token (hash) identified transaction. |  |

### Return type

**i64**

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/1.2.0+json
- **Accept**: application/1.2.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_unregistered_time_ticket

> models::UnregisteredTimeTicketCreateInfo create_unregistered_time_ticket(body, x_application_origin, x_re_captcha_token, x_lang, x_currency, x_tx_token)
Create unregistered time ticket.

Create unregistered time ticket. Only FLEX time ticket type is allowed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateUnregisteredTimeTicketRequest**](CreateUnregisteredTimeTicketRequest.md) |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_re_captcha_token** | Option<**String**> | Token (hash) provided Google ReCaptcha v3. |  |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]
**x_tx_token** | Option<**String**> | Token (hash) identified transaction. |  |

### Return type

[**models::UnregisteredTimeTicketCreateInfo**](UnregisteredTimeTicketCreateInfo.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/1.2.0+json
- **Accept**: application/1.2.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_time_tickets

> models::AccountTimeTickets get_all_time_tickets(x_application_origin, x_lang, limit, offset, departure_from, departure_to, arrival_from, arrival_to, ticket_states, sort_direction)
Get IDs of time tickets and time tickets of a user.

Returns the IDs of all time tickets and full info of all time tickets of a logged in user account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Page size limit (states how many records should return). |  |[default to 10]
**offset** | Option<**i32**> | Page number (value must be dividable by limit). |  |[default to 0]
**departure_from** | Option<**String**> | Filtres time tickets with validFrom date higher than |  |
**departure_to** | Option<**String**> | Filtres time tickets with validFrom date lower than |  |
**arrival_from** | Option<**String**> | Filtres time tickets with validTo date higher than |  |
**arrival_to** | Option<**String**> | Filtres time tickets with validTo date lower than |  |
**ticket_states** | Option<[**Vec<String>**](String.md)> | Filtres time tickets by state (UNPAID, VALID, USED) |  |
**sort_direction** | Option<**String**> | Sort direction (ASC, DESC) |  |[default to ASC]

### Return type

[**models::AccountTimeTickets**](AccountTimeTickets.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_valid_time_tickets

> models::AccountTimeTickets get_all_valid_time_tickets(x_application_origin, x_lang, limit, offset, valid_from, valid_to, ticket_states, sort_direction)
Get IDs of time tickets and time tickets of a user.

Returns the IDs of all time tickets and full info of all time tickets of a logged in user account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Page size limit (states how many records should return). |  |[default to 10]
**offset** | Option<**i32**> | Page number (value must be dividable by limit). |  |[default to 0]
**valid_from** | Option<**String**> | Filtres time tickets with validFrom higher than |  |
**valid_to** | Option<**String**> | Filtres time tickets with validTo less than |  |[default to 9999-01-01T00:00+01:00]
**ticket_states** | Option<[**Vec<String>**](String.md)> | Filtres tickets by state (UNPAID, VALID, CANCELED, DELETED, EXPIRED, TO_BE_EXPIRED) |  |[default to ["UNPAID","VALID","CANCELED","DELETED"]]
**sort_direction** | Option<**String**> | Sort direction (ASC, DESC) |  |[default to ASC]

### Return type

[**models::AccountTimeTickets**](AccountTimeTickets.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_ticket_by_id

> models::TimeTicket get_time_ticket_by_id(ticket_id, x_application_origin, x_lang)
Get ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::TimeTicket**](TimeTicket.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_ticket_qr_code

> models::QrCodeTimeTicket get_time_ticket_qr_code(ticket_id, x_application_origin, x_lang)
Get QR code for time ticket

Get QR code content in CSV format (European CSV/DSV). Fields separated by the semicolon character and row does not terminate with the newline character.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::QrCodeTimeTicket**](QrCodeTimeTicket.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_ticket_qr_code_png

> models::QrCodeTimeTicket get_time_ticket_qr_code_png(ticket_id, x_application_origin, x_lang)
GR code for time ticket

Get QR code as a PNG image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::QrCodeTimeTicket**](QrCodeTimeTicket.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unpaid_time_tickets

> Vec<models::TimeTicket> get_unpaid_time_tickets(x_application_origin, x_lang)
Get unpaid time tickets of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<models::TimeTicket>**](TimeTicket.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_time_ticket

> String print_time_ticket(time_ticket_id, x_application_origin, x_lang)
Print time ticket

Time ticket in pdf format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (get, print, cancel, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

**String**

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_time_ticket_pdf

> std::path::PathBuf print_time_ticket_pdf(time_ticket_id, x_application_origin, x_lang)
Print time ticket in pdf

Note the unusual HTML output, not JSON like most other methods!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (get, print, cancel, etc.) | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_time_ticket_by_email

> send_time_ticket_by_email(time_ticket_id, email, x_application_origin, x_lang)
Send time ticket to email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_ticket_id** | **i64** | ID of the timeTicket with which to perform the given action (get, print, cancel, etc.) | [required] |
**email** | [**TicketEmailRequest**](TicketEmailRequest.md) |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

 (empty response body)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

