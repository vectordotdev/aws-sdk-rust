// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateEnvironmentOperationsRoleOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DisassociateEnvironmentOperationsRoleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateEnvironmentOperationsRoleOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateEnvironmentOperationsRoleOutput`](crate::operation::disassociate_environment_operations_role::DisassociateEnvironmentOperationsRoleOutput).
    pub fn builder() -> crate::operation::disassociate_environment_operations_role::builders::DisassociateEnvironmentOperationsRoleOutputBuilder{
        crate::operation::disassociate_environment_operations_role::builders::DisassociateEnvironmentOperationsRoleOutputBuilder::default()
    }
}

/// A builder for [`DisassociateEnvironmentOperationsRoleOutput`](crate::operation::disassociate_environment_operations_role::DisassociateEnvironmentOperationsRoleOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateEnvironmentOperationsRoleOutputBuilder {
    _request_id: Option<String>,
}
impl DisassociateEnvironmentOperationsRoleOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateEnvironmentOperationsRoleOutput`](crate::operation::disassociate_environment_operations_role::DisassociateEnvironmentOperationsRoleOutput).
    pub fn build(self) -> crate::operation::disassociate_environment_operations_role::DisassociateEnvironmentOperationsRoleOutput{
        crate::operation::disassociate_environment_operations_role::DisassociateEnvironmentOperationsRoleOutput {
            _request_id: self._request_id,
        }
    }
}
