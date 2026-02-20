# \LiffApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_liff_app**](LiffApi.md#add_liff_app) | **Post** /liff/v1/apps | Create LIFF app
[**delete_liff_app**](LiffApi.md#delete_liff_app) | **Delete** /liff/v1/apps/{liffId} | Delete LIFF app from a channel
[**get_all_liff_apps**](LiffApi.md#get_all_liff_apps) | **Get** /liff/v1/apps | Get all LIFF apps
[**update_liff_app**](LiffApi.md#update_liff_app) | **Put** /liff/v1/apps/{liffId} | Update LIFF app from a channel



## add_liff_app

> models::AddLiffAppResponse add_liff_app(add_liff_app_request)
Create LIFF app

Adding the LIFF app to a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_liff_app_request** | [**AddLiffAppRequest**](AddLiffAppRequest.md) |  | [required] |

### Return type

[**models::AddLiffAppResponse**](AddLiffAppResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_liff_app

> delete_liff_app(liff_id)
Delete LIFF app from a channel

Deletes a LIFF app from a channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**liff_id** | **String** | ID of the LIFF app to be updated | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_liff_apps

> models::GetAllLiffAppsResponse get_all_liff_apps()
Get all LIFF apps

Gets information on all the LIFF apps added to the channel.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAllLiffAppsResponse**](GetAllLiffAppsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_liff_app

> update_liff_app(liff_id, update_liff_app_request)
Update LIFF app from a channel

Update LIFF app settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**liff_id** | **String** | ID of the LIFF app to be updated | [required] |
**update_liff_app_request** | [**UpdateLiffAppRequest**](UpdateLiffAppRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

