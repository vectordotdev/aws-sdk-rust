// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a location impacted by a health event in Amazon CloudWatch Internet Monitor.</p>
/// <p>Geographic regions are hierarchically categorized into country, subdivision, metro and city geographic granularities. The geographic region is identified based on the IP address used at the client locations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImpactedLocation {
    /// <p>The name of the network at an impacted location.</p>
    #[doc(hidden)]
    pub as_name: ::std::option::Option<::std::string::String>,
    /// <p>The Autonomous System Number (ASN) of the network at an impacted location.</p>
    #[doc(hidden)]
    pub as_number: ::std::option::Option<i64>,
    /// <p>The name of the country where the health event is located.</p>
    #[doc(hidden)]
    pub country: ::std::option::Option<::std::string::String>,
    /// <p>The subdivision location where the health event is located. The subdivision usually maps to states in most countries (including the United States). For United Kingdom, it maps to a country (England, Scotland, Wales) or province (Northern Ireland).</p>
    #[doc(hidden)]
    pub subdivision: ::std::option::Option<::std::string::String>,
    /// <p>The metro area where the health event is located.</p>
    /// <p>Metro indicates a metropolitan region in the United States, such as the region around New York City. In non-US countries, this is a second-level subdivision. For example, in the United Kingdom, it could be a county, a London borough, a unitary authority, council area, and so on.</p>
    #[doc(hidden)]
    pub metro: ::std::option::Option<::std::string::String>,
    /// <p>The name of the city where the health event is located.</p>
    #[doc(hidden)]
    pub city: ::std::option::Option<::std::string::String>,
    /// <p>The latitude where the health event is located.</p>
    #[doc(hidden)]
    pub latitude: ::std::option::Option<f64>,
    /// <p>The longitude where the health event is located.</p>
    #[doc(hidden)]
    pub longitude: ::std::option::Option<f64>,
    /// <p>The country code where the health event is located. The ISO 3166-2 codes for the country is provided, when available. </p>
    #[doc(hidden)]
    pub country_code: ::std::option::Option<::std::string::String>,
    /// <p>The subdivision code where the health event is located. The ISO 3166-2 codes for country subdivisions is provided, when available. </p>
    #[doc(hidden)]
    pub subdivision_code: ::std::option::Option<::std::string::String>,
    /// <p>The service location where the health event is located.</p>
    #[doc(hidden)]
    pub service_location: ::std::option::Option<::std::string::String>,
    /// <p>The status of the health event at an impacted location.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::HealthEventStatus>,
    /// <p>The cause of the impairment. There are two types of network impairments: Amazon Web Services network issues or internet issues. Internet issues are typically a problem with a network provider, like an internet service provider (ISP).</p>
    #[doc(hidden)]
    pub caused_by: ::std::option::Option<crate::types::NetworkImpairment>,
    /// <p>The calculated health at a specific location.</p>
    #[doc(hidden)]
    pub internet_health: ::std::option::Option<crate::types::InternetHealth>,
}
impl ImpactedLocation {
    /// <p>The name of the network at an impacted location.</p>
    pub fn as_name(&self) -> ::std::option::Option<&str> {
        self.as_name.as_deref()
    }
    /// <p>The Autonomous System Number (ASN) of the network at an impacted location.</p>
    pub fn as_number(&self) -> ::std::option::Option<i64> {
        self.as_number
    }
    /// <p>The name of the country where the health event is located.</p>
    pub fn country(&self) -> ::std::option::Option<&str> {
        self.country.as_deref()
    }
    /// <p>The subdivision location where the health event is located. The subdivision usually maps to states in most countries (including the United States). For United Kingdom, it maps to a country (England, Scotland, Wales) or province (Northern Ireland).</p>
    pub fn subdivision(&self) -> ::std::option::Option<&str> {
        self.subdivision.as_deref()
    }
    /// <p>The metro area where the health event is located.</p>
    /// <p>Metro indicates a metropolitan region in the United States, such as the region around New York City. In non-US countries, this is a second-level subdivision. For example, in the United Kingdom, it could be a county, a London borough, a unitary authority, council area, and so on.</p>
    pub fn metro(&self) -> ::std::option::Option<&str> {
        self.metro.as_deref()
    }
    /// <p>The name of the city where the health event is located.</p>
    pub fn city(&self) -> ::std::option::Option<&str> {
        self.city.as_deref()
    }
    /// <p>The latitude where the health event is located.</p>
    pub fn latitude(&self) -> ::std::option::Option<f64> {
        self.latitude
    }
    /// <p>The longitude where the health event is located.</p>
    pub fn longitude(&self) -> ::std::option::Option<f64> {
        self.longitude
    }
    /// <p>The country code where the health event is located. The ISO 3166-2 codes for the country is provided, when available. </p>
    pub fn country_code(&self) -> ::std::option::Option<&str> {
        self.country_code.as_deref()
    }
    /// <p>The subdivision code where the health event is located. The ISO 3166-2 codes for country subdivisions is provided, when available. </p>
    pub fn subdivision_code(&self) -> ::std::option::Option<&str> {
        self.subdivision_code.as_deref()
    }
    /// <p>The service location where the health event is located.</p>
    pub fn service_location(&self) -> ::std::option::Option<&str> {
        self.service_location.as_deref()
    }
    /// <p>The status of the health event at an impacted location.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::HealthEventStatus> {
        self.status.as_ref()
    }
    /// <p>The cause of the impairment. There are two types of network impairments: Amazon Web Services network issues or internet issues. Internet issues are typically a problem with a network provider, like an internet service provider (ISP).</p>
    pub fn caused_by(&self) -> ::std::option::Option<&crate::types::NetworkImpairment> {
        self.caused_by.as_ref()
    }
    /// <p>The calculated health at a specific location.</p>
    pub fn internet_health(&self) -> ::std::option::Option<&crate::types::InternetHealth> {
        self.internet_health.as_ref()
    }
}
impl ImpactedLocation {
    /// Creates a new builder-style object to manufacture [`ImpactedLocation`](crate::types::ImpactedLocation).
    pub fn builder() -> crate::types::builders::ImpactedLocationBuilder {
        crate::types::builders::ImpactedLocationBuilder::default()
    }
}

