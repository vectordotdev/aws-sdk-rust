// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Settings for SCTE-35 signals from ESAM. Include this in your job settings to put SCTE-35 markers in your HLS and transport stream outputs at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct M2tsScte35Esam {
    /// Packet Identifier (PID) of the SCTE-35 stream in the transport stream generated by ESAM.
    #[doc(hidden)]
    pub scte35_esam_pid: ::std::option::Option<i32>,
}
impl M2tsScte35Esam {
    /// Packet Identifier (PID) of the SCTE-35 stream in the transport stream generated by ESAM.
    pub fn scte35_esam_pid(&self) -> ::std::option::Option<i32> {
        self.scte35_esam_pid
    }
}
impl M2tsScte35Esam {
    /// Creates a new builder-style object to manufacture [`M2tsScte35Esam`](crate::types::M2tsScte35Esam).
    pub fn builder() -> crate::types::builders::M2tsScte35EsamBuilder {
        crate::types::builders::M2tsScte35EsamBuilder::default()
    }
}

/// A builder for [`M2tsScte35Esam`](crate::types::M2tsScte35Esam).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct M2tsScte35EsamBuilder {
    pub(crate) scte35_esam_pid: ::std::option::Option<i32>,
}
impl M2tsScte35EsamBuilder {
    /// Packet Identifier (PID) of the SCTE-35 stream in the transport stream generated by ESAM.
    pub fn scte35_esam_pid(mut self, input: i32) -> Self {
        self.scte35_esam_pid = ::std::option::Option::Some(input);
        self
    }
    /// Packet Identifier (PID) of the SCTE-35 stream in the transport stream generated by ESAM.
    pub fn set_scte35_esam_pid(mut self, input: ::std::option::Option<i32>) -> Self {
        self.scte35_esam_pid = input;
        self
    }
    /// Consumes the builder and constructs a [`M2tsScte35Esam`](crate::types::M2tsScte35Esam).
    pub fn build(self) -> crate::types::M2tsScte35Esam {
        crate::types::M2tsScte35Esam {
            scte35_esam_pid: self.scte35_esam_pid,
        }
    }
}
