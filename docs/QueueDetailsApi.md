# \QueueDetailsApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_queue_details**](QueueDetailsApi.md#list_queue_details) | **GET** /api/v1/queue/details | 



## list_queue_details

> Vec<models::QueueResource> list_queue_details(author_id, book_ids, include_author, include_book)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | Option<**i32**> |  |  |
**book_ids** | Option<[**Vec<i32>**](I32.md)> |  |  |
**include_author** | Option<**bool**> |  |  |[default to false]
**include_book** | Option<**bool**> |  |  |[default to true]

### Return type

[**Vec<models::QueueResource>**](QueueResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

