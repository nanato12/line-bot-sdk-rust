# ReplyMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reply_token** | **String** | replyToken received via webhook. | 
**messages** | [**Vec<models::Message>**](Message.md) | List of messages. | 
**notification_disabled** | Option<**bool**> | `true`: The user doesn’t receive a push notification when a message is sent. `false`: The user receives a push notification when the message is sent (unless they have disabled push notifications in LINE and/or their device). The default value is false.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


