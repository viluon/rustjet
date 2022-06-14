# PriceClass

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seat_class_key** | **String** | Enum.name | 
**conditions** | [**crate::models::PriceConditions**](PriceConditions.md) |  | 
**services** | **Vec<String>** | Services icons (wifi, etc.) | 
**free_seats_count** | **i32** |  | 
**price** | **f32** |  | 
**credit_price** | **f32** |  | 
**price_source** | **String** | Pricing ID - used for price, services or terms confirmation after route search | 
**customer_notifications** | **Vec<String>** | Customer notifications | 
**action_price** | Option<[**crate::models::ActionPrice**](ActionPrice.md)> |  | [optional]
**tariffs** | **Vec<String>** |  | 
**tariff_notifications** | Option<[**crate::models::TariffNotifications**](TariffNotifications.md)> |  | [optional]
**bookable** | **bool** | There are free seats in class for reservation if TRUE otherwise FALSE | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


