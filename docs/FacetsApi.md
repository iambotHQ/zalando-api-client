# \FacetsApi

All URIs are relative to *https://api.zalando.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**FacetsGet**](FacetsApi.md#FacetsGet) | **Get** /facets | Shop Facets


# **FacetsGet**
> ::models::Facets FacetsGet(optional)
Shop Facets

Zalando API Facets Schema

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **age_group** | [**Vec&lt;String&gt;**](String.md)| filters by age group (eg: kids) | 
 **article_id** | [**Vec&lt;String&gt;**](String.md)| The &#x60;articleIds&#x60; to use use for filtering.  One or more &#x60;articleIds&#x60; might be used as a filter criteria. Submit multiple &#x60;articleId&#x60; request parameters for more than one to be used. They will be treated as &#x60;OR&#x60; criteria. | 
 **activation_date** | [**Vec&lt;String&gt;**](String.md)| period or time the articles are activated for selling in the shop | 
 **article_model_id** | [**Vec&lt;String&gt;**](String.md)| filters by article ModelId | 
 **assortment_area** | [**Vec&lt;String&gt;**](String.md)| filters by classification of articles (eg: maternity)  | 
 **brand** | [**Vec&lt;String&gt;**](String.md)| filters by brand key given by user (eg: SA5) | 
 **brandfamily** | [**Vec&lt;String&gt;**](String.md)| filters by brand family key (eg: nike)  | 
 **category** | [**Vec&lt;String&gt;**](String.md)| filters by category (eg: Socks, Rain Coats) | 
 **color** | [**Vec&lt;String&gt;**](String.md)| filters by color (eg: red, blue) | 
 **den** | [**Vec&lt;String&gt;**](String.md)| filters by den  | 
 **filling** | [**Vec&lt;String&gt;**](String.md)| filters by different kinds of garment filling materials (eg: satin, wolle) | 
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
 **accept_language** | **String**| Specify which Shop to use.  A standard &#x60;Accept-Language&#x60; header which specifies the shop that should be used. E.g. &#x60;de-DE&#x60; will use the German shop (as does [https://www.zalando.de](https://www/zalando.de) and &#x60;de-AT&#x60; will use the Austrian one.  The shop choosen will e.g. define the currency used for prices as well as the language for product names and descriptions. Furthermore it will impact which articles are available as they might differ between countries. | 
 **fields** | [**Vec&lt;String&gt;**](String.md)| Comma separated list of fields that should be returned. Fields of subobjects are specified with dots as separator. Fields of objects within lists are specified in the same way.  Example: id,name,brand.key,brand.name, units.id,units.size,units.price.formatted | 

### Return type

[**::models::Facets**](Facets.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

