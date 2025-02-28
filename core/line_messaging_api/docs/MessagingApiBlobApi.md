# \MessagingApiBlobApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_message_content**](MessagingApiBlobApi.md#get_message_content) | **Get** /v2/bot/message/{messageId}/content | 
[**get_message_content_preview**](MessagingApiBlobApi.md#get_message_content_preview) | **Get** /v2/bot/message/{messageId}/content/preview | 
[**get_message_content_transcoding_by_message_id**](MessagingApiBlobApi.md#get_message_content_transcoding_by_message_id) | **Get** /v2/bot/message/{messageId}/content/transcoding | 
[**get_rich_menu_image**](MessagingApiBlobApi.md#get_rich_menu_image) | **Get** /v2/bot/richmenu/{richMenuId}/content | 
[**set_rich_menu_image**](MessagingApiBlobApi.md#set_rich_menu_image) | **Post** /v2/bot/richmenu/{richMenuId}/content | 



## get_message_content

> std::path::PathBuf get_message_content(message_id)


Download image, video, and audio data sent from users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID of video or audio | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_content_preview

> std::path::PathBuf get_message_content_preview(message_id)


Get a preview image of the image or video

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID of image or video | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_content_transcoding_by_message_id

> models::GetMessageContentTranscodingResponse get_message_content_transcoding_by_message_id(message_id)


Verify the preparation status of a video or audio for getting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID of video or audio | [required] |

### Return type

[**models::GetMessageContentTranscodingResponse**](GetMessageContentTranscodingResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu_image

> std::path::PathBuf get_rich_menu_image(rich_menu_id)


Download rich menu image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_id** | **String** | ID of the rich menu with the image to be downloaded | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_rich_menu_image

> set_rich_menu_image(rich_menu_id, body)


Upload rich menu image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_id** | **String** | The ID of the rich menu to attach the image to | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

