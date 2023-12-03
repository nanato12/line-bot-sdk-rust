# GetStatisticsPerUnitResponseClick

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seq** | **i64** | The URL's serial number. | 
**url** | **String** | URL. | 
**click** | Option<**i64**> | Number of times the URL in the bubble was opened. | [optional]
**unique_click** | Option<**i64**> | Number of users that opened the URL in the bubble. | [optional]
**unique_click_of_request** | Option<**i64**> | Number of users who opened this url through any link in the message. If another message bubble contains the same URL and a user opens both links, it's counted only once.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


