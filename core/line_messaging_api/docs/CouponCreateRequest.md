# CouponCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acquisition_condition** | [**models::AcquisitionConditionRequest**](AcquisitionConditionRequest.md) |  | 
**barcode_image_url** | Option<**String**> | URL of the barcode image associated with the coupon. Used for in-store redemption. | [optional]
**coupon_code** | Option<**String**> | Unique code to be presented by the user to redeem the coupon. Optional. | [optional]
**description** | Option<**String**> | Detailed description of the coupon. Displayed to users. | [optional]
**end_timestamp** | **i64** | Coupon expiration time (epoch seconds). Coupon cannot be used after this time. | 
**image_url** | Option<**String**> | URL of the main image representing the coupon. Displayed in the coupon list. | [optional]
**max_use_count_per_ticket** | **i32** | Maximum number of times a single coupon ticket can be used. Use -1 to indicate no limit. | 
**start_timestamp** | **i64** | Coupon start time (epoch seconds). Coupon can be used from this time. | 
**title** | **String** | Title of the coupon. Displayed in the coupon list. | 
**usage_condition** | Option<**String**> | Conditions for using the coupon. Shown to users. | [optional]
**reward** | Option<[**models::CouponRewardRequest**](CouponRewardRequest.md)> |  | [optional]
**visibility** | **Visibility** | Visibility of the coupon. Determines who can see or acquire the coupon. (enum: UNLISTED, PUBLIC) | 
**timezone** | **Timezone** | Timezone for interpreting start and end timestamps. (enum: ETC_GMT_MINUS_12, ETC_GMT_MINUS_11, PACIFIC_HONOLULU, AMERICA_ANCHORAGE, AMERICA_LOS_ANGELES, AMERICA_PHOENIX, AMERICA_CHICAGO, AMERICA_NEW_YORK, AMERICA_CARACAS, AMERICA_SANTIAGO, AMERICA_ST_JOHNS, AMERICA_SAO_PAULO, ETC_GMT_MINUS_2, ATLANTIC_CAPE_VERDE, EUROPE_LONDON, EUROPE_PARIS, EUROPE_ISTANBUL, EUROPE_MOSCOW, ASIA_TEHRAN, ASIA_TBILISI, ASIA_KABUL, ASIA_TASHKENT, ASIA_COLOMBO, ASIA_KATHMANDU, ASIA_ALMATY, ASIA_RANGOON, ASIA_BANGKOK, ASIA_TAIPEI, ASIA_TOKYO, AUSTRALIA_DARWIN, AUSTRALIA_SYDNEY, ASIA_VLADIVOSTOK, ETC_GMT_PLUS_12, PACIFIC_TONGATAPU) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


