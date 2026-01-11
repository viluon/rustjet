# SroTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sro_ticket_id** | **i64** |  | 
**price** | **f64** | Final price for ticket, addons etc. | 
**unpaid** | **f64** | Final price to be paid | 
**currency** | [**models::Currency**](Currency.md) |  | 
**state** | [**models::TicketState**](TicketState.md) |  | 
**seat_class_key** | **String** |  | 
**conditions** | [**models::SroConditions**](SroConditions.md) |  | 
**customer_actions** | [**models::CustomerActions**](CustomerActions.md) |  | 
**route_sections** | [**Vec<models::TicketSection>**](TicketSection.md) |  | 
**payment_id** | **i64** | Payment ID (groupTransactionID). Available only with paid ticket. Necessary for further action with ticket (for example: print invoice) | 
**bills** | [**Vec<models::SroTicketBill>**](SroTicketBill.md) |  | 
**passengers_info** | [**models::SroTicketPassenger**](SroTicketPassenger.md) |  | 
**delay** | **String** | Textual information about the first delay on the route | 
**travel_time** | **String** | Textual information about the travel time on a given section | 
**estimated_arrival_time** | **String** | Estimated arrival date time (arrival + delay) | 
**affiliate_ticket** | **bool** | Returns true in case the ticket was crited by affiliate partner. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


