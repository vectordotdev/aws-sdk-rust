// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateWhatIfForecastExportInput {
    /// <p>The name of the what-if forecast to export.</p>
    #[doc(hidden)]
    pub what_if_forecast_export_name: ::std::option::Option<::std::string::String>,
    /// <p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p>
    #[doc(hidden)]
    pub what_if_forecast_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p>
    /// <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p>
    #[doc(hidden)]
    pub destination: ::std::option::Option<crate::types::DataDestination>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The format of the exported data, CSV or PARQUET.</p>
    #[doc(hidden)]
    pub format: ::std::option::Option<::std::string::String>,
}
impl CreateWhatIfForecastExportInput {
    /// <p>The name of the what-if forecast to export.</p>
    pub fn what_if_forecast_export_name(&self) -> ::std::option::Option<&str> {
        self.what_if_forecast_export_name.as_deref()
    }
    /// <p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p>
    pub fn what_if_forecast_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.what_if_forecast_arns.as_deref()
    }
    /// <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p>
    /// <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p>
    pub fn destination(&self) -> ::std::option::Option<&crate::types::DataDestination> {
        self.destination.as_ref()
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The format of the exported data, CSV or PARQUET.</p>
    pub fn format(&self) -> ::std::option::Option<&str> {
        self.format.as_deref()
    }
}
impl CreateWhatIfForecastExportInput {
    /// Creates a new builder-style object to manufacture [`CreateWhatIfForecastExportInput`](crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportInput).
    pub fn builder() -> crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportInputBuilder{
        crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportInputBuilder::default()
    }
}

/// A builder for [`CreateWhatIfForecastExportInput`](crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateWhatIfForecastExportInputBuilder {
    pub(crate) what_if_forecast_export_name: ::std::option::Option<::std::string::String>,
    pub(crate) what_if_forecast_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) destination: ::std::option::Option<crate::types::DataDestination>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) format: ::std::option::Option<::std::string::String>,
}
impl CreateWhatIfForecastExportInputBuilder {
    /// <p>The name of the what-if forecast to export.</p>
    pub fn what_if_forecast_export_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.what_if_forecast_export_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the what-if forecast to export.</p>
    pub fn set_what_if_forecast_export_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.what_if_forecast_export_name = input;
        self
    }
    /// Appends an item to `what_if_forecast_arns`.
    ///
    /// To override the contents of this collection use [`set_what_if_forecast_arns`](Self::set_what_if_forecast_arns).
    ///
    /// <p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p>
    pub fn what_if_forecast_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.what_if_forecast_arns.unwrap_or_default();
        v.push(input.into());
        self.what_if_forecast_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p>
    pub fn set_what_if_forecast_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.what_if_forecast_arns = input;
        self
    }
    /// <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p>
    /// <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p>
    pub fn destination(mut self, input: crate::types::DataDestination) -> Self {
        self.destination = ::std::option::Option::Some(input);
        self
    }
    /// <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p>
    /// <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p>
    pub fn set_destination(
        mut self,
        input: ::std::option::Option<crate::types::DataDestination>,
    ) -> Self {
        self.destination = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The format of the exported data, CSV or PARQUET.</p>
    pub fn format(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.format = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The format of the exported data, CSV or PARQUET.</p>
    pub fn set_format(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.format = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateWhatIfForecastExportInput`](crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportInput {
                what_if_forecast_export_name: self.what_if_forecast_export_name,
                what_if_forecast_arns: self.what_if_forecast_arns,
                destination: self.destination,
                tags: self.tags,
                format: self.format,
            },
        )
    }
}
