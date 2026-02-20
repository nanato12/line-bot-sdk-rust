# NarrowcastRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | [**Vec<models::Message>**](Message.md) | List of Message objects. | 
**recipient** | Option<[**models::Recipient**](Recipient.md)> |  | [optional]
**filter** | Option<[**models::Filter**](Filter.md)> |  | [optional]
**limit** | Option<[**models::Limit**](Limit.md)> |  | [optional]
**notification_disabled** | Option<**bool**> | `true`: The user doesn’t receive a push notification when a message is sent. `false`: The user receives a push notification when the message is sent (unless they have disabled push notifications in LINE and/or their device). The default value is false.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


