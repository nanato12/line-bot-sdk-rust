# \ChannelAccessTokenApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gets_all_valid_channel_access_token_key_ids**](ChannelAccessTokenApi.md#gets_all_valid_channel_access_token_key_ids) | **Get** /oauth2/v2.1/tokens/kid | 
[**issue_channel_token**](ChannelAccessTokenApi.md#issue_channel_token) | **Post** /v2/oauth/accessToken | 
[**issue_channel_token_by_jwt**](ChannelAccessTokenApi.md#issue_channel_token_by_jwt) | **Post** /oauth2/v2.1/token | 
[**issue_stateless_channel_token**](ChannelAccessTokenApi.md#issue_stateless_channel_token) | **Post** /oauth2/v3/token | 
[**revoke_channel_token**](ChannelAccessTokenApi.md#revoke_channel_token) | **Post** /v2/oauth/revoke | 
[**revoke_channel_token_by_jwt**](ChannelAccessTokenApi.md#revoke_channel_token_by_jwt) | **Post** /oauth2/v2.1/revoke | 
[**verify_channel_token**](ChannelAccessTokenApi.md#verify_channel_token) | **Post** /v2/oauth/verify | 
[**verify_channel_token_by_jwt**](ChannelAccessTokenApi.md#verify_channel_token_by_jwt) | **Get** /oauth2/v2.1/verify | 



## gets_all_valid_channel_access_token_key_ids

> models::ChannelAccessTokenKeyIdsResponse gets_all_valid_channel_access_token_key_ids(client_assertion_type, client_assertion)


Gets all valid channel access token key IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_assertion_type** | **String** | `urn:ietf:params:oauth:client-assertion-type:jwt-bearer` | [required] |
**client_assertion** | **String** | A JSON Web Token (JWT) (opens new window)the client needs to create and sign with the private key. | [required] |

### Return type

[**models::ChannelAccessTokenKeyIdsResponse**](ChannelAccessTokenKeyIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_channel_token

> models::IssueShortLivedChannelAccessTokenResponse issue_channel_token(grant_type, client_id, client_secret)


Issue short-lived channel access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | `client_credentials` | [required] |
**client_id** | **String** | Channel ID. | [required] |
**client_secret** | **String** | Channel secret. | [required] |

### Return type

[**models::IssueShortLivedChannelAccessTokenResponse**](IssueShortLivedChannelAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_channel_token_by_jwt

> models::IssueChannelAccessTokenResponse issue_channel_token_by_jwt(grant_type, client_assertion_type, client_assertion)


Issues a channel access token that allows you to specify a desired expiration date. This method lets you use JWT assertion for authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | client_credentials | [required] |
**client_assertion_type** | **String** | urn:ietf:params:oauth:client-assertion-type:jwt-bearer | [required] |
**client_assertion** | **String** | A JSON Web Token the client needs to create and sign with the private key of the Assertion Signing Key. | [required] |

### Return type

[**models::IssueChannelAccessTokenResponse**](IssueChannelAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_stateless_channel_token

> models::IssueStatelessChannelAccessTokenResponse issue_stateless_channel_token(grant_type, client_assertion_type, client_assertion, client_id, client_secret)


Issues a new stateless channel access token, which doesn't have max active token limit unlike the other token types. The newly issued token is only valid for 15 minutes but can not be revoked until it naturally expires. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | Option<**String**> | `client_credentials` |  |
**client_assertion_type** | Option<**String**> | URL-encoded value of `urn:ietf:params:oauth:client-assertion-type:jwt-bearer` |  |
**client_assertion** | Option<**String**> | A JSON Web Token the client needs to create and sign with the private key of the Assertion Signing Key. |  |
**client_id** | Option<**String**> | Channel ID. |  |
**client_secret** | Option<**String**> | Channel secret. |  |

### Return type

[**models::IssueStatelessChannelAccessTokenResponse**](IssueStatelessChannelAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_channel_token

> revoke_channel_token(access_token)


Revoke short-lived or long-lived channel access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** | Channel access token | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_channel_token_by_jwt

> revoke_channel_token_by_jwt(client_id, client_secret, access_token)


Revoke channel access token v2.1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Channel ID | [required] |
**client_secret** | **String** | Channel Secret | [required] |
**access_token** | **String** | Channel access token | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_channel_token

> models::VerifyChannelAccessTokenResponse verify_channel_token(access_token)


Verify the validity of short-lived and long-lived channel access tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** | A short-lived or long-lived channel access token. | [required] |

### Return type

[**models::VerifyChannelAccessTokenResponse**](VerifyChannelAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_channel_token_by_jwt

> models::VerifyChannelAccessTokenResponse verify_channel_token_by_jwt(access_token)


You can verify whether a Channel access token with a user-specified expiration (Channel Access Token v2.1) is valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** | Channel access token with a user-specified expiration (Channel Access Token v2.1). | [required] |

### Return type

[**models::VerifyChannelAccessTokenResponse**](VerifyChannelAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

