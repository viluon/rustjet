# FormQuestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**question_id** | **i32** | Question identifier. If empty form wont be send to server during saving (for example: static elements without customer entry) | 
**_type** | [**crate::models::FormQuestionType**](FormQuestionType.md) |  | 
**text** | **String** | Qustion title, text paragraph or form title | 
**watermark** | Option<**String**> | Text fields watermark | [optional]
**options** | Option<[**Vec<crate::models::FormAnswer>**](FormAnswer.md)> | List of options (only if there is select option type) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


