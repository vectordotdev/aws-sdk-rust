// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateEndpointConfigInput {
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    #[doc(hidden)]
    pub endpoint_config_name: ::std::option::Option<::std::string::String>,
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    #[doc(hidden)]
    pub production_variants:
        ::std::option::Option<::std::vec::Vec<crate::types::ProductionVariant>>,
    /// <p>Configuration to control how SageMaker captures inference data.</p>
    #[doc(hidden)]
    pub data_capture_config: ::std::option::Option<crate::types::DataCaptureConfig>,
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>
    /// <p>The KmsKeyId can be any of the following formats: </p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the Amazon Web Services Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in Amazon Web Services KMS </a> </p> <note>
    /// <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can't request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p>
    /// <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p>
    /// <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p>
    /// </note>
    #[doc(hidden)]
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies configuration for how an endpoint performs asynchronous inference. This is a required field in order for your Endpoint to be invoked using <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpointAsync.html">InvokeEndpointAsync</a>.</p>
    #[doc(hidden)]
    pub async_inference_config: ::std::option::Option<crate::types::AsyncInferenceConfig>,
    /// <p>A member of <code>CreateEndpointConfig</code> that enables explainers.</p>
    #[doc(hidden)]
    pub explainer_config: ::std::option::Option<crate::types::ExplainerConfig>,
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>. If you use this field, you can only specify one variant for <code>ProductionVariants</code> and one variant for <code>ShadowProductionVariants</code>.</p>
    #[doc(hidden)]
    pub shadow_production_variants:
        ::std::option::Option<::std::vec::Vec<crate::types::ProductionVariant>>,
}
impl CreateEndpointConfigInput {
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    pub fn endpoint_config_name(&self) -> ::std::option::Option<&str> {
        self.endpoint_config_name.as_deref()
    }
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    pub fn production_variants(&self) -> ::std::option::Option<&[crate::types::ProductionVariant]> {
        self.production_variants.as_deref()
    }
    /// <p>Configuration to control how SageMaker captures inference data.</p>
    pub fn data_capture_config(&self) -> ::std::option::Option<&crate::types::DataCaptureConfig> {
        self.data_capture_config.as_ref()
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>
    /// <p>The KmsKeyId can be any of the following formats: </p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the Amazon Web Services Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in Amazon Web Services KMS </a> </p> <note>
    /// <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can't request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p>
    /// <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p>
    /// <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p>
    /// </note>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>Specifies configuration for how an endpoint performs asynchronous inference. This is a required field in order for your Endpoint to be invoked using <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpointAsync.html">InvokeEndpointAsync</a>.</p>
    pub fn async_inference_config(
        &self,
    ) -> ::std::option::Option<&crate::types::AsyncInferenceConfig> {
        self.async_inference_config.as_ref()
    }
    /// <p>A member of <code>CreateEndpointConfig</code> that enables explainers.</p>
    pub fn explainer_config(&self) -> ::std::option::Option<&crate::types::ExplainerConfig> {
        self.explainer_config.as_ref()
    }
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>. If you use this field, you can only specify one variant for <code>ProductionVariants</code> and one variant for <code>ShadowProductionVariants</code>.</p>
    pub fn shadow_production_variants(
        &self,
    ) -> ::std::option::Option<&[crate::types::ProductionVariant]> {
        self.shadow_production_variants.as_deref()
    }
}
impl CreateEndpointConfigInput {
    /// Creates a new builder-style object to manufacture [`CreateEndpointConfigInput`](crate::operation::create_endpoint_config::CreateEndpointConfigInput).
    pub fn builder(
    ) -> crate::operation::create_endpoint_config::builders::CreateEndpointConfigInputBuilder {
        crate::operation::create_endpoint_config::builders::CreateEndpointConfigInputBuilder::default()
    }
}

/// A builder for [`CreateEndpointConfigInput`](crate::operation::create_endpoint_config::CreateEndpointConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateEndpointConfigInputBuilder {
    pub(crate) endpoint_config_name: ::std::option::Option<::std::string::String>,
    pub(crate) production_variants:
        ::std::option::Option<::std::vec::Vec<crate::types::ProductionVariant>>,
    pub(crate) data_capture_config: ::std::option::Option<crate::types::DataCaptureConfig>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) async_inference_config: ::std::option::Option<crate::types::AsyncInferenceConfig>,
    pub(crate) explainer_config: ::std::option::Option<crate::types::ExplainerConfig>,
    pub(crate) shadow_production_variants:
        ::std::option::Option<::std::vec::Vec<crate::types::ProductionVariant>>,
}
impl CreateEndpointConfigInputBuilder {
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    pub fn endpoint_config_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.endpoint_config_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    pub fn set_endpoint_config_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.endpoint_config_name = input;
        self
    }
    /// Appends an item to `production_variants`.
    ///
    /// To override the contents of this collection use [`set_production_variants`](Self::set_production_variants).
    ///
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    pub fn production_variants(mut self, input: crate::types::ProductionVariant) -> Self {
        let mut v = self.production_variants.unwrap_or_default();
        v.push(input);
        self.production_variants = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    pub fn set_production_variants(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ProductionVariant>>,
    ) -> Self {
        self.production_variants = input;
        self
    }
    /// <p>Configuration to control how SageMaker captures inference data.</p>
    pub fn data_capture_config(mut self, input: crate::types::DataCaptureConfig) -> Self {
        self.data_capture_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration to control how SageMaker captures inference data.</p>
    pub fn set_data_capture_config(
        mut self,
        input: ::std::option::Option<crate::types::DataCaptureConfig>,
    ) -> Self {
        self.data_capture_config = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>
    /// <p>The KmsKeyId can be any of the following formats: </p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the Amazon Web Services Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in Amazon Web Services KMS </a> </p> <note>
    /// <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can't request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p>
    /// <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p>
    /// <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p>
    /// </note>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>
    /// <p>The KmsKeyId can be any of the following formats: </p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the Amazon Web Services Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in Amazon Web Services KMS </a> </p> <note>
    /// <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can't request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p>
    /// <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p>
    /// <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p>
    /// </note>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>Specifies configuration for how an endpoint performs asynchronous inference. This is a required field in order for your Endpoint to be invoked using <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpointAsync.html">InvokeEndpointAsync</a>.</p>
    pub fn async_inference_config(mut self, input: crate::types::AsyncInferenceConfig) -> Self {
        self.async_inference_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies configuration for how an endpoint performs asynchronous inference. This is a required field in order for your Endpoint to be invoked using <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpointAsync.html">InvokeEndpointAsync</a>.</p>
    pub fn set_async_inference_config(
        mut self,
        input: ::std::option::Option<crate::types::AsyncInferenceConfig>,
    ) -> Self {
        self.async_inference_config = input;
        self
    }
    /// <p>A member of <code>CreateEndpointConfig</code> that enables explainers.</p>
    pub fn explainer_config(mut self, input: crate::types::ExplainerConfig) -> Self {
        self.explainer_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>A member of <code>CreateEndpointConfig</code> that enables explainers.</p>
    pub fn set_explainer_config(
        mut self,
        input: ::std::option::Option<crate::types::ExplainerConfig>,
    ) -> Self {
        self.explainer_config = input;
        self
    }
    /// Appends an item to `shadow_production_variants`.
    ///
    /// To override the contents of this collection use [`set_shadow_production_variants`](Self::set_shadow_production_variants).
    ///
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>. If you use this field, you can only specify one variant for <code>ProductionVariants</code> and one variant for <code>ShadowProductionVariants</code>.</p>
    pub fn shadow_production_variants(mut self, input: crate::types::ProductionVariant) -> Self {
        let mut v = self.shadow_production_variants.unwrap_or_default();
        v.push(input);
        self.shadow_production_variants = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>. If you use this field, you can only specify one variant for <code>ProductionVariants</code> and one variant for <code>ShadowProductionVariants</code>.</p>
    pub fn set_shadow_production_variants(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ProductionVariant>>,
    ) -> Self {
        self.shadow_production_variants = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateEndpointConfigInput`](crate::operation::create_endpoint_config::CreateEndpointConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_endpoint_config::CreateEndpointConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_endpoint_config::CreateEndpointConfigInput {
                endpoint_config_name: self.endpoint_config_name,
                production_variants: self.production_variants,
                data_capture_config: self.data_capture_config,
                tags: self.tags,
                kms_key_id: self.kms_key_id,
                async_inference_config: self.async_inference_config,
                explainer_config: self.explainer_config,
                shadow_production_variants: self.shadow_production_variants,
            },
        )
    }
}
