# TextMessageContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type | 
**id** | **String** | Message ID | 
**text** | **String** | Message text. | 
**emojis** | Option<[**Vec<crate::models::Emoji>**](Emoji.md)> | Array of one or more LINE emoji objects. Only included in the message event when the text property contains a LINE emoji. | [optional]
**mention** | Option<[**crate::models::Mention**](Mention.md)> |  | [optional]
**quote_token** | **String** | Quote token to quote this message.  | 
**quoted_message_id** | Option<**String**> | Message ID of a quoted message. Only included when the received message quotes a past message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


