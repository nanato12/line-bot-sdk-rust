# ScenarioResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scenario_id** | Option<**String**> | Scenario ID executed | [optional]
**revision** | Option<**i32**> | Revision number of the scenario set containing the executed scenario | [optional]
**start_time** | **i64** | Timestamp for when execution of scenario action started (milliseconds, LINE app time) | 
**end_time** | **i64** | Timestamp for when execution of scenario was completed (milliseconds, LINE app time) | 
**result_code** | **String** | Scenario execution completion status | 
**action_results** | Option<[**Vec<crate::models::ActionResult>**](ActionResult.md)> | Execution result of individual operations specified in action. Only included when things.result.resultCode is success. | [optional]
**ble_notification_payload** | Option<**String**> | Data contained in notification. | [optional]
**error_reason** | Option<**String**> | Error reason. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


