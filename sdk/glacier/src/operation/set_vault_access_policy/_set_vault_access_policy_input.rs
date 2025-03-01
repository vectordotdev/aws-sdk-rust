// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>SetVaultAccessPolicy input.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the vault.</p>
    #[doc(hidden)]
    pub vault_name: ::std::option::Option<::std::string::String>,
    /// <p>The vault access policy as a JSON string.</p>
    #[doc(hidden)]
    pub policy: ::std::option::Option<crate::types::VaultAccessPolicy>,
}
impl SetVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(&self) -> ::std::option::Option<&str> {
        self.vault_name.as_deref()
    }
    /// <p>The vault access policy as a JSON string.</p>
    pub fn policy(&self) -> ::std::option::Option<&crate::types::VaultAccessPolicy> {
        self.policy.as_ref()
    }
}
impl SetVaultAccessPolicyInput {
    /// Creates a new builder-style object to manufacture [`SetVaultAccessPolicyInput`](crate::operation::set_vault_access_policy::SetVaultAccessPolicyInput).
    pub fn builder(
    ) -> crate::operation::set_vault_access_policy::builders::SetVaultAccessPolicyInputBuilder {
        crate::operation::set_vault_access_policy::builders::SetVaultAccessPolicyInputBuilder::default()
    }
}

/// A builder for [`SetVaultAccessPolicyInput`](crate::operation::set_vault_access_policy::SetVaultAccessPolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SetVaultAccessPolicyInputBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) vault_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy: ::std::option::Option<crate::types::VaultAccessPolicy>,
}
impl SetVaultAccessPolicyInputBuilder {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vault_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vault_name = input;
        self
    }
    /// <p>The vault access policy as a JSON string.</p>
    pub fn policy(mut self, input: crate::types::VaultAccessPolicy) -> Self {
        self.policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The vault access policy as a JSON string.</p>
    pub fn set_policy(
        mut self,
        input: ::std::option::Option<crate::types::VaultAccessPolicy>,
    ) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`SetVaultAccessPolicyInput`](crate::operation::set_vault_access_policy::SetVaultAccessPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::set_vault_access_policy::SetVaultAccessPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::set_vault_access_policy::SetVaultAccessPolicyInput {
                account_id: self.account_id,
                vault_name: self.vault_name,
                policy: self.policy,
            },
        )
    }
}
