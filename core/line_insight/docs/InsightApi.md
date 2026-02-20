# \InsightApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_friends_demographics**](InsightApi.md#get_friends_demographics) | **Get** /v2/bot/insight/demographic | 
[**get_message_event**](InsightApi.md#get_message_event) | **Get** /v2/bot/insight/message/event | Get user interaction statistics
[**get_number_of_followers**](InsightApi.md#get_number_of_followers) | **Get** /v2/bot/insight/followers | Get number of followers
[**get_number_of_message_deliveries**](InsightApi.md#get_number_of_message_deliveries) | **Get** /v2/bot/insight/message/delivery | Get number of message deliveries
[**get_statistics_per_unit**](InsightApi.md#get_statistics_per_unit) | **Get** /v2/bot/insight/message/event/aggregation | 



## get_friends_demographics

> models::GetFriendsDemographicsResponse get_friends_demographics()


Retrieves the demographic attributes for a LINE Official Account's friends.You can only retrieve information about friends for LINE Official Accounts created by users in Japan (JP), Thailand (TH), Taiwan (TW) and Indonesia (ID). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetFriendsDemographicsResponse**](GetFriendsDemographicsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_event

> models::GetMessageEventResponse get_message_event(request_id)
Get user interaction statistics

Returns statistics about how users interact with narrowcast messages or broadcast messages sent from your LINE Official Account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request ID of a narrowcast message or broadcast message. Each Messaging API request has a request ID.  | [required] |

### Return type

[**models::GetMessageEventResponse**](GetMessageEventResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_of_followers

> models::GetNumberOfFollowersResponse get_number_of_followers(date)
Get number of followers

Returns the number of users who have added the LINE Official Account on or before a specified date. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> | Date for which to retrieve the number of followers.  Format: yyyyMMdd (e.g. 20191231) Timezone: UTC+9  |  |

### Return type

[**models::GetNumberOfFollowersResponse**](GetNumberOfFollowersResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_of_message_deliveries

> models::GetNumberOfMessageDeliveriesResponse get_number_of_message_deliveries(date)
Get number of message deliveries

Returns the number of messages sent from LINE Official Account on a specified day. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | Date for which to retrieve number of sent messages. - Format: yyyyMMdd (e.g. 20191231) - Timezone: UTC+9  | [required] |

### Return type

[**models::GetNumberOfMessageDeliveriesResponse**](GetNumberOfMessageDeliveriesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_statistics_per_unit

> models::GetStatisticsPerUnitResponse get_statistics_per_unit(custom_aggregation_unit, from, to)


You can check the per-unit statistics of how users interact with push messages and multicast messages sent from your LINE Official Account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_aggregation_unit** | **String** | Name of aggregation unit specified when sending the message. Case-sensitive. For example, `Promotion_a` and `Promotion_A` are regarded as different unit names.  | [required] |
**from** | **String** | Start date of aggregation period.  Format: yyyyMMdd (e.g. 20210301) Time zone: UTC+9  | [required] |
**to** | **String** | End date of aggregation period. The end date can be specified for up to 30 days later. For example, if the start date is 20210301, the latest end date is 20210331.  Format: yyyyMMdd (e.g. 20210301) Time zone: UTC+9  | [required] |

### Return type

[**models::GetStatisticsPerUnitResponse**](GetStatisticsPerUnitResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

