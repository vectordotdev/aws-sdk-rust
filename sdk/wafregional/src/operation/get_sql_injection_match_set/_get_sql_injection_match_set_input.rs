// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A request to get a <code>SqlInjectionMatchSet</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSqlInjectionMatchSetInput {
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <code>CreateSqlInjectionMatchSet</code> and by <code>ListSqlInjectionMatchSets</code>.</p>
    #[doc(hidden)]
    pub sql_injection_match_set_id: ::std::option::Option<::std::string::String>,
}
impl GetSqlInjectionMatchSetInput {
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <code>CreateSqlInjectionMatchSet</code> and by <code>ListSqlInjectionMatchSets</code>.</p>
    pub fn sql_injection_match_set_id(&self) -> ::std::option::Option<&str> {
        self.sql_injection_match_set_id.as_deref()
    }
}
impl GetSqlInjectionMatchSetInput {
    /// Creates a new builder-style object to manufacture [`GetSqlInjectionMatchSetInput`](crate::operation::get_sql_injection_match_set::GetSqlInjectionMatchSetInput).
    pub fn builder(
    ) -> crate::operation::get_sql_injection_match_set::builders::GetSqlInjectionMatchSetInputBuilder
    {
        crate::operation::get_sql_injection_match_set::builders::GetSqlInjectionMatchSetInputBuilder::default()
    }
}

/// A builder for [`GetSqlInjectionMatchSetInput`](crate::operation::get_sql_injection_match_set::GetSqlInjectionMatchSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetSqlInjectionMatchSetInputBuilder {
    pub(crate) sql_injection_match_set_id: ::std::option::Option<::std::string::String>,
}
impl GetSqlInjectionMatchSetInputBuilder {
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <code>CreateSqlInjectionMatchSet</code> and by <code>ListSqlInjectionMatchSets</code>.</p>
    pub fn sql_injection_match_set_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sql_injection_match_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <code>CreateSqlInjectionMatchSet</code> and by <code>ListSqlInjectionMatchSets</code>.</p>
    pub fn set_sql_injection_match_set_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sql_injection_match_set_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetSqlInjectionMatchSetInput`](crate::operation::get_sql_injection_match_set::GetSqlInjectionMatchSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_sql_injection_match_set::GetSqlInjectionMatchSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_sql_injection_match_set::GetSqlInjectionMatchSetInput {
                sql_injection_match_set_id: self.sql_injection_match_set_id,
            },
        )
    }
}
