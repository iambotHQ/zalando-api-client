# Article

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | unique article id | [default to null]
**model_id** | **String** | unique article model id | [default to null]
**name** | **String** | article name | [default to null]
**shop_url** | **String** | url of the article within the Zalando webshop | [default to null]
**color** | **String** | the main color of the article | [default to null]
**available** | **bool** | will be true if at least one article unit is available | [default to null]
**season** | **String** | collection season the article belongs to | [default to null]
**season_year** | **String** | collection year the article belongs to. Could be either a year or &#39;ALL&#39; | [default to null]
**activation_date** | **String** | timestamp the article was available in the Zalando webshop | [default to null]
**additional_infos** | **Vec<String>** | any additional information of the article  | [default to null]
**genders** | **Vec<String>** | gender of the article belongs to | [default to null]
**age_groups** | **Vec<String>** | age group of the article belongs to | [default to null]
**brand** | [***::models::Brand**](Brand.md) |  | [default to null]
**category_keys** | **Vec<String>** | category keys of the article belongs to | [default to null]
**attributes** | [**Vec<::models::ArticleAttribute>**](Article-Attribute.md) | article attributes | [default to null]
**units** | [**Vec<::models::ArticleUnit>**](Article-Unit.md) | price of the article | [default to null]
**tags** | **Vec<String>** |  | [optional] [default to null]
**media** | [***::models::ArticleMedia**](Article-Media.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


