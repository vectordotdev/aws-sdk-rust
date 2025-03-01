// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReleasePhoneNumberInput {
    /// <p>A unique identifier for the phone number.</p>
    #[doc(hidden)]
    pub phone_number_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
}
impl ReleasePhoneNumberInput {
    /// <p>A unique identifier for the phone number.</p>
    pub fn phone_number_id(&self) -> ::std::option::Option<&str> {
        self.phone_number_id.as_deref()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl ReleasePhoneNumberInput {
    /// Creates a new builder-style object to manufacture [`ReleasePhoneNumberInput`](crate::operation::release_phone_number::ReleasePhoneNumberInput).
    pub fn builder(
    ) -> crate::operation::release_phone_number::builders::ReleasePhoneNumberInputBuilder {
        crate::operation::release_phone_number::builders::ReleasePhoneNumberInputBuilder::default()
    }
}

/// A builder for [`ReleasePhoneNumberInput`](crate::operation::release_phone_number::ReleasePhoneNumberInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReleasePhoneNumberInputBuilder {
    pub(crate) phone_number_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
}
impl ReleasePhoneNumberInputBuilder {
    /// <p>A unique identifier for the phone number.</p>
    pub fn phone_number_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.phone_number_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn set_phone_number_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.phone_number_id = input;
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ReleasePhoneNumberInput`](crate::operation::release_phone_number::ReleasePhoneNumberInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::release_phone_number::ReleasePhoneNumberInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::release_phone_number::ReleasePhoneNumberInput {
                phone_number_id: self.phone_number_id,
                client_token: self.client_token,
            },
        )
    }
}
