# CouponResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acquisition_condition** | Option<[**models::AcquisitionConditionResponse**](AcquisitionConditionResponse.md)> |  | [optional]
**barcode_image_url** | Option<**String**> | URL of the barcode image associated with the coupon. Used for in-store redemption. | [optional]
**coupon_code** | Option<**String**> | Unique code to be presented by the user to redeem the coupon. | [optional]
**description** | Option<**String**> | Detailed description of the coupon. Displayed to users. | [optional]
**end_timestamp** | Option<**i64**> | Coupon expiration time (epoch seconds). Coupon cannot be used after this time. | [optional]
**image_url** | Option<**String**> | URL of the main image representing the coupon. Displayed in the coupon list. | [optional]
**max_acquire_count** | Option<**i64**> | Maximum number of coupons that can be issued in total. | [optional]
**max_use_count_per_ticket** | Option<**i32**> | Maximum number of times a single coupon ticket can be used. | [optional]
**max_ticket_per_user** | Option<**i64**> | Maximum number of coupon tickets a single user can acquire. | [optional]
**start_timestamp** | Option<**i64**> | Coupon start time (epoch seconds). Coupon can be used from this time. | [optional]
**title** | Option<**String**> | Title of the coupon. Displayed in the coupon list. | [optional]
**usage_condition** | Option<**String**> | Conditions for using the coupon. Shown to users. | [optional]
**reward** | Option<[**models::CouponRewardResponse**](CouponRewardResponse.md)> |  | [optional]
**visibility** | Option<**Visibility**> | Visibility of the coupon. Determines who can see or acquire the coupon. (enum: UNLISTED, PUBLIC, PRIVATE) | [optional]
**timezone** | Option<**Timezone**> | Timezone for interpreting start and end timestamps. (enum: ETC_GMT_MINUS_12, ETC_GMT_MINUS_11, PACIFIC_HONOLULU, AMERICA_ANCHORAGE, AMERICA_LOS_ANGELES, AMERICA_PHOENIX, AMERICA_CHICAGO, AMERICA_NEW_YORK, AMERICA_CARACAS, AMERICA_SANTIAGO, AMERICA_ST_JOHNS, AMERICA_SAO_PAULO, ETC_GMT_MINUS_2, ATLANTIC_CAPE_VERDE, EUROPE_LONDON, EUROPE_PARIS, EUROPE_ISTANBUL, EUROPE_MOSCOW, ASIA_TEHRAN, ASIA_TBILISI, ASIA_KABUL, ASIA_TASHKENT, ASIA_COLOMBO, ASIA_KATHMANDU, ASIA_ALMATY, ASIA_RANGOON, ASIA_BANGKOK, ASIA_TAIPEI, ASIA_TOKYO, AUSTRALIA_DARWIN, AUSTRALIA_SYDNEY, ASIA_VLADIVOSTOK, ETC_GMT_PLUS_12, PACIFIC_TONGATAPU) | [optional]
**coupon_id** | Option<**String**> | Unique identifier of the coupon. | [optional]
**created_timestamp** | Option<**i64**> | Created timestamp (seconds) of the coupon. | [optional]
**status** | Option<**Status**> | Current status of the coupon. (enum: DRAFT, RUNNING, CLOSED) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


