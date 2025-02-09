# Book

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**author_metadata_id** | Option<**i32**> |  | [optional]
**foreign_book_id** | Option<**String**> |  | [optional]
**foreign_edition_id** | Option<**String**> |  | [optional]
**title_slug** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**release_date** | Option<**String**> |  | [optional]
**links** | Option<[**Vec<models::Links>**](Links.md)> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**related_books** | Option<**Vec<i32>**> |  | [optional]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**last_search_time** | Option<**String**> |  | [optional]
**clean_title** | Option<**String**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**any_edition_ok** | Option<**bool**> |  | [optional]
**last_info_sync** | Option<**String**> |  | [optional]
**added** | Option<**String**> |  | [optional]
**add_options** | Option<[**models::AddBookOptions**](AddBookOptions.md)> |  | [optional]
**author_metadata** | Option<[**models::AuthorMetadataLazyLoaded**](AuthorMetadataLazyLoaded.md)> |  | [optional]
**author** | Option<[**models::AuthorLazyLoaded**](AuthorLazyLoaded.md)> |  | [optional]
**editions** | Option<[**models::EditionListLazyLoaded**](EditionListLazyLoaded.md)> |  | [optional]
**book_files** | Option<[**models::BookFileListLazyLoaded**](BookFileListLazyLoaded.md)> |  | [optional]
**series_links** | Option<[**models::SeriesBookLinkListLazyLoaded**](SeriesBookLinkListLazyLoaded.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


