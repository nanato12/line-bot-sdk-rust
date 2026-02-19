# UserMentionee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Mentioned target. | 
**index** | **i32** | Index position of the user mention for a character in text, with the first character being at position 0. | 
**length** | **i32** | The length of the text of the mentioned user. For a mention @example, 8 is the length. | 
**user_id** | Option<**String**> | User ID of the mentioned user. Only included if mention.mentions[].type is user and the user consents to the LINE Official Account obtaining their user profile information. | [optional]
**is_self** | Option<**bool**> | Whether the mentioned user is the bot that receives the webhook. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


