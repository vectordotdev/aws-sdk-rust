// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateQuantumTaskInput {
    /// <p>The client token associated with the request.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the device to run the task on.</p>
    #[doc(hidden)]
    pub device_arn: ::std::option::Option<::std::string::String>,
    /// <p>The parameters for the device to run the task on.</p>
    #[doc(hidden)]
    pub device_parameters: ::std::option::Option<::std::string::String>,
    /// <p>The number of shots to use for the task.</p>
    #[doc(hidden)]
    pub shots: ::std::option::Option<i64>,
    /// <p>The S3 bucket to store task result files in.</p>
    #[doc(hidden)]
    pub output_s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p>The key prefix for the location in the S3 bucket to store task results in.</p>
    #[doc(hidden)]
    pub output_s3_key_prefix: ::std::option::Option<::std::string::String>,
    /// <p>The action associated with the task.</p>
    #[doc(hidden)]
    pub action: ::std::option::Option<::std::string::String>,
    /// <p>Tags to be added to the quantum task you're creating.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>The token for an Amazon Braket job that associates it with the quantum task.</p>
    #[doc(hidden)]
    pub job_token: ::std::option::Option<::std::string::String>,
}
impl CreateQuantumTaskInput {
    /// <p>The client token associated with the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The ARN of the device to run the task on.</p>
    pub fn device_arn(&self) -> ::std::option::Option<&str> {
        self.device_arn.as_deref()
    }
    /// <p>The parameters for the device to run the task on.</p>
    pub fn device_parameters(&self) -> ::std::option::Option<&str> {
        self.device_parameters.as_deref()
    }
    /// <p>The number of shots to use for the task.</p>
    pub fn shots(&self) -> ::std::option::Option<i64> {
        self.shots
    }
    /// <p>The S3 bucket to store task result files in.</p>
    pub fn output_s3_bucket(&self) -> ::std::option::Option<&str> {
        self.output_s3_bucket.as_deref()
    }
    /// <p>The key prefix for the location in the S3 bucket to store task results in.</p>
    pub fn output_s3_key_prefix(&self) -> ::std::option::Option<&str> {
        self.output_s3_key_prefix.as_deref()
    }
    /// <p>The action associated with the task.</p>
    pub fn action(&self) -> ::std::option::Option<&str> {
        self.action.as_deref()
    }
    /// <p>Tags to be added to the quantum task you're creating.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>The token for an Amazon Braket job that associates it with the quantum task.</p>
    pub fn job_token(&self) -> ::std::option::Option<&str> {
        self.job_token.as_deref()
    }
}
impl CreateQuantumTaskInput {
    /// Creates a new builder-style object to manufacture [`CreateQuantumTaskInput`](crate::operation::create_quantum_task::CreateQuantumTaskInput).
    pub fn builder(
    ) -> crate::operation::create_quantum_task::builders::CreateQuantumTaskInputBuilder {
        crate::operation::create_quantum_task::builders::CreateQuantumTaskInputBuilder::default()
    }
}

/// A builder for [`CreateQuantumTaskInput`](crate::operation::create_quantum_task::CreateQuantumTaskInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateQuantumTaskInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) device_arn: ::std::option::Option<::std::string::String>,
    pub(crate) device_parameters: ::std::option::Option<::std::string::String>,
    pub(crate) shots: ::std::option::Option<i64>,
    pub(crate) output_s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) output_s3_key_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) action: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) job_token: ::std::option::Option<::std::string::String>,
}
impl CreateQuantumTaskInputBuilder {
    /// <p>The client token associated with the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The ARN of the device to run the task on.</p>
    pub fn device_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the device to run the task on.</p>
    pub fn set_device_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_arn = input;
        self
    }
    /// <p>The parameters for the device to run the task on.</p>
    pub fn device_parameters(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.device_parameters = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The parameters for the device to run the task on.</p>
    pub fn set_device_parameters(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.device_parameters = input;
        self
    }
    /// <p>The number of shots to use for the task.</p>
    pub fn shots(mut self, input: i64) -> Self {
        self.shots = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of shots to use for the task.</p>
    pub fn set_shots(mut self, input: ::std::option::Option<i64>) -> Self {
        self.shots = input;
        self
    }
    /// <p>The S3 bucket to store task result files in.</p>
    pub fn output_s3_bucket(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.output_s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The S3 bucket to store task result files in.</p>
    pub fn set_output_s3_bucket(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.output_s3_bucket = input;
        self
    }
    /// <p>The key prefix for the location in the S3 bucket to store task results in.</p>
    pub fn output_s3_key_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.output_s3_key_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key prefix for the location in the S3 bucket to store task results in.</p>
    pub fn set_output_s3_key_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.output_s3_key_prefix = input;
        self
    }
    /// <p>The action associated with the task.</p>
    pub fn action(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The action associated with the task.</p>
    pub fn set_action(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags to be added to the quantum task you're creating.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Tags to be added to the quantum task you're creating.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The token for an Amazon Braket job that associates it with the quantum task.</p>
    pub fn job_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for an Amazon Braket job that associates it with the quantum task.</p>
    pub fn set_job_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateQuantumTaskInput`](crate::operation::create_quantum_task::CreateQuantumTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_quantum_task::CreateQuantumTaskInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_quantum_task::CreateQuantumTaskInput {
                client_token: self.client_token,
                device_arn: self.device_arn,
                device_parameters: self.device_parameters,
                shots: self.shots,
                output_s3_bucket: self.output_s3_bucket,
                output_s3_key_prefix: self.output_s3_key_prefix,
                action: self.action,
                tags: self.tags,
                job_token: self.job_token,
            },
        )
    }
}
