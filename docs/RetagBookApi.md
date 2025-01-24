# \RetagBookApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_retag**](RetagBookApi.md#list_retag) | **GET** /api/v1/retag | 



## list_retag

> Vec<models::RetagBookResource> list_retag(author_id, book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | Option<**i32**> |  |  |
**book_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::RetagBookResource>**](RetagBookResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

