# Ticket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the ticket | 
**route_id** | **String** | route id (section0.id,section1.id, ... sectionx.id) | 
**price** | **f32** | Final price for ticket, addons and insurance etc. | 
**unpaid** | **f32** | Final price to be paid | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**state** | [**crate::models::TicketState**](TicketState.md) |  | 
**seat_class_key** | **String** |  | 
**conditions** | [**crate::models::PriceConditions**](PriceConditions.md) |  | 
**expiration_date** | Option<**String**> |  | [optional]
**expirate_at** | Option<[**crate::models::TimePeriod**](TimePeriod.md)> |  | [optional]
**customer_notifications** | Option<**Vec<String>**> |  | [optional]
**customer_actions** | [**crate::models::CustomerActions**](CustomerActions.md) |  | 
**route_sections** | [**Vec<crate::models::TicketSection>**](TicketSection.md) |  | 
**addons** | Option<[**Vec<crate::models::OrderedAddon>**](OrderedAddon.md)> |  | [optional]
**payment_id** | Option<**i64**> | Payment ID (groupTransactionID). Available only with paid ticket. Necessary for further action with ticket (for example: print invoice) | [optional]
**bills** | Option<[**Vec<crate::models::TicketBill>**](TicketBill.md)> |  | [optional]
**used_code_discount** | Option<[**crate::models::CodeDiscount**](CodeDiscount.md)> |  | [optional]
**used_percentual_discounts** | Option<[**Vec<crate::models::PercentualDiscount>**](PercentualDiscount.md)> | Applied procentual discounts | [optional]
**transfers_info** | Option<[**crate::models::TransfersInfo**](TransfersInfo.md)> |  | [optional]
**surcharge** | Option<**f32**> | Total count of all irreversible surcharges in current currency | [optional]
**cancel_charge_sum** | Option<**f32**> | Total count of all charges and surcharge | [optional]
**cancel_money_back_sum** | Option<**f32**> | Total count of reversible amounts | [optional]
**passengers_info** | [**crate::models::PassengersInfo**](PassengersInfo.md) |  | 
**delay** | Option<**String**> | Textual information about the first delay on the route | [optional]
**travel_time** | Option<**String**> | Textual information about the travel time on a given section | [optional]
**estimated_arrival_time** | Option<**String**> | Estimated arrival date time (arrival + delay) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


