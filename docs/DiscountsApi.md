# \DiscountsApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_percentual_by_user**](DiscountsApi.md#get_percentual_by_user) | **GET** /discounts/percentual | Get possible discounts for given account.
[**verify_code_discount**](DiscountsApi.md#verify_code_discount) | **POST** /discounts/code/{code}/verify | Verify code discount compatibility with a given route
[**verify_percentual_discount**](DiscountsApi.md#verify_percentual_discount) | **POST** /discounts/percentual/{discountId}/verify | Verify percentual discount compatibility with a given route



## get_percentual_by_user

> Vec<crate::models::Discount> get_percentual_by_user(x_lang)
Get possible discounts for given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::Discount>**](Discount.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_code_discount

> crate::models::VerifiedDiscountResponse verify_code_discount(code, ticket_request, x_lang, x_currency)
Verify code discount compatibility with a given route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**ticket_request** | [**VerifyDiscountRequest**](VerifyDiscountRequest.md) | Potential ticket information which needs to be checked by discount terms | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**crate::models::VerifiedDiscountResponse**](VerifiedDiscountResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_percentual_discount

> crate::models::VerifiedDiscountResponse verify_percentual_discount(discount_id, request, x_lang, x_currency)
Verify percentual discount compatibility with a given route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discount_id** | **i64** | Percentage discount ID which is supposed to be applied on route. | [required] |
**request** | [**VerifyDiscountRequest**](VerifyDiscountRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**x_currency** | Option<**String**> | Monetary unit to be used in responses. Use either \"CZK\" for Czech Crown, or \"EUR\" for Euro. Defaults to \"EUR\" on any other value. |  |[default to EUR]

### Return type

[**crate::models::VerifiedDiscountResponse**](VerifiedDiscountResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

