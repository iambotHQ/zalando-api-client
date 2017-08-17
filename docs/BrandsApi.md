# \BrandsApi

All URIs are relative to *https://api.zalando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**BrandsGet**](BrandsApi.md#BrandsGet) | **Get** /brands | Shop Brands
[**BrandsKeyGet**](BrandsApi.md#BrandsKeyGet) | **Get** /brands/{key} | Get Single Brand by Key


# **BrandsGet**
> ::models::Brands BrandsGet(optional)
Shop Brands

Zalando API Brands Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key** | [**Vec&lt;String&gt;**](String.md)| Request Brand by key | 
 **name** | [**Vec&lt;String&gt;**](String.md)| Request Brand by name | 
 **brand_family_name** | [**Vec&lt;String&gt;**](String.md)| Request Brand by brandFamilyName | 
 **brand_family_key** | [**Vec&lt;String&gt;**](String.md)| Request Brand by brandFamilyKey | 
 **page** | **String**| to request with required page number or pagination | 
 **page_size** | **String**| to request with required page size in a page | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::Brands**](Brands.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **BrandsKeyGet**
> ::models::Brand BrandsKeyGet(key, optional)
Get Single Brand by Key

Zalando API Brand Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **key** | **String**| To get unique Brand by key. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key** | **String**| To get unique Brand by key. | 
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::Brand**](Brand.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

