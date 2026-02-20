# GetSharedAudienceDataResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience_group** | Option<[**models::AudienceGroup**](AudienceGroup.md)> |  | [optional]
**jobs** | Option<[**Vec<models::AudienceGroupJob>**](AudienceGroupJob.md)> | An array of jobs. This array is used to keep track of each attempt to add new user IDs or IFAs to an audience for uploading user IDs. Empty array is returned for any other type of audience. Max: 50  | [optional]
**owner** | Option<[**models::DetailedOwner**](DetailedOwner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


