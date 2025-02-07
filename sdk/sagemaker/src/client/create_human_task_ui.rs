// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateHumanTaskUi`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`human_task_ui_name(impl ::std::convert::Into<String>)`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::human_task_ui_name) / [`set_human_task_ui_name(Option<String>)`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::set_human_task_ui_name): <p>The name of the user interface you are creating.</p>
    ///   - [`ui_template(UiTemplate)`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::ui_template) / [`set_ui_template(Option<UiTemplate>)`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::set_ui_template): <p>The Liquid template for the worker user interface.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::set_tags): <p>An array of key-value pairs that contain metadata to help you categorize and organize a human review workflow user interface. Each tag consists of a key and a value, both of which you define.</p>
    /// - On success, responds with [`CreateHumanTaskUiOutput`](crate::operation::create_human_task_ui::CreateHumanTaskUiOutput) with field(s):
    ///   - [`human_task_ui_arn(Option<String>)`](crate::operation::create_human_task_ui::CreateHumanTaskUiOutput::human_task_ui_arn): <p>The Amazon Resource Name (ARN) of the human review workflow user interface you create.</p>
    /// - On failure, responds with [`SdkError<CreateHumanTaskUiError>`](crate::operation::create_human_task_ui::CreateHumanTaskUiError)
    pub fn create_human_task_ui(
        &self,
    ) -> crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder {
        crate::operation::create_human_task_ui::builders::CreateHumanTaskUiFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
