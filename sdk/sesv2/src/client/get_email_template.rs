// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEmailTemplate`](crate::operation::get_email_template::builders::GetEmailTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_name(impl ::std::convert::Into<String>)`](crate::operation::get_email_template::builders::GetEmailTemplateFluentBuilder::template_name) / [`set_template_name(Option<String>)`](crate::operation::get_email_template::builders::GetEmailTemplateFluentBuilder::set_template_name): <p>The name of the template.</p>
    /// - On success, responds with [`GetEmailTemplateOutput`](crate::operation::get_email_template::GetEmailTemplateOutput) with field(s):
    ///   - [`template_name(Option<String>)`](crate::operation::get_email_template::GetEmailTemplateOutput::template_name): <p>The name of the template.</p>
    ///   - [`template_content(Option<EmailTemplateContent>)`](crate::operation::get_email_template::GetEmailTemplateOutput::template_content): <p>The content of the email template, composed of a subject line, an HTML part, and a text-only part.</p>
    /// - On failure, responds with [`SdkError<GetEmailTemplateError>`](crate::operation::get_email_template::GetEmailTemplateError)
    pub fn get_email_template(
        &self,
    ) -> crate::operation::get_email_template::builders::GetEmailTemplateFluentBuilder {
        crate::operation::get_email_template::builders::GetEmailTemplateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
