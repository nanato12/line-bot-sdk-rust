# \ManageAudienceApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_audience_group**](ManageAudienceApi.md#activate_audience_group) | **Put** /v2/bot/audienceGroup/{audienceGroupId}/activate | 
[**add_audience_to_audience_group**](ManageAudienceApi.md#add_audience_to_audience_group) | **Put** /v2/bot/audienceGroup/upload | 
[**create_audience_group**](ManageAudienceApi.md#create_audience_group) | **Post** /v2/bot/audienceGroup/upload | 
[**create_click_based_audience_group**](ManageAudienceApi.md#create_click_based_audience_group) | **Post** /v2/bot/audienceGroup/click | 
[**create_imp_based_audience_group**](ManageAudienceApi.md#create_imp_based_audience_group) | **Post** /v2/bot/audienceGroup/imp | 
[**delete_audience_group**](ManageAudienceApi.md#delete_audience_group) | **Delete** /v2/bot/audienceGroup/{audienceGroupId} | 
[**get_audience_data**](ManageAudienceApi.md#get_audience_data) | **Get** /v2/bot/audienceGroup/{audienceGroupId} | 
[**get_audience_group_authority_level**](ManageAudienceApi.md#get_audience_group_authority_level) | **Get** /v2/bot/audienceGroup/authorityLevel | 
[**get_audience_groups**](ManageAudienceApi.md#get_audience_groups) | **Get** /v2/bot/audienceGroup/list | 
[**update_audience_group_authority_level**](ManageAudienceApi.md#update_audience_group_authority_level) | **Put** /v2/bot/audienceGroup/authorityLevel | 
[**update_audience_group_description**](ManageAudienceApi.md#update_audience_group_description) | **Put** /v2/bot/audienceGroup/{audienceGroupId}/updateDescription | 



## activate_audience_group

> activate_audience_group(audience_group_id)


Activate audience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audience_group_id** | **i64** | The audience ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_audience_to_audience_group

> add_audience_to_audience_group(add_audience_to_audience_group_request)


Add user IDs or Identifiers for Advertisers (IFAs) to an audience for uploading user IDs (by JSON)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_audience_to_audience_group_request** | [**AddAudienceToAudienceGroupRequest**](AddAudienceToAudienceGroupRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_audience_group

> crate::models::CreateAudienceGroupResponse create_audience_group(create_audience_group_request)


Create audience for uploading user IDs (by JSON)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_audience_group_request** | [**CreateAudienceGroupRequest**](CreateAudienceGroupRequest.md) |  | [required] |

### Return type

[**crate::models::CreateAudienceGroupResponse**](CreateAudienceGroupResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_click_based_audience_group

> crate::models::CreateClickBasedAudienceGroupResponse create_click_based_audience_group(create_click_based_audience_group_request)


Create audience for click-based retargeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_click_based_audience_group_request** | [**CreateClickBasedAudienceGroupRequest**](CreateClickBasedAudienceGroupRequest.md) |  | [required] |

### Return type

[**crate::models::CreateClickBasedAudienceGroupResponse**](CreateClickBasedAudienceGroupResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_imp_based_audience_group

> crate::models::CreateImpBasedAudienceGroupResponse create_imp_based_audience_group(create_imp_based_audience_group_request)


Create audience for impression-based retargeting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_imp_based_audience_group_request** | [**CreateImpBasedAudienceGroupRequest**](CreateImpBasedAudienceGroupRequest.md) |  | [required] |

### Return type

[**crate::models::CreateImpBasedAudienceGroupResponse**](CreateImpBasedAudienceGroupResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_audience_group

> delete_audience_group(audience_group_id)


Delete audience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audience_group_id** | **i64** | The audience ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audience_data

> crate::models::GetAudienceDataResponse get_audience_data(audience_group_id)


Gets audience data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audience_group_id** | **i64** | The audience ID. | [required] |

### Return type

[**crate::models::GetAudienceDataResponse**](GetAudienceDataResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audience_group_authority_level

> crate::models::GetAudienceGroupAuthorityLevelResponse get_audience_group_authority_level()


Get the authority level of the audience

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAudienceGroupAuthorityLevelResponse**](GetAudienceGroupAuthorityLevelResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audience_groups

> crate::models::GetAudienceGroupsResponse get_audience_groups(page, description, status, size, includes_external_public_groups, create_route)


Gets data for more than one audience.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i64** | The page to return when getting (paginated) results. Must be 1 or higher. | [required] |
**description** | Option<**String**> | The name of the audience(s) to return. You can search for partial matches. This is case-insensitive, meaning AUDIENCE and audience are considered identical. If omitted, the name of the audience(s) will not be used as a search criterion.  |  |
**status** | Option<[**AudienceGroupStatus**](.md)> | The status of the audience(s) to return. If omitted, the status of the audience(s) will not be used as a search criterion.  |  |
**size** | Option<**i64**> | The number of audiences per page. Default: 20 Max: 40  |  |
**includes_external_public_groups** | Option<**bool**> | true (default): Get public audiences created in all channels linked to the same bot. false: Get audiences created in the same channel.  |  |
**create_route** | Option<[**AudienceGroupCreateRoute**](.md)> | How the audience was created. If omitted, all audiences are included.  `OA_MANAGER`: Return only audiences created with LINE Official Account Manager (opens new window). `MESSAGING_API`: Return only audiences created with Messaging API.  |  |

### Return type

[**crate::models::GetAudienceGroupsResponse**](GetAudienceGroupsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_audience_group_authority_level

> update_audience_group_authority_level(update_audience_group_authority_level_request)


Change the authority level of the audience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_audience_group_authority_level_request** | [**UpdateAudienceGroupAuthorityLevelRequest**](UpdateAudienceGroupAuthorityLevelRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_audience_group_description

> update_audience_group_description(audience_group_id, update_audience_group_description_request)


Renames an existing audience.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audience_group_id** | **i64** | The audience ID. | [required] |
**update_audience_group_description_request** | [**UpdateAudienceGroupDescriptionRequest**](UpdateAudienceGroupDescriptionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

