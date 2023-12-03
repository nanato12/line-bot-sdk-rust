# StickerMessageContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type | 
**id** | **String** | Message ID | 
**package_id** | **String** | Package ID | 
**sticker_id** | **String** | Sticker ID | 
**sticker_resource_type** | **String** |  | 
**keywords** | Option<**Vec<String>**> | Array of up to 15 keywords describing the sticker. If a sticker has 16 or more keywords, a random selection of 15 keywords will be returned. The keyword selection is random for each event, so different keywords may be returned for the same sticker.  | [optional]
**text** | Option<**String**> | Any text entered by the user. This property is only included for message stickers. Max character limit: 100  | [optional]
**quote_token** | **String** | Quote token to quote this message.  | 
**quoted_message_id** | Option<**String**> | Message ID of a quoted message. Only included when the received message quotes a past message.   | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


