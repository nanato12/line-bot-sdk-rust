# CreateAudienceGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The audience's name. This is case-insensitive, meaning AUDIENCE and audience are considered identical. Max character limit: 120  | [optional]
**is_ifa_audience** | Option<**bool**> | To specify recipients by IFAs: set true. To specify recipients by user IDs: set false or omit isIfaAudience property.  | [optional]
**upload_description** | Option<**String**> | The description to register for the job (in jobs[].description).  | [optional]
**audiences** | Option<[**Vec<models::Audience>**](Audience.md)> | An array of user IDs or IFAs. Max number: 10,000  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


