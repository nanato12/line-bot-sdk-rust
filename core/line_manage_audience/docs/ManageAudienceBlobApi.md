# \ManageAudienceBlobApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_ids_to_audience**](ManageAudienceBlobApi.md#add_user_ids_to_audience) | **Put** /v2/bot/audienceGroup/upload/byFile | 
[**create_audience_for_uploading_user_ids**](ManageAudienceBlobApi.md#create_audience_for_uploading_user_ids) | **Post** /v2/bot/audienceGroup/upload/byFile | 



## add_user_ids_to_audience

> add_user_ids_to_audience(file, audience_group_id, upload_description)


Add user IDs or Identifiers for Advertisers (IFAs) to an audience for uploading user IDs (by file).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | A text file with one user ID or IFA entered per line. Specify text/plain as Content-Type. Max file number: 1 Max number: 1,500,000  | [required] |
**audience_group_id** | Option<**i64**> | The audience ID. |  |
**upload_description** | Option<**String**> | The description to register with the job |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_audience_for_uploading_user_ids

> models::CreateAudienceGroupResponse create_audience_for_uploading_user_ids(file, description, is_ifa_audience, upload_description)


Create audience for uploading user IDs (by file).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | A text file with one user ID or IFA entered per line. Specify text/plain as Content-Type. Max file number: 1 Max number: 1,500,000  | [required] |
**description** | Option<**String**> | The audience's name. This is case-insensitive, meaning AUDIENCE and audience are considered identical. Max character limit: 120  |  |
**is_ifa_audience** | Option<**bool**> | To specify recipients by IFAs: set `true`. To specify recipients by user IDs: set `false` or omit isIfaAudience property.  |  |
**upload_description** | Option<**String**> | The description to register for the job (in `jobs[].description`).  |  |

### Return type

[**models::CreateAudienceGroupResponse**](CreateAudienceGroupResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

