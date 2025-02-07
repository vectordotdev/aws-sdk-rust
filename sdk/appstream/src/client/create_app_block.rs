// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAppBlock`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::set_name): <p>The name of the app block.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::set_description): <p>The description of the app block.</p>
    ///   - [`display_name(impl ::std::convert::Into<String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::display_name) / [`set_display_name(Option<String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::set_display_name): <p>The display name of the app block. This is not displayed to the user.</p>
    ///   - [`source_s3_location(S3Location)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::source_s3_location) / [`set_source_s3_location(Option<S3Location>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::set_source_s3_location): <p>The source S3 location of the app block.</p>
    ///   - [`setup_script_details(ScriptDetails)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::setup_script_details) / [`set_setup_script_details(Option<ScriptDetails>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::set_setup_script_details): <p>The setup script details of the app block.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::set_tags): <p>The tags assigned to the app block.</p>
    /// - On success, responds with [`CreateAppBlockOutput`](crate::operation::create_app_block::CreateAppBlockOutput) with field(s):
    ///   - [`app_block(Option<AppBlock>)`](crate::operation::create_app_block::CreateAppBlockOutput::app_block): <p>The app block.</p>
    /// - On failure, responds with [`SdkError<CreateAppBlockError>`](crate::operation::create_app_block::CreateAppBlockError)
    pub fn create_app_block(
        &self,
    ) -> crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder {
        crate::operation::create_app_block::builders::CreateAppBlockFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
