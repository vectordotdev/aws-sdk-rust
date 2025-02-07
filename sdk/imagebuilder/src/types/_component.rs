// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A detailed view of a component.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Component {
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the component.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The version of the component.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
    /// <p>The description of the component.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The change description of the component.</p>
    #[doc(hidden)]
    pub change_description: ::std::option::Option<::std::string::String>,
    /// <p>The component type specifies whether Image Builder uses the component to build the image or only to test it.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::ComponentType>,
    /// <p>The operating system platform of the component.</p>
    #[doc(hidden)]
    pub platform: ::std::option::Option<crate::types::Platform>,
    /// <p>The operating system (OS) version supported by the component. If the OS information is available, Image Builder performs a prefix match against the base image OS version during image recipe creation.</p>
    #[doc(hidden)]
    pub supported_os_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Describes the current status of the component. This is used for components that are no longer active.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::ComponentState>,
    /// <p>Contains parameter details for each of the parameters that the component document defined for the component.</p>
    #[doc(hidden)]
    pub parameters: ::std::option::Option<::std::vec::Vec<crate::types::ComponentParameterDetail>>,
    /// <p>The owner of the component.</p>
    #[doc(hidden)]
    pub owner: ::std::option::Option<::std::string::String>,
    /// <p>Component data contains the YAML document content for the component.</p>
    #[doc(hidden)]
    pub data: ::std::option::Option<::std::string::String>,
    /// <p>The KMS key identifier used to encrypt the component.</p>
    #[doc(hidden)]
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The encryption status of the component.</p>
    #[doc(hidden)]
    pub encrypted: ::std::option::Option<bool>,
    /// <p>The date that Image Builder created the component.</p>
    #[doc(hidden)]
    pub date_created: ::std::option::Option<::std::string::String>,
    /// <p>The tags that apply to the component.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>Contains the name of the publisher if this is a third-party component. Otherwise, this property is empty.</p>
    #[doc(hidden)]
    pub publisher: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether component source is hidden from view in the console, and from component detail results for API, CLI, or SDK operations.</p>
    #[doc(hidden)]
    pub obfuscate: bool,
}
impl Component {
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the component.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The version of the component.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
    /// <p>The description of the component.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The change description of the component.</p>
    pub fn change_description(&self) -> ::std::option::Option<&str> {
        self.change_description.as_deref()
    }
    /// <p>The component type specifies whether Image Builder uses the component to build the image or only to test it.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ComponentType> {
        self.r#type.as_ref()
    }
    /// <p>The operating system platform of the component.</p>
    pub fn platform(&self) -> ::std::option::Option<&crate::types::Platform> {
        self.platform.as_ref()
    }
    /// <p>The operating system (OS) version supported by the component. If the OS information is available, Image Builder performs a prefix match against the base image OS version during image recipe creation.</p>
    pub fn supported_os_versions(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.supported_os_versions.as_deref()
    }
    /// <p>Describes the current status of the component. This is used for components that are no longer active.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::ComponentState> {
        self.state.as_ref()
    }
    /// <p>Contains parameter details for each of the parameters that the component document defined for the component.</p>
    pub fn parameters(&self) -> ::std::option::Option<&[crate::types::ComponentParameterDetail]> {
        self.parameters.as_deref()
    }
    /// <p>The owner of the component.</p>
    pub fn owner(&self) -> ::std::option::Option<&str> {
        self.owner.as_deref()
    }
    /// <p>Component data contains the YAML document content for the component.</p>
    pub fn data(&self) -> ::std::option::Option<&str> {
        self.data.as_deref()
    }
    /// <p>The KMS key identifier used to encrypt the component.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>The encryption status of the component.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
    }
    /// <p>The date that Image Builder created the component.</p>
    pub fn date_created(&self) -> ::std::option::Option<&str> {
        self.date_created.as_deref()
    }
    /// <p>The tags that apply to the component.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>Contains the name of the publisher if this is a third-party component. Otherwise, this property is empty.</p>
    pub fn publisher(&self) -> ::std::option::Option<&str> {
        self.publisher.as_deref()
    }
    /// <p>Indicates whether component source is hidden from view in the console, and from component detail results for API, CLI, or SDK operations.</p>
    pub fn obfuscate(&self) -> bool {
        self.obfuscate
    }
}
impl Component {
    /// Creates a new builder-style object to manufacture [`Component`](crate::types::Component).
    pub fn builder() -> crate::types::builders::ComponentBuilder {
        crate::types::builders::ComponentBuilder::default()
    }
}

