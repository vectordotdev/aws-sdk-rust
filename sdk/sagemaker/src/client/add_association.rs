// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddAssociation`](crate::operation::add_association::builders::AddAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_arn(impl ::std::convert::Into<String>)`](crate::operation::add_association::builders::AddAssociationFluentBuilder::source_arn) / [`set_source_arn(Option<String>)`](crate::operation::add_association::builders::AddAssociationFluentBuilder::set_source_arn): <p>The ARN of the source.</p>
    ///   - [`destination_arn(impl ::std::convert::Into<String>)`](crate::operation::add_association::builders::AddAssociationFluentBuilder::destination_arn) / [`set_destination_arn(Option<String>)`](crate::operation::add_association::builders::AddAssociationFluentBuilder::set_destination_arn): <p>The Amazon Resource Name (ARN) of the destination.</p>
    ///   - [`association_type(AssociationEdgeType)`](crate::operation::add_association::builders::AddAssociationFluentBuilder::association_type) / [`set_association_type(Option<AssociationEdgeType>)`](crate::operation::add_association::builders::AddAssociationFluentBuilder::set_association_type): <p>The type of association. The following are suggested uses for each type. Amazon SageMaker places no restrictions on their use.</p>  <ul>   <li> <p>ContributedTo - The source contributed to the destination or had a part in enabling the destination. For example, the training data contributed to the training job.</p> </li>   <li> <p>AssociatedWith - The source is connected to the destination. For example, an approval workflow is associated with a model deployment.</p> </li>   <li> <p>DerivedFrom - The destination is a modification of the source. For example, a digest output of a channel input for a processing job is derived from the original inputs.</p> </li>   <li> <p>Produced - The source generated the destination. For example, a training job produced a model artifact.</p> </li>  </ul>
    /// - On success, responds with [`AddAssociationOutput`](crate::operation::add_association::AddAssociationOutput) with field(s):
    ///   - [`source_arn(Option<String>)`](crate::operation::add_association::AddAssociationOutput::source_arn): <p>The ARN of the source.</p>
    ///   - [`destination_arn(Option<String>)`](crate::operation::add_association::AddAssociationOutput::destination_arn): <p>The Amazon Resource Name (ARN) of the destination.</p>
    /// - On failure, responds with [`SdkError<AddAssociationError>`](crate::operation::add_association::AddAssociationError)
    pub fn add_association(
        &self,
    ) -> crate::operation::add_association::builders::AddAssociationFluentBuilder {
        crate::operation::add_association::builders::AddAssociationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
