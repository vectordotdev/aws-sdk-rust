// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetContainerRecipePolicy`](crate::operation::get_container_recipe_policy::builders::GetContainerRecipePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`container_recipe_arn(impl ::std::convert::Into<String>)`](crate::operation::get_container_recipe_policy::builders::GetContainerRecipePolicyFluentBuilder::container_recipe_arn) / [`set_container_recipe_arn(Option<String>)`](crate::operation::get_container_recipe_policy::builders::GetContainerRecipePolicyFluentBuilder::set_container_recipe_arn): <p>The Amazon Resource Name (ARN) of the container recipe for the policy being requested.</p>
    /// - On success, responds with [`GetContainerRecipePolicyOutput`](crate::operation::get_container_recipe_policy::GetContainerRecipePolicyOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::get_container_recipe_policy::GetContainerRecipePolicyOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`policy(Option<String>)`](crate::operation::get_container_recipe_policy::GetContainerRecipePolicyOutput::policy): <p>The container recipe policy object that is returned.</p>
    /// - On failure, responds with [`SdkError<GetContainerRecipePolicyError>`](crate::operation::get_container_recipe_policy::GetContainerRecipePolicyError)
    pub fn get_container_recipe_policy(&self) -> crate::operation::get_container_recipe_policy::builders::GetContainerRecipePolicyFluentBuilder{
        crate::operation::get_container_recipe_policy::builders::GetContainerRecipePolicyFluentBuilder::new(self.handle.clone())
    }
}
