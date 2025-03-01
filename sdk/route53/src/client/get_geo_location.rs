// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetGeoLocation`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`continent_code(impl ::std::convert::Into<String>)`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::continent_code) / [`set_continent_code(Option<String>)`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::set_continent_code): <p>For geolocation resource record sets, a two-letter abbreviation that identifies a continent. Amazon Route 53 supports the following continent codes:</p>  <ul>   <li> <p> <b>AF</b>: Africa</p> </li>   <li> <p> <b>AN</b>: Antarctica</p> </li>   <li> <p> <b>AS</b>: Asia</p> </li>   <li> <p> <b>EU</b>: Europe</p> </li>   <li> <p> <b>OC</b>: Oceania</p> </li>   <li> <p> <b>NA</b>: North America</p> </li>   <li> <p> <b>SA</b>: South America</p> </li>  </ul>
    ///   - [`country_code(impl ::std::convert::Into<String>)`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::country_code) / [`set_country_code(Option<String>)`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::set_country_code): <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    ///   - [`subdivision_code(impl ::std::convert::Into<String>)`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::subdivision_code) / [`set_subdivision_code(Option<String>)`](crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::set_subdivision_code): <p>The code for the subdivision, such as a particular state within the United States. For a list of US state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. For a list of all supported subdivision codes, use the <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListGeoLocations.html">ListGeoLocations</a> API.</p>
    /// - On success, responds with [`GetGeoLocationOutput`](crate::operation::get_geo_location::GetGeoLocationOutput) with field(s):
    ///   - [`geo_location_details(Option<GeoLocationDetails>)`](crate::operation::get_geo_location::GetGeoLocationOutput::geo_location_details): <p>A complex type that contains the codes and full continent, country, and subdivision names for the specified geolocation code.</p>
    /// - On failure, responds with [`SdkError<GetGeoLocationError>`](crate::operation::get_geo_location::GetGeoLocationError)
    pub fn get_geo_location(
        &self,
    ) -> crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder {
        crate::operation::get_geo_location::builders::GetGeoLocationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
