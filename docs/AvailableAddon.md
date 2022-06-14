# AvailableAddon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addon_id** | **i64** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**icon_url** | **String** |  | 
**info_url** | Option<**String**> |  | [optional]
**info_url_label** | Option<**String**> |  | [optional]
**price** | **f32** |  | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**max_count** | Option<**i32**> | Maximal amount of current addon in one order. If no limit is set, this addon will be limitless | [optional]
**conditions** | [**crate::models::AvailableAddonConditions**](AvailableAddonConditions.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


