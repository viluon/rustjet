# TimeTicketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**station1_id** | **i64** | Valid station ID. Independently of the order  | 
**station2_id** | **i64** | Valid station ID. Independently of the order | 
**valid_from** | **String** | Date from ticket is valid. | 
**tariff** | **String** | Tariff type | 
**r#type** | [**models::TimeTicketType**](TimeTicketType.md) |  | 
**seat_class** | **String** | Seat class - enum.PricedSeatClass (TRAIN_2ND_CLASS, TRAIN_1ST_CLASS) | 
**line_group_code** | **String** | Line group code. | 
**line_number** | **i32** | Line number. | 
**connection_code** | Option<**String**> | Connection code. | [optional]
**operator** | Option<**String**> | Operator code. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


