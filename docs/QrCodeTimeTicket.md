# QrCodeTimeTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**qr_code_version** | Option<**i32**> | Version of QR code | [optional]
**qr_code_ticket_type** | Option<[**models::QrCodeTicketType**](QrCodeTicketType.md)> |  | [optional]
**ticket_id** | Option<**i64**> | If there is used discount there is as well ticket ID which is linked to it. | [optional]
**from_station_id** | Option<**i64**> | Valid station ID (city ID unsupported at this level) | [optional]
**to_station_id** | Option<**i64**> | Valid station ID (city ID unsupported at this level) | [optional]
**valid_from** | Option<**String**> | Start of validity time ticket | [optional]
**valid_to** | Option<**String**> | Expiration date of time ticket | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


