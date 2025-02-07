// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a bundle task.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BundleTask {
    /// <p>The ID of the bundle task.</p>
    #[doc(hidden)]
    pub bundle_id: ::std::option::Option<::std::string::String>,
    /// <p>If the task fails, a description of the error.</p>
    #[doc(hidden)]
    pub bundle_task_error: ::std::option::Option<crate::types::BundleTaskError>,
    /// <p>The ID of the instance associated with this bundle task.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The level of task completion, as a percent (for example, 20%).</p>
    #[doc(hidden)]
    pub progress: ::std::option::Option<::std::string::String>,
    /// <p>The time this task started.</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The state of the task.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::BundleTaskState>,
    /// <p>The Amazon S3 storage locations.</p>
    #[doc(hidden)]
    pub storage: ::std::option::Option<crate::types::Storage>,
    /// <p>The time of the most recent update for the task.</p>
    #[doc(hidden)]
    pub update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BundleTask {
    /// <p>The ID of the bundle task.</p>
    pub fn bundle_id(&self) -> ::std::option::Option<&str> {
        self.bundle_id.as_deref()
    }
    /// <p>If the task fails, a description of the error.</p>
    pub fn bundle_task_error(&self) -> ::std::option::Option<&crate::types::BundleTaskError> {
        self.bundle_task_error.as_ref()
    }
    /// <p>The ID of the instance associated with this bundle task.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The level of task completion, as a percent (for example, 20%).</p>
    pub fn progress(&self) -> ::std::option::Option<&str> {
        self.progress.as_deref()
    }
    /// <p>The time this task started.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The state of the task.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::BundleTaskState> {
        self.state.as_ref()
    }
    /// <p>The Amazon S3 storage locations.</p>
    pub fn storage(&self) -> ::std::option::Option<&crate::types::Storage> {
        self.storage.as_ref()
    }
    /// <p>The time of the most recent update for the task.</p>
    pub fn update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
}
impl BundleTask {
    /// Creates a new builder-style object to manufacture [`BundleTask`](crate::types::BundleTask).
    pub fn builder() -> crate::types::builders::BundleTaskBuilder {
        crate::types::builders::BundleTaskBuilder::default()
    }
}

/// A builder for [`BundleTask`](crate::types::BundleTask).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BundleTaskBuilder {
    pub(crate) bundle_id: ::std::option::Option<::std::string::String>,
    pub(crate) bundle_task_error: ::std::option::Option<crate::types::BundleTaskError>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) progress: ::std::option::Option<::std::string::String>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) state: ::std::option::Option<crate::types::BundleTaskState>,
    pub(crate) storage: ::std::option::Option<crate::types::Storage>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BundleTaskBuilder {
    /// <p>The ID of the bundle task.</p>
    pub fn bundle_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bundle_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the bundle task.</p>
    pub fn set_bundle_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bundle_id = input;
        self
    }
    /// <p>If the task fails, a description of the error.</p>
    pub fn bundle_task_error(mut self, input: crate::types::BundleTaskError) -> Self {
        self.bundle_task_error = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the task fails, a description of the error.</p>
    pub fn set_bundle_task_error(
        mut self,
        input: ::std::option::Option<crate::types::BundleTaskError>,
    ) -> Self {
        self.bundle_task_error = input;
        self
    }
    /// <p>The ID of the instance associated with this bundle task.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance associated with this bundle task.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The level of task completion, as a percent (for example, 20%).</p>
    pub fn progress(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.progress = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The level of task completion, as a percent (for example, 20%).</p>
    pub fn set_progress(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.progress = input;
        self
    }
    /// <p>The time this task started.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time this task started.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The state of the task.</p>
    pub fn state(mut self, input: crate::types::BundleTaskState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the task.</p>
    pub fn set_state(
        mut self,
        input: ::std::option::Option<crate::types::BundleTaskState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The Amazon S3 storage locations.</p>
    pub fn storage(mut self, input: crate::types::Storage) -> Self {
        self.storage = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon S3 storage locations.</p>
    pub fn set_storage(mut self, input: ::std::option::Option<crate::types::Storage>) -> Self {
        self.storage = input;
        self
    }
    /// <p>The time of the most recent update for the task.</p>
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time of the most recent update for the task.</p>
    pub fn set_update_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_time = input;
        self
    }
    /// Consumes the builder and constructs a [`BundleTask`](crate::types::BundleTask).
    pub fn build(self) -> crate::types::BundleTask {
        crate::types::BundleTask {
            bundle_id: self.bundle_id,
            bundle_task_error: self.bundle_task_error,
            instance_id: self.instance_id,
            progress: self.progress,
            start_time: self.start_time,
            state: self.state,
            storage: self.storage,
            update_time: self.update_time,
        }
    }
}
