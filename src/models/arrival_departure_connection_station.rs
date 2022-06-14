/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArrivalDepartureConnectionStation {
    /// Station ID used for search in locations
    #[serde(rename = "stationId")]
    pub station_id: i64,
    /// Planned station arrival time (1st station in order does not have arrival time)
    #[serde(rename = "arrival", skip_serializing_if = "Option::is_none")]
    pub arrival: Option<String>,
    /// Planned station departure time (1st station in order does not have arrival time)
    #[serde(rename = "departure", skip_serializing_if = "Option::is_none")]
    pub departure: Option<String>,
    /// Platform
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Is departing station?
    #[serde(rename = "departingStation", skip_serializing_if = "Option::is_none")]
    pub departing_station: Option<bool>,
}

impl ArrivalDepartureConnectionStation {
    pub fn new(station_id: i64) -> ArrivalDepartureConnectionStation {
        ArrivalDepartureConnectionStation {
            station_id,
            arrival: None,
            departure: None,
            platform: None,
            departing_station: None,
        }
    }
}

