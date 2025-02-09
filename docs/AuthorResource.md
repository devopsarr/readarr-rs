# AuthorResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**author_metadata_id** | Option<**i32**> |  | [optional]
**status** | Option<[**models::AuthorStatusType**](AuthorStatusType.md)> |  | [optional]
**ended** | Option<**bool**> |  | [optional][readonly]
**author_name** | Option<**String**> |  | [optional]
**author_name_last_first** | Option<**String**> |  | [optional]
**foreign_author_id** | Option<**String**> |  | [optional]
**title_slug** | Option<**String**> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**disambiguation** | Option<**String**> |  | [optional]
**links** | Option<[**Vec<models::Links>**](Links.md)> |  | [optional]
**next_book** | Option<[**models::Book**](Book.md)> |  | [optional]
**last_book** | Option<[**models::Book**](Book.md)> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**remote_poster** | Option<**String**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**quality_profile_id** | Option<**i32**> |  | [optional]
**metadata_profile_id** | Option<**i32**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**monitor_new_items** | Option<[**models::NewItemMonitorTypes**](NewItemMonitorTypes.md)> |  | [optional]
**root_folder_path** | Option<**String**> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**clean_name** | Option<**String**> |  | [optional]
**sort_name** | Option<**String**> |  | [optional]
**sort_name_last_first** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<i32>**> |  | [optional]
**added** | Option<**String**> |  | [optional]
**add_options** | Option<[**models::AddAuthorOptions**](AddAuthorOptions.md)> |  | [optional]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**statistics** | Option<[**models::AuthorStatisticsResource**](AuthorStatisticsResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


