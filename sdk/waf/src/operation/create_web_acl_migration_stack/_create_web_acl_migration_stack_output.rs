// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateWebAclMigrationStackOutput {
    /// <p>The URL of the template created in Amazon S3. </p>
    #[doc(hidden)]
    pub s3_object_url: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateWebAclMigrationStackOutput {
    /// <p>The URL of the template created in Amazon S3. </p>
    pub fn s3_object_url(&self) -> ::std::option::Option<&str> {
        self.s3_object_url.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateWebAclMigrationStackOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateWebAclMigrationStackOutput {
    /// Creates a new builder-style object to manufacture [`CreateWebAclMigrationStackOutput`](crate::operation::create_web_acl_migration_stack::CreateWebAclMigrationStackOutput).
    pub fn builder() -> crate::operation::create_web_acl_migration_stack::builders::CreateWebAclMigrationStackOutputBuilder{
        crate::operation::create_web_acl_migration_stack::builders::CreateWebAclMigrationStackOutputBuilder::default()
    }
}

/// A builder for [`CreateWebAclMigrationStackOutput`](crate::operation::create_web_acl_migration_stack::CreateWebAclMigrationStackOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateWebAclMigrationStackOutputBuilder {
    pub(crate) s3_object_url: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateWebAclMigrationStackOutputBuilder {
    /// <p>The URL of the template created in Amazon S3. </p>
    pub fn s3_object_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.s3_object_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the template created in Amazon S3. </p>
    pub fn set_s3_object_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.s3_object_url = input;
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
    /// Consumes the builder and constructs a [`CreateWebAclMigrationStackOutput`](crate::operation::create_web_acl_migration_stack::CreateWebAclMigrationStackOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_web_acl_migration_stack::CreateWebAclMigrationStackOutput {
        crate::operation::create_web_acl_migration_stack::CreateWebAclMigrationStackOutput {
            s3_object_url: self.s3_object_url,
            _request_id: self._request_id,
        }
    }
}
