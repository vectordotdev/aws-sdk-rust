// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The latest known metered size (in bytes) of data stored in the file system, in its <code>Value</code> field, and the time at which that size was determined in its <code>Timestamp</code> field. The value doesn't represent the size of a consistent snapshot of the file system, but it is eventually consistent when there are no writes to the file system. That is, the value represents the actual size only if the file system is not modified for a period longer than a couple of hours. Otherwise, the value is not necessarily the exact size the file system was at any instant in time.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FileSystemSize {
    /// <p>The latest known metered size (in bytes) of data stored in the file system.</p>
    #[doc(hidden)]
    pub value: i64,
    /// <p>The time at which the size of data, returned in the <code>Value</code> field, was determined. The value is the integer number of seconds since 1970-01-01T00:00:00Z.</p>
    #[doc(hidden)]
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.</p>
    #[doc(hidden)]
    pub value_in_ia: ::std::option::Option<i64>,
    /// <p>The latest known metered size (in bytes) of data stored in the Standard storage class.</p>
    #[doc(hidden)]
    pub value_in_standard: ::std::option::Option<i64>,
}
impl FileSystemSize {
    /// <p>The latest known metered size (in bytes) of data stored in the file system.</p>
    pub fn value(&self) -> i64 {
        self.value
    }
    /// <p>The time at which the size of data, returned in the <code>Value</code> field, was determined. The value is the integer number of seconds since 1970-01-01T00:00:00Z.</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.</p>
    pub fn value_in_ia(&self) -> ::std::option::Option<i64> {
        self.value_in_ia
    }
    /// <p>The latest known metered size (in bytes) of data stored in the Standard storage class.</p>
    pub fn value_in_standard(&self) -> ::std::option::Option<i64> {
        self.value_in_standard
    }
}
impl FileSystemSize {
    /// Creates a new builder-style object to manufacture [`FileSystemSize`](crate::types::FileSystemSize).
    pub fn builder() -> crate::types::builders::FileSystemSizeBuilder {
        crate::types::builders::FileSystemSizeBuilder::default()
    }
}

/// A builder for [`FileSystemSize`](crate::types::FileSystemSize).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FileSystemSizeBuilder {
    pub(crate) value: ::std::option::Option<i64>,
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) value_in_ia: ::std::option::Option<i64>,
    pub(crate) value_in_standard: ::std::option::Option<i64>,
}
impl FileSystemSizeBuilder {
    /// <p>The latest known metered size (in bytes) of data stored in the file system.</p>
    pub fn value(mut self, input: i64) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The latest known metered size (in bytes) of data stored in the file system.</p>
    pub fn set_value(mut self, input: ::std::option::Option<i64>) -> Self {
        self.value = input;
        self
    }
    /// <p>The time at which the size of data, returned in the <code>Value</code> field, was determined. The value is the integer number of seconds since 1970-01-01T00:00:00Z.</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which the size of data, returned in the <code>Value</code> field, was determined. The value is the integer number of seconds since 1970-01-01T00:00:00Z.</p>
    pub fn set_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.</p>
    pub fn value_in_ia(mut self, input: i64) -> Self {
        self.value_in_ia = ::std::option::Option::Some(input);
        self
    }
    /// <p>The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.</p>
    pub fn set_value_in_ia(mut self, input: ::std::option::Option<i64>) -> Self {
        self.value_in_ia = input;
        self
    }
    /// <p>The latest known metered size (in bytes) of data stored in the Standard storage class.</p>
    pub fn value_in_standard(mut self, input: i64) -> Self {
        self.value_in_standard = ::std::option::Option::Some(input);
        self
    }
    /// <p>The latest known metered size (in bytes) of data stored in the Standard storage class.</p>
    pub fn set_value_in_standard(mut self, input: ::std::option::Option<i64>) -> Self {
        self.value_in_standard = input;
        self
    }
    /// Consumes the builder and constructs a [`FileSystemSize`](crate::types::FileSystemSize).
    pub fn build(self) -> crate::types::FileSystemSize {
        crate::types::FileSystemSize {
            value: self.value.unwrap_or_default(),
            timestamp: self.timestamp,
            value_in_ia: self.value_in_ia,
            value_in_standard: self.value_in_standard,
        }
    }
}
