// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of calling the operation. The operation returns one object that is successfully processed by the operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDetectSyntaxItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    #[doc(hidden)]
    pub index: ::std::option::Option<i32>,
    /// <p>The syntax tokens for the words in the document, one token for each word.</p>
    #[doc(hidden)]
    pub syntax_tokens: ::std::option::Option<::std::vec::Vec<crate::types::SyntaxToken>>,
}
impl BatchDetectSyntaxItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    pub fn index(&self) -> ::std::option::Option<i32> {
        self.index
    }
    /// <p>The syntax tokens for the words in the document, one token for each word.</p>
    pub fn syntax_tokens(&self) -> ::std::option::Option<&[crate::types::SyntaxToken]> {
        self.syntax_tokens.as_deref()
    }
}
impl BatchDetectSyntaxItemResult {
    /// Creates a new builder-style object to manufacture [`BatchDetectSyntaxItemResult`](crate::types::BatchDetectSyntaxItemResult).
    pub fn builder() -> crate::types::builders::BatchDetectSyntaxItemResultBuilder {
        crate::types::builders::BatchDetectSyntaxItemResultBuilder::default()
    }
}

/// A builder for [`BatchDetectSyntaxItemResult`](crate::types::BatchDetectSyntaxItemResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchDetectSyntaxItemResultBuilder {
    pub(crate) index: ::std::option::Option<i32>,
    pub(crate) syntax_tokens: ::std::option::Option<::std::vec::Vec<crate::types::SyntaxToken>>,
}
impl BatchDetectSyntaxItemResultBuilder {
    /// <p>The zero-based index of the document in the input list.</p>
    pub fn index(mut self, input: i32) -> Self {
        self.index = ::std::option::Option::Some(input);
        self
    }
    /// <p>The zero-based index of the document in the input list.</p>
    pub fn set_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.index = input;
        self
    }
    /// Appends an item to `syntax_tokens`.
    ///
    /// To override the contents of this collection use [`set_syntax_tokens`](Self::set_syntax_tokens).
    ///
    /// <p>The syntax tokens for the words in the document, one token for each word.</p>
    pub fn syntax_tokens(mut self, input: crate::types::SyntaxToken) -> Self {
        let mut v = self.syntax_tokens.unwrap_or_default();
        v.push(input);
        self.syntax_tokens = ::std::option::Option::Some(v);
        self
    }
    /// <p>The syntax tokens for the words in the document, one token for each word.</p>
    pub fn set_syntax_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SyntaxToken>>,
    ) -> Self {
        self.syntax_tokens = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchDetectSyntaxItemResult`](crate::types::BatchDetectSyntaxItemResult).
    pub fn build(self) -> crate::types::BatchDetectSyntaxItemResult {
        crate::types::BatchDetectSyntaxItemResult {
            index: self.index,
            syntax_tokens: self.syntax_tokens,
        }
    }
}
