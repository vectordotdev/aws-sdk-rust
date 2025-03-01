// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>For a SQL-based Kinesis Data Analytics application, provides additional mapping information when JSON is the record format on the streaming source.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonMappingParameters {
    /// <p>The path to the top-level parent that contains the records.</p>
    #[doc(hidden)]
    pub record_row_path: ::std::option::Option<::std::string::String>,
}
impl JsonMappingParameters {
    /// <p>The path to the top-level parent that contains the records.</p>
    pub fn record_row_path(&self) -> ::std::option::Option<&str> {
        self.record_row_path.as_deref()
    }
}
impl JsonMappingParameters {
    /// Creates a new builder-style object to manufacture [`JsonMappingParameters`](crate::types::JsonMappingParameters).
    pub fn builder() -> crate::types::builders::JsonMappingParametersBuilder {
        crate::types::builders::JsonMappingParametersBuilder::default()
    }
}

/// A builder for [`JsonMappingParameters`](crate::types::JsonMappingParameters).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JsonMappingParametersBuilder {
    pub(crate) record_row_path: ::std::option::Option<::std::string::String>,
}
impl JsonMappingParametersBuilder {
    /// <p>The path to the top-level parent that contains the records.</p>
    pub fn record_row_path(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.record_row_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to the top-level parent that contains the records.</p>
    pub fn set_record_row_path(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.record_row_path = input;
        self
    }
    /// Consumes the builder and constructs a [`JsonMappingParameters`](crate::types::JsonMappingParameters).
    pub fn build(self) -> crate::types::JsonMappingParameters {
        crate::types::JsonMappingParameters {
            record_row_path: self.record_row_path,
        }
    }
}
