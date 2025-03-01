// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeObservationInput {
    /// <p>The ID of the observation.</p>
    #[doc(hidden)]
    pub observation_id: ::std::option::Option<::std::string::String>,
}
impl DescribeObservationInput {
    /// <p>The ID of the observation.</p>
    pub fn observation_id(&self) -> ::std::option::Option<&str> {
        self.observation_id.as_deref()
    }
}
impl DescribeObservationInput {
    /// Creates a new builder-style object to manufacture [`DescribeObservationInput`](crate::operation::describe_observation::DescribeObservationInput).
    pub fn builder(
    ) -> crate::operation::describe_observation::builders::DescribeObservationInputBuilder {
        crate::operation::describe_observation::builders::DescribeObservationInputBuilder::default()
    }
}

/// A builder for [`DescribeObservationInput`](crate::operation::describe_observation::DescribeObservationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeObservationInputBuilder {
    pub(crate) observation_id: ::std::option::Option<::std::string::String>,
}
impl DescribeObservationInputBuilder {
    /// <p>The ID of the observation.</p>
    pub fn observation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.observation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the observation.</p>
    pub fn set_observation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.observation_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeObservationInput`](crate::operation::describe_observation::DescribeObservationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_observation::DescribeObservationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_observation::DescribeObservationInput {
                observation_id: self.observation_id,
            },
        )
    }
}
