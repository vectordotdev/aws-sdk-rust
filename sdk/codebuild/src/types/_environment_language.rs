// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A set of Docker images that are related by programming language and are managed by CodeBuild.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnvironmentLanguage {
    /// <p>The programming language for the Docker images.</p>
    #[doc(hidden)]
    pub language: ::std::option::Option<crate::types::LanguageType>,
    /// <p>The list of Docker images that are related by the specified programming language.</p>
    #[doc(hidden)]
    pub images: ::std::option::Option<::std::vec::Vec<crate::types::EnvironmentImage>>,
}
impl EnvironmentLanguage {
    /// <p>The programming language for the Docker images.</p>
    pub fn language(&self) -> ::std::option::Option<&crate::types::LanguageType> {
        self.language.as_ref()
    }
    /// <p>The list of Docker images that are related by the specified programming language.</p>
    pub fn images(&self) -> ::std::option::Option<&[crate::types::EnvironmentImage]> {
        self.images.as_deref()
    }
}
impl EnvironmentLanguage {
    /// Creates a new builder-style object to manufacture [`EnvironmentLanguage`](crate::types::EnvironmentLanguage).
    pub fn builder() -> crate::types::builders::EnvironmentLanguageBuilder {
        crate::types::builders::EnvironmentLanguageBuilder::default()
    }
}

/// A builder for [`EnvironmentLanguage`](crate::types::EnvironmentLanguage).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnvironmentLanguageBuilder {
    pub(crate) language: ::std::option::Option<crate::types::LanguageType>,
    pub(crate) images: ::std::option::Option<::std::vec::Vec<crate::types::EnvironmentImage>>,
}
impl EnvironmentLanguageBuilder {
    /// <p>The programming language for the Docker images.</p>
    pub fn language(mut self, input: crate::types::LanguageType) -> Self {
        self.language = ::std::option::Option::Some(input);
        self
    }
    /// <p>The programming language for the Docker images.</p>
    pub fn set_language(
        mut self,
        input: ::std::option::Option<crate::types::LanguageType>,
    ) -> Self {
        self.language = input;
        self
    }
    /// Appends an item to `images`.
    ///
    /// To override the contents of this collection use [`set_images`](Self::set_images).
    ///
    /// <p>The list of Docker images that are related by the specified programming language.</p>
    pub fn images(mut self, input: crate::types::EnvironmentImage) -> Self {
        let mut v = self.images.unwrap_or_default();
        v.push(input);
        self.images = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of Docker images that are related by the specified programming language.</p>
    pub fn set_images(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EnvironmentImage>>,
    ) -> Self {
        self.images = input;
        self
    }
    /// Consumes the builder and constructs a [`EnvironmentLanguage`](crate::types::EnvironmentLanguage).
    pub fn build(self) -> crate::types::EnvironmentLanguage {
        crate::types::EnvironmentLanguage {
            language: self.language,
            images: self.images,
        }
    }
}
