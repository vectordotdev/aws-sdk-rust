// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCapacityReservationsInput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the maximum number of results to return.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListCapacityReservationsInput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Specifies the maximum number of results to return.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListCapacityReservationsInput {
    /// Creates a new builder-style object to manufacture [`ListCapacityReservationsInput`](crate::operation::list_capacity_reservations::ListCapacityReservationsInput).
    pub fn builder(
    ) -> crate::operation::list_capacity_reservations::builders::ListCapacityReservationsInputBuilder
    {
        crate::operation::list_capacity_reservations::builders::ListCapacityReservationsInputBuilder::default()
    }
}

/// A builder for [`ListCapacityReservationsInput`](crate::operation::list_capacity_reservations::ListCapacityReservationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCapacityReservationsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListCapacityReservationsInputBuilder {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Specifies the maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListCapacityReservationsInput`](crate::operation::list_capacity_reservations::ListCapacityReservationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_capacity_reservations::ListCapacityReservationsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_capacity_reservations::ListCapacityReservationsInput {
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
