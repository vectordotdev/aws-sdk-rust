// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies an integer hyperparameter and it's range of tunable values. This object is part of the <code>ParameterRanges</code> object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IntegerParameterRange {
    /// <p>The name of the hyperparameter to tune.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum tunable value of the hyperparameter.</p>
    #[doc(hidden)]
    pub max_value: ::std::option::Option<i32>,
    /// <p>The minimum tunable value of the hyperparameter.</p>
    #[doc(hidden)]
    pub min_value: ::std::option::Option<i32>,
    /// <p>The scale that hyperparameter tuning uses to search the hyperparameter range. Valid values:</p>
    /// <dl>
    /// <dt>
    /// Auto
    /// </dt>
    /// <dd>
    /// <p>Amazon Forecast hyperparameter tuning chooses the best scale for the hyperparameter.</p>
    /// </dd>
    /// <dt>
    /// Linear
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p>
    /// </dd>
    /// <dt>
    /// Logarithmic
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p>
    /// <p>Logarithmic scaling works only for ranges that have values greater than 0.</p>
    /// </dd>
    /// <dt>
    /// ReverseLogarithmic
    /// </dt>
    /// <dd>
    /// <p>Not supported for <code>IntegerParameterRange</code>.</p>
    /// <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0 &lt;= x &lt; 1.0.</p>
    /// </dd>
    /// </dl>
    /// <p>For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p>
    #[doc(hidden)]
    pub scaling_type: ::std::option::Option<crate::types::ScalingType>,
}
impl IntegerParameterRange {
    /// <p>The name of the hyperparameter to tune.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The maximum tunable value of the hyperparameter.</p>
    pub fn max_value(&self) -> ::std::option::Option<i32> {
        self.max_value
    }
    /// <p>The minimum tunable value of the hyperparameter.</p>
    pub fn min_value(&self) -> ::std::option::Option<i32> {
        self.min_value
    }
    /// <p>The scale that hyperparameter tuning uses to search the hyperparameter range. Valid values:</p>
    /// <dl>
    /// <dt>
    /// Auto
    /// </dt>
    /// <dd>
    /// <p>Amazon Forecast hyperparameter tuning chooses the best scale for the hyperparameter.</p>
    /// </dd>
    /// <dt>
    /// Linear
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p>
    /// </dd>
    /// <dt>
    /// Logarithmic
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p>
    /// <p>Logarithmic scaling works only for ranges that have values greater than 0.</p>
    /// </dd>
    /// <dt>
    /// ReverseLogarithmic
    /// </dt>
    /// <dd>
    /// <p>Not supported for <code>IntegerParameterRange</code>.</p>
    /// <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0 &lt;= x &lt; 1.0.</p>
    /// </dd>
    /// </dl>
    /// <p>For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p>
    pub fn scaling_type(&self) -> ::std::option::Option<&crate::types::ScalingType> {
        self.scaling_type.as_ref()
    }
}
impl IntegerParameterRange {
    /// Creates a new builder-style object to manufacture [`IntegerParameterRange`](crate::types::IntegerParameterRange).
    pub fn builder() -> crate::types::builders::IntegerParameterRangeBuilder {
        crate::types::builders::IntegerParameterRangeBuilder::default()
    }
}

/// A builder for [`IntegerParameterRange`](crate::types::IntegerParameterRange).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IntegerParameterRangeBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) max_value: ::std::option::Option<i32>,
    pub(crate) min_value: ::std::option::Option<i32>,
    pub(crate) scaling_type: ::std::option::Option<crate::types::ScalingType>,
}
impl IntegerParameterRangeBuilder {
    /// <p>The name of the hyperparameter to tune.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the hyperparameter to tune.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The maximum tunable value of the hyperparameter.</p>
    pub fn max_value(mut self, input: i32) -> Self {
        self.max_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum tunable value of the hyperparameter.</p>
    pub fn set_max_value(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_value = input;
        self
    }
    /// <p>The minimum tunable value of the hyperparameter.</p>
    pub fn min_value(mut self, input: i32) -> Self {
        self.min_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum tunable value of the hyperparameter.</p>
    pub fn set_min_value(mut self, input: ::std::option::Option<i32>) -> Self {
        self.min_value = input;
        self
    }
    /// <p>The scale that hyperparameter tuning uses to search the hyperparameter range. Valid values:</p>
    /// <dl>
    /// <dt>
    /// Auto
    /// </dt>
    /// <dd>
    /// <p>Amazon Forecast hyperparameter tuning chooses the best scale for the hyperparameter.</p>
    /// </dd>
    /// <dt>
    /// Linear
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p>
    /// </dd>
    /// <dt>
    /// Logarithmic
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p>
    /// <p>Logarithmic scaling works only for ranges that have values greater than 0.</p>
    /// </dd>
    /// <dt>
    /// ReverseLogarithmic
    /// </dt>
    /// <dd>
    /// <p>Not supported for <code>IntegerParameterRange</code>.</p>
    /// <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0 &lt;= x &lt; 1.0.</p>
    /// </dd>
    /// </dl>
    /// <p>For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p>
    pub fn scaling_type(mut self, input: crate::types::ScalingType) -> Self {
        self.scaling_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The scale that hyperparameter tuning uses to search the hyperparameter range. Valid values:</p>
    /// <dl>
    /// <dt>
    /// Auto
    /// </dt>
    /// <dd>
    /// <p>Amazon Forecast hyperparameter tuning chooses the best scale for the hyperparameter.</p>
    /// </dd>
    /// <dt>
    /// Linear
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p>
    /// </dd>
    /// <dt>
    /// Logarithmic
    /// </dt>
    /// <dd>
    /// <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p>
    /// <p>Logarithmic scaling works only for ranges that have values greater than 0.</p>
    /// </dd>
    /// <dt>
    /// ReverseLogarithmic
    /// </dt>
    /// <dd>
    /// <p>Not supported for <code>IntegerParameterRange</code>.</p>
    /// <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0 &lt;= x &lt; 1.0.</p>
    /// </dd>
    /// </dl>
    /// <p>For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p>
    pub fn set_scaling_type(
        mut self,
        input: ::std::option::Option<crate::types::ScalingType>,
    ) -> Self {
        self.scaling_type = input;
        self
    }
    /// Consumes the builder and constructs a [`IntegerParameterRange`](crate::types::IntegerParameterRange).
    pub fn build(self) -> crate::types::IntegerParameterRange {
        crate::types::IntegerParameterRange {
            name: self.name,
            max_value: self.max_value,
            min_value: self.min_value,
            scaling_type: self.scaling_type,
        }
    }
}
