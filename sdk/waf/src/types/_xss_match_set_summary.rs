// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p>
/// <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p>
/// </note>
/// <p>The <code>Id</code> and <code>Name</code> of an <code>XssMatchSet</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct XssMatchSetSummary {
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <code>GetXssMatchSet</code>), update an <code>XssMatchSet</code> (see <code>UpdateXssMatchSet</code>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <code>UpdateRule</code>), and delete an <code>XssMatchSet</code> from AWS WAF (see <code>DeleteXssMatchSet</code>).</p>
    /// <p> <code>XssMatchSetId</code> is returned by <code>CreateXssMatchSet</code> and by <code>ListXssMatchSets</code>.</p>
    #[doc(hidden)]
    pub xss_match_set_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl XssMatchSetSummary {
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <code>GetXssMatchSet</code>), update an <code>XssMatchSet</code> (see <code>UpdateXssMatchSet</code>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <code>UpdateRule</code>), and delete an <code>XssMatchSet</code> from AWS WAF (see <code>DeleteXssMatchSet</code>).</p>
    /// <p> <code>XssMatchSetId</code> is returned by <code>CreateXssMatchSet</code> and by <code>ListXssMatchSets</code>.</p>
    pub fn xss_match_set_id(&self) -> ::std::option::Option<&str> {
        self.xss_match_set_id.as_deref()
    }
    /// <p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl XssMatchSetSummary {
    /// Creates a new builder-style object to manufacture [`XssMatchSetSummary`](crate::types::XssMatchSetSummary).
    pub fn builder() -> crate::types::builders::XssMatchSetSummaryBuilder {
        crate::types::builders::XssMatchSetSummaryBuilder::default()
    }
}

/// A builder for [`XssMatchSetSummary`](crate::types::XssMatchSetSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct XssMatchSetSummaryBuilder {
    pub(crate) xss_match_set_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl XssMatchSetSummaryBuilder {
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <code>GetXssMatchSet</code>), update an <code>XssMatchSet</code> (see <code>UpdateXssMatchSet</code>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <code>UpdateRule</code>), and delete an <code>XssMatchSet</code> from AWS WAF (see <code>DeleteXssMatchSet</code>).</p>
    /// <p> <code>XssMatchSetId</code> is returned by <code>CreateXssMatchSet</code> and by <code>ListXssMatchSets</code>.</p>
    pub fn xss_match_set_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.xss_match_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <code>GetXssMatchSet</code>), update an <code>XssMatchSet</code> (see <code>UpdateXssMatchSet</code>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <code>UpdateRule</code>), and delete an <code>XssMatchSet</code> from AWS WAF (see <code>DeleteXssMatchSet</code>).</p>
    /// <p> <code>XssMatchSetId</code> is returned by <code>CreateXssMatchSet</code> and by <code>ListXssMatchSets</code>.</p>
    pub fn set_xss_match_set_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.xss_match_set_id = input;
        self
    }
    /// <p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`XssMatchSetSummary`](crate::types::XssMatchSetSummary).
    pub fn build(self) -> crate::types::XssMatchSetSummary {
        crate::types::XssMatchSetSummary {
            xss_match_set_id: self.xss_match_set_id,
            name: self.name,
        }
    }
}
