// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request to get an origin access identity's information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCloudFrontOriginAccessIdentityInput {
    /// <p>The identity's ID.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetCloudFrontOriginAccessIdentityInput {
    /// <p>The identity's ID.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetCloudFrontOriginAccessIdentityInput {
    /// Creates a new builder-style object to manufacture [`GetCloudFrontOriginAccessIdentityInput`](crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityInput).
    pub fn builder() -> crate::operation::get_cloud_front_origin_access_identity::builders::GetCloudFrontOriginAccessIdentityInputBuilder{
        crate::operation::get_cloud_front_origin_access_identity::builders::GetCloudFrontOriginAccessIdentityInputBuilder::default()
    }
}

/// A builder for [`GetCloudFrontOriginAccessIdentityInput`](crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCloudFrontOriginAccessIdentityInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetCloudFrontOriginAccessIdentityInputBuilder {
    /// <p>The identity's ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identity's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetCloudFrontOriginAccessIdentityInput`](crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityInput {
                id: self.id
                ,
            }
        )
    }
}
