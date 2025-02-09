# \CalendarFeedApi

All URIs are relative to *http://localhost:8787*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_feed_v1_calendar_readarr_period_ics**](CalendarFeedApi.md#get_feed_v1_calendar_readarr_period_ics) | **GET** /feed/v1/calendar/readarr.ics | 



## get_feed_v1_calendar_readarr_period_ics

> get_feed_v1_calendar_readarr_period_ics(past_days, future_days, tag_list, unmonitored)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**past_days** | Option<**i32**> |  |  |[default to 7]
**future_days** | Option<**i32**> |  |  |[default to 28]
**tag_list** | Option<**String**> |  |  |[default to ]
**unmonitored** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

