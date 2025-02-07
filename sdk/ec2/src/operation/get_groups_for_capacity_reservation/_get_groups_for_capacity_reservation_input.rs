// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetGroupsForCapacityReservationInput {
    /// <p>The ID of the Capacity Reservation. If you specify a Capacity Reservation that is shared with you, the operation returns only Capacity Reservation groups that you own.</p>
    #[doc(hidden)]
    pub capacity_reservation_id: ::std::option::Option<::std::string::String>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl GetGroupsForCapacityReservationInput {
    /// <p>The ID of the Capacity Reservation. If you specify a Capacity Reservation that is shared with you, the operation returns only Capacity Reservation groups that you own.</p>
    pub fn capacity_reservation_id(&self) -> ::std::option::Option<&str> {
        self.capacity_reservation_id.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl GetGroupsForCapacityReservationInput {
    /// Creates a new builder-style object to manufacture [`GetGroupsForCapacityReservationInput`](crate::operation::get_groups_for_capacity_reservation::GetGroupsForCapacityReservationInput).
    pub fn builder() -> crate::operation::get_groups_for_capacity_reservation::builders::GetGroupsForCapacityReservationInputBuilder{
        crate::operation::get_groups_for_capacity_reservation::builders::GetGroupsForCapacityReservationInputBuilder::default()
    }
}

/// A builder for [`GetGroupsForCapacityReservationInput`](crate::operation::get_groups_for_capacity_reservation::GetGroupsForCapacityReservationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetGroupsForCapacityReservationInputBuilder {
    pub(crate) capacity_reservation_id: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl GetGroupsForCapacityReservationInputBuilder {
    /// <p>The ID of the Capacity Reservation. If you specify a Capacity Reservation that is shared with you, the operation returns only Capacity Reservation groups that you own.</p>
    pub fn capacity_reservation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.capacity_reservation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation. If you specify a Capacity Reservation that is shared with you, the operation returns only Capacity Reservation groups that you own.</p>
    pub fn set_capacity_reservation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.capacity_reservation_id = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`GetGroupsForCapacityReservationInput`](crate::operation::get_groups_for_capacity_reservation::GetGroupsForCapacityReservationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_groups_for_capacity_reservation::GetGroupsForCapacityReservationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_groups_for_capacity_reservation::GetGroupsForCapacityReservationInput {
                capacity_reservation_id: self.capacity_reservation_id
                ,
                next_token: self.next_token
                ,
                max_results: self.max_results
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
