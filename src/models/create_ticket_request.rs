/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */

/// CreateTicketRequest : TODO: passenger by m�l m�t p��znak isInsuarence + data...



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    #[serde(rename = "route")]
    pub route: Box<crate::models::CreateTicketRouteRequest>,
    #[serde(rename = "selectedAddons", skip_serializing_if = "Option::is_none")]
    pub selected_addons: Option<Vec<crate::models::SelectedAddon>>,
    #[serde(rename = "passengers")]
    pub passengers: Vec<crate::models::Passenger>,
    /// Flat rate discount from fare price (does not apply on addons, insurance and charges). Applies first (before percentual discount)
    #[serde(rename = "codeDiscount", skip_serializing_if = "Option::is_none")]
    pub code_discount: Option<String>,
    /// Percentual discount from fare price (does not apply on addons, insurance and charges). Applies as seconds (after flat rate discount)
    #[serde(rename = "percentualDiscountIds", skip_serializing_if = "Option::is_none")]
    pub percentual_discount_ids: Option<Vec<i64>>,
}

impl CreateTicketRequest {
    /// TODO: passenger by m�l m�t p��znak isInsuarence + data...
    pub fn new(route: crate::models::CreateTicketRouteRequest, passengers: Vec<crate::models::Passenger>) -> CreateTicketRequest {
        CreateTicketRequest {
            route: Box::new(route),
            selected_addons: None,
            passengers,
            code_discount: None,
            percentual_discount_ids: None,
        }
    }
}


