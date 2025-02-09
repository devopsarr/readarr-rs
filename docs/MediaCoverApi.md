# \MediaCoverApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_media_cover_author_by_filename**](MediaCoverApi.md#get_media_cover_author_by_filename) | **GET** /api/v1/mediacover/author/{authorId}/{filename} | 
[**get_media_cover_book_by_filename**](MediaCoverApi.md#get_media_cover_book_by_filename) | **GET** /api/v1/mediacover/book/{bookId}/{filename} | 



## get_media_cover_author_by_filename

> get_media_cover_author_by_filename(author_id, filename)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | **i32** |  | [required] |
**filename** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_cover_book_by_filename

> get_media_cover_book_by_filename(book_id, filename)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **i32** |  | [required] |
**filename** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

