// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelLegalHold`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`legal_hold_id(impl ::std::convert::Into<String>)`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::legal_hold_id) / [`set_legal_hold_id(Option<String>)`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::set_legal_hold_id): <p>Legal hold ID required to remove the specified legal hold on a recovery point.</p>
    ///   - [`cancel_description(impl ::std::convert::Into<String>)`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::cancel_description) / [`set_cancel_description(Option<String>)`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::set_cancel_description): <p>String describing the reason for removing the legal hold.</p>
    ///   - [`retain_record_in_days(i64)`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::retain_record_in_days) / [`set_retain_record_in_days(Option<i64>)`](crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::set_retain_record_in_days): <p>The integer amount in days specifying amount of days after this API operation to remove legal hold.</p>
    /// - On success, responds with [`CancelLegalHoldOutput`](crate::operation::cancel_legal_hold::CancelLegalHoldOutput)
    /// - On failure, responds with [`SdkError<CancelLegalHoldError>`](crate::operation::cancel_legal_hold::CancelLegalHoldError)
    pub fn cancel_legal_hold(
        &self,
    ) -> crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder {
        crate::operation::cancel_legal_hold::builders::CancelLegalHoldFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
