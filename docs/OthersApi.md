# \OthersApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_conta_form**](OthersApi.md#send_conta_form) | **POST** /support/sendContactForm | 
[**unsubscribe_from_survey**](OthersApi.md#unsubscribe_from_survey) | **PATCH** /unsubscription/survey | 



## send_conta_form

> models::SuccessResponse send_conta_form(request, x_application_origin, x_lang, x_re_captcha_token)


Sends support \"Contact our director\" form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**ContactFormRequest**](ContactFormRequest.md) |  | [required] |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_re_captcha_token** | Option<**String**> | Token (hash) provided Google ReCaptcha v3. |  |

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/1.1.0+json
- **Accept**: application/1.1.0+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_from_survey

> models::SuccessResponse unsubscribe_from_survey(ticket_id, hash, x_application_origin)


Unsubscribes rating challenge emails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | Option<**i64**> | TicketID of ticket included in link to unsubscribe email for surveys. |  |
**hash** | Option<**String**> | Unique hash of ticket included in link to unsubscribe email for surveys. |  |
**x_application_origin** | Option<**String**> | Application origin - APP - Mobile application (Android / Apple) - AFF - Affiliate application which is managed by third party - CAT - Web application used to sell catering - DEV - Only for development and testing - DOT - Check-in application for ticket sales on a train or bus - KSK - Kiosks - NOT - Unknown application type |  |[default to NOT]

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

