# \LineModuleAttachApi

All URIs are relative to *https://manager.line.biz*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_module**](LineModuleAttachApi.md#attach_module) | **Post** /module/auth/v1/token | 



## attach_module

> models::AttachModuleResponse attach_module(grant_type, code, redirect_uri, code_verifier, client_id, client_secret, region, basic_search_id, scope, brand_type)


Attach by operation of the module channel provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | authorization_code | [required] |
**code** | **String** | Authorization code received from the LINE Platform. | [required] |
**redirect_uri** | **String** | Specify the redirect_uri specified in the URL for authentication and authorization. | [required] |
**code_verifier** | Option<**String**> | Specify when using PKCE (Proof Key for Code Exchange) defined in the OAuth 2.0 extension specification as a countermeasure against authorization code interception attacks. |  |
**client_id** | Option<**String**> | Instead of using Authorization header, you can use this parameter to specify the channel ID of the module channel. You can find the channel ID of the module channel in the LINE Developers Console.  |  |
**client_secret** | Option<**String**> | Instead of using Authorization header, you can use this parameter to specify the channel secret of the module channel. You can find the channel secret of the module channel in the LINE Developers Console.  |  |
**region** | Option<**String**> | If you specified a value for region in the URL for authentication and authorization, specify the same value.  |  |
**basic_search_id** | Option<**String**> | If you specified a value for basic_search_id in the URL for authentication and authorization, specify the same value. |  |
**scope** | Option<**String**> | If you specified a value for scope in the URL for authentication and authorization, specify the same value. |  |
**brand_type** | Option<**String**> | If you specified a value for brand_type in the URL for authentication and authorization, specify the same value. |  |

### Return type

[**models::AttachModuleResponse**](AttachModuleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

