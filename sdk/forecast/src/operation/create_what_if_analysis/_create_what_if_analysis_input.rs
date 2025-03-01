// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateWhatIfAnalysisInput {
    /// <p>The name of the what-if analysis. Each name must be unique.</p>
    #[doc(hidden)]
    pub what_if_analysis_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the baseline forecast.</p>
    #[doc(hidden)]
    pub forecast_arn: ::std::option::Option<::std::string::String>,
    /// <p>Defines the set of time series that are used in the what-if analysis with a <code>TimeSeriesIdentifiers</code> object. What-if analyses are performed only for the time series in this object.</p>
    /// <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>
    /// <ul>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>Format</code> </p> </li>
    /// <li> <p> <code>Schema</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub time_series_selector: ::std::option::Option<crate::types::TimeSeriesSelector>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateWhatIfAnalysisInput {
    /// <p>The name of the what-if analysis. Each name must be unique.</p>
    pub fn what_if_analysis_name(&self) -> ::std::option::Option<&str> {
        self.what_if_analysis_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the baseline forecast.</p>
    pub fn forecast_arn(&self) -> ::std::option::Option<&str> {
        self.forecast_arn.as_deref()
    }
    /// <p>Defines the set of time series that are used in the what-if analysis with a <code>TimeSeriesIdentifiers</code> object. What-if analyses are performed only for the time series in this object.</p>
    /// <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>
    /// <ul>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>Format</code> </p> </li>
    /// <li> <p> <code>Schema</code> </p> </li>
    /// </ul>
    pub fn time_series_selector(&self) -> ::std::option::Option<&crate::types::TimeSeriesSelector> {
        self.time_series_selector.as_ref()
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateWhatIfAnalysisInput {
    /// Creates a new builder-style object to manufacture [`CreateWhatIfAnalysisInput`](crate::operation::create_what_if_analysis::CreateWhatIfAnalysisInput).
    pub fn builder(
    ) -> crate::operation::create_what_if_analysis::builders::CreateWhatIfAnalysisInputBuilder {
        crate::operation::create_what_if_analysis::builders::CreateWhatIfAnalysisInputBuilder::default()
    }
}

/// A builder for [`CreateWhatIfAnalysisInput`](crate::operation::create_what_if_analysis::CreateWhatIfAnalysisInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateWhatIfAnalysisInputBuilder {
    pub(crate) what_if_analysis_name: ::std::option::Option<::std::string::String>,
    pub(crate) forecast_arn: ::std::option::Option<::std::string::String>,
    pub(crate) time_series_selector: ::std::option::Option<crate::types::TimeSeriesSelector>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateWhatIfAnalysisInputBuilder {
    /// <p>The name of the what-if analysis. Each name must be unique.</p>
    pub fn what_if_analysis_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.what_if_analysis_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the what-if analysis. Each name must be unique.</p>
    pub fn set_what_if_analysis_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.what_if_analysis_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the baseline forecast.</p>
    pub fn forecast_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.forecast_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the baseline forecast.</p>
    pub fn set_forecast_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.forecast_arn = input;
        self
    }
    /// <p>Defines the set of time series that are used in the what-if analysis with a <code>TimeSeriesIdentifiers</code> object. What-if analyses are performed only for the time series in this object.</p>
    /// <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>
    /// <ul>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>Format</code> </p> </li>
    /// <li> <p> <code>Schema</code> </p> </li>
    /// </ul>
    pub fn time_series_selector(mut self, input: crate::types::TimeSeriesSelector) -> Self {
        self.time_series_selector = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines the set of time series that are used in the what-if analysis with a <code>TimeSeriesIdentifiers</code> object. What-if analyses are performed only for the time series in this object.</p>
    /// <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>
    /// <ul>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>Format</code> </p> </li>
    /// <li> <p> <code>Schema</code> </p> </li>
    /// </ul>
    pub fn set_time_series_selector(
        mut self,
        input: ::std::option::Option<crate::types::TimeSeriesSelector>,
    ) -> Self {
        self.time_series_selector = input;
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
    /// Consumes the builder and constructs a [`CreateWhatIfAnalysisInput`](crate::operation::create_what_if_analysis::CreateWhatIfAnalysisInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_what_if_analysis::CreateWhatIfAnalysisInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_what_if_analysis::CreateWhatIfAnalysisInput {
                what_if_analysis_name: self.what_if_analysis_name,
                forecast_arn: self.forecast_arn,
                time_series_selector: self.time_series_selector,
                tags: self.tags,
            },
        )
    }
}
