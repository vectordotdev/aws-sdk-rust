// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetMultiRegionAccessPointRoutesInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The Multi-Region Access Point ARN.</p>
    #[doc(hidden)]
    pub mrap: ::std::option::Option<::std::string::String>,
}
impl GetMultiRegionAccessPointRoutesInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn mrap(&self) -> ::std::option::Option<&str> {
        self.mrap.as_deref()
    }
}
impl GetMultiRegionAccessPointRoutesInput {
    /// Creates a new builder-style object to manufacture [`GetMultiRegionAccessPointRoutesInput`](crate::operation::get_multi_region_access_point_routes::GetMultiRegionAccessPointRoutesInput).
    pub fn builder() -> crate::operation::get_multi_region_access_point_routes::builders::GetMultiRegionAccessPointRoutesInputBuilder{
        crate::operation::get_multi_region_access_point_routes::builders::GetMultiRegionAccessPointRoutesInputBuilder::default()
    }
}

/// A builder for [`GetMultiRegionAccessPointRoutesInput`](crate::operation::get_multi_region_access_point_routes::GetMultiRegionAccessPointRoutesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetMultiRegionAccessPointRoutesInputBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) mrap: ::std::option::Option<::std::string::String>,
}
impl GetMultiRegionAccessPointRoutesInputBuilder {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn mrap(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mrap = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn set_mrap(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mrap = input;
        self
    }
    /// Consumes the builder and constructs a [`GetMultiRegionAccessPointRoutesInput`](crate::operation::get_multi_region_access_point_routes::GetMultiRegionAccessPointRoutesInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_multi_region_access_point_routes::GetMultiRegionAccessPointRoutesInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_multi_region_access_point_routes::GetMultiRegionAccessPointRoutesInput {
                account_id: self.account_id
                ,
                mrap: self.mrap
                ,
            }
        )
    }
}
