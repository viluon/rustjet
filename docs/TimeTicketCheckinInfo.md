# TimeTicketCheckinInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | Response message. | 
**account_code** | **String** | Client account code which represents number of ticket. | 
**time_ticket_id** | **i64** | Identifier of the time ticket. | 
**state** | [**crate::models::TicketState**](TicketState.md) |  | 
**from_station_id** | Option<**i64**> | Identifier of the departure station. | [optional]
**to_station_id** | Option<**i64**> | Identifier of the arrival station. | [optional]
**tariff** | [**crate::models::Tariff**](Tariff.md) |  | 
**name_and_surname** | **String** | Name and surname of passenger. | 
**seat_class_code** | [**crate::models::SeatClass**](SeatClass.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


