# Rust API client for openapi

A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.1.0
- Package version: 1.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://dpl-qa-ybus-privapi.sa.cz/restapi*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AddonsApi* | [**get_available_addons**](docs/AddonsApi.md#get_available_addons) | **POST** /addons | Get possible addons for given route.
*AddonsApi* | [**order_addons**](docs/AddonsApi.md#order_addons) | **PUT** /addons | Order tickets addons
*AddonsApi* | [**verify_addons**](docs/AddonsApi.md#verify_addons) | **POST** /addons/verify | Verify validity of addons selection
*ConstsApi* | [**consts_translations_language_get**](docs/ConstsApi.md#consts_translations_language_get) | **GET** /consts/translations/{language} | Get the used dictionary of the selected language
*ConstsApi* | [**get_action_prices**](docs/ConstsApi.md#get_action_prices) | **GET** /consts/actionPrices | Returns list of Action Prices
*ConstsApi* | [**get_carriers**](docs/ConstsApi.md#get_carriers) | **GET** /consts/carriers | Returns List of Carriers
*ConstsApi* | [**get_city_pairs**](docs/ConstsApi.md#get_city_pairs) | **GET** /consts/cityPairs | Returns List of possible CityPairs
*ConstsApi* | [**get_locations**](docs/ConstsApi.md#get_locations) | **GET** /consts/locations | List all possible locations served by RegioJet transportation services
*ConstsApi* | [**get_online_paymenth_methods**](docs/ConstsApi.md#get_online_paymenth_methods) | **GET** /consts/paymentMethods | Get the list of all payment methods
*ConstsApi* | [**get_seat_classes**](docs/ConstsApi.md#get_seat_classes) | **GET** /consts/seatClasses | Get all possible seat classes
*ConstsApi* | [**get_tariffs**](docs/ConstsApi.md#get_tariffs) | **GET** /consts/tariffs | Get all possible tariffs
*ConstsApi* | [**get_timetables**](docs/ConstsApi.md#get_timetables) | **GET** /consts/timetables | Get timetables of all connections or of a single one
*ConstsApi* | [**get_vehicle_standards**](docs/ConstsApi.md#get_vehicle_standards) | **GET** /consts/vehicleStandards | Get all possible vehicle standards
*ContentApi* | [**content_layout**](docs/ContentApi.md#content_layout) | **GET** /content/layout | 
*ContentApi* | [**content_news**](docs/ContentApi.md#content_news) | **GET** /content/news | 
*DiscountsApi* | [**get_percentual_by_user**](docs/DiscountsApi.md#get_percentual_by_user) | **GET** /discounts/percentual | Get possible discounts for given account.
*DiscountsApi* | [**verify_code_discount**](docs/DiscountsApi.md#verify_code_discount) | **POST** /discounts/code/{code}/verify | Verify code discount compatibility with a given route
*DiscountsApi* | [**verify_percentual_discount**](docs/DiscountsApi.md#verify_percentual_discount) | **POST** /discounts/percentual/{discountId}/verify | Verify percentual discount compatibility with a given route
*OthersApi* | [**send_conta_form**](docs/OthersApi.md#send_conta_form) | **POST** /support/sendContactForm | 
*PaymentsApi* | [**get_payment_form**](docs/PaymentsApi.md#get_payment_form) | **POST** /payments/form | 
*PaymentsApi* | [**get_payments**](docs/PaymentsApi.md#get_payments) | **GET** /payments | 
*PaymentsApi* | [**get_payments_methods**](docs/PaymentsApi.md#get_payments_methods) | **POST** /payments/methods | 
*PaymentsApi* | [**pay_ticket_by_affiliate**](docs/PaymentsApi.md#pay_ticket_by_affiliate) | **PUT** /payments/{ticketId}/pay | 
*PaymentsApi* | [**payments_credit_add**](docs/PaymentsApi.md#payments_credit_add) | **POST** /payments/credit/add | 
*PaymentsApi* | [**payments_credit_charge**](docs/PaymentsApi.md#payments_credit_charge) | **POST** /payments/credit/charge | 
*PaymentsApi* | [**payments_credit_gift_certificate_add**](docs/PaymentsApi.md#payments_credit_gift_certificate_add) | **POST** /payments/credit/giftCertificate/add | 
*PaymentsApi* | [**payments_pay**](docs/PaymentsApi.md#payments_pay) | **POST** /payments/pay | 
*PaymentsApi* | [**print_bulk_invoice**](docs/PaymentsApi.md#print_bulk_invoice) | **POST** /payments/print/invoice | 
*PaymentsApi* | [**print_invoice**](docs/PaymentsApi.md#print_invoice) | **GET** /payments/{paymentId}/print/invoice | Print payment invoice
*PaymentsApi* | [**print_receipt**](docs/PaymentsApi.md#print_receipt) | **GET** /payments/{paymentId}/print/receipt | Print payment receipt
*PaymentsApi* | [**verify_gift_certificate**](docs/PaymentsApi.md#verify_gift_certificate) | **POST** /payments/credit/giftCertificate/verify | 
*PriceListApi* | [**get_time_ticket_price**](docs/PriceListApi.md#get_time_ticket_price) | **POST** /pricelists/timeticket/{fromStationId}/{toStationId}/{timeTicketType}/{validFrom}/{tariff} | Get timeTicket prices for tariff, ticket type, stationFrom, stationTo, dateTime and currency.
*RoutesApi* | [**get_departures**](docs/RoutesApi.md#get_departures) | **GET** /routes/{stationId}/departures | Get arrivals and departures for the given station.
*RoutesApi* | [**get_passengers_data**](docs/RoutesApi.md#get_passengers_data) | **POST** /routes/{routeId}/passengersData | Get mandatory data of  first passenger and others passengers for the given route.
*RoutesApi* | [**get_route_free_seats**](docs/RoutesApi.md#get_route_free_seats) | **POST** /routes/{routeId}/freeSeats | Get route tandem free seats group by vehicle
*RoutesApi* | [**get_simple_route_detail**](docs/RoutesApi.md#get_simple_route_detail) | **GET** /routes/{routeId}/simple | Get detail for the given route.
*RoutesApi* | [**search_routes**](docs/RoutesApi.md#search_routes) | **GET** /routes/search | Get collection of all routes that satisfy specified search criteria for the route.
*RoutesApi* | [**simple_search_routes**](docs/RoutesApi.md#simple_search_routes) | **GET** /routes/search/simple | Get collection of all routes that satisfy specified search criteria for the route.
*TicketsApi* | [**cancel_ticket**](docs/TicketsApi.md#cancel_ticket) | **PUT** /tickets/{ticketId}/cancel | Delete ticket by ID
*TicketsApi* | [**cancel_ticket_by_affiliate**](docs/TicketsApi.md#cancel_ticket_by_affiliate) | **PUT** /tickets/{accountCode}/{ticketId}/cancel/affiliate | Delete ticket by ID for affiliate partners
*TicketsApi* | [**create_registered_tickets**](docs/TicketsApi.md#create_registered_tickets) | **POST** /tickets/create/registered | Create new ticket(s) for an account
*TicketsApi* | [**create_tickets_by_affiliate**](docs/TicketsApi.md#create_tickets_by_affiliate) | **POST** /tickets/create/affiliate | Create new ticket(s) for new account as affiliate partner
*TicketsApi* | [**create_unregistered_tickets**](docs/TicketsApi.md#create_unregistered_tickets) | **POST** /tickets/create/unregistered | Create new ticket(s) for new account
*TicketsApi* | [**delete_passenger**](docs/TicketsApi.md#delete_passenger) | **DELETE** /tickets/{ticketId}/passengers/{passengerId} | Delete passenger from the ticket
*TicketsApi* | [**get_all_tickets**](docs/TicketsApi.md#get_all_tickets) | **GET** /tickets | Get all tickets of the user
*TicketsApi* | [**get_all_tickets_for_affiliate**](docs/TicketsApi.md#get_all_tickets_for_affiliate) | **GET** /tickets/{accountCode}/affiliate | Get all tickets of the user for affiliate partner
*TicketsApi* | [**get_ticket_by_id**](docs/TicketsApi.md#get_ticket_by_id) | **GET** /tickets/{ticketId} | Get ticket by ID
*TicketsApi* | [**get_ticket_by_idfor_affiliate**](docs/TicketsApi.md#get_ticket_by_idfor_affiliate) | **GET** /tickets/{accountCode}/{ticketId}/affiliate | Get ticket by ID for affiliate partner
*TicketsApi* | [**get_ticket_detail_rating**](docs/TicketsApi.md#get_ticket_detail_rating) | **GET** /tickets/{ticketId}/rating | Get ticket rating questions
*TicketsApi* | [**get_ticket_qr_code**](docs/TicketsApi.md#get_ticket_qr_code) | **GET** /tickets/{ticketId}/qrcode | Get GR code for ticket
*TicketsApi* | [**get_ticket_qr_code_png**](docs/TicketsApi.md#get_ticket_qr_code_png) | **GET** /tickets/{ticketId}/qrcode/png | GR code for ticket
*TicketsApi* | [**get_unpaid_tickets**](docs/TicketsApi.md#get_unpaid_tickets) | **GET** /tickets/unpaid | Get unpaid tickets (and tickets with remaining items to pay) of the user.
*TicketsApi* | [**print_ticket**](docs/TicketsApi.md#print_ticket) | **GET** /tickets/{ticketId}/print | Print ticket
*TicketsApi* | [**put_ticket_detail_rating**](docs/TicketsApi.md#put_ticket_detail_rating) | **PUT** /tickets/{ticketId}/rating | Update ticket rating
*TicketsApi* | [**send_ticket_by_email**](docs/TicketsApi.md#send_ticket_by_email) | **POST** /tickets/{ticketId}/sendByEmail | Send ticket to email
*TicketsApi* | [**update_passenger**](docs/TicketsApi.md#update_passenger) | **PUT** /tickets/{ticketId}/passengers/{passengerId} | Update passenger on the ticket
*TimeTicketsApi* | [**cancel_time_ticket**](docs/TimeTicketsApi.md#cancel_time_ticket) | **PUT** /tickets/timetickets/{timeTicketId}/cancel | Delete time ticket by ID
*TimeTicketsApi* | [**get_time_ticket_by_id**](docs/TimeTicketsApi.md#get_time_ticket_by_id) | **GET** /tickets/timetickets/{timeTicketId} | Get time ticket by ID
*TimeTicketsApi* | [**time_ticket_check_in**](docs/TimeTicketsApi.md#time_ticket_check_in) | **PUT** /tickets/timetickets/checkin/{timeTicketId} | Verify time ticket for connection.
*UsersApi* | [**authenticate**](docs/UsersApi.md#authenticate) | **GET** /users/authenticate | 
*UsersApi* | [**change_user_information**](docs/UsersApi.md#change_user_information) | **PUT** /users/settings/changeUserInformation | 
*UsersApi* | [**change_user_password**](docs/UsersApi.md#change_user_password) | **PUT** /users/settings/changePassword | 
*UsersApi* | [**forgotten_password**](docs/UsersApi.md#forgotten_password) | **POST** /users/forgottenPassword | 
*UsersApi* | [**login_moje_id_account**](docs/UsersApi.md#login_moje_id_account) | **POST** /users/login/mojeId | 
*UsersApi* | [**login_registered_account**](docs/UsersApi.md#login_registered_account) | **POST** /users/login/registeredAccount | 
*UsersApi* | [**login_unregistered_account**](docs/UsersApi.md#login_unregistered_account) | **POST** /users/login/unregisteredAccount | 
*UsersApi* | [**reset_password**](docs/UsersApi.md#reset_password) | **POST** /users/resetPassword | 
*UsersApi* | [**signup_registered_account**](docs/UsersApi.md#signup_registered_account) | **POST** /users/signup/registeredAccount | 
*UsersApi* | [**signup_simple_registered_account**](docs/UsersApi.md#signup_simple_registered_account) | **POST** /users/signup/registeredAccount/simple | 
*UsersApi* | [**user_logout**](docs/UsersApi.md#user_logout) | **POST** /users/logout | 
*UsersApi* | [**user_settings_change_mojeid**](docs/UsersApi.md#user_settings_change_mojeid) | **PUT** /users/settings/changeMojeid | 
*UsersApi* | [**verify_reset_password_token**](docs/UsersApi.md#verify_reset_password_token) | **GET** /users/resetPassword/verify | 


## Documentation For Models

 - [ActionPrice](docs/ActionPrice.md)
 - [AddGiftCertificateRequest](docs/AddGiftCertificateRequest.md)
 - [AddonConditionsDescriptions](docs/AddonConditionsDescriptions.md)
 - [AddonsVerifyRequest](docs/AddonsVerifyRequest.md)
 - [ArrivalDepartureConnection](docs/ArrivalDepartureConnection.md)
 - [ArrivalDepartureConnectionStation](docs/ArrivalDepartureConnectionStation.md)
 - [AvailableAddon](docs/AvailableAddon.md)
 - [AvailableAddonConditions](docs/AvailableAddonConditions.md)
 - [AvailableAddonsRequest](docs/AvailableAddonsRequest.md)
 - [BannerBubble](docs/BannerBubble.md)
 - [CancelCharge](docs/CancelCharge.md)
 - [CancelTicketRequest](docs/CancelTicketRequest.md)
 - [Carrier](docs/Carrier.md)
 - [ChangeMojeid](docs/ChangeMojeid.md)
 - [ChangePasswordRequest](docs/ChangePasswordRequest.md)
 - [ChangeUserRequest](docs/ChangeUserRequest.md)
 - [ChargeRequest](docs/ChargeRequest.md)
 - [ChargeResponse](docs/ChargeResponse.md)
 - [City](docs/City.md)
 - [CityPair](docs/CityPair.md)
 - [CodeDiscount](docs/CodeDiscount.md)
 - [Company](docs/Company.md)
 - [ContactFormRequest](docs/ContactFormRequest.md)
 - [Country](docs/Country.md)
 - [CreateRegisteredTicketRequest](docs/CreateRegisteredTicketRequest.md)
 - [CreateTicketRequest](docs/CreateTicketRequest.md)
 - [CreateTicketResponseRegistered](docs/CreateTicketResponseRegistered.md)
 - [CreateTicketResponseUnregistered](docs/CreateTicketResponseUnregistered.md)
 - [CreateTicketRouteRequest](docs/CreateTicketRouteRequest.md)
 - [CreateTicketSectionRequest](docs/CreateTicketSectionRequest.md)
 - [CreateUnregisteredTicketRequest](docs/CreateUnregisteredTicketRequest.md)
 - [CreditAddRequest](docs/CreditAddRequest.md)
 - [CreditAddResponse](docs/CreditAddResponse.md)
 - [Currency](docs/Currency.md)
 - [CustomerActions](docs/CustomerActions.md)
 - [DeletePassengerRequest](docs/DeletePassengerRequest.md)
 - [Discount](docs/Discount.md)
 - [DiscountState](docs/DiscountState.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ForgottenPasswordRequest](docs/ForgottenPasswordRequest.md)
 - [FormAnswer](docs/FormAnswer.md)
 - [FormQuestion](docs/FormQuestion.md)
 - [FormQuestionType](docs/FormQuestionType.md)
 - [GiftCertificateInfo](docs/GiftCertificateInfo.md)
 - [GiftCertificateState](docs/GiftCertificateState.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [Layout](docs/Layout.md)
 - [LayoutFooter](docs/LayoutFooter.md)
 - [LayoutFooterFormUrl](docs/LayoutFooterFormUrl.md)
 - [LayoutFooterPaymentMethods](docs/LayoutFooterPaymentMethods.md)
 - [LayoutHeader](docs/LayoutHeader.md)
 - [Line](docs/Line.md)
 - [MessageField](docs/MessageField.md)
 - [MojeIdLogin](docs/MojeIdLogin.md)
 - [News](docs/News.md)
 - [NewsDiscounts](docs/NewsDiscounts.md)
 - [NewsNews](docs/NewsNews.md)
 - [NewsPromotions](docs/NewsPromotions.md)
 - [Notifications](docs/Notifications.md)
 - [OrderAddonsRequest](docs/OrderAddonsRequest.md)
 - [OrderedAddon](docs/OrderedAddon.md)
 - [Passenger](docs/Passenger.md)
 - [PassengerChangeResponse](docs/PassengerChangeResponse.md)
 - [PassengerRequest](docs/PassengerRequest.md)
 - [PassengersDataRequest](docs/PassengersDataRequest.md)
 - [PassengersDataResponse](docs/PassengersDataResponse.md)
 - [PassengersInfo](docs/PassengersInfo.md)
 - [PayRequest](docs/PayRequest.md)
 - [Payment](docs/Payment.md)
 - [PaymentFormField](docs/PaymentFormField.md)
 - [PaymentFormFieldRequest](docs/PaymentFormFieldRequest.md)
 - [PaymentFormFieldType](docs/PaymentFormFieldType.md)
 - [PaymentFormFieldValue](docs/PaymentFormFieldValue.md)
 - [PaymentMethod](docs/PaymentMethod.md)
 - [PaymentTicketRequest](docs/PaymentTicketRequest.md)
 - [PaymentTicketType](docs/PaymentTicketType.md)
 - [PaymentType](docs/PaymentType.md)
 - [PaymentsMethodsRequest](docs/PaymentsMethodsRequest.md)
 - [PercentualDiscount](docs/PercentualDiscount.md)
 - [PersonalDataType](docs/PersonalDataType.md)
 - [PriceClass](docs/PriceClass.md)
 - [PriceConditions](docs/PriceConditions.md)
 - [PriceConditionsDescriptions](docs/PriceConditionsDescriptions.md)
 - [PrintInvoicesRequest](docs/PrintInvoicesRequest.md)
 - [PutRatingRequest](docs/PutRatingRequest.md)
 - [QrCodeTicket](docs/QrCodeTicket.md)
 - [QrCodeTicketType](docs/QrCodeTicketType.md)
 - [RatingAnsweredFormData](docs/RatingAnsweredFormData.md)
 - [RatingAnsweredFormField](docs/RatingAnsweredFormField.md)
 - [RatingFormData](docs/RatingFormData.md)
 - [RegisteredLogin](docs/RegisteredLogin.md)
 - [ResetPasswordRequest](docs/ResetPasswordRequest.md)
 - [Route](docs/Route.md)
 - [RouteSeatsRequest](docs/RouteSeatsRequest.md)
 - [RouteSeatsResponse](docs/RouteSeatsResponse.md)
 - [RouteSectionRequest](docs/RouteSectionRequest.md)
 - [SearchResult](docs/SearchResult.md)
 - [Seat](docs/Seat.md)
 - [SeatClass](docs/SeatClass.md)
 - [Section](docs/Section.md)
 - [SectionPutRatingRequest](docs/SectionPutRatingRequest.md)
 - [SelectedAddon](docs/SelectedAddon.md)
 - [SelectedSeat](docs/SelectedSeat.md)
 - [SignupRegisteredAccountRequest](docs/SignupRegisteredAccountRequest.md)
 - [SimpleRegisteredAccountRequest](docs/SimpleRegisteredAccountRequest.md)
 - [SimpleRoute](docs/SimpleRoute.md)
 - [SimpleSearchResult](docs/SimpleSearchResult.md)
 - [SimpleSection](docs/SimpleSection.md)
 - [Station](docs/Station.md)
 - [SuccessResponse](docs/SuccessResponse.md)
 - [Surcharge](docs/Surcharge.md)
 - [Tariff](docs/Tariff.md)
 - [TariffNotifications](docs/TariffNotifications.md)
 - [TextBubble](docs/TextBubble.md)
 - [Ticket](docs/Ticket.md)
 - [TicketBill](docs/TicketBill.md)
 - [TicketEmailRequest](docs/TicketEmailRequest.md)
 - [TicketSection](docs/TicketSection.md)
 - [TicketState](docs/TicketState.md)
 - [TimePeriod](docs/TimePeriod.md)
 - [TimeTicket](docs/TimeTicket.md)
 - [TimeTicketBill](docs/TimeTicketBill.md)
 - [TimeTicketCheckinInfo](docs/TimeTicketCheckinInfo.md)
 - [TimeTicketCustomerAction](docs/TimeTicketCustomerAction.md)
 - [TimeTicketPrice](docs/TimeTicketPrice.md)
 - [TimeTicketType](docs/TimeTicketType.md)
 - [TimeTicketsCheckinRequest](docs/TimeTicketsCheckinRequest.md)
 - [Timetable](docs/Timetable.md)
 - [TimetableStation](docs/TimetableStation.md)
 - [Token](docs/Token.md)
 - [TransactionMethod](docs/TransactionMethod.md)
 - [Transfer](docs/Transfer.md)
 - [TransferType](docs/TransferType.md)
 - [TransfersInfo](docs/TransfersInfo.md)
 - [UnregisteredLogin](docs/UnregisteredLogin.md)
 - [User](docs/User.md)
 - [Vehicle](docs/Vehicle.md)
 - [VehicleStandard](docs/VehicleStandard.md)
 - [VehicleType](docs/VehicleType.md)
 - [VerifiedDiscountResponse](docs/VerifiedDiscountResponse.md)
 - [VerifyDiscountRequest](docs/VerifyDiscountRequest.md)
 - [VerifyGiftCertificateRequest](docs/VerifyGiftCertificateRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

developers@studentagency.cz

