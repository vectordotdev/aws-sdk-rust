// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetThreatIntelSetInput {
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    #[doc(hidden)]
    pub detector_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique ID of the threatIntelSet that you want to get.</p>
    #[doc(hidden)]
    pub threat_intel_set_id: ::std::option::Option<::std::string::String>,
}
impl GetThreatIntelSetInput {
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    pub fn detector_id(&self) -> ::std::option::Option<&str> {
        self.detector_id.as_deref()
    }
    /// <p>The unique ID of the threatIntelSet that you want to get.</p>
    pub fn threat_intel_set_id(&self) -> ::std::option::Option<&str> {
        self.threat_intel_set_id.as_deref()
    }
}
impl GetThreatIntelSetInput {
    /// Creates a new builder-style object to manufacture [`GetThreatIntelSetInput`](crate::operation::get_threat_intel_set::GetThreatIntelSetInput).
    pub fn builder(
    ) -> crate::operation::get_threat_intel_set::builders::GetThreatIntelSetInputBuilder {
        crate::operation::get_threat_intel_set::builders::GetThreatIntelSetInputBuilder::default()
    }
}

/// A builder for [`GetThreatIntelSetInput`](crate::operation::get_threat_intel_set::GetThreatIntelSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetThreatIntelSetInputBuilder {
    pub(crate) detector_id: ::std::option::Option<::std::string::String>,
    pub(crate) threat_intel_set_id: ::std::option::Option<::std::string::String>,
}
impl GetThreatIntelSetInputBuilder {
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.detector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.detector_id = input;
        self
    }
    /// <p>The unique ID of the threatIntelSet that you want to get.</p>
    pub fn threat_intel_set_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.threat_intel_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of the threatIntelSet that you want to get.</p>
    pub fn set_threat_intel_set_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.threat_intel_set_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetThreatIntelSetInput`](crate::operation::get_threat_intel_set::GetThreatIntelSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_threat_intel_set::GetThreatIntelSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_threat_intel_set::GetThreatIntelSetInput {
                detector_id: self.detector_id,
                threat_intel_set_id: self.threat_intel_set_id,
            },
        )
    }
}
