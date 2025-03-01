// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to delete open and click tracking options in a configuration set. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteConfigurationSetTrackingOptionsInput {
    /// <p>The name of the configuration set from which you want to delete the tracking options.</p>
    #[doc(hidden)]
    pub configuration_set_name: ::std::option::Option<::std::string::String>,
}
impl DeleteConfigurationSetTrackingOptionsInput {
    /// <p>The name of the configuration set from which you want to delete the tracking options.</p>
    pub fn configuration_set_name(&self) -> ::std::option::Option<&str> {
        self.configuration_set_name.as_deref()
    }
}
impl DeleteConfigurationSetTrackingOptionsInput {
    /// Creates a new builder-style object to manufacture [`DeleteConfigurationSetTrackingOptionsInput`](crate::operation::delete_configuration_set_tracking_options::DeleteConfigurationSetTrackingOptionsInput).
    pub fn builder() -> crate::operation::delete_configuration_set_tracking_options::builders::DeleteConfigurationSetTrackingOptionsInputBuilder{
        crate::operation::delete_configuration_set_tracking_options::builders::DeleteConfigurationSetTrackingOptionsInputBuilder::default()
    }
}

/// A builder for [`DeleteConfigurationSetTrackingOptionsInput`](crate::operation::delete_configuration_set_tracking_options::DeleteConfigurationSetTrackingOptionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteConfigurationSetTrackingOptionsInputBuilder {
    pub(crate) configuration_set_name: ::std::option::Option<::std::string::String>,
}
impl DeleteConfigurationSetTrackingOptionsInputBuilder {
    /// <p>The name of the configuration set from which you want to delete the tracking options.</p>
    pub fn configuration_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_set_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the configuration set from which you want to delete the tracking options.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_set_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteConfigurationSetTrackingOptionsInput`](crate::operation::delete_configuration_set_tracking_options::DeleteConfigurationSetTrackingOptionsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::delete_configuration_set_tracking_options::DeleteConfigurationSetTrackingOptionsInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::delete_configuration_set_tracking_options::DeleteConfigurationSetTrackingOptionsInput {
                configuration_set_name: self.configuration_set_name
                ,
            }
        )
    }
}
