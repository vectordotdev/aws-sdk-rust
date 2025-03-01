// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCollaborationInput {
    /// <p>A list of initial members, not including the creator. This list is immutable.</p>
    #[doc(hidden)]
    pub members: ::std::option::Option<::std::vec::Vec<crate::types::MemberSpecification>>,
    /// <p>The display name for a collaboration.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the collaboration provided by the collaboration owner.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The abilities granted to the collaboration creator.</p>
    #[doc(hidden)]
    pub creator_member_abilities:
        ::std::option::Option<::std::vec::Vec<crate::types::MemberAbility>>,
    /// <p>The display name of the collaboration creator.</p>
    #[doc(hidden)]
    pub creator_display_name: ::std::option::Option<::std::string::String>,
    /// <p>The settings for client-side encryption with Cryptographic Computing for Clean Rooms.</p>
    #[doc(hidden)]
    pub data_encryption_metadata: ::std::option::Option<crate::types::DataEncryptionMetadata>,
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    #[doc(hidden)]
    pub query_log_status: ::std::option::Option<crate::types::CollaborationQueryLogStatus>,
    /// <p>An optional label that you can assign to a resource when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to this resource.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateCollaborationInput {
    /// <p>A list of initial members, not including the creator. This list is immutable.</p>
    pub fn members(&self) -> ::std::option::Option<&[crate::types::MemberSpecification]> {
        self.members.as_deref()
    }
    /// <p>The display name for a collaboration.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A description of the collaboration provided by the collaboration owner.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The abilities granted to the collaboration creator.</p>
    pub fn creator_member_abilities(
        &self,
    ) -> ::std::option::Option<&[crate::types::MemberAbility]> {
        self.creator_member_abilities.as_deref()
    }
    /// <p>The display name of the collaboration creator.</p>
    pub fn creator_display_name(&self) -> ::std::option::Option<&str> {
        self.creator_display_name.as_deref()
    }
    /// <p>The settings for client-side encryption with Cryptographic Computing for Clean Rooms.</p>
    pub fn data_encryption_metadata(
        &self,
    ) -> ::std::option::Option<&crate::types::DataEncryptionMetadata> {
        self.data_encryption_metadata.as_ref()
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn query_log_status(
        &self,
    ) -> ::std::option::Option<&crate::types::CollaborationQueryLogStatus> {
        self.query_log_status.as_ref()
    }
    /// <p>An optional label that you can assign to a resource when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to this resource.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl CreateCollaborationInput {
    /// Creates a new builder-style object to manufacture [`CreateCollaborationInput`](crate::operation::create_collaboration::CreateCollaborationInput).
    pub fn builder(
    ) -> crate::operation::create_collaboration::builders::CreateCollaborationInputBuilder {
        crate::operation::create_collaboration::builders::CreateCollaborationInputBuilder::default()
    }
}

/// A builder for [`CreateCollaborationInput`](crate::operation::create_collaboration::CreateCollaborationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCollaborationInputBuilder {
    pub(crate) members: ::std::option::Option<::std::vec::Vec<crate::types::MemberSpecification>>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) creator_member_abilities:
        ::std::option::Option<::std::vec::Vec<crate::types::MemberAbility>>,
    pub(crate) creator_display_name: ::std::option::Option<::std::string::String>,
    pub(crate) data_encryption_metadata:
        ::std::option::Option<crate::types::DataEncryptionMetadata>,
    pub(crate) query_log_status: ::std::option::Option<crate::types::CollaborationQueryLogStatus>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateCollaborationInputBuilder {
    /// Appends an item to `members`.
    ///
    /// To override the contents of this collection use [`set_members`](Self::set_members).
    ///
    /// <p>A list of initial members, not including the creator. This list is immutable.</p>
    pub fn members(mut self, input: crate::types::MemberSpecification) -> Self {
        let mut v = self.members.unwrap_or_default();
        v.push(input);
        self.members = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of initial members, not including the creator. This list is immutable.</p>
    pub fn set_members(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MemberSpecification>>,
    ) -> Self {
        self.members = input;
        self
    }
    /// <p>The display name for a collaboration.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name for a collaboration.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A description of the collaboration provided by the collaboration owner.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the collaboration provided by the collaboration owner.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `creator_member_abilities`.
    ///
    /// To override the contents of this collection use [`set_creator_member_abilities`](Self::set_creator_member_abilities).
    ///
    /// <p>The abilities granted to the collaboration creator.</p>
    pub fn creator_member_abilities(mut self, input: crate::types::MemberAbility) -> Self {
        let mut v = self.creator_member_abilities.unwrap_or_default();
        v.push(input);
        self.creator_member_abilities = ::std::option::Option::Some(v);
        self
    }
    /// <p>The abilities granted to the collaboration creator.</p>
    pub fn set_creator_member_abilities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MemberAbility>>,
    ) -> Self {
        self.creator_member_abilities = input;
        self
    }
    /// <p>The display name of the collaboration creator.</p>
    pub fn creator_display_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.creator_display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of the collaboration creator.</p>
    pub fn set_creator_display_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.creator_display_name = input;
        self
    }
    /// <p>The settings for client-side encryption with Cryptographic Computing for Clean Rooms.</p>
    pub fn data_encryption_metadata(mut self, input: crate::types::DataEncryptionMetadata) -> Self {
        self.data_encryption_metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>The settings for client-side encryption with Cryptographic Computing for Clean Rooms.</p>
    pub fn set_data_encryption_metadata(
        mut self,
        input: ::std::option::Option<crate::types::DataEncryptionMetadata>,
    ) -> Self {
        self.data_encryption_metadata = input;
        self
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn query_log_status(mut self, input: crate::types::CollaborationQueryLogStatus) -> Self {
        self.query_log_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn set_query_log_status(
        mut self,
        input: ::std::option::Option<crate::types::CollaborationQueryLogStatus>,
    ) -> Self {
        self.query_log_status = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An optional label that you can assign to a resource when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to this resource.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>An optional label that you can assign to a resource when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to this resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateCollaborationInput`](crate::operation::create_collaboration::CreateCollaborationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_collaboration::CreateCollaborationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_collaboration::CreateCollaborationInput {
                members: self.members,
                name: self.name,
                description: self.description,
                creator_member_abilities: self.creator_member_abilities,
                creator_display_name: self.creator_display_name,
                data_encryption_metadata: self.data_encryption_metadata,
                query_log_status: self.query_log_status,
                tags: self.tags,
            },
        )
    }
}
