// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The target of a mitigation action task. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetectMitigationActionsTaskTarget {
    /// <p> The unique identifiers of the violations. </p>
    #[doc(hidden)]
    pub violation_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p> The name of the security profile. </p>
    #[doc(hidden)]
    pub security_profile_name: ::std::option::Option<::std::string::String>,
    /// <p> The name of the behavior. </p>
    #[doc(hidden)]
    pub behavior_name: ::std::option::Option<::std::string::String>,
}
impl DetectMitigationActionsTaskTarget {
    /// <p> The unique identifiers of the violations. </p>
    pub fn violation_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.violation_ids.as_deref()
    }
    /// <p> The name of the security profile. </p>
    pub fn security_profile_name(&self) -> ::std::option::Option<&str> {
        self.security_profile_name.as_deref()
    }
    /// <p> The name of the behavior. </p>
    pub fn behavior_name(&self) -> ::std::option::Option<&str> {
        self.behavior_name.as_deref()
    }
}
impl DetectMitigationActionsTaskTarget {
    /// Creates a new builder-style object to manufacture [`DetectMitigationActionsTaskTarget`](crate::types::DetectMitigationActionsTaskTarget).
    pub fn builder() -> crate::types::builders::DetectMitigationActionsTaskTargetBuilder {
        crate::types::builders::DetectMitigationActionsTaskTargetBuilder::default()
    }
}

/// A builder for [`DetectMitigationActionsTaskTarget`](crate::types::DetectMitigationActionsTaskTarget).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DetectMitigationActionsTaskTargetBuilder {
    pub(crate) violation_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_profile_name: ::std::option::Option<::std::string::String>,
    pub(crate) behavior_name: ::std::option::Option<::std::string::String>,
}
impl DetectMitigationActionsTaskTargetBuilder {
    /// Appends an item to `violation_ids`.
    ///
    /// To override the contents of this collection use [`set_violation_ids`](Self::set_violation_ids).
    ///
    /// <p> The unique identifiers of the violations. </p>
    pub fn violation_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.violation_ids.unwrap_or_default();
        v.push(input.into());
        self.violation_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p> The unique identifiers of the violations. </p>
    pub fn set_violation_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.violation_ids = input;
        self
    }
    /// <p> The name of the security profile. </p>
    pub fn security_profile_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.security_profile_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the security profile. </p>
    pub fn set_security_profile_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.security_profile_name = input;
        self
    }
    /// <p> The name of the behavior. </p>
    pub fn behavior_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.behavior_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the behavior. </p>
    pub fn set_behavior_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.behavior_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DetectMitigationActionsTaskTarget`](crate::types::DetectMitigationActionsTaskTarget).
    pub fn build(self) -> crate::types::DetectMitigationActionsTaskTarget {
        crate::types::DetectMitigationActionsTaskTarget {
            violation_ids: self.violation_ids,
            security_profile_name: self.security_profile_name,
            behavior_name: self.behavior_name,
        }
    }
}
