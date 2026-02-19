# GetJoinedMembershipUsersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_ids** | **Vec<String>** | A list of user IDs who joined the membership. Users who have not agreed to the bot user agreement, are not following the bot, or are not active will be excluded. If there are no users in the membership, an empty list will be returned.  | 
**next** | Option<**String**> | A continuation token to get next remaining membership user IDs. Returned only when there are remaining user IDs that weren't returned in the userIds property in the previous request. The continuation token expires in 24 hours (86,400 seconds).   | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


