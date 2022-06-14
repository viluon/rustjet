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
pub struct Notifications {
    #[serde(rename = "newsletter")]
    pub newsletter: bool,
    #[serde(rename = "reservationChange")]
    pub reservation_change: bool,
    #[serde(rename = "routeRatingSurvey")]
    pub route_rating_survey: bool,
}

impl Notifications {
    pub fn new(newsletter: bool, reservation_change: bool, route_rating_survey: bool) -> Notifications {
        Notifications {
            newsletter,
            reservation_change,
            route_rating_survey,
        }
    }
}

