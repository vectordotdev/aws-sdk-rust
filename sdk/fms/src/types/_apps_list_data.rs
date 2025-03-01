// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An Firewall Manager applications list.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AppsListData {
    /// <p>The ID of the Firewall Manager applications list.</p>
    #[doc(hidden)]
    pub list_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Firewall Manager applications list.</p>
    #[doc(hidden)]
    pub list_name: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for each update to the list. When you update the list, the update token must match the token of the current version of the application list. You can retrieve the update token by getting the list. </p>
    #[doc(hidden)]
    pub list_update_token: ::std::option::Option<::std::string::String>,
    /// <p>The time that the Firewall Manager applications list was created.</p>
    #[doc(hidden)]
    pub create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time that the Firewall Manager applications list was last updated.</p>
    #[doc(hidden)]
    pub last_update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>An array of applications in the Firewall Manager applications list.</p>
    #[doc(hidden)]
    pub apps_list: ::std::option::Option<::std::vec::Vec<crate::types::App>>,
    /// <p>A map of previous version numbers to their corresponding <code>App</code> object arrays.</p>
    #[doc(hidden)]
    pub previous_apps_list: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::App>>,
    >,
}
impl AppsListData {
    /// <p>The ID of the Firewall Manager applications list.</p>
    pub fn list_id(&self) -> ::std::option::Option<&str> {
        self.list_id.as_deref()
    }
    /// <p>The name of the Firewall Manager applications list.</p>
    pub fn list_name(&self) -> ::std::option::Option<&str> {
        self.list_name.as_deref()
    }
    /// <p>A unique identifier for each update to the list. When you update the list, the update token must match the token of the current version of the application list. You can retrieve the update token by getting the list. </p>
    pub fn list_update_token(&self) -> ::std::option::Option<&str> {
        self.list_update_token.as_deref()
    }
    /// <p>The time that the Firewall Manager applications list was created.</p>
    pub fn create_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>The time that the Firewall Manager applications list was last updated.</p>
    pub fn last_update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_update_time.as_ref()
    }
    /// <p>An array of applications in the Firewall Manager applications list.</p>
    pub fn apps_list(&self) -> ::std::option::Option<&[crate::types::App]> {
        self.apps_list.as_deref()
    }
    /// <p>A map of previous version numbers to their corresponding <code>App</code> object arrays.</p>
    pub fn previous_apps_list(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::App>>,
    > {
        self.previous_apps_list.as_ref()
    }
}
impl AppsListData {
    /// Creates a new builder-style object to manufacture [`AppsListData`](crate::types::AppsListData).
    pub fn builder() -> crate::types::builders::AppsListDataBuilder {
        crate::types::builders::AppsListDataBuilder::default()
    }
}

/// A builder for [`AppsListData`](crate::types::AppsListData).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AppsListDataBuilder {
    pub(crate) list_id: ::std::option::Option<::std::string::String>,
    pub(crate) list_name: ::std::option::Option<::std::string::String>,
    pub(crate) list_update_token: ::std::option::Option<::std::string::String>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) apps_list: ::std::option::Option<::std::vec::Vec<crate::types::App>>,
    pub(crate) previous_apps_list: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::App>>,
    >,
}
impl AppsListDataBuilder {
    /// <p>The ID of the Firewall Manager applications list.</p>
    pub fn list_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.list_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Firewall Manager applications list.</p>
    pub fn set_list_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.list_id = input;
        self
    }
    /// <p>The name of the Firewall Manager applications list.</p>
    pub fn list_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.list_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Firewall Manager applications list.</p>
    pub fn set_list_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.list_name = input;
        self
    }
    /// <p>A unique identifier for each update to the list. When you update the list, the update token must match the token of the current version of the application list. You can retrieve the update token by getting the list. </p>
    pub fn list_update_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.list_update_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for each update to the list. When you update the list, the update token must match the token of the current version of the application list. You can retrieve the update token by getting the list. </p>
    pub fn set_list_update_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.list_update_token = input;
        self
    }
    /// <p>The time that the Firewall Manager applications list was created.</p>
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time that the Firewall Manager applications list was created.</p>
    pub fn set_create_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The time that the Firewall Manager applications list was last updated.</p>
    pub fn last_update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time that the Firewall Manager applications list was last updated.</p>
    pub fn set_last_update_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_update_time = input;
        self
    }
    /// Appends an item to `apps_list`.
    ///
    /// To override the contents of this collection use [`set_apps_list`](Self::set_apps_list).
    ///
    /// <p>An array of applications in the Firewall Manager applications list.</p>
    pub fn apps_list(mut self, input: crate::types::App) -> Self {
        let mut v = self.apps_list.unwrap_or_default();
        v.push(input);
        self.apps_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of applications in the Firewall Manager applications list.</p>
    pub fn set_apps_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::App>>,
    ) -> Self {
        self.apps_list = input;
        self
    }
    /// Adds a key-value pair to `previous_apps_list`.
    ///
    /// To override the contents of this collection use [`set_previous_apps_list`](Self::set_previous_apps_list).
    ///
    /// <p>A map of previous version numbers to their corresponding <code>App</code> object arrays.</p>
    pub fn previous_apps_list(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: ::std::vec::Vec<crate::types::App>,
    ) -> Self {
        let mut hash_map = self.previous_apps_list.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.previous_apps_list = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of previous version numbers to their corresponding <code>App</code> object arrays.</p>
    pub fn set_previous_apps_list(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<crate::types::App>>,
        >,
    ) -> Self {
        self.previous_apps_list = input;
        self
    }
    /// Consumes the builder and constructs a [`AppsListData`](crate::types::AppsListData).
    pub fn build(self) -> crate::types::AppsListData {
        crate::types::AppsListData {
            list_id: self.list_id,
            list_name: self.list_name,
            list_update_token: self.list_update_token,
            create_time: self.create_time,
            last_update_time: self.last_update_time,
            apps_list: self.apps_list,
            previous_apps_list: self.previous_apps_list,
        }
    }
}
