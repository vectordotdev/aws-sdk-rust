// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteApp`](crate::operation::delete_app::builders::DeleteAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl ::std::convert::Into<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::set_domain_id): <p>The domain ID.</p>
    ///   - [`user_profile_name(impl ::std::convert::Into<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::user_profile_name) / [`set_user_profile_name(Option<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::set_user_profile_name): <p>The user profile name. If this value is not set, then <code>SpaceName</code> must be set.</p>
    ///   - [`app_type(AppType)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::app_type) / [`set_app_type(Option<AppType>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::set_app_type): <p>The type of app.</p>
    ///   - [`app_name(impl ::std::convert::Into<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::app_name) / [`set_app_name(Option<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::set_app_name): <p>The name of the app.</p>
    ///   - [`space_name(impl ::std::convert::Into<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::space_name) / [`set_space_name(Option<String>)`](crate::operation::delete_app::builders::DeleteAppFluentBuilder::set_space_name): <p>The name of the space. If this value is not set, then <code>UserProfileName</code> must be set.</p>
    /// - On success, responds with [`DeleteAppOutput`](crate::operation::delete_app::DeleteAppOutput)
    /// - On failure, responds with [`SdkError<DeleteAppError>`](crate::operation::delete_app::DeleteAppError)
    pub fn delete_app(&self) -> crate::operation::delete_app::builders::DeleteAppFluentBuilder {
        crate::operation::delete_app::builders::DeleteAppFluentBuilder::new(self.handle.clone())
    }
}
