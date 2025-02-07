// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateSkillFromSkillGroup`](crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`skill_group_arn(impl ::std::convert::Into<String>)`](crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder::skill_group_arn) / [`set_skill_group_arn(Option<String>)`](crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder::set_skill_group_arn): <p>The unique identifier of a skill. Required.</p>
    ///   - [`skill_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder::skill_id) / [`set_skill_id(Option<String>)`](crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder::set_skill_id): <p>The ARN of a skill group to associate to a skill.</p>
    /// - On success, responds with [`DisassociateSkillFromSkillGroupOutput`](crate::operation::disassociate_skill_from_skill_group::DisassociateSkillFromSkillGroupOutput)
    /// - On failure, responds with [`SdkError<DisassociateSkillFromSkillGroupError>`](crate::operation::disassociate_skill_from_skill_group::DisassociateSkillFromSkillGroupError)
    pub fn disassociate_skill_from_skill_group(&self) -> crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder{
        crate::operation::disassociate_skill_from_skill_group::builders::DisassociateSkillFromSkillGroupFluentBuilder::new(self.handle.clone())
    }
}
