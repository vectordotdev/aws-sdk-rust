// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchUpdatePhoneNumberInput {
    /// <p>Lists the phone numbers in the update request.</p>
    #[doc(hidden)]
    pub update_phone_number_request_items:
        ::std::option::Option<::std::vec::Vec<crate::types::UpdatePhoneNumberRequestItem>>,
}
impl BatchUpdatePhoneNumberInput {
    /// <p>Lists the phone numbers in the update request.</p>
    pub fn update_phone_number_request_items(
        &self,
    ) -> ::std::option::Option<&[crate::types::UpdatePhoneNumberRequestItem]> {
        self.update_phone_number_request_items.as_deref()
    }
}
impl BatchUpdatePhoneNumberInput {
    /// Creates a new builder-style object to manufacture [`BatchUpdatePhoneNumberInput`](crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberInput).
    pub fn builder(
    ) -> crate::operation::batch_update_phone_number::builders::BatchUpdatePhoneNumberInputBuilder
    {
        crate::operation::batch_update_phone_number::builders::BatchUpdatePhoneNumberInputBuilder::default()
    }
}

/// A builder for [`BatchUpdatePhoneNumberInput`](crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchUpdatePhoneNumberInputBuilder {
    pub(crate) update_phone_number_request_items:
        ::std::option::Option<::std::vec::Vec<crate::types::UpdatePhoneNumberRequestItem>>,
}
impl BatchUpdatePhoneNumberInputBuilder {
    /// Appends an item to `update_phone_number_request_items`.
    ///
    /// To override the contents of this collection use [`set_update_phone_number_request_items`](Self::set_update_phone_number_request_items).
    ///
    /// <p>Lists the phone numbers in the update request.</p>
    pub fn update_phone_number_request_items(
        mut self,
        input: crate::types::UpdatePhoneNumberRequestItem,
    ) -> Self {
        let mut v = self.update_phone_number_request_items.unwrap_or_default();
        v.push(input);
        self.update_phone_number_request_items = ::std::option::Option::Some(v);
        self
    }
    /// <p>Lists the phone numbers in the update request.</p>
    pub fn set_update_phone_number_request_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UpdatePhoneNumberRequestItem>>,
    ) -> Self {
        self.update_phone_number_request_items = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchUpdatePhoneNumberInput`](crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberInput {
                update_phone_number_request_items: self.update_phone_number_request_items,
            },
        )
    }
}
