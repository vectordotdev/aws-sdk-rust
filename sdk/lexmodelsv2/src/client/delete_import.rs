// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteImport`](crate::operation::delete_import::builders::DeleteImportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`import_id(impl ::std::convert::Into<String>)`](crate::operation::delete_import::builders::DeleteImportFluentBuilder::import_id) / [`set_import_id(Option<String>)`](crate::operation::delete_import::builders::DeleteImportFluentBuilder::set_import_id): <p>The unique identifier of the import to delete.</p>
    /// - On success, responds with [`DeleteImportOutput`](crate::operation::delete_import::DeleteImportOutput) with field(s):
    ///   - [`import_id(Option<String>)`](crate::operation::delete_import::DeleteImportOutput::import_id): <p>The unique identifier of the deleted import.</p>
    ///   - [`import_status(Option<ImportStatus>)`](crate::operation::delete_import::DeleteImportOutput::import_status): <p>The current status of the deletion. When the deletion is complete, the import will no longer be returned by the <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_ListImports.html">ListImports</a> operation and calls to the <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_DescribeImport.html">DescribeImport</a> operation with the import identifier will fail.</p>
    /// - On failure, responds with [`SdkError<DeleteImportError>`](crate::operation::delete_import::DeleteImportError)
    pub fn delete_import(
        &self,
    ) -> crate::operation::delete_import::builders::DeleteImportFluentBuilder {
        crate::operation::delete_import::builders::DeleteImportFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
