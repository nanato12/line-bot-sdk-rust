# FlexImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**url** | **String** | Image URL (Max character limit: 2000) Protocol: HTTPS (TLS 1.2 or later) Image format: JPEG or PNG Maximum image size: 1024×1024 pixels Maximum file size: 10 MB (300 KB when the animated property is true)  | 
**flex** | Option<**i32**> | The ratio of the width or height of this component within the parent box. | [optional]
**margin** | Option<**String**> | The minimum amount of space to include before this component in its parent container.  | [optional]
**position** | Option<**String**> | Reference for offsetTop, offsetBottom, offsetStart, and offsetEnd. Specify one of the following values:  `relative`: Use the previous box as reference. `absolute`: Use the top left of parent element as reference. The default value is relative.  | [optional]
**offset_top** | Option<**String**> | Offset. | [optional]
**offset_bottom** | Option<**String**> | Offset. | [optional]
**offset_start** | Option<**String**> | Offset. | [optional]
**offset_end** | Option<**String**> | Offset. | [optional]
**align** | Option<**String**> | Alignment style in horizontal direction.  | [optional]
**gravity** | Option<**String**> | Alignment style in vertical direction. | [optional]
**size** | Option<**String**> | The maximum image width. This is md by default.  | [optional][default to md]
**aspect_ratio** | Option<**String**> | Aspect ratio of the image. `{width}:{height}` format. Specify the value of `{width}` and `{height}` in the range from `1` to `100000`. However, you cannot set `{height}` to a value that is more than three times the value of `{width}`. The default value is `1:1`.  | [optional]
**aspect_mode** | Option<**String**> | The display style of the image if the aspect ratio of the image and that specified by the aspectRatio property do not match.  | [optional]
**background_color** | Option<**String**> | Background color of the image. Use a hexadecimal color code. | [optional]
**action** | Option<[**models::Action**](Action.md)> |  | [optional]
**animated** | Option<**bool**> | When this is `true`, an animated image (APNG) plays. You can specify a value of true up to 10 images in a single message. You can't send messages that exceed this limit. This is `false` by default. Animated images larger than 300 KB aren't played back.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


