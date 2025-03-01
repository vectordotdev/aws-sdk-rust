// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an action and whether the action is enabled or disabled for users during their streaming sessions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UserSetting {
    /// <p>The action that is enabled or disabled.</p>
    #[doc(hidden)]
    pub action: ::std::option::Option<crate::types::Action>,
    /// <p>Indicates whether the action is enabled or disabled.</p>
    #[doc(hidden)]
    pub permission: ::std::option::Option<crate::types::Permission>,
}
impl UserSetting {
    /// <p>The action that is enabled or disabled.</p>
    pub fn action(&self) -> ::std::option::Option<&crate::types::Action> {
        self.action.as_ref()
    }
    /// <p>Indicates whether the action is enabled or disabled.</p>
    pub fn permission(&self) -> ::std::option::Option<&crate::types::Permission> {
        self.permission.as_ref()
    }
}
impl UserSetting {
    /// Creates a new builder-style object to manufacture [`UserSetting`](crate::types::UserSetting).
    pub fn builder() -> crate::types::builders::UserSettingBuilder {
        crate::types::builders::UserSettingBuilder::default()
    }
}

/// A builder for [`UserSetting`](crate::types::UserSetting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UserSettingBuilder {
    pub(crate) action: ::std::option::Option<crate::types::Action>,
    pub(crate) permission: ::std::option::Option<crate::types::Permission>,
}
impl UserSettingBuilder {
    /// <p>The action that is enabled or disabled.</p>
    pub fn action(mut self, input: crate::types::Action) -> Self {
        self.action = ::std::option::Option::Some(input);
        self
    }
    /// <p>The action that is enabled or disabled.</p>
    pub fn set_action(mut self, input: ::std::option::Option<crate::types::Action>) -> Self {
        self.action = input;
        self
    }
    /// <p>Indicates whether the action is enabled or disabled.</p>
    pub fn permission(mut self, input: crate::types::Permission) -> Self {
        self.permission = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the action is enabled or disabled.</p>
    pub fn set_permission(
        mut self,
        input: ::std::option::Option<crate::types::Permission>,
    ) -> Self {
        self.permission = input;
        self
    }
    /// Consumes the builder and constructs a [`UserSetting`](crate::types::UserSetting).
    pub fn build(self) -> crate::types::UserSetting {
        crate::types::UserSetting {
            action: self.action,
            permission: self.permission,
        }
    }
}
