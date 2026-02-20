# MulticastRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | [**Vec<models::Message>**](Message.md) | Messages to send | 
**to** | **Vec<String>** | Array of user IDs. Use userId values which are returned in webhook event objects. Do not use LINE IDs found on LINE. | 
**notification_disabled** | Option<**bool**> | `true`: The user doesn’t receive a push notification when a message is sent. `false`: The user receives a push notification when the message is sent (unless they have disabled push notifications in LINE and/or their device). The default value is false.  | [optional][default to false]
**custom_aggregation_units** | Option<**Vec<String>**> | Name of aggregation unit. Case-sensitive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


