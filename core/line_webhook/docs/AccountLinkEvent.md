# AccountLinkEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the event | 
**source** | Option<[**crate::models::Source**](Source.md)> |  | [optional]
**timestamp** | **i64** | Time of the event in milliseconds. | 
**mode** | [**crate::models::EventMode**](EventMode.md) |  | 
**webhook_event_id** | **String** | Webhook Event ID. An ID that uniquely identifies a webhook event. This is a string in ULID format. | 
**delivery_context** | [**crate::models::DeliveryContext**](DeliveryContext.md) |  | 
**reply_token** | Option<**String**> | Reply token used to send reply message to this event. This property won't be included if linking the account has failed. | [optional]
**link** | [**crate::models::LinkContent**](LinkContent.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


