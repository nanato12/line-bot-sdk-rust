# PushMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**to** | **String** | ID of the receiver. | 
**messages** | [**Vec<models::Message>**](Message.md) | List of Message objects. | 
**notification_disabled** | Option<**bool**> | `true`: The user doesn’t receive a push notification when a message is sent. `false`: The user receives a push notification when the message is sent (unless they have disabled push notifications in LINE and/or their device). The default value is false.  | [optional][default to false]
**custom_aggregation_units** | Option<**Vec<String>**> | List of aggregation unit name. Case-sensitive. This functions can only be used by corporate users who have submitted the required applications.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


