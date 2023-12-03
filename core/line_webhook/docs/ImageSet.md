# ImageSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Image set ID. Only included when multiple images are sent simultaneously. | 
**index** | Option<**i32**> | An index starting from 1, indicating the image number in a set of images sent simultaneously. Only included when multiple images are sent simultaneously. However, it won't be included if the sender is using LINE 11.15 or earlier for Android. | [optional]
**total** | Option<**i32**> | The total number of images sent simultaneously. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


