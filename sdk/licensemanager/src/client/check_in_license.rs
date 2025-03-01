// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CheckInLicense`](crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`license_consumption_token(impl ::std::convert::Into<String>)`](crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder::license_consumption_token) / [`set_license_consumption_token(Option<String>)`](crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder::set_license_consumption_token): <p>License consumption token.</p>
    ///   - [`beneficiary(impl ::std::convert::Into<String>)`](crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder::beneficiary) / [`set_beneficiary(Option<String>)`](crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder::set_beneficiary): <p>License beneficiary.</p>
    /// - On success, responds with [`CheckInLicenseOutput`](crate::operation::check_in_license::CheckInLicenseOutput)
    /// - On failure, responds with [`SdkError<CheckInLicenseError>`](crate::operation::check_in_license::CheckInLicenseError)
    pub fn check_in_license(
        &self,
    ) -> crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder {
        crate::operation::check_in_license::builders::CheckInLicenseFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
