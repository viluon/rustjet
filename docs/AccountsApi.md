# \AccountsApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_type**](AccountsApi.md#get_account_type) | **GET** /accounts/{accountCode}/type | Get account type
[**login**](AccountsApi.md#login) | **POST** /accounts/{accountCode}/login | 



## get_account_type

> models::AccountTypeResponse get_account_type(account_code, x_body_hash, x_re_captcha_token)
Get account type

Get account type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_code** | **String** | Account code | [required] |
**x_body_hash** | Option<**String**> | Hash of the request body HMAC(SHA-3-512(body), secretKey). |  |
**x_re_captcha_token** | Option<**String**> | Token (hash) provided Google ReCaptcha v3. |  |

### Return type

[**models::AccountTypeResponse**](AccountTypeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> models::Token login(account_code, x_lang)


Login account with account code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_code** | **String** |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

