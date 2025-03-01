// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a simulation software suite.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SimulationSoftwareSuite {
    /// <p>The name of the simulation software suite.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::SimulationSoftwareSuiteType>,
    /// <p>The version of the simulation software suite.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
}
impl SimulationSoftwareSuite {
    /// <p>The name of the simulation software suite.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::SimulationSoftwareSuiteType> {
        self.name.as_ref()
    }
    /// <p>The version of the simulation software suite.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl SimulationSoftwareSuite {
    /// Creates a new builder-style object to manufacture [`SimulationSoftwareSuite`](crate::types::SimulationSoftwareSuite).
    pub fn builder() -> crate::types::builders::SimulationSoftwareSuiteBuilder {
        crate::types::builders::SimulationSoftwareSuiteBuilder::default()
    }
}

/// A builder for [`SimulationSoftwareSuite`](crate::types::SimulationSoftwareSuite).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SimulationSoftwareSuiteBuilder {
    pub(crate) name: ::std::option::Option<crate::types::SimulationSoftwareSuiteType>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl SimulationSoftwareSuiteBuilder {
    /// <p>The name of the simulation software suite.</p>
    pub fn name(mut self, input: crate::types::SimulationSoftwareSuiteType) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the simulation software suite.</p>
    pub fn set_name(
        mut self,
        input: ::std::option::Option<crate::types::SimulationSoftwareSuiteType>,
    ) -> Self {
        self.name = input;
        self
    }
    /// <p>The version of the simulation software suite.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the simulation software suite.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`SimulationSoftwareSuite`](crate::types::SimulationSoftwareSuite).
    pub fn build(self) -> crate::types::SimulationSoftwareSuite {
        crate::types::SimulationSoftwareSuite {
            name: self.name,
            version: self.version,
        }
    }
}
