# CallbackRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | **String** | User ID of a bot that should receive webhook events. The user ID value is a string that matches the regular expression, `U[0-9a-f]{32}`.  | 
**events** | [**Vec<models::Event>**](Event.md) | Array of webhook event objects. The LINE Platform may send an empty array that doesn't include a webhook event object to confirm communication.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