/// A builder for [`ImpactedLocation`](crate::types::ImpactedLocation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImpactedLocationBuilder {
    pub(crate) as_name: ::std::option::Option<::std::string::String>,
    pub(crate) as_number: ::std::option::Option<i64>,
    pub(crate) country: ::std::option::Option<::std::string::String>,
    pub(crate) subdivision: ::std::option::Option<::std::string::String>,
    pub(crate) metro: ::std::option::Option<::std::string::String>,
    pub(crate) city: ::std::option::Option<::std::string::String>,
    pub(crate) latitude: ::std::option::Option<f64>,
    pub(crate) longitude: ::std::option::Option<f64>,
    pub(crate) country_code: ::std::option::Option<::std::string::String>,
    pub(crate) subdivision_code: ::std::option::Option<::std::string::String>,
    pub(crate) service_location: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::HealthEventStatus>,
    pub(crate) caused_by: ::std::option::Option<crate::types::NetworkImpairment>,
    pub(crate) internet_health: ::std::option::Option<crate::types::InternetHealth>,
}
impl ImpactedLocationBuilder {
    /// <p>The name of the network at an impacted location.</p>
    pub fn as_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.as_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the network at an impacted location.</p>
    pub fn set_as_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.as_name = input;
        self
    }
    /// <p>The Autonomous System Number (ASN) of the network at an impacted location.</p>
    pub fn as_number(mut self, input: i64) -> Self {
        self.as_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Autonomous System Number (ASN) of the network at an impacted location.</p>
    pub fn set_as_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.as_number = input;
        self
    }
    /// <p>The name of the country where the health event is located.</p>
    pub fn country(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.country = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the country where the health event is located.</p>
    pub fn set_country(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.country = input;
        self
    }
    /// <p>The subdivision location where the health event is located. The subdivision usually maps to states in most countries (including the United States). For United Kingdom, it maps to a country (England, Scotland, Wales) or province (Northern Ireland).</p>
    pub fn subdivision(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subdivision = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subdivision location where the health event is located. The subdivision usually maps to states in most countries (including the United States). For United Kingdom, it maps to a country (England, Scotland, Wales) or province (Northern Ireland).</p>
    pub fn set_subdivision(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subdivision = input;
        self
    }
    /// <p>The metro area where the health event is located.</p>
    /// <p>Metro indicates a metropolitan region in the United States, such as the region around New York City. In non-US countries, this is a second-level subdivision. For example, in the United Kingdom, it could be a county, a London borough, a unitary authority, council area, and so on.</p>
    pub fn metro(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metro = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The metro area where the health event is located.</p>
    /// <p>Metro indicates a metropolitan region in the United States, such as the region around New York City. In non-US countries, this is a second-level subdivision. For example, in the United Kingdom, it could be a county, a London borough, a unitary authority, council area, and so on.</p>
    pub fn set_metro(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metro = input;
        self
    }
    /// <p>The name of the city where the health event is located.</p>
    pub fn city(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.city = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the city where the health event is located.</p>
    pub fn set_city(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.city = input;
        self
    }
    /// <p>The latitude where the health event is located.</p>
    pub fn latitude(mut self, input: f64) -> Self {
        self.latitude = ::std::option::Option::Some(input);
        self
    }
    /// <p>The latitude where the health event is located.</p>
    pub fn set_latitude(mut self, input: ::std::option::Option<f64>) -> Self {
        self.latitude = input;
        self
    }
    /// <p>The longitude where the health event is located.</p>
    pub fn longitude(mut self, input: f64) -> Self {
        self.longitude = ::std::option::Option::Some(input);
        self
    }
    /// <p>The longitude where the health event is located.</p>
    pub fn set_longitude(mut self, input: ::std::option::Option<f64>) -> Self {
        self.longitude = input;
        self
    }
    /// <p>The country code where the health event is located. The ISO 3166-2 codes for the country is provided, when available. </p>
    pub fn country_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.country_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The country code where the health event is located. The ISO 3166-2 codes for the country is provided, when available. </p>
    pub fn set_country_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.country_code = input;
        self
    }
    /// <p>The subdivision code where the health event is located. The ISO 3166-2 codes for country subdivisions is provided, when available. </p>
    pub fn subdivision_code(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.subdivision_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subdivision code where the health event is located. The ISO 3166-2 codes for country subdivisions is provided, when available. </p>
    pub fn set_subdivision_code(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.subdivision_code = input;
        self
    }
    /// <p>The service location where the health event is located.</p>
    pub fn service_location(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.service_location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The service location where the health event is located.</p>
    pub fn set_service_location(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.service_location = input;
        self
    }
    /// <p>The status of the health event at an impacted location.</p>
    pub fn status(mut self, input: crate::types::HealthEventStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the health event at an impacted location.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::HealthEventStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The cause of the impairment. There are two types of network impairments: Amazon Web Services network issues or internet issues. Internet issues are typically a problem with a network provider, like an internet service provider (ISP).</p>
    pub fn caused_by(mut self, input: crate::types::NetworkImpairment) -> Self {
        self.caused_by = ::std::option::Option::Some(input);
        self
    }
    /// <p>The cause of the impairment. There are two types of network impairments: Amazon Web Services network issues or internet issues. Internet issues are typically a problem with a network provider, like an internet service provider (ISP).</p>
    pub fn set_caused_by(
        mut self,
        input: ::std::option::Option<crate::types::NetworkImpairment>,
    ) -> Self {
        self.caused_by = input;
        self
    }
    /// <p>The calculated health at a specific location.</p>
    pub fn internet_health(mut self, input: crate::types::InternetHealth) -> Self {
        self.internet_health = ::std::option::Option::Some(input);
        self
    }
    /// <p>The calculated health at a specific location.</p>
    pub fn set_internet_health(
        mut self,
        input: ::std::option::Option<crate::types::InternetHealth>,
    ) -> Self {
        self.internet_health = input;
        self
    }
    /// Consumes the builder and constructs a [`ImpactedLocation`](crate::types::ImpactedLocation).
    pub fn build(self) -> crate::types::ImpactedLocation {
        crate::types::ImpactedLocation {
            as_name: self.as_name,
            as_number: self.as_number,
            country: self.country,
            subdivision: self.subdivision,
            metro: self.metro,
            city: self.city,
            latitude: self.latitude,
            longitude: self.longitude,
            country_code: self.country_code,
            subdivision_code: self.subdivision_code,
            service_location: self.service_location,
            status: self.status,
            caused_by: self.caused_by,
            internet_health: self.internet_health,
        }
    }
}
