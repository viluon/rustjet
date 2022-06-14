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
pub struct SimpleRoute {
    /// {section0.id},{section1.id}, ... 
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "departureStationId", skip_serializing_if = "Option::is_none")]
    pub departure_station_id: Option<i64>,
    #[serde(rename = "departureTime")]
    pub departure_time: String,
    #[serde(rename = "arrivalStationId", skip_serializing_if = "Option::is_none")]
    pub arrival_station_id: Option<i64>,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: String,
    /// Connection type (for example: train, bus)
    #[serde(rename = "vehicleTypes")]
    pub vehicle_types: Vec<crate::models::VehicleType>,
    /// Amount of transfers
    #[serde(rename = "transfersCount", skip_serializing_if = "Option::is_none")]
    pub transfers_count: Option<i32>,
    /// Returns smallest number of available free seats through all sections
    #[serde(rename = "freeSeatsCount")]
    pub free_seats_count: i32,
    #[serde(rename = "priceFrom")]
    pub price_from: f32,
    #[serde(rename = "priceTo", skip_serializing_if = "Option::is_none")]
    pub price_to: Option<f32>,
    #[serde(rename = "creditPriceFrom")]
    pub credit_price_from: f32,
    #[serde(rename = "creditPriceTo", skip_serializing_if = "Option::is_none")]
    pub credit_price_to: Option<f32>,
    /// Amount of prices
    #[serde(rename = "pricesCount")]
    pub prices_count: i32,
    /// TRUE if any of prices is action price, otherwise FALSE
    #[serde(rename = "actionPrice")]
    pub action_price: bool,
    /// TRUE if there is surcharge on this line, otherwise FALSE
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// Notice of any extraordinarily events on route / traffic limitation
    #[serde(rename = "notices")]
    pub notices: bool,
    /// TRUE if this line (or its part) have support connection, otherwise FALSE
    #[serde(rename = "support")]
    pub support: bool,
    /// TRUE => national, FALSE => international
    #[serde(rename = "nationalTrip", skip_serializing_if = "Option::is_none")]
    pub national_trip: Option<bool>,
    /// TRUE if at least one class have enough free seats for reservation, otherwise FALSE
    #[serde(rename = "bookable")]
    pub bookable: bool,
    /// Textual information about the first delay on the route
    #[serde(rename = "delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// Textual information about the travel time on the route
    #[serde(rename = "travelTime", skip_serializing_if = "Option::is_none")]
    pub travel_time: Option<String>,
    /// Vehicle standard code tag
    #[serde(rename = "vehicleStandardKey", skip_serializing_if = "Option::is_none")]
    pub vehicle_standard_key: Option<String>,
}

impl SimpleRoute {
    pub fn new(id: String, departure_time: String, arrival_time: String, vehicle_types: Vec<crate::models::VehicleType>, free_seats_count: i32, price_from: f32, credit_price_from: f32, prices_count: i32, action_price: bool, surcharge: bool, notices: bool, support: bool, bookable: bool) -> SimpleRoute {
        SimpleRoute {
            id,
            departure_station_id: None,
            departure_time,
            arrival_station_id: None,
            arrival_time,
            vehicle_types,
            transfers_count: None,
            free_seats_count,
            price_from,
            price_to: None,
            credit_price_from,
            credit_price_to: None,
            prices_count,
            action_price,
            surcharge,
            notices,
            support,
            national_trip: None,
            bookable,
            delay: None,
            travel_time: None,
            vehicle_standard_key: None,
        }
    }
}

