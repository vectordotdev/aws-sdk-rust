// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetServiceLinkedRoleDeletionStatus`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`deletion_task_id(impl ::std::convert::Into<String>)`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::deletion_task_id) / [`set_deletion_task_id(Option<String>)`](crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::set_deletion_task_id): <p>The deletion task identifier. This identifier is returned by the <code>DeleteServiceLinkedRole</code> operation in the format <code>task/aws-service-role/   <service-principal-name>    /    <role-name>     /     <task-uuid></task-uuid>    </role-name>   </service-principal-name></code>.</p>
    /// - On success, responds with [`GetServiceLinkedRoleDeletionStatusOutput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput) with field(s):
    ///   - [`status(Option<DeletionTaskStatusType>)`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput::status): <p>The status of the deletion.</p>
    ///   - [`reason(Option<DeletionTaskFailureReasonType>)`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput::reason): <p>An object that contains details about the reason the deletion failed.</p>
    /// - On failure, responds with [`SdkError<GetServiceLinkedRoleDeletionStatusError>`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusError)
    pub fn get_service_linked_role_deletion_status(&self) -> crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder{
        crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusFluentBuilder::new(self.handle.clone())
    }
}
