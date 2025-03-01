// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An HTTP error resulting from creating a vehicle.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVehicleError {
    /// <p>The ID of the vehicle with the error.</p>
    #[doc(hidden)]
    pub vehicle_name: ::std::option::Option<::std::string::String>,
    /// <p>An HTTP error code.</p>
    #[doc(hidden)]
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>A description of the HTTP error.</p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
}
impl CreateVehicleError {
    /// <p>The ID of the vehicle with the error.</p>
    pub fn vehicle_name(&self) -> ::std::option::Option<&str> {
        self.vehicle_name.as_deref()
    }
    /// <p>An HTTP error code.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>A description of the HTTP error.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl CreateVehicleError {
    /// Creates a new builder-style object to manufacture [`CreateVehicleError`](crate::types::CreateVehicleError).
    pub fn builder() -> crate::types::builders::CreateVehicleErrorBuilder {
        crate::types::builders::CreateVehicleErrorBuilder::default()
    }
}

/// A builder for [`CreateVehicleError`](crate::types::CreateVehicleError).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVehicleErrorBuilder {
    pub(crate) vehicle_name: ::std::option::Option<::std::string::String>,
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl CreateVehicleErrorBuilder {
    /// <p>The ID of the vehicle with the error.</p>
    pub fn vehicle_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vehicle_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the vehicle with the error.</p>
    pub fn set_vehicle_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vehicle_name = input;
        self
    }
    /// <p>An HTTP error code.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An HTTP error code.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>A description of the HTTP error.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the HTTP error.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVehicleError`](crate::types::CreateVehicleError).
    pub fn build(self) -> crate::types::CreateVehicleError {
        crate::types::CreateVehicleError {
            vehicle_name: self.vehicle_name,
            code: self.code,
            message: self.message,
        }
    }
}
