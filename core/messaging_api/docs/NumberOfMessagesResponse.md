# NumberOfMessagesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | Aggregation process status. One of:  `ready`: The number of messages can be obtained. `unready`: We haven't finished calculating the number of sent messages for the specified in date. For example, this property is returned when the delivery date or a future date is specified. Calculation usually takes about a day. `unavailable_for_privacy`: The total number of messages on the specified day is less than 20. `out_of_service`: The specified date is earlier than the date on which we first started calculating sent messages (March 31, 2018).  | 
**success** | Option<**i64**> | The number of messages delivered using the phone number on the date specified in `date`. The response has this property only when the value of `status` is `ready`.   | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


