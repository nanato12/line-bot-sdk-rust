# SubscribedMembershipUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**membership_no** | **i32** | The user's member number in the membership plan. | 
**joined_time** | **i32** | UNIX timestamp at which the user subscribed to the membership. | 
**next_billing_date** | **String** | Next payment date for membership plan. - Format: yyyy-MM-dd (e.g. 2024-02-08) - Timezone: UTC+9  | 
**total_subscription_months** | **i32** | The period of time in months that the user has been subscribed to a membership plan. If a user previously canceled and then re-subscribed to the same membership plan, only the period after the re-subscription will be counted. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


