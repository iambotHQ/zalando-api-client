# \ArticlesApi

All URIs are relative to *https://api.zalando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ArticleReviewsGet**](ArticlesApi.md#ArticleReviewsGet) | **Get** /article-reviews | Get Article Reviews
[**ArticleReviewsReviewIdGet**](ArticlesApi.md#ArticleReviewsReviewIdGet) | **Get** /article-reviews/{reviewId} | Get Article Reviews by reviewId
[**ArticleReviewsSummariesArticleModelIdGet**](ArticlesApi.md#ArticleReviewsSummariesArticleModelIdGet) | **Get** /article-reviews-summaries/{articleModelId} | Get Article Reviews Summaries by articleModelId
[**ArticleReviewsSummariesGet**](ArticlesApi.md#ArticleReviewsSummariesGet) | **Get** /article-reviews-summaries | Get Article Reviews Summaries
[**ArticlesArticleIdGet**](ArticlesApi.md#ArticlesArticleIdGet) | **Get** /articles/{articleId} | Get Article by articleId
[**ArticlesArticleIdMediaGet**](ArticlesApi.md#ArticlesArticleIdMediaGet) | **Get** /articles/{articleId}/media | Get Article media by articleId
[**ArticlesArticleIdReviewsGet**](ArticlesApi.md#ArticlesArticleIdReviewsGet) | **Get** /articles/{articleId}/reviews | Get Article reviews by articleId
[**ArticlesArticleIdReviewsSummaryGet**](ArticlesApi.md#ArticlesArticleIdReviewsSummaryGet) | **Get** /articles/{articleId}/reviews-summary | Get Article reviews summary by articleId
[**ArticlesArticleIdUnitsGet**](ArticlesApi.md#ArticlesArticleIdUnitsGet) | **Get** /articles/{articleId}/units | Get Article units by articleId
[**ArticlesArticleIdUnitsUnitIdGet**](ArticlesApi.md#ArticlesArticleIdUnitsUnitIdGet) | **Get** /articles/{articleId}/units/{unitId} | Get Article units by articleId snd unitId
[**ArticlesGet**](ArticlesApi.md#ArticlesGet) | **Get** /articles | Search for Articles


# **ArticleReviewsGet**
> ::models::ArticleReviews ArticleReviewsGet(optional)
Get Article Reviews

Zalando API Article Reviews Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | [**Vec&lt;String&gt;**](String.md)| Article IDs. A list of config SKUs for which the article reviews will be returned. Required if articleModelId is empty.  | 
 **article_model_id** | [**Vec&lt;String&gt;**](String.md)| Article model IDs. A list of model SKUs for which the article reviews will be returned. Required if articleId is empty.  | 
 **min_star_rating** | **String**| To get reviews with minimum star rating. | 
 **max_star_rating** | **String**| To get reviews with maximum star rating. | 
 **sort** | **String**| articles are sorted on reviews provided by customers (eg: best) | [default to newest]
 **page** | **String**| to request with required page number or pagination | 
 **page_size** | **String**| to request with required page size in a page | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleReviews**](Article-Reviews.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticleReviewsReviewIdGet**
> ::models::ArticleReview ArticleReviewsReviewIdGet(review_id, optional)
Get Article Reviews by reviewId

Zalando API ArticleReviews Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **review_id** | **String**| To get unique review by review Id. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **review_id** | **String**| To get unique review by review Id. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleReview**](Article-Review.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticleReviewsSummariesArticleModelIdGet**
> ::models::ArticleReviewsSummary ArticleReviewsSummariesArticleModelIdGet(article_model_id, optional)
Get Article Reviews Summaries by articleModelId

Zalando API ArticleReviewsSummaries Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_model_id** | **String**| To get unique reviews summary from article model Id. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_model_id** | **String**| To get unique reviews summary from article model Id. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleReviewsSummary**](Article-Reviews-Summary.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticleReviewsSummariesGet**
> ::models::ArticleReviewsSummaries ArticleReviewsSummariesGet(article_model_id, optional)
Get Article Reviews Summaries

Zalando API Article Reviews Summaries Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_model_id** | [**Vec&lt;String&gt;**](String.md)| Article model IDs. A list of model SKUs for which the article review summaries will be returned. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_model_id** | [**Vec&lt;String&gt;**](String.md)| Article model IDs. A list of model SKUs for which the article review summaries will be returned. | 
 **page** | **String**| to request with required page number or pagination | 
 **page_size** | **String**| to request with required page size in a page | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleReviewsSummaries**](Article-Reviews-Summaries.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesArticleIdGet**
> ::models::Article ArticlesArticleIdGet(article_id, optional)
Get Article by articleId

Zalando API Article Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_id** | **String**| To get unique article for article Id. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | **String**| To get unique article for article Id. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::Article**](Article.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesArticleIdMediaGet**
> ::models::ArticleMedia ArticlesArticleIdMediaGet(article_id, optional)
Get Article media by articleId

Zalando API Article Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_id** | **String**| To get unique article for article Id media. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | **String**| To get unique article for article Id media. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleMedia**](Article-Media.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesArticleIdReviewsGet**
> ::models::ArticleReviews ArticlesArticleIdReviewsGet(article_id, optional)
Get Article reviews by articleId

Zalando API Article Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_id** | **String**| To get unique article for article Id reviews. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | **String**| To get unique article for article Id reviews. | 
 **min_star_rating** | **String**| To get reviews with minimum star rating. | 
 **max_star_rating** | **String**| To get reviews with maximum star rating. | 
 **sort** | **String**| articles are sorted on reviews provided by customers (eg: best) | [default to newest]
 **page** | **String**| to request with required page number or pagination | 
 **page_size** | **String**| to request with required page size in a page | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleReviews**](Article-Reviews.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesArticleIdReviewsSummaryGet**
> ::models::ArticleReviewsSummary ArticlesArticleIdReviewsSummaryGet(article_id, optional)
Get Article reviews summary by articleId

Zalando API Article Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_id** | **String**| To get unique article for article Id reviews summary. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | **String**| To get unique article for article Id reviews summary. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleReviewsSummary**](Article-Reviews-Summary.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesArticleIdUnitsGet**
> ::models::ArticleUnits ArticlesArticleIdUnitsGet(article_id, optional)
Get Article units by articleId

Zalando API Article Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_id** | **String**| To get unique article for article Id units. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | **String**| To get unique article for article Id units. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleUnits**](Article-Units.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesArticleIdUnitsUnitIdGet**
> ::models::ArticleUnit ArticlesArticleIdUnitsUnitIdGet(article_id, unit_id, optional)
Get Article units by articleId snd unitId

Zalando API Article Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_id** | **String**| To get unique article for article Id. | 
  **unit_id** | **String**| To get unique article for article Id unit. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | **String**| To get unique article for article Id. | 
 **unit_id** | **String**| To get unique article for article Id unit. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::ArticleUnit**](Article-Unit.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArticlesGet**
> ::models::Articles ArticlesGet(optional)
Search for Articles

Search for articles based on various different possible filter like e.g. the `brandFamily`, the `catagory` or a specific `color`. See [Filters](https://todo) for a list of all available filter options.  The default `pageSize` for responses is set to `20`. You can add a `pageSize` request parameter to adjust that. Please keep in mind that the maximum `pageSize` for this resource is `200`.  The endpoint also supports reducing the fields returned for each article via the `fields` parameter. Please refer to [fields parameter](https://todo) for more details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_id** | [**Vec&lt;String&gt;**](String.md)| The &#x60;articleIds&#x60; to use use for filtering.  One or more &#x60;articleIds&#x60; might be used as a filter criteria. Submit multiple &#x60;articleId&#x60; request parameters for more than one to be used. They will be treated as &#x60;OR&#x60; criteria. | 
 **article_model_id** | [**Vec&lt;String&gt;**](String.md)| filters by article ModelId | 
 **article_unit_id** | [**Vec&lt;String&gt;**](String.md)| filters by article&#39;s unit id | 
 **activation_date** | [**Vec&lt;String&gt;**](String.md)| period or time the articles are activated for selling in the shop | 
 **age_group** | [**Vec&lt;String&gt;**](String.md)| filters by age group (eg: kids) | 
 **assortment_area** | [**Vec&lt;String&gt;**](String.md)| filters by classification of articles (eg: maternity)  | 
 **brand** | [**Vec&lt;String&gt;**](String.md)| filters by brand key given by user (eg: SA5) | 
 **brandfamily** | [**Vec&lt;String&gt;**](String.md)| filters by brand family key (eg: nike)  | 
 **category** | [**Vec&lt;String&gt;**](String.md)| filters by category (eg: Socks, Rain Coats) | 
 **color** | [**Vec&lt;String&gt;**](String.md)| filters by color (eg: red, blue) | 
 **den** | [**Vec&lt;String&gt;**](String.md)| filters by den  | 
 **filling** | [**Vec&lt;String&gt;**](String.md)| filters by different kinds of garment filling materials (eg: satin, wolle) | 
 **full_text** | **String**| filters by text (eg: search by &#39;as&#39; gives result with articles of brand Sass) | 
 **gender** | [**Vec&lt;String&gt;**](String.md)| filters by gender | 
 **heel_form** | [**Vec&lt;String&gt;**](String.md)| filters by heel form (eg: flat) | 
 **heel_height** | [**Vec&lt;String&gt;**](String.md)| filters by height of the heel size or length (eg: xs) | 
 **length** | **String**| filters by garments length (eg: 3/4 length, knee-length) | 
 **occasion** | [**Vec&lt;String&gt;**](String.md)| filters by type of occasion (eg: party, business) | 
 **pattern** | [**Vec&lt;String&gt;**](String.md)| filters by pattern on the garments (eg: animal print, plain) | 
 **price** | **String**| filters all articles in price range (eg: 9-90) | 
 **sale** | [**Vec&lt;String&gt;**](String.md)| filters discounted articles marked as sale | 
 **season** | [**Vec&lt;String&gt;**](String.md)| filters by season (Autumn/Winter or Spring/Summer) | 
 **shaft_height** | [**Vec&lt;String&gt;**](String.md)| filters by shaft height (eg: s, xs) | 
 **shaft_width** | [**Vec&lt;String&gt;**](String.md)| filters by shaft width (eg: s, l) | 
 **shirt_collar** | [**Vec&lt;String&gt;**](String.md)| filters by shirt collar styles (eg: low V neck, lined collar) | 
 **shoe_fastener** | [**Vec&lt;String&gt;**](String.md)| filters by shoe fastener types (eg: buckle, lacing) | 
 **shoe_toecap** | [**Vec&lt;String&gt;**](String.md)| filters by shoe toe cap variants (eg: pointed, square) | 
 **shop_area** | [**Vec&lt;String&gt;**](String.md)| filters by classification of articles | 
 **size** | **String**| filters by size | 
 **sports** | [**Vec&lt;String&gt;**](String.md)| filters by different sport activities (eg: football) | 
 **technology** | [**Vec&lt;String&gt;**](String.md)| filters by technology used to produce the articles | 
 **trouser_rise** | [**Vec&lt;String&gt;**](String.md)| filters by trouser rise | 
 **upper_material** | [**Vec&lt;String&gt;**](String.md)| filters by different type of upper material used on garments (eg: lace) | 
 **volume** | [**Vec&lt;String&gt;**](String.md)| filters by volume | 
 **page** | **String**| to request with required page number or pagination | 
 **page_size** | **String**| to request with required page size in a page | 
 **sort** | **String**| sorting order (eg: popularity) | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::Articles**](Articles.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

