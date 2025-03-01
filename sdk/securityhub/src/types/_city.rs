// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a city.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct City {
    /// <p>The name of the city.</p>
    #[doc(hidden)]
    pub city_name: ::std::option::Option<::std::string::String>,
}
impl City {
    /// <p>The name of the city.</p>
    pub fn city_name(&self) -> ::std::option::Option<&str> {
        self.city_name.as_deref()
    }
}
impl City {
    /// Creates a new builder-style object to manufacture [`City`](crate::types::City).
    pub fn builder() -> crate::types::builders::CityBuilder {
        crate::types::builders::CityBuilder::default()
    }
}

/// A builder for [`City`](crate::types::City).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CityBuilder {
    pub(crate) city_name: ::std::option::Option<::std::string::String>,
}
impl CityBuilder {
    /// <p>The name of the city.</p>
    pub fn city_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.city_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the city.</p>
    pub fn set_city_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.city_name = input;
        self
    }
    /// Consumes the builder and constructs a [`City`](crate::types::City).
    pub fn build(self) -> crate::types::City {
        crate::types::City {
            city_name: self.city_name,
        }
    }
}
