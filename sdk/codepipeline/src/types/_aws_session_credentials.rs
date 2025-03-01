// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the S3 bucket used to store artifact for the pipeline in AWS CodePipeline.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AwsSessionCredentials {
    /// <p>The access key for the session.</p>
    #[doc(hidden)]
    pub access_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The secret access key for the session.</p>
    #[doc(hidden)]
    pub secret_access_key: ::std::option::Option<::std::string::String>,
    /// <p>The token for the session.</p>
    #[doc(hidden)]
    pub session_token: ::std::option::Option<::std::string::String>,
}
impl AwsSessionCredentials {
    /// <p>The access key for the session.</p>
    pub fn access_key_id(&self) -> ::std::option::Option<&str> {
        self.access_key_id.as_deref()
    }
    /// <p>The secret access key for the session.</p>
    pub fn secret_access_key(&self) -> ::std::option::Option<&str> {
        self.secret_access_key.as_deref()
    }
    /// <p>The token for the session.</p>
    pub fn session_token(&self) -> ::std::option::Option<&str> {
        self.session_token.as_deref()
    }
}
impl ::std::fmt::Debug for AwsSessionCredentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AwsSessionCredentials");
        formatter.field("access_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
        formatter.field("session_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl AwsSessionCredentials {
    /// Creates a new builder-style object to manufacture [`AwsSessionCredentials`](crate::types::AwsSessionCredentials).
    pub fn builder() -> crate::types::builders::AwsSessionCredentialsBuilder {
        crate::types::builders::AwsSessionCredentialsBuilder::default()
    }
}

/// A builder for [`AwsSessionCredentials`](crate::types::AwsSessionCredentials).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AwsSessionCredentialsBuilder {
    pub(crate) access_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) secret_access_key: ::std::option::Option<::std::string::String>,
    pub(crate) session_token: ::std::option::Option<::std::string::String>,
}
impl AwsSessionCredentialsBuilder {
    /// <p>The access key for the session.</p>
    pub fn access_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.access_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The access key for the session.</p>
    pub fn set_access_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.access_key_id = input;
        self
    }
    /// <p>The secret access key for the session.</p>
    pub fn secret_access_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.secret_access_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The secret access key for the session.</p>
    pub fn set_secret_access_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.secret_access_key = input;
        self
    }
    /// <p>The token for the session.</p>
    pub fn session_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.session_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the session.</p>
    pub fn set_session_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.session_token = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsSessionCredentials`](crate::types::AwsSessionCredentials).
    pub fn build(self) -> crate::types::AwsSessionCredentials {
        crate::types::AwsSessionCredentials {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
            session_token: self.session_token,
        }
    }
}
impl ::std::fmt::Debug for AwsSessionCredentialsBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AwsSessionCredentialsBuilder");
        formatter.field("access_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
        formatter.field("session_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
