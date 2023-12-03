# CreateClickBasedAudienceGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The audience's name. This is case-insensitive, meaning AUDIENCE and audience are considered identical. Max character limit: 120  | [optional]
**request_id** | Option<**String**> | The request ID of a broadcast or narrowcast message sent in the past 60 days. Each Messaging API request has a request ID.  | [optional]
**click_url** | Option<**String**> | The URL clicked by the user. If empty, users who clicked any URL in the message are added to the list of recipients. Max character limit: 2,000  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


