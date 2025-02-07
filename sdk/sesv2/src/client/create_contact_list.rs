// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateContactList`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contact_list_name(impl ::std::convert::Into<String>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::contact_list_name) / [`set_contact_list_name(Option<String>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::set_contact_list_name): <p>The name of the contact list.</p>
    ///   - [`topics(Vec<Topic>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::topics) / [`set_topics(Option<Vec<Topic>>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::set_topics): <p>An interest group, theme, or label within a list. A contact list can have multiple topics.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::set_description): <p>A description of what the contact list is about.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::set_tags): <p>The tags associated with a contact list.</p>
    /// - On success, responds with [`CreateContactListOutput`](crate::operation::create_contact_list::CreateContactListOutput)
    /// - On failure, responds with [`SdkError<CreateContactListError>`](crate::operation::create_contact_list::CreateContactListError)
    pub fn create_contact_list(
        &self,
    ) -> crate::operation::create_contact_list::builders::CreateContactListFluentBuilder {
        crate::operation::create_contact_list::builders::CreateContactListFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
