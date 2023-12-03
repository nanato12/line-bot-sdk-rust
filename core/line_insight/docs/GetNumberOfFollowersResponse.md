# GetNumberOfFollowersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Calculation status. | [optional]
**followers** | Option<**i64**> | The number of times, as of the specified date, that a user added this LINE Official Account as a friend for the first time. The number doesn't decrease even if a user later blocks the account or when they delete their LINE account.  | [optional]
**targeted_reaches** | Option<**i64**> | The number of users, as of the specified date, that the LINE Official Account can reach through targeted messages based on gender, age, and/or region. This number only includes users who are active on LINE or LINE services and whose demographics have a high level of certainty.  | [optional]
**blocks** | Option<**i64**> | The number of users blocking the account as of the specified date. The number decreases when a user unblocks the account.    | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


