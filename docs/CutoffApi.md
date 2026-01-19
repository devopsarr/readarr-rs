# \CutoffApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_wanted_cutoff**](CutoffApi.md#get_wanted_cutoff) | **GET** /api/v1/wanted/cutoff | 
[**get_wanted_cutoff_by_id**](CutoffApi.md#get_wanted_cutoff_by_id) | **GET** /api/v1/wanted/cutoff/{id} | 



## get_wanted_cutoff

> models::BookResourcePagingResource get_wanted_cutoff(page, page_size, sort_key, sort_direction, include_author, monitored)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](SortDirection.md)> |  |  |
**include_author** | Option<**bool**> |  |  |[default to false]
**monitored** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::BookResourcePagingResource**](BookResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wanted_cutoff_by_id

> models::BookResource get_wanted_cutoff_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::BookResource**](BookResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

