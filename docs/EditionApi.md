# \EditionApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_edition**](EditionApi.md#list_edition) | **GET** /api/v1/edition | 



## list_edition

> Vec<models::EditionResource> list_edition(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | Option<[**Vec<i32>**](I32.md)> |  |  |

### Return type

[**Vec<models::EditionResource>**](EditionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

