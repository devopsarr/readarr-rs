# \AuthorApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_author**](AuthorApi.md#create_author) | **POST** /api/v1/author | 
[**delete_author**](AuthorApi.md#delete_author) | **DELETE** /api/v1/author/{id} | 
[**get_author_by_id**](AuthorApi.md#get_author_by_id) | **GET** /api/v1/author/{id} | 
[**list_author**](AuthorApi.md#list_author) | **GET** /api/v1/author | 
[**update_author**](AuthorApi.md#update_author) | **PUT** /api/v1/author/{id} | 



## create_author

> models::AuthorResource create_author(author_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_resource** | Option<[**AuthorResource**](AuthorResource.md)> |  |  |

### Return type

[**models::AuthorResource**](AuthorResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_author

> delete_author(id, delete_files, add_import_list_exclusion)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**delete_files** | Option<**bool**> |  |  |[default to false]
**add_import_list_exclusion** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_author_by_id

> models::AuthorResource get_author_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AuthorResource**](AuthorResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_author

> Vec<models::AuthorResource> list_author()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AuthorResource>**](AuthorResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_author

> models::AuthorResource update_author(id, move_files, author_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**move_files** | Option<**bool**> |  |  |[default to false]
**author_resource** | Option<[**AuthorResource**](AuthorResource.md)> |  |  |

### Return type

[**models::AuthorResource**](AuthorResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

