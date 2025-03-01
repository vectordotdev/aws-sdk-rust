// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A high-level overview of a distribution configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DistributionConfigurationSummary {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the distribution configuration.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the distribution configuration.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The date on which the distribution configuration was created.</p>
    #[doc(hidden)]
    pub date_created: ::std::option::Option<::std::string::String>,
    /// <p>The date on which the distribution configuration was updated.</p>
    #[doc(hidden)]
    pub date_updated: ::std::option::Option<::std::string::String>,
    /// <p>The tags associated with the distribution configuration.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>A list of Regions where the container image is distributed to.</p>
    #[doc(hidden)]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DistributionConfigurationSummary {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the distribution configuration.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the distribution configuration.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The date on which the distribution configuration was created.</p>
    pub fn date_created(&self) -> ::std::option::Option<&str> {
        self.date_created.as_deref()
    }
    /// <p>The date on which the distribution configuration was updated.</p>
    pub fn date_updated(&self) -> ::std::option::Option<&str> {
        self.date_updated.as_deref()
    }
    /// <p>The tags associated with the distribution configuration.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>A list of Regions where the container image is distributed to.</p>
    pub fn regions(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.regions.as_deref()
    }
}
impl DistributionConfigurationSummary {
    /// Creates a new builder-style object to manufacture [`DistributionConfigurationSummary`](crate::types::DistributionConfigurationSummary).
    pub fn builder() -> crate::types::builders::DistributionConfigurationSummaryBuilder {
        crate::types::builders::DistributionConfigurationSummaryBuilder::default()
    }
}

/// A builder for [`DistributionConfigurationSummary`](crate::types::DistributionConfigurationSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DistributionConfigurationSummaryBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) date_created: ::std::option::Option<::std::string::String>,
    pub(crate) date_updated: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DistributionConfigurationSummaryBuilder {
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the distribution configuration.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the distribution configuration.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the distribution configuration.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the distribution configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the distribution configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The date on which the distribution configuration was created.</p>
    pub fn date_created(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.date_created = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The date on which the distribution configuration was created.</p>
    pub fn set_date_created(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.date_created = input;
        self
    }
    /// <p>The date on which the distribution configuration was updated.</p>
    pub fn date_updated(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.date_updated = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The date on which the distribution configuration was updated.</p>
    pub fn set_date_updated(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.date_updated = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags associated with the distribution configuration.</p>
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
    /// <p>The tags associated with the distribution configuration.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Appends an item to `regions`.
    ///
    /// To override the contents of this collection use [`set_regions`](Self::set_regions).
    ///
    /// <p>A list of Regions where the container image is distributed to.</p>
    pub fn regions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.regions.unwrap_or_default();
        v.push(input.into());
        self.regions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of Regions where the container image is distributed to.</p>
    pub fn set_regions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.regions = input;
        self
    }
    /// Consumes the builder and constructs a [`DistributionConfigurationSummary`](crate::types::DistributionConfigurationSummary).
    pub fn build(self) -> crate::types::DistributionConfigurationSummary {
        crate::types::DistributionConfigurationSummary {
            arn: self.arn,
            name: self.name,
            description: self.description,
            date_created: self.date_created,
            date_updated: self.date_updated,
            tags: self.tags,
            regions: self.regions,
        }
    }
}
