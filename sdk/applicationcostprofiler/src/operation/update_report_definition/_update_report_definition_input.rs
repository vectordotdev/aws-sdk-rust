// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateReportDefinitionInput {
    /// <p>Required. ID of the report to update.</p>
    #[doc(hidden)]
    pub report_id: ::std::option::Option<::std::string::String>,
    /// <p>Required. Description of the report.</p>
    #[doc(hidden)]
    pub report_description: ::std::option::Option<::std::string::String>,
    /// <p>Required. The cadence to generate the report.</p>
    #[doc(hidden)]
    pub report_frequency: ::std::option::Option<crate::types::ReportFrequency>,
    /// <p>Required. The format to use for the generated report.</p>
    #[doc(hidden)]
    pub format: ::std::option::Option<crate::types::Format>,
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    #[doc(hidden)]
    pub destination_s3_location: ::std::option::Option<crate::types::S3Location>,
}
impl UpdateReportDefinitionInput {
    /// <p>Required. ID of the report to update.</p>
    pub fn report_id(&self) -> ::std::option::Option<&str> {
        self.report_id.as_deref()
    }
    /// <p>Required. Description of the report.</p>
    pub fn report_description(&self) -> ::std::option::Option<&str> {
        self.report_description.as_deref()
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn report_frequency(&self) -> ::std::option::Option<&crate::types::ReportFrequency> {
        self.report_frequency.as_ref()
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn format(&self) -> ::std::option::Option<&crate::types::Format> {
        self.format.as_ref()
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn destination_s3_location(&self) -> ::std::option::Option<&crate::types::S3Location> {
        self.destination_s3_location.as_ref()
    }
}
impl UpdateReportDefinitionInput {
    /// Creates a new builder-style object to manufacture [`UpdateReportDefinitionInput`](crate::operation::update_report_definition::UpdateReportDefinitionInput).
    pub fn builder(
    ) -> crate::operation::update_report_definition::builders::UpdateReportDefinitionInputBuilder
    {
        crate::operation::update_report_definition::builders::UpdateReportDefinitionInputBuilder::default()
    }
}

/// A builder for [`UpdateReportDefinitionInput`](crate::operation::update_report_definition::UpdateReportDefinitionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateReportDefinitionInputBuilder {
    pub(crate) report_id: ::std::option::Option<::std::string::String>,
    pub(crate) report_description: ::std::option::Option<::std::string::String>,
    pub(crate) report_frequency: ::std::option::Option<crate::types::ReportFrequency>,
    pub(crate) format: ::std::option::Option<crate::types::Format>,
    pub(crate) destination_s3_location: ::std::option::Option<crate::types::S3Location>,
}
impl UpdateReportDefinitionInputBuilder {
    /// <p>Required. ID of the report to update.</p>
    pub fn report_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.report_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Required. ID of the report to update.</p>
    pub fn set_report_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.report_id = input;
        self
    }
    /// <p>Required. Description of the report.</p>
    pub fn report_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.report_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Required. Description of the report.</p>
    pub fn set_report_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.report_description = input;
        self
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn report_frequency(mut self, input: crate::types::ReportFrequency) -> Self {
        self.report_frequency = ::std::option::Option::Some(input);
        self
    }
    /// <p>Required. The cadence to generate the report.</p>
    pub fn set_report_frequency(
        mut self,
        input: ::std::option::Option<crate::types::ReportFrequency>,
    ) -> Self {
        self.report_frequency = input;
        self
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn format(mut self, input: crate::types::Format) -> Self {
        self.format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Required. The format to use for the generated report.</p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::Format>) -> Self {
        self.format = input;
        self
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn destination_s3_location(mut self, input: crate::types::S3Location) -> Self {
        self.destination_s3_location = ::std::option::Option::Some(input);
        self
    }
    /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
    pub fn set_destination_s3_location(
        mut self,
        input: ::std::option::Option<crate::types::S3Location>,
    ) -> Self {
        self.destination_s3_location = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateReportDefinitionInput`](crate::operation::update_report_definition::UpdateReportDefinitionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_report_definition::UpdateReportDefinitionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_report_definition::UpdateReportDefinitionInput {
                report_id: self.report_id,
                report_description: self.report_description,
                report_frequency: self.report_frequency,
                format: self.format,
                destination_s3_location: self.destination_s3_location,
            },
        )
    }
}
