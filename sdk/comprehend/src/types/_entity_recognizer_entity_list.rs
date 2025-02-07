// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the entity list submitted with an entity recognizer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EntityRecognizerEntityList {
    /// <p>Specifies the Amazon S3 location where the entity list is located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    #[doc(hidden)]
    pub s3_uri: ::std::option::Option<::std::string::String>,
}
impl EntityRecognizerEntityList {
    /// <p>Specifies the Amazon S3 location where the entity list is located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    pub fn s3_uri(&self) -> ::std::option::Option<&str> {
        self.s3_uri.as_deref()
    }
}
impl EntityRecognizerEntityList {
    /// Creates a new builder-style object to manufacture [`EntityRecognizerEntityList`](crate::types::EntityRecognizerEntityList).
    pub fn builder() -> crate::types::builders::EntityRecognizerEntityListBuilder {
        crate::types::builders::EntityRecognizerEntityListBuilder::default()
    }
}

/// A builder for [`EntityRecognizerEntityList`](crate::types::EntityRecognizerEntityList).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EntityRecognizerEntityListBuilder {
    pub(crate) s3_uri: ::std::option::Option<::std::string::String>,
}
impl EntityRecognizerEntityListBuilder {
    /// <p>Specifies the Amazon S3 location where the entity list is located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    pub fn s3_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the Amazon S3 location where the entity list is located. The URI must be in the same Region as the API endpoint that you are calling.</p>
    pub fn set_s3_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_uri = input;
        self
    }
    /// Consumes the builder and constructs a [`EntityRecognizerEntityList`](crate::types::EntityRecognizerEntityList).
    pub fn build(self) -> crate::types::EntityRecognizerEntityList {
        crate::types::EntityRecognizerEntityList {
            s3_uri: self.s3_uri,
        }
    }
}
