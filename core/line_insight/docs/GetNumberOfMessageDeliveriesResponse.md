# GetNumberOfMessageDeliveriesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Status of the counting process. | [optional]
**broadcast** | Option<**i64**> | Number of messages sent to all of this LINE Official Account's friends (broadcast messages). | [optional]
**targeting** | Option<**i64**> | Number of messages sent to some of this LINE Official Account's friends, based on specific attributes (targeted messages). | [optional]
**auto_response** | Option<**i64**> | Number of auto-response messages sent. | [optional]
**welcome_response** | Option<**i64**> | Number of greeting messages sent. | [optional]
**chat** | Option<**i64**> | Number of messages sent from LINE Official Account Manager [Chat screen](https://www.linebiz.com/jp/manual/OfficialAccountManager/chats/) (only available in Japanese). | [optional]
**api_broadcast** | Option<**i64**> | Number of broadcast messages sent with the `Send broadcast message` Messaging API operation. | [optional]
**api_push** | Option<**i64**> | Number of push messages sent with the `Send push message` Messaging API operation. | [optional]
**api_multicast** | Option<**i64**> | Number of multicast messages sent with the `Send multicast message` Messaging API operation. | [optional]
**api_narrowcast** | Option<**i64**> | Number of narrowcast messages sent with the `Send narrowcast message` Messaging API operation. | [optional]
**api_reply** | Option<**i64**> | Number of replies sent with the `Send reply message` Messaging API operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


