// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_dataset_entries::_update_dataset_entries_output::UpdateDatasetEntriesOutputBuilder;

pub use crate::operation::update_dataset_entries::_update_dataset_entries_input::UpdateDatasetEntriesInputBuilder;

/// Fluent builder constructing a request to `UpdateDatasetEntries`.
///
/// <p>Adds or updates one or more entries (images) in a dataset. An entry is a JSON Line which contains the information for a single image, including the image location, assigned labels, and object location bounding boxes. For more information, see Image-Level labels in manifest files and Object localization in manifest files in the <i>Amazon Rekognition Custom Labels Developer Guide</i>. </p>
/// <p>If the <code>source-ref</code> field in the JSON line references an existing image, the existing image in the dataset is updated. If <code>source-ref</code> field doesn't reference an existing image, the image is added as a new image to the dataset. </p>
/// <p>You specify the changes that you want to make in the <code>Changes</code> input parameter. There isn't a limit to the number JSON Lines that you can change, but the size of <code>Changes</code> must be less than 5MB.</p>
/// <p> <code>UpdateDatasetEntries</code> returns immediatly, but the dataset update might take a while to complete. Use <code>DescribeDataset</code> to check the current status. The dataset updated successfully if the value of <code>Status</code> is <code>UPDATE_COMPLETE</code>. </p>
/// <p>To check if any non-terminal errors occured, call <code>ListDatasetEntries</code> and check for the presence of <code>errors</code> lists in the JSON Lines.</p>
/// <p>Dataset update fails if a terminal error occurs (<code>Status</code> = <code>UPDATE_FAILED</code>). Currently, you can't access the terminal error information from the Amazon Rekognition Custom Labels SDK. </p>
/// <p>This operation requires permissions to perform the <code>rekognition:UpdateDatasetEntries</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDatasetEntriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_dataset_entries::builders::UpdateDatasetEntriesInputBuilder,
}
impl UpdateDatasetEntriesFluentBuilder {
    /// Creates a new `UpdateDatasetEntries`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_dataset_entries::UpdateDatasetEntries,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_dataset_entries::UpdateDatasetEntriesError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_dataset_entries::UpdateDatasetEntriesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_dataset_entries::UpdateDatasetEntriesError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_dataset_entries::UpdateDatasetEntriesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_dataset_entries::UpdateDatasetEntriesError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_dataset_entries::UpdateDatasetEntries,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_dataset_entries::UpdateDatasetEntriesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p> The Amazon Resource Name (ARN) of the dataset that you want to update. </p>
    pub fn dataset_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_arn(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the dataset that you want to update. </p>
    pub fn set_dataset_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_arn(input);
        self
    }
    /// <p> The changes that you want to make to the dataset. </p>
    pub fn changes(mut self, input: crate::types::DatasetChanges) -> Self {
        self.inner = self.inner.changes(input);
        self
    }
    /// <p> The changes that you want to make to the dataset. </p>
    pub fn set_changes(
        mut self,
        input: ::std::option::Option<crate::types::DatasetChanges>,
    ) -> Self {
        self.inner = self.inner.set_changes(input);
        self
    }
}
