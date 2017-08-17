# \RecommendationsApi

All URIs are relative to *https://api.zalando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**RecommendationsArticleIdsGet**](RecommendationsApi.md#RecommendationsArticleIdsGet) | **Get** /recommendations/{articleIds} | Get Recommendations by articleId


# **RecommendationsArticleIdsGet**
> ::models::Recommendations RecommendationsArticleIdsGet(article_ids, optional)
Get Recommendations by articleId

Zalando API Recommendations Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **article_ids** | [**Vec&lt;String&gt;**](String.md)| To get Recommendations by articleIds. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **article_ids** | [**Vec&lt;String&gt;**](String.md)| To get Recommendations by articleIds. | 
 **max_results** | **String**| To get maximum results of Recommendations by articleId. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::Recommendations**](Recommendations.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

