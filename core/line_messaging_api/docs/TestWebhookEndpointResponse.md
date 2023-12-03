# TestWebhookEndpointResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | Option<**bool**> | Result of the communication from the LINE platform to the webhook URL. | [optional]
**timestamp** | **String** | Time of the event in milliseconds. Even in the case of a redelivered webhook, it represents the time the event occurred, not the time it was redelivered.  | 
**status_code** | **i32** | The HTTP status code. If the webhook response isn't received, the status code is set to zero or a negative number. | 
**reason** | **String** | Reason for the response. | 
**detail** | **String** | Details of the response. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


