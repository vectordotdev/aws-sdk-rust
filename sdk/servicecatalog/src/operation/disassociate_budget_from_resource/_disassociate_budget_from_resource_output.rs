// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateBudgetFromResourceOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DisassociateBudgetFromResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateBudgetFromResourceOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateBudgetFromResourceOutput`](crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceOutput).
    pub fn builder() -> crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceOutputBuilder{
        crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceOutputBuilder::default()
    }
}

/// A builder for [`DisassociateBudgetFromResourceOutput`](crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateBudgetFromResourceOutputBuilder {
    _request_id: Option<String>,
}
impl DisassociateBudgetFromResourceOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateBudgetFromResourceOutput`](crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceOutput).
    pub fn build(
        self,
    ) -> crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceOutput
    {
        crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceOutput {
            _request_id: self._request_id,
        }
    }
}
