# Category

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | The unique key for a category | [default to null]
**cid** | **i32** | The numeric ID for a category. | [optional] [default to null]
**name** | **String** | Name of the category | [default to null]
**parent_key** | **String** | The key of the parent category | [optional] [default to null]
**child_keys** | **Vec<String>** | The list of keys of the child categories | [default to null]
**_type** | **String** | The type of the category. | [optional] [default to null]
**outlet** | **bool** | Containing articles are from last seasons | [optional] [default to null]
**hidden** | **bool** | The category is hidden and not shown on the Zalando web shop | [optional] [default to null]
**target_group** | **String** | The target group of the articles from this category | [default to null]
**suggested_filters** | **Vec<String>** | list of filter names that are reasonable to use within this category | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


