/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VehicleType {
    #[serde(rename = "BUS")]
    BUS,
    #[serde(rename = "TRAIN")]
    TRAIN,

}

impl ToString for VehicleType {
    fn to_string(&self) -> String {
        match self {
            Self::BUS => String::from("BUS"),
            Self::TRAIN => String::from("TRAIN"),
        }
    }
}

impl Default for VehicleType {
    fn default() -> VehicleType {
        Self::BUS
    }
}




