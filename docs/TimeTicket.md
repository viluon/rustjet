# TimeTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the ticket | 
**account_code** | **String** |  | 
**ticket_code** | **String** |  | 
**price** | **f64** |  | 
**unpaid** | **f64** |  | 
**name** | **String** |  | 
**sur_name** | **String** |  | 
**tariff** | [**models::Tariff**](Tariff.md) |  | 
**r#type** | [**models::TimeTicketType**](TimeTicketType.md) |  | 
**state** | [**models::TicketState**](TicketState.md) |  | 
**seat_class** | **String** |  | 
**conditions** | [**models::TimeTicketConditions**](TimeTicketConditions.md) |  | 
**customer_notifications** | Option<**Vec<String>**> |  | [optional]
**customer_actions** | [**models::CustomerActions**](CustomerActions.md) |  | 
**bills** | [**Vec<models::TimeTicketBill>**](TimeTicketBill.md) |  | 
**station1_id** | **i64** |  | 
**station2_id** | **i64** |  | 
**valid_from** | **String** |  | 
**valid_to** | **String** |  | 
**payment_id** | **i64** | Time ticket group transaction ID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


