# \PaymentsApi

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payment_form**](PaymentsApi.md#get_payment_form) | **POST** /payments/form | 
[**get_payments**](PaymentsApi.md#get_payments) | **GET** /payments | 
[**get_payments_methods**](PaymentsApi.md#get_payments_methods) | **POST** /payments/methods | 
[**pay_ticket_by_affiliate**](PaymentsApi.md#pay_ticket_by_affiliate) | **PUT** /payments/{ticketId}/pay | 
[**payments_credit_add**](PaymentsApi.md#payments_credit_add) | **POST** /payments/credit/add | 
[**payments_credit_charge**](PaymentsApi.md#payments_credit_charge) | **POST** /payments/credit/charge | 
[**payments_credit_gift_certificate_add**](PaymentsApi.md#payments_credit_gift_certificate_add) | **POST** /payments/credit/giftCertificate/add | 
[**payments_pay**](PaymentsApi.md#payments_pay) | **POST** /payments/pay | 
[**print_bulk_invoice**](PaymentsApi.md#print_bulk_invoice) | **POST** /payments/print/invoice | 
[**print_invoice**](PaymentsApi.md#print_invoice) | **GET** /payments/{paymentId}/print/invoice | Print payment invoice
[**print_receipt**](PaymentsApi.md#print_receipt) | **GET** /payments/{paymentId}/print/receipt | Print payment receipt
[**verify_gift_certificate**](PaymentsApi.md#verify_gift_certificate) | **POST** /payments/credit/giftCertificate/verify | 



## get_payment_form

> Vec<crate::models::PaymentFormField> get_payment_form(payment_form_request, x_lang)


Get the form desription for a payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_form_request** | [**PaymentFormFieldRequest**](PaymentFormFieldRequest.md) | Ticket IDs and selected payment method | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**Vec<crate::models::PaymentFormField>**](PaymentFormField.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payments

> Vec<crate::models::Payment> get_payments(x_lang, limit, offset, date_from, date_to, _type, sort_direction)


Get history of transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**limit** | Option<**i32**> | Page size limit (states how many elements should return). |  |[default to 10]
**offset** | Option<**i32**> | Page number (value must be dividable by limit) |  |[default to 0]
**date_from** | Option<**String**> | Filtres payments with realization date higher than |  |
**date_to** | Option<**String**> | Filtres payments with realization date lower than |  |
**_type** | Option<[**Vec<String>**](String.md)> | Filtres payments type(s) |  |
**sort_direction** | Option<**String**> | Sorting direction |  |[default to DESC]

### Return type

[**Vec<crate::models::Payment>**](Payment.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payments_methods

> Vec<crate::models::PaymentMethod> get_payments_methods(x_lang, payments_methods_request)


Get list of payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]
**payments_methods_request** | Option<[**PaymentsMethodsRequest**](PaymentsMethodsRequest.md)> | Collection of ticket IDs (may be empty or missing) |  |

### Return type

[**Vec<crate::models::PaymentMethod>**](PaymentMethod.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pay_ticket_by_affiliate

> crate::models::SuccessResponse pay_ticket_by_affiliate(ticket_id, x_lang)


Mark a ticket as paid directly

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **i64** | ID of the ticket with which to perform the given action (print, cancel, rate, etc.) | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_credit_add

> crate::models::CreditAddResponse payments_credit_add(credit_add_request, x_lang)


Add credit through a payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_add_request** | [**CreditAddRequest**](CreditAddRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::CreditAddResponse**](CreditAddResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_credit_charge

> crate::models::ChargeResponse payments_credit_charge(charge_request, x_lang)


Make a payment witch charge credit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**charge_request** | [**ChargeRequest**](ChargeRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::ChargeResponse**](ChargeResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_credit_gift_certificate_add

> crate::models::SuccessResponse payments_credit_gift_certificate_add(add_gift_certificate_request, x_lang)


Add credit through a gift certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_gift_certificate_request** | [**AddGiftCertificateRequest**](AddGiftCertificateRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_pay

> crate::models::CreditAddResponse payments_pay(pay_request, x_lang)


Make a payment through a payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pay_request** | [**PayRequest**](PayRequest.md) | Ticket(s) payment request | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::CreditAddResponse**](CreditAddResponse.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_bulk_invoice

> String print_bulk_invoice(payments, x_lang)


Print invoice for given payments (Da�ov� doklad). Note the unusual HTML output, not JSON like most other methods!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payments** | [**PrintInvoicesRequest**](PrintInvoicesRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

**String**

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_invoice

> String print_invoice(payment_id, x_lang)
Print payment invoice

Note the unusual HTML output, not JSON like most other methods!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **i64** | ID of the payment | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

**String**

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_receipt

> String print_receipt(payment_id, x_lang)
Print payment receipt

Note the unusual HTML output, not JSON like most other methods!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **i64** | ID of the payment | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

**String**

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_gift_certificate

> crate::models::GiftCertificateInfo verify_gift_certificate(verify_gift_certificate_request, x_lang)


Verify validity of a gift certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_gift_certificate_request** | [**VerifyGiftCertificateRequest**](VerifyGiftCertificateRequest.md) |  | [required] |
**x_lang** | Option<**String**> | The language into which to localise the response. It is not an ISO country code, even though some values are the same. Possible values are  - `cs` (Czech) - `sk` (Slovak) - `de` (German) - `hu` (Hungarian) - `fr` (French) - `es` (Spanish) - `ru` (Russian) - `ua` (Ukrainian) - `zh` (Chinese) - `en` (English)  Defaults to \"en\" on any other value. All endpoints accept this parameter, though the one for translations (/consts/translations/{lang}) ingores it and uses the language found in path instead.  |  |[default to en]

### Return type

[**crate::models::GiftCertificateInfo**](GiftCertificateInfo.md)

### Authorization

[Bearer](../README.md#Bearer), [BearerHash](../README.md#BearerHash)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

