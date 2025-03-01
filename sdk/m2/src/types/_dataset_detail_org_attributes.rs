// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Additional details about the data set. Different attributes correspond to different data set organizations. The values are populated based on datasetOrg, storageType and backend (Blu Age or Micro Focus).</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum DatasetDetailOrgAttributes {
    /// <p>The generation data group of the data set.</p>
    Gdg(crate::types::GdgDetailAttributes),
    /// <p>The details of a VSAM data set.</p>
    Vsam(crate::types::VsamDetailAttributes),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl DatasetDetailOrgAttributes {
    /// Tries to convert the enum instance into [`Gdg`](crate::types::DatasetDetailOrgAttributes::Gdg), extracting the inner [`GdgDetailAttributes`](crate::types::GdgDetailAttributes).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_gdg(&self) -> ::std::result::Result<&crate::types::GdgDetailAttributes, &Self> {
        if let DatasetDetailOrgAttributes::Gdg(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Gdg`](crate::types::DatasetDetailOrgAttributes::Gdg).
    pub fn is_gdg(&self) -> bool {
        self.as_gdg().is_ok()
    }
    /// Tries to convert the enum instance into [`Vsam`](crate::types::DatasetDetailOrgAttributes::Vsam), extracting the inner [`VsamDetailAttributes`](crate::types::VsamDetailAttributes).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_vsam(&self) -> ::std::result::Result<&crate::types::VsamDetailAttributes, &Self> {
        if let DatasetDetailOrgAttributes::Vsam(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Vsam`](crate::types::DatasetDetailOrgAttributes::Vsam).
    pub fn is_vsam(&self) -> bool {
        self.as_vsam().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
