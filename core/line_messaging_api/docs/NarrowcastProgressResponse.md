# NarrowcastProgressResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phase** | **String** | The current status. One of:  `waiting`: Messages are not yet ready to be sent. They are currently being filtered or processed in some way. `sending`: Messages are currently being sent. `succeeded`: Messages were sent successfully. This may not mean the messages were successfully received. `failed`: Messages failed to be sent. Use the failedDescription property to find the cause of the failure.  | 
**success_count** | Option<**i64**> | The number of users who successfully received the message. | [optional]
**failure_count** | Option<**i64**> | The number of users who failed to send the message. | [optional]
**target_count** | Option<**i64**> | The number of intended recipients of the message. | [optional]
**failed_description** | Option<**String**> | The reason the message failed to be sent. This is only included with a `phase` property value of `failed`. | [optional]
**error_code** | Option<**i64**> | Error summary. This is only included with a phase property value of failed. One of:  `1`: An internal error occurred. `2`: An error occurred because there weren't enough recipients. `3`: A conflict error of requests occurs because a request that has already been accepted is retried. `4`: An audience of less than 50 recipients is included as a condition of sending.  | [optional]
**accepted_time** | **String** | Narrowcast message request accepted time in milliseconds.  Format: ISO 8601 (e.g. 2020-12-03T10:15:30.121Z) Timezone: UTC  | 
**completed_time** | Option<**String**> | Processing of narrowcast message request completion time in milliseconds. Returned when the phase property is succeeded or failed.  Format: ISO 8601 (e.g. 2020-12-03T10:15:30.121Z) Timezone: UTC  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


