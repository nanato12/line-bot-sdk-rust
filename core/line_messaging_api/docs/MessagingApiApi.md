# \MessagingApiApi

All URIs are relative to *https://api.line.me*

Method | HTTP request | Description
------------- | ------------- | -------------
[**broadcast**](MessagingApiApi.md#broadcast) | **Post** /v2/bot/message/broadcast | 
[**cancel_default_rich_menu**](MessagingApiApi.md#cancel_default_rich_menu) | **Delete** /v2/bot/user/all/richmenu | 
[**create_rich_menu**](MessagingApiApi.md#create_rich_menu) | **Post** /v2/bot/richmenu | 
[**create_rich_menu_alias**](MessagingApiApi.md#create_rich_menu_alias) | **Post** /v2/bot/richmenu/alias | 
[**delete_rich_menu**](MessagingApiApi.md#delete_rich_menu) | **Delete** /v2/bot/richmenu/{richMenuId} | 
[**delete_rich_menu_alias**](MessagingApiApi.md#delete_rich_menu_alias) | **Delete** /v2/bot/richmenu/alias/{richMenuAliasId} | 
[**get_aggregation_unit_name_list**](MessagingApiApi.md#get_aggregation_unit_name_list) | **Get** /v2/bot/message/aggregation/list | 
[**get_aggregation_unit_usage**](MessagingApiApi.md#get_aggregation_unit_usage) | **Get** /v2/bot/message/aggregation/info | 
[**get_bot_info**](MessagingApiApi.md#get_bot_info) | **Get** /v2/bot/info | 
[**get_default_rich_menu_id**](MessagingApiApi.md#get_default_rich_menu_id) | **Get** /v2/bot/user/all/richmenu | 
[**get_followers**](MessagingApiApi.md#get_followers) | **Get** /v2/bot/followers/ids | 
[**get_group_member_count**](MessagingApiApi.md#get_group_member_count) | **Get** /v2/bot/group/{groupId}/members/count | 
[**get_group_member_profile**](MessagingApiApi.md#get_group_member_profile) | **Get** /v2/bot/group/{groupId}/member/{userId} | 
[**get_group_members_ids**](MessagingApiApi.md#get_group_members_ids) | **Get** /v2/bot/group/{groupId}/members/ids | 
[**get_group_summary**](MessagingApiApi.md#get_group_summary) | **Get** /v2/bot/group/{groupId}/summary | 
[**get_joined_membership_users**](MessagingApiApi.md#get_joined_membership_users) | **Get** /v2/bot/membership/{membershipId}/users/ids | 
[**get_membership_list**](MessagingApiApi.md#get_membership_list) | **Get** /v2/bot/membership/list | 
[**get_membership_subscription**](MessagingApiApi.md#get_membership_subscription) | **Get** /v2/bot/membership/subscription/{userId} | 
[**get_message_quota**](MessagingApiApi.md#get_message_quota) | **Get** /v2/bot/message/quota | 
[**get_message_quota_consumption**](MessagingApiApi.md#get_message_quota_consumption) | **Get** /v2/bot/message/quota/consumption | 
[**get_narrowcast_progress**](MessagingApiApi.md#get_narrowcast_progress) | **Get** /v2/bot/message/progress/narrowcast | 
[**get_number_of_sent_broadcast_messages**](MessagingApiApi.md#get_number_of_sent_broadcast_messages) | **Get** /v2/bot/message/delivery/broadcast | 
[**get_number_of_sent_multicast_messages**](MessagingApiApi.md#get_number_of_sent_multicast_messages) | **Get** /v2/bot/message/delivery/multicast | 
[**get_number_of_sent_push_messages**](MessagingApiApi.md#get_number_of_sent_push_messages) | **Get** /v2/bot/message/delivery/push | 
[**get_number_of_sent_reply_messages**](MessagingApiApi.md#get_number_of_sent_reply_messages) | **Get** /v2/bot/message/delivery/reply | 
[**get_pnp_message_statistics**](MessagingApiApi.md#get_pnp_message_statistics) | **Get** /v2/bot/message/delivery/pnp | 
[**get_profile**](MessagingApiApi.md#get_profile) | **Get** /v2/bot/profile/{userId} | 
[**get_rich_menu**](MessagingApiApi.md#get_rich_menu) | **Get** /v2/bot/richmenu/{richMenuId} | 
[**get_rich_menu_alias**](MessagingApiApi.md#get_rich_menu_alias) | **Get** /v2/bot/richmenu/alias/{richMenuAliasId} | 
[**get_rich_menu_alias_list**](MessagingApiApi.md#get_rich_menu_alias_list) | **Get** /v2/bot/richmenu/alias/list | 
[**get_rich_menu_batch_progress**](MessagingApiApi.md#get_rich_menu_batch_progress) | **Get** /v2/bot/richmenu/progress/batch | 
[**get_rich_menu_id_of_user**](MessagingApiApi.md#get_rich_menu_id_of_user) | **Get** /v2/bot/user/{userId}/richmenu | 
[**get_rich_menu_list**](MessagingApiApi.md#get_rich_menu_list) | **Get** /v2/bot/richmenu/list | 
[**get_room_member_count**](MessagingApiApi.md#get_room_member_count) | **Get** /v2/bot/room/{roomId}/members/count | 
[**get_room_member_profile**](MessagingApiApi.md#get_room_member_profile) | **Get** /v2/bot/room/{roomId}/member/{userId} | 
[**get_room_members_ids**](MessagingApiApi.md#get_room_members_ids) | **Get** /v2/bot/room/{roomId}/members/ids | 
[**get_webhook_endpoint**](MessagingApiApi.md#get_webhook_endpoint) | **Get** /v2/bot/channel/webhook/endpoint | 
[**issue_link_token**](MessagingApiApi.md#issue_link_token) | **Post** /v2/bot/user/{userId}/linkToken | 
[**leave_group**](MessagingApiApi.md#leave_group) | **Post** /v2/bot/group/{groupId}/leave | 
[**leave_room**](MessagingApiApi.md#leave_room) | **Post** /v2/bot/room/{roomId}/leave | 
[**link_rich_menu_id_to_user**](MessagingApiApi.md#link_rich_menu_id_to_user) | **Post** /v2/bot/user/{userId}/richmenu/{richMenuId} | 
[**link_rich_menu_id_to_users**](MessagingApiApi.md#link_rich_menu_id_to_users) | **Post** /v2/bot/richmenu/bulk/link | 
[**mark_messages_as_read**](MessagingApiApi.md#mark_messages_as_read) | **Post** /v2/bot/message/markAsRead | 
[**multicast**](MessagingApiApi.md#multicast) | **Post** /v2/bot/message/multicast | 
[**narrowcast**](MessagingApiApi.md#narrowcast) | **Post** /v2/bot/message/narrowcast | 
[**push_message**](MessagingApiApi.md#push_message) | **Post** /v2/bot/message/push | 
[**push_messages_by_phone**](MessagingApiApi.md#push_messages_by_phone) | **Post** /bot/pnp/push | 
[**reply_message**](MessagingApiApi.md#reply_message) | **Post** /v2/bot/message/reply | 
[**rich_menu_batch**](MessagingApiApi.md#rich_menu_batch) | **Post** /v2/bot/richmenu/batch | 
[**set_default_rich_menu**](MessagingApiApi.md#set_default_rich_menu) | **Post** /v2/bot/user/all/richmenu/{richMenuId} | 
[**set_webhook_endpoint**](MessagingApiApi.md#set_webhook_endpoint) | **Put** /v2/bot/channel/webhook/endpoint | 
[**show_loading_animation**](MessagingApiApi.md#show_loading_animation) | **Post** /v2/bot/chat/loading/start | 
[**test_webhook_endpoint**](MessagingApiApi.md#test_webhook_endpoint) | **Post** /v2/bot/channel/webhook/test | 
[**unlink_rich_menu_id_from_user**](MessagingApiApi.md#unlink_rich_menu_id_from_user) | **Delete** /v2/bot/user/{userId}/richmenu | 
[**unlink_rich_menu_id_from_users**](MessagingApiApi.md#unlink_rich_menu_id_from_users) | **Post** /v2/bot/richmenu/bulk/unlink | 
[**update_rich_menu_alias**](MessagingApiApi.md#update_rich_menu_alias) | **Post** /v2/bot/richmenu/alias/{richMenuAliasId} | 
[**validate_broadcast**](MessagingApiApi.md#validate_broadcast) | **Post** /v2/bot/message/validate/broadcast | 
[**validate_multicast**](MessagingApiApi.md#validate_multicast) | **Post** /v2/bot/message/validate/multicast | 
[**validate_narrowcast**](MessagingApiApi.md#validate_narrowcast) | **Post** /v2/bot/message/validate/narrowcast | 
[**validate_push**](MessagingApiApi.md#validate_push) | **Post** /v2/bot/message/validate/push | 
[**validate_reply**](MessagingApiApi.md#validate_reply) | **Post** /v2/bot/message/validate/reply | 
[**validate_rich_menu_batch_request**](MessagingApiApi.md#validate_rich_menu_batch_request) | **Post** /v2/bot/richmenu/validate/batch | 
[**validate_rich_menu_object**](MessagingApiApi.md#validate_rich_menu_object) | **Post** /v2/bot/richmenu/validate | 



## broadcast

> serde_json::Value broadcast(broadcast_request, x_line_retry_key)


Sends a message to multiple users at any time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_request** | [**BroadcastRequest**](BroadcastRequest.md) |  | [required] |
**x_line_retry_key** | Option<**uuid::Uuid**> | Retry key. Specifies the UUID in hexadecimal format (e.g., `123e4567-e89b-12d3-a456-426614174000`) generated by any method. The retry key isn't generated by LINE. Each developer must generate their own retry key.  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_default_rich_menu

> cancel_default_rich_menu()


Cancel default rich menu

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_rich_menu

> models::RichMenuIdResponse create_rich_menu(rich_menu_request)


Create rich menu

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_request** | [**RichMenuRequest**](RichMenuRequest.md) |  | [required] |

### Return type

[**models::RichMenuIdResponse**](RichMenuIdResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_rich_menu_alias

> create_rich_menu_alias(create_rich_menu_alias_request)


Create rich menu alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_rich_menu_alias_request** | [**CreateRichMenuAliasRequest**](CreateRichMenuAliasRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rich_menu

> delete_rich_menu(rich_menu_id)


Deletes a rich menu.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_id** | **String** | ID of a rich menu | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rich_menu_alias

> delete_rich_menu_alias(rich_menu_alias_id)


Delete rich menu alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_alias_id** | **String** | Rich menu alias ID that you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aggregation_unit_name_list

> models::GetAggregationUnitNameListResponse get_aggregation_unit_name_list(limit, start)


Get name list of units used this month

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**String**> | The maximum number of aggregation units you can get per request.  |  |
**start** | Option<**String**> | Value of the continuation token found in the next property of the JSON object returned in the response. If you can't get all the aggregation units in one request, include this parameter to get the remaining array.  |  |

### Return type

[**models::GetAggregationUnitNameListResponse**](GetAggregationUnitNameListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aggregation_unit_usage

> models::GetAggregationUnitUsageResponse get_aggregation_unit_usage()


Get number of units used this month

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAggregationUnitUsageResponse**](GetAggregationUnitUsageResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot_info

> models::BotInfoResponse get_bot_info()


Get bot info

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BotInfoResponse**](BotInfoResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_rich_menu_id

> models::RichMenuIdResponse get_default_rich_menu_id()


Gets the ID of the default rich menu set with the Messaging API.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RichMenuIdResponse**](RichMenuIdResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_followers

> models::GetFollowersResponse get_followers(start, limit)


Get a list of users who added your LINE Official Account as a friend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> | Value of the continuation token found in the next property of the JSON object returned in the response. Include this parameter to get the next array of user IDs.  |  |
**limit** | Option<**i32**> | The maximum number of user IDs to retrieve in a single request. |  |[default to 300]

### Return type

[**models::GetFollowersResponse**](GetFollowersResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_member_count

> models::GroupMemberCountResponse get_group_member_count(group_id)


Get number of users in a group chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

[**models::GroupMemberCountResponse**](GroupMemberCountResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_member_profile

> models::GroupUserProfileResponse get_group_member_profile(group_id, user_id)


Get group chat member profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**user_id** | **String** | User ID | [required] |

### Return type

[**models::GroupUserProfileResponse**](GroupUserProfileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_members_ids

> models::MembersIdsResponse get_group_members_ids(group_id, start)


Get group chat member user IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**start** | Option<**String**> | Value of the continuation token found in the `next` property of the JSON object returned in the response. Include this parameter to get the next array of user IDs for the members of the group.  |  |

### Return type

[**models::MembersIdsResponse**](MembersIdsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_summary

> models::GroupSummaryResponse get_group_summary(group_id)


Get group chat summary

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

[**models::GroupSummaryResponse**](GroupSummaryResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_joined_membership_users

> models::GetJoinedMembershipUsersResponse get_joined_membership_users(membership_id, start, limit)


Get a list of user IDs who joined the membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i32** | Membership plan ID. | [required] |
**start** | Option<**String**> | A continuation token to get next remaining membership user IDs. Returned only when there are remaining user IDs that weren't returned in the userIds property in the previous request. The continuation token expires in 24 hours (86,400 seconds).  |  |
**limit** | Option<**i32**> | The max number of items to return for this API call. The value is set to 300 by default, but the max acceptable value is 1000.  |  |[default to 300]

### Return type

[**models::GetJoinedMembershipUsersResponse**](GetJoinedMembershipUsersResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_membership_list

> models::MembershipListResponse get_membership_list()


Get a list of memberships.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MembershipListResponse**](MembershipListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_membership_subscription

> models::GetMembershipSubscriptionResponse get_membership_subscription(user_id)


Get a user's membership subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**models::GetMembershipSubscriptionResponse**](GetMembershipSubscriptionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_quota

> models::MessageQuotaResponse get_message_quota()


Gets the target limit for sending messages in the current month. The total number of the free messages and the additional messages is returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MessageQuotaResponse**](MessageQuotaResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_quota_consumption

> models::QuotaConsumptionResponse get_message_quota_consumption()


Gets the number of messages sent in the current month.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::QuotaConsumptionResponse**](QuotaConsumptionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_narrowcast_progress

> models::NarrowcastProgressResponse get_narrowcast_progress(request_id)


Gets the status of a narrowcast message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | The narrowcast message's request ID. Each Messaging API request has a request ID. | [required] |

### Return type

[**models::NarrowcastProgressResponse**](NarrowcastProgressResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_of_sent_broadcast_messages

> models::NumberOfMessagesResponse get_number_of_sent_broadcast_messages(date)


Get number of sent broadcast messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | Date the messages were sent  Format: yyyyMMdd (e.g. 20191231) Timezone: UTC+9  | [required] |

### Return type

[**models::NumberOfMessagesResponse**](NumberOfMessagesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_of_sent_multicast_messages

> models::NumberOfMessagesResponse get_number_of_sent_multicast_messages(date)


Get number of sent multicast messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | Date the messages were sent  Format: `yyyyMMdd` (e.g. `20191231`) Timezone: UTC+9  | [required] |

### Return type

[**models::NumberOfMessagesResponse**](NumberOfMessagesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_of_sent_push_messages

> models::NumberOfMessagesResponse get_number_of_sent_push_messages(date)


Get number of sent push messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | Date the messages were sent  Format: `yyyyMMdd` (e.g. `20191231`) Timezone: UTC+9  | [required] |

### Return type

[**models::NumberOfMessagesResponse**](NumberOfMessagesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_of_sent_reply_messages

> models::NumberOfMessagesResponse get_number_of_sent_reply_messages(date)


Get number of sent reply messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | Date the messages were sent  Format: `yyyyMMdd` (e.g. `20191231`) Timezone: UTC+9  | [required] |

### Return type

[**models::NumberOfMessagesResponse**](NumberOfMessagesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pnp_message_statistics

> models::NumberOfMessagesResponse get_pnp_message_statistics(date)


Get number of sent LINE notification messages　

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | Date the message was sent  Format: `yyyyMMdd` (Example:`20211231`) Time zone: UTC+9  | [required] |

### Return type

[**models::NumberOfMessagesResponse**](NumberOfMessagesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile

> models::UserProfileResponse get_profile(user_id)


Get profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**models::UserProfileResponse**](UserProfileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu

> models::RichMenuResponse get_rich_menu(rich_menu_id)


Gets a rich menu via a rich menu ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_id** | **String** | ID of a rich menu | [required] |

### Return type

[**models::RichMenuResponse**](RichMenuResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu_alias

> models::RichMenuAliasResponse get_rich_menu_alias(rich_menu_alias_id)


Get rich menu alias information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_alias_id** | **String** | The rich menu alias ID whose information you want to obtain. | [required] |

### Return type

[**models::RichMenuAliasResponse**](RichMenuAliasResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu_alias_list

> models::RichMenuAliasListResponse get_rich_menu_alias_list()


Get list of rich menu alias

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RichMenuAliasListResponse**](RichMenuAliasListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu_batch_progress

> models::RichMenuBatchProgressResponse get_rich_menu_batch_progress(request_id)


Get the status of Replace or unlink a linked rich menus in batches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | A request ID used to batch control the rich menu linked to the user. Each Messaging API request has a request ID. | [required] |

### Return type

[**models::RichMenuBatchProgressResponse**](RichMenuBatchProgressResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu_id_of_user

> models::RichMenuIdResponse get_rich_menu_id_of_user(user_id)


Get rich menu ID of user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. Found in the `source` object of webhook event objects. Do not use the LINE ID used in LINE. | [required] |

### Return type

[**models::RichMenuIdResponse**](RichMenuIdResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rich_menu_list

> models::RichMenuListResponse get_rich_menu_list()


Get rich menu list

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RichMenuListResponse**](RichMenuListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_member_count

> models::RoomMemberCountResponse get_room_member_count(room_id)


Get number of users in a multi-person chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | Room ID | [required] |

### Return type

[**models::RoomMemberCountResponse**](RoomMemberCountResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_member_profile

> models::RoomUserProfileResponse get_room_member_profile(room_id, user_id)


Get multi-person chat member profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | Room ID | [required] |
**user_id** | **String** | User ID | [required] |

### Return type

[**models::RoomUserProfileResponse**](RoomUserProfileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_members_ids

> models::MembersIdsResponse get_room_members_ids(room_id, start)


Get multi-person chat member user IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | Room ID | [required] |
**start** | Option<**String**> | Value of the continuation token found in the `next` property of the JSON object returned in the response. Include this parameter to get the next array of user IDs for the members of the group.  |  |

### Return type

[**models::MembersIdsResponse**](MembersIdsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_endpoint

> models::GetWebhookEndpointResponse get_webhook_endpoint()


Get webhook endpoint information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWebhookEndpointResponse**](GetWebhookEndpointResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_link_token

> models::IssueLinkTokenResponse issue_link_token(user_id)


Issue link token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID for the LINE account to be linked. Found in the `source` object of account link event objects. Do not use the LINE ID used in LINE.  | [required] |

### Return type

[**models::IssueLinkTokenResponse**](IssueLinkTokenResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_group

> leave_group(group_id)


Leave group chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_room

> leave_room(room_id)


Leave multi-person chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | Room ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_rich_menu_id_to_user

> link_rich_menu_id_to_user(user_id, rich_menu_id)


Link rich menu to user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. Found in the `source` object of webhook event objects. Do not use the LINE ID used in LINE. | [required] |
**rich_menu_id** | **String** | ID of a rich menu | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_rich_menu_id_to_users

> link_rich_menu_id_to_users(rich_menu_bulk_link_request)


Link rich menu to multiple users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_bulk_link_request** | [**RichMenuBulkLinkRequest**](RichMenuBulkLinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_messages_as_read

> mark_messages_as_read(mark_messages_as_read_request)


Mark messages from users as read

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mark_messages_as_read_request** | [**MarkMessagesAsReadRequest**](MarkMessagesAsReadRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## multicast

> serde_json::Value multicast(multicast_request, x_line_retry_key)


An API that efficiently sends the same message to multiple user IDs. You can't send messages to group chats or multi-person chats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**multicast_request** | [**MulticastRequest**](MulticastRequest.md) |  | [required] |
**x_line_retry_key** | Option<**uuid::Uuid**> | Retry key. Specifies the UUID in hexadecimal format (e.g., `123e4567-e89b-12d3-a456-426614174000`) generated by any method. The retry key isn't generated by LINE. Each developer must generate their own retry key.  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## narrowcast

> serde_json::Value narrowcast(narrowcast_request, x_line_retry_key)


Send narrowcast message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**narrowcast_request** | [**NarrowcastRequest**](NarrowcastRequest.md) |  | [required] |
**x_line_retry_key** | Option<**uuid::Uuid**> | Retry key. Specifies the UUID in hexadecimal format (e.g., `123e4567-e89b-12d3-a456-426614174000`) generated by any method. The retry key isn't generated by LINE. Each developer must generate their own retry key.  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_message

> models::PushMessageResponse push_message(push_message_request, x_line_retry_key)


Sends a message to a user, group chat, or multi-person chat at any time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_message_request** | [**PushMessageRequest**](PushMessageRequest.md) |  | [required] |
**x_line_retry_key** | Option<**uuid::Uuid**> | Retry key. Specifies the UUID in hexadecimal format (e.g., `123e4567-e89b-12d3-a456-426614174000`) generated by any method. The retry key isn't generated by LINE. Each developer must generate their own retry key.  |  |

### Return type

[**models::PushMessageResponse**](PushMessageResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_messages_by_phone

> push_messages_by_phone(pnp_messages_request, x_line_delivery_tag)


Send LINE notification message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pnp_messages_request** | [**PnpMessagesRequest**](PnpMessagesRequest.md) |  | [required] |
**x_line_delivery_tag** | Option<**String**> | String returned in the delivery.data property of the delivery completion event via Webhook. |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reply_message

> models::ReplyMessageResponse reply_message(reply_message_request)


Send reply message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reply_message_request** | [**ReplyMessageRequest**](ReplyMessageRequest.md) |  | [required] |

### Return type

[**models::ReplyMessageResponse**](ReplyMessageResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rich_menu_batch

> rich_menu_batch(rich_menu_batch_request)


You can use this endpoint to batch control the rich menu linked to the users using the endpoint such as Link rich menu to user. The following operations are available:  1. Replace a rich menu with another rich menu for all users linked to a specific rich menu 2. Unlink a rich menu for all users linked to a specific rich menu 3. Unlink a rich menu for all users linked the rich menu 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_batch_request** | [**RichMenuBatchRequest**](RichMenuBatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_rich_menu

> set_default_rich_menu(rich_menu_id)


Set default rich menu

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_id** | **String** | ID of a rich menu | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_webhook_endpoint

> set_webhook_endpoint(set_webhook_endpoint_request)


Set webhook endpoint URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_webhook_endpoint_request** | [**SetWebhookEndpointRequest**](SetWebhookEndpointRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_loading_animation

> serde_json::Value show_loading_animation(show_loading_animation_request)


Display a loading animation in one-on-one chats between users and LINE Official Accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_loading_animation_request** | [**ShowLoadingAnimationRequest**](ShowLoadingAnimationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_webhook_endpoint

> models::TestWebhookEndpointResponse test_webhook_endpoint(test_webhook_endpoint_request)


Test webhook endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_webhook_endpoint_request** | Option<[**TestWebhookEndpointRequest**](TestWebhookEndpointRequest.md)> |  |  |

### Return type

[**models::TestWebhookEndpointResponse**](TestWebhookEndpointResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_rich_menu_id_from_user

> unlink_rich_menu_id_from_user(user_id)


Unlink rich menu from user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. Found in the `source` object of webhook event objects. Do not use the LINE ID used in LINE. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_rich_menu_id_from_users

> unlink_rich_menu_id_from_users(rich_menu_bulk_unlink_request)


Unlink rich menus from multiple users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_bulk_unlink_request** | [**RichMenuBulkUnlinkRequest**](RichMenuBulkUnlinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rich_menu_alias

> update_rich_menu_alias(rich_menu_alias_id, update_rich_menu_alias_request)


Update rich menu alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_alias_id** | **String** | The rich menu alias ID you want to update. | [required] |
**update_rich_menu_alias_request** | [**UpdateRichMenuAliasRequest**](UpdateRichMenuAliasRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_broadcast

> validate_broadcast(validate_message_request)


Validate message objects of a broadcast message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_message_request** | [**ValidateMessageRequest**](ValidateMessageRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_multicast

> validate_multicast(validate_message_request)


Validate message objects of a multicast message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_message_request** | [**ValidateMessageRequest**](ValidateMessageRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_narrowcast

> validate_narrowcast(validate_message_request)


Validate message objects of a narrowcast message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_message_request** | [**ValidateMessageRequest**](ValidateMessageRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_push

> validate_push(validate_message_request)


Validate message objects of a push message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_message_request** | [**ValidateMessageRequest**](ValidateMessageRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_reply

> validate_reply(validate_message_request)


Validate message objects of a reply message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_message_request** | [**ValidateMessageRequest**](ValidateMessageRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_rich_menu_batch_request

> validate_rich_menu_batch_request(rich_menu_batch_request)


Validate a request body of the Replace or unlink the linked rich menus in batches endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_batch_request** | [**RichMenuBatchRequest**](RichMenuBatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_rich_menu_object

> validate_rich_menu_object(rich_menu_request)


Validate rich menu object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_menu_request** | [**RichMenuRequest**](RichMenuRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

