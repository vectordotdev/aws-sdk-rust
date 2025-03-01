// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchSuspendUser`](crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`user_id_list(Vec<String>)`](crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder::user_id_list) / [`set_user_id_list(Option<Vec<String>>)`](crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder::set_user_id_list): <p>The request containing the user IDs to suspend.</p>
    /// - On success, responds with [`BatchSuspendUserOutput`](crate::operation::batch_suspend_user::BatchSuspendUserOutput) with field(s):
    ///   - [`user_errors(Option<Vec<UserError>>)`](crate::operation::batch_suspend_user::BatchSuspendUserOutput::user_errors): <p>If the <code>BatchSuspendUser</code> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    /// - On failure, responds with [`SdkError<BatchSuspendUserError>`](crate::operation::batch_suspend_user::BatchSuspendUserError)
    pub fn batch_suspend_user(
        &self,
    ) -> crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder {
        crate::operation::batch_suspend_user::builders::BatchSuspendUserFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
