# ErrorResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | Message containing information about the error. | 
**details** | Option<[**Vec<models::ErrorDetail>**](ErrorDetail.md)> | An array of error details. If the array is empty, this property will not be included in the response. | [optional]
**sent_messages** | Option<[**Vec<models::SentMessage>**](SentMessage.md)> | Array of sent messages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


