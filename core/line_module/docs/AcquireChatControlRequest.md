# AcquireChatControlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expired** | Option<**bool**> | `True`: After the time limit (ttl) has passed, the initiative (Chat Control) will return to the Primary Channel. (Default) `False`: There's no time limit and the initiative (Chat Control) doesn't change over time.  | [optional]
**ttl** | Option<**i32**> | The time it takes for initiative (Chat Control) to return to the Primary Channel (the time that the module channel stays on the Active Channel). The value is specified in seconds. The maximum value is one year (3600 * 24 * 365). The default value is 3600 (1 hour).  * Ignored if the value of expired is false.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


