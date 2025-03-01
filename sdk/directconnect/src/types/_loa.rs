// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a Letter of Authorization - Connecting Facility Assignment (LOA-CFA) for a connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Loa {
    /// <p>The binary contents of the LOA-CFA document.</p>
    #[doc(hidden)]
    pub loa_content: ::std::option::Option<::aws_smithy_types::Blob>,
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    #[doc(hidden)]
    pub loa_content_type: ::std::option::Option<crate::types::LoaContentType>,
}
impl Loa {
    /// <p>The binary contents of the LOA-CFA document.</p>
    pub fn loa_content(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.loa_content.as_ref()
    }
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    pub fn loa_content_type(&self) -> ::std::option::Option<&crate::types::LoaContentType> {
        self.loa_content_type.as_ref()
    }
}
impl Loa {
    /// Creates a new builder-style object to manufacture [`Loa`](crate::types::Loa).
    pub fn builder() -> crate::types::builders::LoaBuilder {
        crate::types::builders::LoaBuilder::default()
    }
}

/// A builder for [`Loa`](crate::types::Loa).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LoaBuilder {
    pub(crate) loa_content: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) loa_content_type: ::std::option::Option<crate::types::LoaContentType>,
}
impl LoaBuilder {
    /// <p>The binary contents of the LOA-CFA document.</p>
    pub fn loa_content(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.loa_content = ::std::option::Option::Some(input);
        self
    }
    /// <p>The binary contents of the LOA-CFA document.</p>
    pub fn set_loa_content(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.loa_content = input;
        self
    }
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    pub fn loa_content_type(mut self, input: crate::types::LoaContentType) -> Self {
        self.loa_content_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    pub fn set_loa_content_type(
        mut self,
        input: ::std::option::Option<crate::types::LoaContentType>,
    ) -> Self {
        self.loa_content_type = input;
        self
    }
    /// Consumes the builder and constructs a [`Loa`](crate::types::Loa).
    pub fn build(self) -> crate::types::Loa {
        crate::types::Loa {
            loa_content: self.loa_content,
            loa_content_type: self.loa_content_type,
        }
    }
}
