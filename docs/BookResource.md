# BookResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**author_title** | Option<**String**> |  | [optional]
**series_title** | Option<**String**> |  | [optional]
**disambiguation** | Option<**String**> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**author_id** | Option<**i32**> |  | [optional]
**foreign_book_id** | Option<**String**> |  | [optional]
**foreign_edition_id** | Option<**String**> |  | [optional]
**title_slug** | Option<**String**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**any_edition_ok** | Option<**bool**> |  | [optional]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**release_date** | Option<**String**> |  | [optional]
**page_count** | Option<**i32**> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**author** | Option<[**models::AuthorResource**](AuthorResource.md)> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**links** | Option<[**Vec<models::Links>**](Links.md)> |  | [optional]
**statistics** | Option<[**models::BookStatisticsResource**](BookStatisticsResource.md)> |  | [optional]
**added** | Option<**String**> |  | [optional]
**add_options** | Option<[**models::AddBookOptions**](AddBookOptions.md)> |  | [optional]
**remote_cover** | Option<**String**> |  | [optional]
**editions** | Option<[**Vec<models::EditionResource>**](EditionResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


