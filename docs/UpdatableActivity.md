# UpdatableActivity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**commute** | **bool** | Whether this activity is a commute | [optional] [default to null]
**trainer** | **bool** | Whether this activity was recorded on a training machine | [optional] [default to null]
**hide_from_home** | **bool** | Whether this activity is muted | [optional] [default to null]
**description** | **String** | The description of the activity | [optional] [default to null]
**name** | **String** | The name of the activity | [optional] [default to null]
**_type** | [***::models::ActivityType**](ActivityType.md) | Deprecated. Prefer to use sport_type. In a request where both type and sport_type are present, this field will be ignored | [optional] [default to null]
**sport_type** | [***::models::SportType**](SportType.md) |  | [optional] [default to null]
**gear_id** | **String** | Identifier for the gear associated with the activity. ‘none’ clears gear from activity | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


