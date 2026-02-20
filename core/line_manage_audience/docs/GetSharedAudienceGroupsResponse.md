# GetSharedAudienceGroupsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience_groups** | Option<[**Vec<models::AudienceGroup>**](AudienceGroup.md)> | An array of audience data. If there are no audiences that match the specified filter, an empty array will be returned. | [optional]
**has_next_page** | Option<**bool**> | true when this is not the last page. | [optional]
**total_count** | Option<**i64**> | The total number of audiences that can be returned with the specified filter. | [optional]
**read_write_audience_group_total_count** | Option<**i64**> | Of the audiences you can get with the specified filter, the number of audiences with the update permission set to READ_WRITE. | [optional]
**page** | Option<**i64**> | The current page number. | [optional]
**size** | Option<**i64**> | The maximum number of audiences on the current page. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


