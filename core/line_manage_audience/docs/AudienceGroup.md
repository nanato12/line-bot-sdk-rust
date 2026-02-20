# AudienceGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience_group_id** | Option<**i64**> | The audience ID. | [optional]
**r#type** | Option<[**models::AudienceGroupType**](AudienceGroupType.md)> |  | [optional]
**description** | Option<**String**> | The audience's name. | [optional]
**status** | Option<[**models::AudienceGroupStatus**](AudienceGroupStatus.md)> |  | [optional]
**failed_type** | Option<[**models::AudienceGroupFailedType**](AudienceGroupFailedType.md)> |  | [optional]
**audience_count** | Option<**i64**> | The number of users included in the audience. | [optional]
**created** | Option<**i64**> | When the audience was created (in UNIX time). | [optional]
**request_id** | Option<**String**> | The request ID that was specified when the audience was created. This is only included when `audienceGroup.type` is CLICK or IMP.  | [optional]
**click_url** | Option<**String**> | The URL that was specified when the audience was created. This is only included when `audienceGroup.type` is CLICK and link URL is specified.  | [optional]
**is_ifa_audience** | Option<**bool**> | The value indicating the type of account to be sent, as specified when creating the audience for uploading user IDs.  | [optional]
**permission** | Option<[**models::AudienceGroupPermission**](AudienceGroupPermission.md)> |  | [optional]
**create_route** | Option<[**models::AudienceGroupCreateRoute**](AudienceGroupCreateRoute.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


