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
pub struct SimpleSection {
    #[serde(rename = "sectionId")]
    pub section_id: i64,
    /// Valid station ID (city ID unsupported at this level)
    #[serde(rename = "fromStationId")]
    pub from_station_id: i64,
    /// Valid station ID (city ID unsupported at this level)
    #[serde(rename = "toStationId")]
    pub to_station_id: i64,
}

impl SimpleSection {
    pub fn new(section_id: i64, from_station_id: i64, to_station_id: i64) -> SimpleSection {
        SimpleSection {
            section_id,
            from_station_id,
            to_station_id,
        }
    }
}


