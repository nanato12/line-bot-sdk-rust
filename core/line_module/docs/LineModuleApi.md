# \LineModuleApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acquire_chat_control**](LineModuleApi.md#acquire_chat_control) | **Post** /v2/bot/chat/{chatId}/control/acquire | 
[**detach_module**](LineModuleApi.md#detach_module) | **Post** /v2/bot/channel/detach | 
[**get_modules**](LineModuleApi.md#get_modules) | **Get** /v2/bot/list | 
[**release_chat_control**](LineModuleApi.md#release_chat_control) | **Post** /v2/bot/chat/{chatId}/control/release | 



## acquire_chat_control

> acquire_chat_control(chat_id, acquire_chat_control_request)


If the Standby Channel wants to take the initiative (Chat Control), it calls the Acquire Control API. The channel that was previously an Active Channel will automatically switch to a Standby Channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | The `userId`, `roomId`, or `groupId` | [required] |
**acquire_chat_control_request** | Option<[**AcquireChatControlRequest**](AcquireChatControlRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_module

> detach_module(detach_module_request)


The module channel admin calls the Detach API to detach the module channel from a LINE Official Account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**detach_module_request** | Option<[**DetachModuleRequest**](DetachModuleRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_modules

> models::GetModulesResponse get_modules(start, limit)


Gets a list of basic information about the bots of multiple LINE Official Accounts that have attached module channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> | Value of the continuation token found in the next property of the JSON object returned in the response. If you can't get all basic information about the bots in one request, include this parameter to get the remaining array.  |  |
**limit** | Option<**i32**> | Specify the maximum number of bots that you get basic information from. The default value is 100. Max value: 100  |  |[default to 100]

### Return type

[**models::GetModulesResponse**](GetModulesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## release_chat_control

> release_chat_control(chat_id)


To return the initiative (Chat Control) of Active Channel to Primary Channel, call the Release Control API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | The `userId`, `roomId`, or `groupId` | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

