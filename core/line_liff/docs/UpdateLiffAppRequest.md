# UpdateLiffAppRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**view** | Option<[**models::UpdateLiffView**](UpdateLiffView.md)> |  | [optional]
**description** | Option<**String**> | Name of the LIFF app.  The LIFF app name can't include \"LINE\" or similar strings, or inappropriate strings.  | [optional]
**features** | Option<[**models::LiffFeatures**](LiffFeatures.md)> |  | [optional]
**permanent_link_pattern** | Option<**String**> | How additional information in LIFF URLs is handled. Specify `concat`.  | [optional]
**scope** | Option<[**Vec<models::LiffScope>**](LiffScope.md)> |  | [optional]
**bot_prompt** | Option<[**models::LiffBotPrompt**](LiffBotPrompt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


