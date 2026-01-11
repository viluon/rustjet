# UnregisteredTimeTicketCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**station1_id** | **i64** |  | 
**station2_id** | **i64** |  | 
**time_ticket_type** | [**models::TimeTicketType**](TimeTicketType.md) |  | 
**valid_from** | **String** | Ticket validity DateTime. | 
**seat_class_key** | **String** | PricedSeatClass enum.name | 
**tariff** | [**models::TariffEnum**](TariffEnum.md) |  | 
**email** | **String** |  | 
**line_group_code** | **String** | Line group code. | 
**line_number** | **i32** | Line number. | 
**connection_code** | Option<**String**> | Connection code. | [optional]
**operator** | Option<**String**> | Operator code. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


