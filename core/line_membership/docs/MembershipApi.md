# \MembershipApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_membership_list**](MembershipApi.md#get_membership_list) | **Get** /membership/v1/list | 
[**get_membership_subscription**](MembershipApi.md#get_membership_subscription) | **Get** /membership/v1/subscription/{userId} | 



## get_membership_list

> crate::models::MembershipListResponse get_membership_list()


Get a list of memberships.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MembershipListResponse**](MembershipListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_membership_subscription

> crate::models::GetMembershipSubscriptionResponse get_membership_subscription(user_id)


Get a user's membership subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::GetMembershipSubscriptionResponse**](GetMembershipSubscriptionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

