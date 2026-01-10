# TimeTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the time ticket | 
**account_code** | Option<**i64**> | Account code of the time ticket owner | [optional]
**station1_id** | **i64** | ID of the station one | 
**station2_id** | **i64** | ID of the station two | 
**name** | Option<**String**> | Name of the ticket owner | [optional]
**surname** | Option<**String**> | Surname of the ticket owner | [optional]
**tariff** | [**models::Tariff**](Tariff.md) |  | 
**r#type** | [**models::TimeTicketType**](TimeTicketType.md) |  | 
**state** | [**models::TicketState**](TicketState.md) |  | 
**seat_class** | [**models::SeatClass**](SeatClass.md) |  | 
**valid_from** | **String** |  | 
**valid_to** | Option<**String**> |  | [optional]
**customer_actions** | [**models::TimeTicketCustomerAction**](TimeTicketCustomerAction.md) |  | 
**conditions** | [**models::PriceConditions**](PriceConditions.md) |  | 
**bills** | Option<[**Vec<models::TimeTicketBill>**](TimeTicketBill.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


