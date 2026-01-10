# \AppleWalletApi

All URIs are relative to *https://brn-qa-ybus-pubapi.sa.cz/restapi*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_apple_wallet_pass**](AppleWalletApi.md#get_apple_wallet_pass) | **GET** /apple-wallet/{ticketCode}/{sectionId} | Get apple wallet pass for given ticket code and section id.



## get_apple_wallet_pass

> std::path::PathBuf get_apple_wallet_pass(ticket_code, section_id)
Get apple wallet pass for given ticket code and section id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_code** | **String** | Unique numeric code representing a single ticket. | [required] |
**section_id** | **i64** | ID of bus connection. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerHash](../README.md#BearerHash), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.apple.pkpass

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

