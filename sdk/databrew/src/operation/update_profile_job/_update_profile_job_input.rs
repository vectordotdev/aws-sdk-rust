// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateProfileJobInput {
    /// <p>Configuration for profile jobs. Used to select columns, do evaluations, and override default parameters of evaluations. When configuration is null, the profile job will run with default settings.</p>
    #[doc(hidden)]
    pub configuration: ::std::option::Option<crate::types::ProfileConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the job.</p>
    #[doc(hidden)]
    pub encryption_key_arn: ::std::option::Option<::std::string::String>,
    /// <p>The encryption mode for the job, which can be one of the following:</p>
    /// <ul>
    /// <li> <p> <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p> </li>
    /// <li> <p> <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub encryption_mode: ::std::option::Option<crate::types::EncryptionMode>,
    /// <p>The name of the job to be updated.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled, CloudWatch writes one log stream for each job run.</p>
    #[doc(hidden)]
    pub log_subscription: ::std::option::Option<crate::types::LogSubscription>,
    /// <p>The maximum number of compute nodes that DataBrew can use when the job processes data.</p>
    #[doc(hidden)]
    pub max_capacity: i32,
    /// <p>The maximum number of times to retry the job after a job run fails.</p>
    #[doc(hidden)]
    pub max_retries: i32,
    /// <p>Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read input data, or write output from a job.</p>
    #[doc(hidden)]
    pub output_location: ::std::option::Option<crate::types::S3Location>,
    /// <p>List of validation configurations that are applied to the profile job.</p>
    #[doc(hidden)]
    pub validation_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::ValidationConfiguration>>,
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to be assumed when DataBrew runs the job.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The job's timeout in minutes. A job that attempts to run longer than this timeout period ends with a status of <code>TIMEOUT</code>.</p>
    #[doc(hidden)]
    pub timeout: i32,
    /// <p>Sample configuration for Profile Jobs only. Determines the number of rows on which the Profile job will be executed. If a JobSample value is not provided for profile jobs, the default value will be used. The default value is CUSTOM_ROWS for the mode parameter and 20000 for the size parameter.</p>
    #[doc(hidden)]
    pub job_sample: ::std::option::Option<crate::types::JobSample>,
}
impl UpdateProfileJobInput {
    /// <p>Configuration for profile jobs. Used to select columns, do evaluations, and override default parameters of evaluations. When configuration is null, the profile job will run with default settings.</p>
    pub fn configuration(&self) -> ::std::option::Option<&crate::types::ProfileConfiguration> {
        self.configuration.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the job.</p>
    pub fn encryption_key_arn(&self) -> ::std::option::Option<&str> {
        self.encryption_key_arn.as_deref()
    }
    /// <p>The encryption mode for the job, which can be one of the following:</p>
    /// <ul>
    /// <li> <p> <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p> </li>
    /// <li> <p> <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p> </li>
    /// </ul>
    pub fn encryption_mode(&self) -> ::std::option::Option<&crate::types::EncryptionMode> {
        self.encryption_mode.as_ref()
    }
    /// <p>The name of the job to be updated.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled, CloudWatch writes one log stream for each job run.</p>
    pub fn log_subscription(&self) -> ::std::option::Option<&crate::types::LogSubscription> {
        self.log_subscription.as_ref()
    }
    /// <p>The maximum number of compute nodes that DataBrew can use when the job processes data.</p>
    pub fn max_capacity(&self) -> i32 {
        self.max_capacity
    }
    /// <p>The maximum number of times to retry the job after a job run fails.</p>
    pub fn max_retries(&self) -> i32 {
        self.max_retries
    }
    /// <p>Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read input data, or write output from a job.</p>
    pub fn output_location(&self) -> ::std::option::Option<&crate::types::S3Location> {
        self.output_location.as_ref()
    }
    /// <p>List of validation configurations that are applied to the profile job.</p>
    pub fn validation_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::ValidationConfiguration]> {
        self.validation_configurations.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to be assumed when DataBrew runs the job.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The job's timeout in minutes. A job that attempts to run longer than this timeout period ends with a status of <code>TIMEOUT</code>.</p>
    pub fn timeout(&self) -> i32 {
        self.timeout
    }
    /// <p>Sample configuration for Profile Jobs only. Determines the number of rows on which the Profile job will be executed. If a JobSample value is not provided for profile jobs, the default value will be used. The default value is CUSTOM_ROWS for the mode parameter and 20000 for the size parameter.</p>
    pub fn job_sample(&self) -> ::std::option::Option<&crate::types::JobSample> {
        self.job_sample.as_ref()
    }
}
impl UpdateProfileJobInput {
    /// Creates a new builder-style object to manufacture [`UpdateProfileJobInput`](crate::operation::update_profile_job::UpdateProfileJobInput).
    pub fn builder() -> crate::operation::update_profile_job::builders::UpdateProfileJobInputBuilder
    {
        crate::operation::update_profile_job::builders::UpdateProfileJobInputBuilder::default()
    }
}

/// A builder for [`UpdateProfileJobInput`](crate::operation::update_profile_job::UpdateProfileJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateProfileJobInputBuilder {
    pub(crate) configuration: ::std::option::Option<crate::types::ProfileConfiguration>,
    pub(crate) encryption_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_mode: ::std::option::Option<crate::types::EncryptionMode>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) log_subscription: ::std::option::Option<crate::types::LogSubscription>,
    pub(crate) max_capacity: ::std::option::Option<i32>,
    pub(crate) max_retries: ::std::option::Option<i32>,
    pub(crate) output_location: ::std::option::Option<crate::types::S3Location>,
    pub(crate) validation_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::ValidationConfiguration>>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) timeout: ::std::option::Option<i32>,
    pub(crate) job_sample: ::std::option::Option<crate::types::JobSample>,
}
impl UpdateProfileJobInputBuilder {
    /// <p>Configuration for profile jobs. Used to select columns, do evaluations, and override default parameters of evaluations. When configuration is null, the profile job will run with default settings.</p>
    pub fn configuration(mut self, input: crate::types::ProfileConfiguration) -> Self {
        self.configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration for profile jobs. Used to select columns, do evaluations, and override default parameters of evaluations. When configuration is null, the profile job will run with default settings.</p>
    pub fn set_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ProfileConfiguration>,
    ) -> Self {
        self.configuration = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the job.</p>
    pub fn encryption_key_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.encryption_key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the job.</p>
    pub fn set_encryption_key_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.encryption_key_arn = input;
        self
    }
    /// <p>The encryption mode for the job, which can be one of the following:</p>
    /// <ul>
    /// <li> <p> <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p> </li>
    /// <li> <p> <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p> </li>
    /// </ul>
    pub fn encryption_mode(mut self, input: crate::types::EncryptionMode) -> Self {
        self.encryption_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption mode for the job, which can be one of the following:</p>
    /// <ul>
    /// <li> <p> <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p> </li>
    /// <li> <p> <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p> </li>
    /// </ul>
    pub fn set_encryption_mode(
        mut self,
        input: ::std::option::Option<crate::types::EncryptionMode>,
    ) -> Self {
        self.encryption_mode = input;
        self
    }
    /// <p>The name of the job to be updated.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the job to be updated.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled, CloudWatch writes one log stream for each job run.</p>
    pub fn log_subscription(mut self, input: crate::types::LogSubscription) -> Self {
        self.log_subscription = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled, CloudWatch writes one log stream for each job run.</p>
    pub fn set_log_subscription(
        mut self,
        input: ::std::option::Option<crate::types::LogSubscription>,
    ) -> Self {
        self.log_subscription = input;
        self
    }
    /// <p>The maximum number of compute nodes that DataBrew can use when the job processes data.</p>
    pub fn max_capacity(mut self, input: i32) -> Self {
        self.max_capacity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of compute nodes that DataBrew can use when the job processes data.</p>
    pub fn set_max_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_capacity = input;
        self
    }
    /// <p>The maximum number of times to retry the job after a job run fails.</p>
    pub fn max_retries(mut self, input: i32) -> Self {
        self.max_retries = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of times to retry the job after a job run fails.</p>
    pub fn set_max_retries(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_retries = input;
        self
    }
    /// <p>Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read input data, or write output from a job.</p>
    pub fn output_location(mut self, input: crate::types::S3Location) -> Self {
        self.output_location = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read input data, or write output from a job.</p>
    pub fn set_output_location(
        mut self,
        input: ::std::option::Option<crate::types::S3Location>,
    ) -> Self {
        self.output_location = input;
        self
    }
    /// Appends an item to `validation_configurations`.
    ///
    /// To override the contents of this collection use [`set_validation_configurations`](Self::set_validation_configurations).
    ///
    /// <p>List of validation configurations that are applied to the profile job.</p>
    pub fn validation_configurations(
        mut self,
        input: crate::types::ValidationConfiguration,
    ) -> Self {
        let mut v = self.validation_configurations.unwrap_or_default();
        v.push(input);
        self.validation_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of validation configurations that are applied to the profile job.</p>
    pub fn set_validation_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ValidationConfiguration>>,
    ) -> Self {
        self.validation_configurations = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to be assumed when DataBrew runs the job.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to be assumed when DataBrew runs the job.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The job's timeout in minutes. A job that attempts to run longer than this timeout period ends with a status of <code>TIMEOUT</code>.</p>
    pub fn timeout(mut self, input: i32) -> Self {
        self.timeout = ::std::option::Option::Some(input);
        self
    }
    /// <p>The job's timeout in minutes. A job that attempts to run longer than this timeout period ends with a status of <code>TIMEOUT</code>.</p>
    pub fn set_timeout(mut self, input: ::std::option::Option<i32>) -> Self {
        self.timeout = input;
        self
    }
    /// <p>Sample configuration for Profile Jobs only. Determines the number of rows on which the Profile job will be executed. If a JobSample value is not provided for profile jobs, the default value will be used. The default value is CUSTOM_ROWS for the mode parameter and 20000 for the size parameter.</p>
    pub fn job_sample(mut self, input: crate::types::JobSample) -> Self {
        self.job_sample = ::std::option::Option::Some(input);
        self
    }
    /// <p>Sample configuration for Profile Jobs only. Determines the number of rows on which the Profile job will be executed. If a JobSample value is not provided for profile jobs, the default value will be used. The default value is CUSTOM_ROWS for the mode parameter and 20000 for the size parameter.</p>
    pub fn set_job_sample(mut self, input: ::std::option::Option<crate::types::JobSample>) -> Self {
        self.job_sample = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateProfileJobInput`](crate::operation::update_profile_job::UpdateProfileJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_profile_job::UpdateProfileJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_profile_job::UpdateProfileJobInput {
                configuration: self.configuration,
                encryption_key_arn: self.encryption_key_arn,
                encryption_mode: self.encryption_mode,
                name: self.name,
                log_subscription: self.log_subscription,
                max_capacity: self.max_capacity.unwrap_or_default(),
                max_retries: self.max_retries.unwrap_or_default(),
                output_location: self.output_location,
                validation_configurations: self.validation_configurations,
                role_arn: self.role_arn,
                timeout: self.timeout.unwrap_or_default(),
                job_sample: self.job_sample,
            },
        )
    }
}
