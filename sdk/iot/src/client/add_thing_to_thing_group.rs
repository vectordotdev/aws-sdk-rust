// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddThingToThingGroup`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`thing_group_name(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::thing_group_name) / [`set_thing_group_name(Option<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::set_thing_group_name): <p>The name of the group to which you are adding a thing.</p>
    ///   - [`thing_group_arn(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::thing_group_arn) / [`set_thing_group_arn(Option<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::set_thing_group_arn): <p>The ARN of the group to which you are adding a thing.</p>
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::set_thing_name): <p>The name of the thing to add to a group.</p>
    ///   - [`thing_arn(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::thing_arn) / [`set_thing_arn(Option<String>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::set_thing_arn): <p>The ARN of the thing to add to a group.</p>
    ///   - [`override_dynamic_groups(bool)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::override_dynamic_groups) / [`set_override_dynamic_groups(Option<bool>)`](crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::set_override_dynamic_groups): <p>Override dynamic thing groups with static thing groups when 10-group limit is reached. If a thing belongs to 10 thing groups, and one or more of those groups are dynamic thing groups, adding a thing to a static group removes the thing from the last dynamic group.</p>
    /// - On success, responds with [`AddThingToThingGroupOutput`](crate::operation::add_thing_to_thing_group::AddThingToThingGroupOutput)
    /// - On failure, responds with [`SdkError<AddThingToThingGroupError>`](crate::operation::add_thing_to_thing_group::AddThingToThingGroupError)
    pub fn add_thing_to_thing_group(
        &self,
    ) -> crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder
    {
        crate::operation::add_thing_to_thing_group::builders::AddThingToThingGroupFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
