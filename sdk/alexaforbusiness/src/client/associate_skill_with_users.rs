// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateSkillWithUsers`](crate::operation::associate_skill_with_users::builders::AssociateSkillWithUsersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`skill_id(impl ::std::convert::Into<String>)`](crate::operation::associate_skill_with_users::builders::AssociateSkillWithUsersFluentBuilder::skill_id) / [`set_skill_id(Option<String>)`](crate::operation::associate_skill_with_users::builders::AssociateSkillWithUsersFluentBuilder::set_skill_id): <p>The private skill ID you want to make available to enrolled users.</p>
    /// - On success, responds with [`AssociateSkillWithUsersOutput`](crate::operation::associate_skill_with_users::AssociateSkillWithUsersOutput)
    /// - On failure, responds with [`SdkError<AssociateSkillWithUsersError>`](crate::operation::associate_skill_with_users::AssociateSkillWithUsersError)
    pub fn associate_skill_with_users(
        &self,
    ) -> crate::operation::associate_skill_with_users::builders::AssociateSkillWithUsersFluentBuilder
    {
        crate::operation::associate_skill_with_users::builders::AssociateSkillWithUsersFluentBuilder::new(self.handle.clone())
    }
}
