# GetFollowersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_ids** | **Vec<String>** | An array of strings indicating user IDs of users that have added the LINE Official Account as a friend. Only users of LINE for iOS and LINE for Android are included in `userIds`.  | 
**next** | Option<**String**> | A continuation token to get the next array of user IDs. Returned only when there are remaining user IDs that weren't returned in `userIds` in the original request. The number of user IDs in the `userIds` element doesn't have to reach the maximum number specified by `limit` for the `next` property to be included in the response.   | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


