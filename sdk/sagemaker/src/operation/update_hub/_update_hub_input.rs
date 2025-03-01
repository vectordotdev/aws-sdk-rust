// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateHubInput {
    /// <p>The name of the hub to update.</p>
    #[doc(hidden)]
    pub hub_name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the updated hub.</p>
    #[doc(hidden)]
    pub hub_description: ::std::option::Option<::std::string::String>,
    /// <p>The display name of the hub.</p>
    #[doc(hidden)]
    pub hub_display_name: ::std::option::Option<::std::string::String>,
    /// <p>The searchable keywords for the hub.</p>
    #[doc(hidden)]
    pub hub_search_keywords: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UpdateHubInput {
    /// <p>The name of the hub to update.</p>
    pub fn hub_name(&self) -> ::std::option::Option<&str> {
        self.hub_name.as_deref()
    }
    /// <p>A description of the updated hub.</p>
    pub fn hub_description(&self) -> ::std::option::Option<&str> {
        self.hub_description.as_deref()
    }
    /// <p>The display name of the hub.</p>
    pub fn hub_display_name(&self) -> ::std::option::Option<&str> {
        self.hub_display_name.as_deref()
    }
    /// <p>The searchable keywords for the hub.</p>
    pub fn hub_search_keywords(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.hub_search_keywords.as_deref()
    }
}
impl UpdateHubInput {
    /// Creates a new builder-style object to manufacture [`UpdateHubInput`](crate::operation::update_hub::UpdateHubInput).
    pub fn builder() -> crate::operation::update_hub::builders::UpdateHubInputBuilder {
        crate::operation::update_hub::builders::UpdateHubInputBuilder::default()
    }
}

/// A builder for [`UpdateHubInput`](crate::operation::update_hub::UpdateHubInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateHubInputBuilder {
    pub(crate) hub_name: ::std::option::Option<::std::string::String>,
    pub(crate) hub_description: ::std::option::Option<::std::string::String>,
    pub(crate) hub_display_name: ::std::option::Option<::std::string::String>,
    pub(crate) hub_search_keywords: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UpdateHubInputBuilder {
    /// <p>The name of the hub to update.</p>
    pub fn hub_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.hub_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the hub to update.</p>
    pub fn set_hub_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.hub_name = input;
        self
    }
    /// <p>A description of the updated hub.</p>
    pub fn hub_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.hub_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the updated hub.</p>
    pub fn set_hub_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.hub_description = input;
        self
    }
    /// <p>The display name of the hub.</p>
    pub fn hub_display_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.hub_display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of the hub.</p>
    pub fn set_hub_display_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.hub_display_name = input;
        self
    }
    /// Appends an item to `hub_search_keywords`.
    ///
    /// To override the contents of this collection use [`set_hub_search_keywords`](Self::set_hub_search_keywords).
    ///
    /// <p>The searchable keywords for the hub.</p>
    pub fn hub_search_keywords(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.hub_search_keywords.unwrap_or_default();
        v.push(input.into());
        self.hub_search_keywords = ::std::option::Option::Some(v);
        self
    }
    /// <p>The searchable keywords for the hub.</p>
    pub fn set_hub_search_keywords(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.hub_search_keywords = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateHubInput`](crate::operation::update_hub::UpdateHubInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_hub::UpdateHubInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_hub::UpdateHubInput {
            hub_name: self.hub_name,
            hub_description: self.hub_description,
            hub_display_name: self.hub_display_name,
            hub_search_keywords: self.hub_search_keywords,
        })
    }
}
