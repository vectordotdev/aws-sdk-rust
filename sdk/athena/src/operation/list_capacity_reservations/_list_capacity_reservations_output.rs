// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCapacityReservationsOutput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The capacity reservations for the current account.</p>
    #[doc(hidden)]
    pub capacity_reservations:
        ::std::option::Option<::std::vec::Vec<crate::types::CapacityReservation>>,
    _request_id: Option<String>,
}
impl ListCapacityReservationsOutput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The capacity reservations for the current account.</p>
    pub fn capacity_reservations(
        &self,
    ) -> ::std::option::Option<&[crate::types::CapacityReservation]> {
        self.capacity_reservations.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListCapacityReservationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCapacityReservationsOutput {
    /// Creates a new builder-style object to manufacture [`ListCapacityReservationsOutput`](crate::operation::list_capacity_reservations::ListCapacityReservationsOutput).
    pub fn builder(
    ) -> crate::operation::list_capacity_reservations::builders::ListCapacityReservationsOutputBuilder
    {
        crate::operation::list_capacity_reservations::builders::ListCapacityReservationsOutputBuilder::default()
    }
}

/// A builder for [`ListCapacityReservationsOutput`](crate::operation::list_capacity_reservations::ListCapacityReservationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCapacityReservationsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) capacity_reservations:
        ::std::option::Option<::std::vec::Vec<crate::types::CapacityReservation>>,
    _request_id: Option<String>,
}
impl ListCapacityReservationsOutputBuilder {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `capacity_reservations`.
    ///
    /// To override the contents of this collection use [`set_capacity_reservations`](Self::set_capacity_reservations).
    ///
    /// <p>The capacity reservations for the current account.</p>
    pub fn capacity_reservations(mut self, input: crate::types::CapacityReservation) -> Self {
        let mut v = self.capacity_reservations.unwrap_or_default();
        v.push(input);
        self.capacity_reservations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The capacity reservations for the current account.</p>
    pub fn set_capacity_reservations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CapacityReservation>>,
    ) -> Self {
        self.capacity_reservations = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListCapacityReservationsOutput`](crate::operation::list_capacity_reservations::ListCapacityReservationsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_capacity_reservations::ListCapacityReservationsOutput {
        crate::operation::list_capacity_reservations::ListCapacityReservationsOutput {
            next_token: self.next_token,
            capacity_reservations: self.capacity_reservations,
            _request_id: self._request_id,
        }
    }
}
