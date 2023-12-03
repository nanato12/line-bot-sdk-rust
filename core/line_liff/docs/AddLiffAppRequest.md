# AddLiffAppRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**view** | [**crate::models::LiffView**](LiffView.md) |  | 
**description** | Option<**String**> | Name of the LIFF app.  The LIFF app name can't include \"LINE\" or similar strings, or inappropriate strings.  | [optional]
**features** | Option<[**crate::models::LiffFeatures**](LiffFeatures.md)> |  | [optional]
**permanent_link_pattern** | Option<**String**> | How additional information in LIFF URLs is handled. Specify `concat`.  | [optional]
**scope** | Option<[**Vec<crate::models::LiffScope>**](LiffScope.md)> |  | [optional]
**bot_prompt** | Option<[**crate::models::LiffBotPrompt**](LiffBotPrompt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


