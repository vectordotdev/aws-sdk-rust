// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(crate::types::error::InternalServiceError),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequestException(crate::types::error::InvalidRequestException),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeletedException(crate::types::error::PipelineDeletedException),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFoundException(crate::types::error::PipelineNotFoundException),
    /// <p>The specified task was not found. </p>
    TaskNotFoundException(crate::types::error::TaskNotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServiceError(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::PipelineDeletedException(inner) => inner.fmt(f),
            Error::PipelineNotFoundException(inner) => inner.fmt(f),
            Error::TaskNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::activate_pipeline::ActivatePipelineError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::activate_pipeline::ActivatePipelineError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::activate_pipeline::ActivatePipelineError> for Error {
    fn from(err: crate::operation::activate_pipeline::ActivatePipelineError) -> Self {
        match err {
            crate::operation::activate_pipeline::ActivatePipelineError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::activate_pipeline::ActivatePipelineError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::activate_pipeline::ActivatePipelineError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::activate_pipeline::ActivatePipelineError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::activate_pipeline::ActivatePipelineError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::add_tags::AddTagsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::add_tags::AddTagsError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::add_tags::AddTagsError> for Error {
    fn from(err: crate::operation::add_tags::AddTagsError) -> Self {
        match err {
            crate::operation::add_tags::AddTagsError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::add_tags::AddTagsError::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::operation::add_tags::AddTagsError::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::operation::add_tags::AddTagsError::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::operation::add_tags::AddTagsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_pipeline::CreatePipelineError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::create_pipeline::CreatePipelineError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::create_pipeline::CreatePipelineError> for Error {
    fn from(err: crate::operation::create_pipeline::CreatePipelineError) -> Self {
        match err {
            crate::operation::create_pipeline::CreatePipelineError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::create_pipeline::CreatePipelineError::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::operation::create_pipeline::CreatePipelineError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::deactivate_pipeline::DeactivatePipelineError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::deactivate_pipeline::DeactivatePipelineError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::deactivate_pipeline::DeactivatePipelineError> for Error {
    fn from(err: crate::operation::deactivate_pipeline::DeactivatePipelineError) -> Self {
        match err {
            crate::operation::deactivate_pipeline::DeactivatePipelineError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::deactivate_pipeline::DeactivatePipelineError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::deactivate_pipeline::DeactivatePipelineError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::deactivate_pipeline::DeactivatePipelineError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::deactivate_pipeline::DeactivatePipelineError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_pipeline::DeletePipelineError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::delete_pipeline::DeletePipelineError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::delete_pipeline::DeletePipelineError> for Error {
    fn from(err: crate::operation::delete_pipeline::DeletePipelineError) -> Self {
        match err {
            crate::operation::delete_pipeline::DeletePipelineError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::delete_pipeline::DeletePipelineError::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::operation::delete_pipeline::DeletePipelineError::PipelineNotFoundException(
                inner,
            ) => Error::PipelineNotFoundException(inner),
            crate::operation::delete_pipeline::DeletePipelineError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_objects::DescribeObjectsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_objects::DescribeObjectsError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_objects::DescribeObjectsError> for Error {
    fn from(err: crate::operation::describe_objects::DescribeObjectsError) -> Self {
        match err {
            crate::operation::describe_objects::DescribeObjectsError::InternalServiceError(
                inner,
            ) => Error::InternalServiceError(inner),
            crate::operation::describe_objects::DescribeObjectsError::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::operation::describe_objects::DescribeObjectsError::PipelineDeletedException(
                inner,
            ) => Error::PipelineDeletedException(inner),
            crate::operation::describe_objects::DescribeObjectsError::PipelineNotFoundException(
                inner,
            ) => Error::PipelineNotFoundException(inner),
            crate::operation::describe_objects::DescribeObjectsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_pipelines::DescribePipelinesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::describe_pipelines::DescribePipelinesError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_pipelines::DescribePipelinesError> for Error {
    fn from(err: crate::operation::describe_pipelines::DescribePipelinesError) -> Self {
        match err {
            crate::operation::describe_pipelines::DescribePipelinesError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::describe_pipelines::DescribePipelinesError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::describe_pipelines::DescribePipelinesError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::describe_pipelines::DescribePipelinesError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::describe_pipelines::DescribePipelinesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::evaluate_expression::EvaluateExpressionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::evaluate_expression::EvaluateExpressionError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::evaluate_expression::EvaluateExpressionError> for Error {
    fn from(err: crate::operation::evaluate_expression::EvaluateExpressionError) -> Self {
        match err {
            crate::operation::evaluate_expression::EvaluateExpressionError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::evaluate_expression::EvaluateExpressionError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::evaluate_expression::EvaluateExpressionError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::evaluate_expression::EvaluateExpressionError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::evaluate_expression::EvaluateExpressionError::TaskNotFoundException(inner) => Error::TaskNotFoundException(inner),
            crate::operation::evaluate_expression::EvaluateExpressionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_pipeline_definition::GetPipelineDefinitionError> for Error {
    fn from(err: crate::operation::get_pipeline_definition::GetPipelineDefinitionError) -> Self {
        match err {
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::get_pipeline_definition::GetPipelineDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_pipelines::ListPipelinesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_pipelines::ListPipelinesError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_pipelines::ListPipelinesError> for Error {
    fn from(err: crate::operation::list_pipelines::ListPipelinesError) -> Self {
        match err {
            crate::operation::list_pipelines::ListPipelinesError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::list_pipelines::ListPipelinesError::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::operation::list_pipelines::ListPipelinesError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::poll_for_task::PollForTaskError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::poll_for_task::PollForTaskError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::poll_for_task::PollForTaskError> for Error {
    fn from(err: crate::operation::poll_for_task::PollForTaskError) -> Self {
        match err {
            crate::operation::poll_for_task::PollForTaskError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::poll_for_task::PollForTaskError::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::operation::poll_for_task::PollForTaskError::TaskNotFoundException(inner) => {
                Error::TaskNotFoundException(inner)
            }
            crate::operation::poll_for_task::PollForTaskError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::put_pipeline_definition::PutPipelineDefinitionError> for Error {
    fn from(err: crate::operation::put_pipeline_definition::PutPipelineDefinitionError) -> Self {
        match err {
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::put_pipeline_definition::PutPipelineDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<::aws_smithy_http::result::SdkError<crate::operation::query_objects::QueryObjectsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::query_objects::QueryObjectsError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::query_objects::QueryObjectsError> for Error {
    fn from(err: crate::operation::query_objects::QueryObjectsError) -> Self {
        match err {
            crate::operation::query_objects::QueryObjectsError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::query_objects::QueryObjectsError::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::operation::query_objects::QueryObjectsError::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::operation::query_objects::QueryObjectsError::PipelineNotFoundException(
                inner,
            ) => Error::PipelineNotFoundException(inner),
            crate::operation::query_objects::QueryObjectsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::remove_tags::RemoveTagsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::remove_tags::RemoveTagsError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::remove_tags::RemoveTagsError> for Error {
    fn from(err: crate::operation::remove_tags::RemoveTagsError) -> Self {
        match err {
            crate::operation::remove_tags::RemoveTagsError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::remove_tags::RemoveTagsError::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::operation::remove_tags::RemoveTagsError::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::operation::remove_tags::RemoveTagsError::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::operation::remove_tags::RemoveTagsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::report_task_progress::ReportTaskProgressError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::report_task_progress::ReportTaskProgressError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::report_task_progress::ReportTaskProgressError> for Error {
    fn from(err: crate::operation::report_task_progress::ReportTaskProgressError) -> Self {
        match err {
            crate::operation::report_task_progress::ReportTaskProgressError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::report_task_progress::ReportTaskProgressError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::report_task_progress::ReportTaskProgressError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::report_task_progress::ReportTaskProgressError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::report_task_progress::ReportTaskProgressError::TaskNotFoundException(inner) => Error::TaskNotFoundException(inner),
            crate::operation::report_task_progress::ReportTaskProgressError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError>
    for Error
{
    fn from(
        err: crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError,
    ) -> Self {
        match err {
            crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::report_task_runner_heartbeat::ReportTaskRunnerHeartbeatError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::set_status::SetStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::set_status::SetStatusError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::set_status::SetStatusError> for Error {
    fn from(err: crate::operation::set_status::SetStatusError) -> Self {
        match err {
            crate::operation::set_status::SetStatusError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::set_status::SetStatusError::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::operation::set_status::SetStatusError::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::operation::set_status::SetStatusError::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::operation::set_status::SetStatusError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::set_task_status::SetTaskStatusError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::set_task_status::SetTaskStatusError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::set_task_status::SetTaskStatusError> for Error {
    fn from(err: crate::operation::set_task_status::SetTaskStatusError) -> Self {
        match err {
            crate::operation::set_task_status::SetTaskStatusError::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::operation::set_task_status::SetTaskStatusError::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::operation::set_task_status::SetTaskStatusError::PipelineDeletedException(
                inner,
            ) => Error::PipelineDeletedException(inner),
            crate::operation::set_task_status::SetTaskStatusError::PipelineNotFoundException(
                inner,
            ) => Error::PipelineNotFoundException(inner),
            crate::operation::set_task_status::SetTaskStatusError::TaskNotFoundException(inner) => {
                Error::TaskNotFoundException(inner)
            }
            crate::operation::set_task_status::SetTaskStatusError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError>
    for Error
{
    fn from(
        err: crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError,
    ) -> Self {
        match err {
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError::InternalServiceError(inner) => Error::InternalServiceError(inner),
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError::PipelineDeletedException(inner) => Error::PipelineDeletedException(inner),
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError::PipelineNotFoundException(inner) => Error::PipelineNotFoundException(inner),
            crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::InternalServiceError(inner) => inner.source(),
            Error::InvalidRequestException(inner) => inner.source(),
            Error::PipelineDeletedException(inner) => inner.source(),
            Error::PipelineNotFoundException(inner) => inner.source(),
            Error::TaskNotFoundException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::InternalServiceError(e) => e.request_id(),
            Self::InvalidRequestException(e) => e.request_id(),
            Self::PipelineDeletedException(e) => e.request_id(),
            Self::PipelineNotFoundException(e) => e.request_id(),
            Self::TaskNotFoundException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
