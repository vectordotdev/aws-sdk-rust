// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVocabulary`](crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`vocabulary_id(impl ::std::convert::Into<String>)`](crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder::vocabulary_id) / [`set_vocabulary_id(Option<String>)`](crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder::set_vocabulary_id): <p>The identifier of the custom vocabulary.</p>
    /// - On success, responds with [`DeleteVocabularyOutput`](crate::operation::delete_vocabulary::DeleteVocabularyOutput) with field(s):
    ///   - [`vocabulary_arn(Option<String>)`](crate::operation::delete_vocabulary::DeleteVocabularyOutput::vocabulary_arn): <p>The Amazon Resource Name (ARN) of the custom vocabulary.</p>
    ///   - [`vocabulary_id(Option<String>)`](crate::operation::delete_vocabulary::DeleteVocabularyOutput::vocabulary_id): <p>The identifier of the custom vocabulary.</p>
    ///   - [`state(Option<VocabularyState>)`](crate::operation::delete_vocabulary::DeleteVocabularyOutput::state): <p>The current state of the custom vocabulary.</p>
    /// - On failure, responds with [`SdkError<DeleteVocabularyError>`](crate::operation::delete_vocabulary::DeleteVocabularyError)
    pub fn delete_vocabulary(
        &self,
    ) -> crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder {
        crate::operation::delete_vocabulary::builders::DeleteVocabularyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
