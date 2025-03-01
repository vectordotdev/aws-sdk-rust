// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An Internet Control Message Protocol (ICMP) type and code.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IcmpTypeCode {
    /// <p>The ICMP code for which to deny or allow access. To deny or allow all codes, use the value <code>-1</code>.</p>
    #[doc(hidden)]
    pub code: i32,
    /// <p>The ICMP type for which to deny or allow access. To deny or allow all types, use the value <code>-1</code>.</p>
    #[doc(hidden)]
    pub r#type: i32,
}
impl IcmpTypeCode {
    /// <p>The ICMP code for which to deny or allow access. To deny or allow all codes, use the value <code>-1</code>.</p>
    pub fn code(&self) -> i32 {
        self.code
    }
    /// <p>The ICMP type for which to deny or allow access. To deny or allow all types, use the value <code>-1</code>.</p>
    pub fn r#type(&self) -> i32 {
        self.r#type
    }
}
impl IcmpTypeCode {
    /// Creates a new builder-style object to manufacture [`IcmpTypeCode`](crate::types::IcmpTypeCode).
    pub fn builder() -> crate::types::builders::IcmpTypeCodeBuilder {
        crate::types::builders::IcmpTypeCodeBuilder::default()
    }
}

/// A builder for [`IcmpTypeCode`](crate::types::IcmpTypeCode).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IcmpTypeCodeBuilder {
    pub(crate) code: ::std::option::Option<i32>,
    pub(crate) r#type: ::std::option::Option<i32>,
}
impl IcmpTypeCodeBuilder {
    /// <p>The ICMP code for which to deny or allow access. To deny or allow all codes, use the value <code>-1</code>.</p>
    pub fn code(mut self, input: i32) -> Self {
        self.code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ICMP code for which to deny or allow access. To deny or allow all codes, use the value <code>-1</code>.</p>
    pub fn set_code(mut self, input: ::std::option::Option<i32>) -> Self {
        self.code = input;
        self
    }
    /// <p>The ICMP type for which to deny or allow access. To deny or allow all types, use the value <code>-1</code>.</p>
    pub fn r#type(mut self, input: i32) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ICMP type for which to deny or allow access. To deny or allow all types, use the value <code>-1</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<i32>) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`IcmpTypeCode`](crate::types::IcmpTypeCode).
    pub fn build(self) -> crate::types::IcmpTypeCode {
        crate::types::IcmpTypeCode {
            code: self.code.unwrap_or_default(),
            r#type: self.r#type.unwrap_or_default(),
        }
    }
}
