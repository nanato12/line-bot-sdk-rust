# GetMessageEventResponseOverview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | Request ID. | [optional]
**timestamp** | Option<**i64**> | UNIX timestamp for message delivery time in seconds. | [optional]
**delivered** | Option<**i64**> | Number of messages delivered. This property shows values of less than 20. However, if all messages have not been sent, it will be null.  | [optional]
**unique_impression** | Option<**i64**> | Number of users who opened the message, meaning they displayed at least 1 bubble. | [optional]
**unique_click** | Option<**i64**> | Number of users who opened any URL in the message. | [optional]
**unique_media_played** | Option<**i64**> | Number of users who started playing any video or audio in the message. | [optional]
**unique_media_played100_percent** | Option<**i64**> | Number of users who played the entirety of any video or audio in the message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


