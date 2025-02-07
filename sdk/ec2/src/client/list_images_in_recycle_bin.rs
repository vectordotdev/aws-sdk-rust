// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListImagesInRecycleBin`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`image_ids(Vec<String>)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::image_ids) / [`set_image_ids(Option<Vec<String>>)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::set_image_ids): <p>The IDs of the AMIs to list. Omit this parameter to list all of the AMIs that are in the Recycle Bin. You can specify up to 20 IDs in a single request.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    ///   - [`max_results(i32)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    ///   - [`dry_run(bool)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ListImagesInRecycleBinOutput`](crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput) with field(s):
    ///   - [`images(Option<Vec<ImageRecycleBinInfo>>)`](crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput::images): <p>Information about the AMIs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<ListImagesInRecycleBinError>`](crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinError)
    pub fn list_images_in_recycle_bin(
        &self,
    ) -> crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder
    {
        crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinFluentBuilder::new(self.handle.clone())
    }
}
