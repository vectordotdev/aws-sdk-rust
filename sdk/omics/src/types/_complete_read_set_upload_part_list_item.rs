// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Part of the response to the CompleteReadSetUpload API, including metadata. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CompleteReadSetUploadPartListItem {
    /// <p> A number identifying the part in a read set upload. </p>
    #[doc(hidden)]
    pub part_number: ::std::option::Option<i32>,
    /// <p> The source file of the part being uploaded. </p>
    #[doc(hidden)]
    pub part_source: ::std::option::Option<crate::types::ReadSetPartSource>,
    /// <p> A unique identifier used to confirm that parts are being added to the correct upload. </p>
    #[doc(hidden)]
    pub checksum: ::std::option::Option<::std::string::String>,
}
impl CompleteReadSetUploadPartListItem {
    /// <p> A number identifying the part in a read set upload. </p>
    pub fn part_number(&self) -> ::std::option::Option<i32> {
        self.part_number
    }
    /// <p> The source file of the part being uploaded. </p>
    pub fn part_source(&self) -> ::std::option::Option<&crate::types::ReadSetPartSource> {
        self.part_source.as_ref()
    }
    /// <p> A unique identifier used to confirm that parts are being added to the correct upload. </p>
    pub fn checksum(&self) -> ::std::option::Option<&str> {
        self.checksum.as_deref()
    }
}
impl CompleteReadSetUploadPartListItem {
    /// Creates a new builder-style object to manufacture [`CompleteReadSetUploadPartListItem`](crate::types::CompleteReadSetUploadPartListItem).
    pub fn builder() -> crate::types::builders::CompleteReadSetUploadPartListItemBuilder {
        crate::types::builders::CompleteReadSetUploadPartListItemBuilder::default()
    }
}

/// A builder for [`CompleteReadSetUploadPartListItem`](crate::types::CompleteReadSetUploadPartListItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CompleteReadSetUploadPartListItemBuilder {
    pub(crate) part_number: ::std::option::Option<i32>,
    pub(crate) part_source: ::std::option::Option<crate::types::ReadSetPartSource>,
    pub(crate) checksum: ::std::option::Option<::std::string::String>,
}
impl CompleteReadSetUploadPartListItemBuilder {
    /// <p> A number identifying the part in a read set upload. </p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.part_number = ::std::option::Option::Some(input);
        self
    }
    /// <p> A number identifying the part in a read set upload. </p>
    pub fn set_part_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.part_number = input;
        self
    }
    /// <p> The source file of the part being uploaded. </p>
    pub fn part_source(mut self, input: crate::types::ReadSetPartSource) -> Self {
        self.part_source = ::std::option::Option::Some(input);
        self
    }
    /// <p> The source file of the part being uploaded. </p>
    pub fn set_part_source(
        mut self,
        input: ::std::option::Option<crate::types::ReadSetPartSource>,
    ) -> Self {
        self.part_source = input;
        self
    }
    /// <p> A unique identifier used to confirm that parts are being added to the correct upload. </p>
    pub fn checksum(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.checksum = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> A unique identifier used to confirm that parts are being added to the correct upload. </p>
    pub fn set_checksum(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.checksum = input;
        self
    }
    /// Consumes the builder and constructs a [`CompleteReadSetUploadPartListItem`](crate::types::CompleteReadSetUploadPartListItem).
    pub fn build(self) -> crate::types::CompleteReadSetUploadPartListItem {
        crate::types::CompleteReadSetUploadPartListItem {
            part_number: self.part_number,
            part_source: self.part_source,
            checksum: self.checksum,
        }
    }
}
