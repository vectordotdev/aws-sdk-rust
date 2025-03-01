// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreatePullThroughCacheRuleInput {
    /// <p>The repository name prefix to use when caching images from the source registry.</p>
    #[doc(hidden)]
    pub ecr_repository_prefix: ::std::option::Option<::std::string::String>,
    /// <p>The registry URL of the upstream public registry to use as the source for the pull through cache rule.</p>
    #[doc(hidden)]
    pub upstream_registry_url: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services account ID associated with the registry to create the pull through cache rule for. If you do not specify a registry, the default registry is assumed.</p>
    #[doc(hidden)]
    pub registry_id: ::std::option::Option<::std::string::String>,
}
impl CreatePullThroughCacheRuleInput {
    /// <p>The repository name prefix to use when caching images from the source registry.</p>
    pub fn ecr_repository_prefix(&self) -> ::std::option::Option<&str> {
        self.ecr_repository_prefix.as_deref()
    }
    /// <p>The registry URL of the upstream public registry to use as the source for the pull through cache rule.</p>
    pub fn upstream_registry_url(&self) -> ::std::option::Option<&str> {
        self.upstream_registry_url.as_deref()
    }
    /// <p>The Amazon Web Services account ID associated with the registry to create the pull through cache rule for. If you do not specify a registry, the default registry is assumed.</p>
    pub fn registry_id(&self) -> ::std::option::Option<&str> {
        self.registry_id.as_deref()
    }
}
impl CreatePullThroughCacheRuleInput {
    /// Creates a new builder-style object to manufacture [`CreatePullThroughCacheRuleInput`](crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleInput).
    pub fn builder() -> crate::operation::create_pull_through_cache_rule::builders::CreatePullThroughCacheRuleInputBuilder{
        crate::operation::create_pull_through_cache_rule::builders::CreatePullThroughCacheRuleInputBuilder::default()
    }
}

/// A builder for [`CreatePullThroughCacheRuleInput`](crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreatePullThroughCacheRuleInputBuilder {
    pub(crate) ecr_repository_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) upstream_registry_url: ::std::option::Option<::std::string::String>,
    pub(crate) registry_id: ::std::option::Option<::std::string::String>,
}
impl CreatePullThroughCacheRuleInputBuilder {
    /// <p>The repository name prefix to use when caching images from the source registry.</p>
    pub fn ecr_repository_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ecr_repository_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The repository name prefix to use when caching images from the source registry.</p>
    pub fn set_ecr_repository_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ecr_repository_prefix = input;
        self
    }
    /// <p>The registry URL of the upstream public registry to use as the source for the pull through cache rule.</p>
    pub fn upstream_registry_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.upstream_registry_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The registry URL of the upstream public registry to use as the source for the pull through cache rule.</p>
    pub fn set_upstream_registry_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.upstream_registry_url = input;
        self
    }
    /// <p>The Amazon Web Services account ID associated with the registry to create the pull through cache rule for. If you do not specify a registry, the default registry is assumed.</p>
    pub fn registry_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.registry_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with the registry to create the pull through cache rule for. If you do not specify a registry, the default registry is assumed.</p>
    pub fn set_registry_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.registry_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreatePullThroughCacheRuleInput`](crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_pull_through_cache_rule::CreatePullThroughCacheRuleInput {
                ecr_repository_prefix: self.ecr_repository_prefix,
                upstream_registry_url: self.upstream_registry_url,
                registry_id: self.registry_id,
            },
        )
    }
}
