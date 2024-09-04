# SummaryClub

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The club&#39;s unique identifier. | [optional] [default to null]
**resource_state** | **i32** | Resource state, indicates level of detail. Possible values: 1 -&gt; \&quot;meta\&quot;, 2 -&gt; \&quot;summary\&quot;, 3 -&gt; \&quot;detail\&quot; | [optional] [default to null]
**name** | **String** | The club&#39;s name. | [optional] [default to null]
**profile_medium** | **String** | URL to a 60x60 pixel profile picture. | [optional] [default to null]
**cover_photo** | **String** | URL to a ~1185x580 pixel cover photo. | [optional] [default to null]
**cover_photo_small** | **String** | URL to a ~360x176  pixel cover photo. | [optional] [default to null]
**sport_type** | **String** | Deprecated. Prefer to use activity_types. | [optional] [default to null]
**activity_types** | [**Vec<::models::ActivityType>**](ActivityType.md) | The activity types that count for a club. This takes precedence over sport_type. | [optional] [default to null]
**city** | **String** | The club&#39;s city. | [optional] [default to null]
**state** | **String** | The club&#39;s state or geographical region. | [optional] [default to null]
**country** | **String** | The club&#39;s country. | [optional] [default to null]
**private** | **bool** | Whether the club is private. | [optional] [default to null]
**member_count** | **i32** | The club&#39;s member count. | [optional] [default to null]
**featured** | **bool** | Whether the club is featured or not. | [optional] [default to null]
**verified** | **bool** | Whether the club is verified or not. | [optional] [default to null]
**url** | **String** | The club&#39;s vanity URL. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


