# CreateAudienceGroupResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience_group_id** | Option<**i64**> | The audience ID. | [optional]
**create_route** | Option<**String**> | How the audience was created.  `MESSAGING_API`: An audience created with Messaging API.  | [optional]
**r#type** | Option<[**crate::models::AudienceGroupType**](AudienceGroupType.md)> |  | [optional]
**description** | Option<**String**> | The audience's name. | [optional]
**created** | Option<**i64**> | When the audience was created (in UNIX time). | [optional]
**permission** | Option<**String**> | Audience's update permission. Audiences linked to the same channel will be READ_WRITE.  `READ`: Can use only. `READ_WRITE`: Can use and update.  | [optional]
**expire_timestamp** | Option<**f32**> | Time of audience expiration. Only returned for specific audiences.  | [optional]
**is_ifa_audience** | Option<**bool**> | The value indicating the type of account to be sent, as specified when creating the audience for uploading user IDs. One of:  `true`: Accounts are specified with IFAs. `false` (default): Accounts are specified with user IDs.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


