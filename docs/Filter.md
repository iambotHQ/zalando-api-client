# Filter

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The unique name for a filter | [default to null]
**multi_value** | **bool** | can this filter be used multiple times with different values in one search request | [default to null]
**_type** | **String** | filter enum types | [default to null]
**values** | [**Vec<::models::FilterValue>**](Filter-Value.md) | only if type is &#39;enum&#39; this list contains all possible filter values | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


