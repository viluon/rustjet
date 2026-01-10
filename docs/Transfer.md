# Transfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_station_id** | **i64** | Station ID from which customer needs to transfer | 
**to_station_id** | **i64** | Station ID to which customer needs to transfer | 
**r#type** | [**models::TransferType**](TransferType.md) |  | 
**calculated_transfer_time** | [**models::TimePeriod**](TimePeriod.md) |  | 
**determined_transfer_time** | Option<[**models::TimePeriod**](TimePeriod.md)> |  | [optional]
**description** | Option<**String**> | Transfer specification | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


