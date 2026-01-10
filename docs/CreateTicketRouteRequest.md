# CreateTicketRouteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**route_id** | **String** | route id (section0.id,section1.id, ... sectionx.id) | 
**seat_class** | **String** |  | 
**price_source** | **String** | Pricing ID - used for confirmation that price, services or conditions werent changed | 
**action_price** | Option<[**models::ActionPrice**](ActionPrice.md)> |  | [optional]
**sections** | [**Vec<models::CreateTicketSectionRequest>**](CreateTicketSectionRequest.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


