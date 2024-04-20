# Membership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**membership_id** | **i32** | Membership plan ID. | 
**title** | **String** | Membership plan name. | 
**description** | **String** | Membership plan description. | 
**benefits** | **Vec<String>** | List of membership plan perks. | 
**price** | **f64** | Monthly fee for membership plan. (e.g. 1500.00) | 
**currency** | **String** | The currency of membership.price. | 
**member_count** | **i32** | Number of members subscribed to the membership plan. | 
**member_limit** | Option<**i32**> | The upper limit of members who can subscribe. If no upper limit is set, it will be null. | 
**is_in_app_purchase** | **bool** | Payment method for users who subscribe to a membership plan. | 
**is_published** | **bool** | Membership plan status. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


