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
pub enum PersonalDataType {
    #[serde(rename = "FIRST_NAME")]
    FIRSTNAME,
    #[serde(rename = "SURNAME")]
    SURNAME,
    #[serde(rename = "BIRTHDAY")]
    BIRTHDAY,
    #[serde(rename = "EMAIL")]
    EMAIL,
    #[serde(rename = "PHONE")]
    PHONE,
    #[serde(rename = "ZIP_CODE")]
    ZIPCODE,
    #[serde(rename = "PERSONAL_NUMBER")]
    PERSONALNUMBER,
    #[serde(rename = "STREET")]
    STREET,
    #[serde(rename = "HOUSE_NUMBER")]
    HOUSENUMBER,
    #[serde(rename = "CITY")]
    CITY,

}

impl ToString for PersonalDataType {
    fn to_string(&self) -> String {
        match self {
            Self::FIRSTNAME => String::from("FIRST_NAME"),
            Self::SURNAME => String::from("SURNAME"),
            Self::BIRTHDAY => String::from("BIRTHDAY"),
            Self::EMAIL => String::from("EMAIL"),
            Self::PHONE => String::from("PHONE"),
            Self::ZIPCODE => String::from("ZIP_CODE"),
            Self::PERSONALNUMBER => String::from("PERSONAL_NUMBER"),
            Self::STREET => String::from("STREET"),
            Self::HOUSENUMBER => String::from("HOUSE_NUMBER"),
            Self::CITY => String::from("CITY"),
        }
    }
}

impl Default for PersonalDataType {
    fn default() -> PersonalDataType {
        Self::FIRSTNAME
    }
}




