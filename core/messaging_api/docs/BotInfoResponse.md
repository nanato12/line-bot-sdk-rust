# BotInfoResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | Bot's user ID | 
**basic_id** | **String** | Bot's basic ID | 
**premium_id** | Option<**String**> | Bot's premium ID. Not included in the response if the premium ID isn't set. | [optional]
**display_name** | **String** | Bot's display name | 
**picture_url** | Option<**String**> | Profile image URL. `https` image URL. Not included in the response if the bot doesn't have a profile image. | [optional]
**chat_mode** | **String** | Chat settings set in the LINE Official Account Manager. One of:  `chat`: Chat is set to \"On\". `bot`: Chat is set to \"Off\".  | 
**mark_as_read_mode** | **String** | Automatic read setting for messages. If the chat is set to \"Off\", auto is returned. If the chat is set to \"On\", manual is returned.  `auto`: Auto read setting is enabled. `manual`: Auto read setting is disabled.   | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


