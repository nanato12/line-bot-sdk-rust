# TextMessageV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of message | 
**quick_reply** | Option<[**crate::models::QuickReply**](QuickReply.md)> |  | [optional]
**sender** | Option<[**crate::models::Sender**](Sender.md)> |  | [optional]
**text** | **String** |  | 
**substitution** | Option<[**::std::collections::HashMap<String, crate::models::SubstitutionObject>**](SubstitutionObject.md)> | A mapping that specifies substitutions for parts enclosed in {} within the `text` field. | [optional]
**quote_token** | Option<**String**> | Quote token of the message you want to quote. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


