# \BookFileApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_book_file**](BookFileApi.md#delete_book_file) | **DELETE** /api/v1/bookfile/{id} | 
[**delete_book_file_bulk**](BookFileApi.md#delete_book_file_bulk) | **DELETE** /api/v1/bookfile/bulk | 
[**get_book_file_by_id**](BookFileApi.md#get_book_file_by_id) | **GET** /api/v1/bookfile/{id} | 
[**list_book_file**](BookFileApi.md#list_book_file) | **GET** /api/v1/bookfile | 
[**put_book_file_editor**](BookFileApi.md#put_book_file_editor) | **PUT** /api/v1/bookfile/editor | 
[**update_book_file**](BookFileApi.md#update_book_file) | **PUT** /api/v1/bookfile/{id} | 



## delete_book_file

> delete_book_file(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_book_file_bulk

> delete_book_file_bulk(book_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_file_list_resource** | Option<[**BookFileListResource**](BookFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_file_by_id

> models::BookFileResource get_book_file_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::BookFileResource**](BookFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_book_file

> Vec<models::BookFileResource> list_book_file(author_id, book_file_ids, book_id, unmapped)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | Option<**i32**> |  |  |
**book_file_ids** | Option<[**Vec<i32>**](I32.md)> |  |  |
**book_id** | Option<[**Vec<i32>**](I32.md)> |  |  |
**unmapped** | Option<**bool**> |  |  |

### Return type

[**Vec<models::BookFileResource>**](BookFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_book_file_editor

> put_book_file_editor(book_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_file_list_resource** | Option<[**BookFileListResource**](BookFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_book_file

> models::BookFileResource update_book_file(id, book_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**book_file_resource** | Option<[**BookFileResource**](BookFileResource.md)> |  |  |

### Return type

[**models::BookFileResource**](BookFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