/// A builder for [`Component`](crate::types::Component).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ComponentBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) change_description: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::ComponentType>,
    pub(crate) platform: ::std::option::Option<crate::types::Platform>,
    pub(crate) supported_os_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) state: ::std::option::Option<crate::types::ComponentState>,
    pub(crate) parameters:
        ::std::option::Option<::std::vec::Vec<crate::types::ComponentParameterDetail>>,
    pub(crate) owner: ::std::option::Option<::std::string::String>,
    pub(crate) data: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) date_created: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) publisher: ::std::option::Option<::std::string::String>,
    pub(crate) obfuscate: ::std::option::Option<bool>,
}
impl ComponentBuilder {
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the component.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the component.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The version of the component.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the component.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The description of the component.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the component.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The change description of the component.</p>
    pub fn change_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.change_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The change description of the component.</p>
    pub fn set_change_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.change_description = input;
        self
    }
    /// <p>The component type specifies whether Image Builder uses the component to build the image or only to test it.</p>
    pub fn r#type(mut self, input: crate::types::ComponentType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The component type specifies whether Image Builder uses the component to build the image or only to test it.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ComponentType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The operating system platform of the component.</p>
    pub fn platform(mut self, input: crate::types::Platform) -> Self {
        self.platform = ::std::option::Option::Some(input);
        self
    }
    /// <p>The operating system platform of the component.</p>
    pub fn set_platform(mut self, input: ::std::option::Option<crate::types::Platform>) -> Self {
        self.platform = input;
        self
    }
    /// Appends an item to `supported_os_versions`.
    ///
    /// To override the contents of this collection use [`set_supported_os_versions`](Self::set_supported_os_versions).
    ///
    /// <p>The operating system (OS) version supported by the component. If the OS information is available, Image Builder performs a prefix match against the base image OS version during image recipe creation.</p>
    pub fn supported_os_versions(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.supported_os_versions.unwrap_or_default();
        v.push(input.into());
        self.supported_os_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The operating system (OS) version supported by the component. If the OS information is available, Image Builder performs a prefix match against the base image OS version during image recipe creation.</p>
    pub fn set_supported_os_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.supported_os_versions = input;
        self
    }
    /// <p>Describes the current status of the component. This is used for components that are no longer active.</p>
    pub fn state(mut self, input: crate::types::ComponentState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the current status of the component. This is used for components that are no longer active.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ComponentState>) -> Self {
        self.state = input;
        self
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>Contains parameter details for each of the parameters that the component document defined for the component.</p>
    pub fn parameters(mut self, input: crate::types::ComponentParameterDetail) -> Self {
        let mut v = self.parameters.unwrap_or_default();
        v.push(input);
        self.parameters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains parameter details for each of the parameters that the component document defined for the component.</p>
    pub fn set_parameters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ComponentParameterDetail>>,
    ) -> Self {
        self.parameters = input;
        self
    }
    /// <p>The owner of the component.</p>
    pub fn owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The owner of the component.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner = input;
        self
    }
    /// <p>Component data contains the YAML document content for the component.</p>
    pub fn data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Component data contains the YAML document content for the component.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data = input;
        self
    }
    /// <p>The KMS key identifier used to encrypt the component.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The KMS key identifier used to encrypt the component.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The encryption status of the component.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption status of the component.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>The date that Image Builder created the component.</p>
    pub fn date_created(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.date_created = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The date that Image Builder created the component.</p>
    pub fn set_date_created(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.date_created = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that apply to the component.</p>
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
    /// <p>The tags that apply to the component.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>Contains the name of the publisher if this is a third-party component. Otherwise, this property is empty.</p>
    pub fn publisher(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.publisher = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains the name of the publisher if this is a third-party component. Otherwise, this property is empty.</p>
    pub fn set_publisher(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.publisher = input;
        self
    }
    /// <p>Indicates whether component source is hidden from view in the console, and from component detail results for API, CLI, or SDK operations.</p>
    pub fn obfuscate(mut self, input: bool) -> Self {
        self.obfuscate = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether component source is hidden from view in the console, and from component detail results for API, CLI, or SDK operations.</p>
    pub fn set_obfuscate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.obfuscate = input;
        self
    }
    /// Consumes the builder and constructs a [`Component`](crate::types::Component).
    pub fn build(self) -> crate::types::Component {
        crate::types::Component {
            arn: self.arn,
            name: self.name,
            version: self.version,
            description: self.description,
            change_description: self.change_description,
            r#type: self.r#type,
            platform: self.platform,
            supported_os_versions: self.supported_os_versions,
            state: self.state,
            parameters: self.parameters,
            owner: self.owner,
            data: self.data,
            kms_key_id: self.kms_key_id,
            encrypted: self.encrypted,
            date_created: self.date_created,
            tags: self.tags,
            publisher: self.publisher,
            obfuscate: self.obfuscate.unwrap_or_default(),
        }
    }
}
