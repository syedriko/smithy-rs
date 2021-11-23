// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn serialize_complete_snapshot_response(
    output: &crate::output::CompleteSnapshotOutput,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let payload =
            crate::operation_ser::serialize_structure_crate_output_complete_snapshot_output(
                output,
            )?;
        #[allow(unused_mut)]
        let mut response = http::Response::builder();
        response.body(aws_smithy_http_server::Body::from(payload))?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_complete_snapshot_error(
    error: &crate::error::CompleteSnapshotError,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let response: http::Response<aws_smithy_http_server::Body>;
        match error {
            crate::error::CompleteSnapshotError::AccessDeniedError(var_1) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_access_denied_error(
                        var_1,
                    )?;
                response = http::Response::builder()
                    .status(403)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::CompleteSnapshotError::InternalServerError(var_2) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_internal_server_error(
                        var_2,
                    )?;
                response = http::Response::builder()
                    .status(500)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::CompleteSnapshotError::RequestThrottledError(var_3) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_request_throttled_error(
                        var_3,
                    )?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::CompleteSnapshotError::ResourceNotFoundError(var_4) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_resource_not_found_error(
                        var_4,
                    )?;
                response = http::Response::builder()
                    .status(404)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::CompleteSnapshotError::ServiceQuotaExceededError(var_5) => {
                let payload = crate::operation_ser::serialize_structure_crate_error_service_quota_exceeded_error(var_5)?;
                response = http::Response::builder()
                    .status(402)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::CompleteSnapshotError::ValidationError(var_6) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_validation_error(var_6)?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
        };
        response
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_list_changed_blocks_response(
    output: &crate::output::ListChangedBlocksOutput,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let payload =
            crate::operation_ser::serialize_structure_crate_output_list_changed_blocks_output(
                output,
            )?;
        #[allow(unused_mut)]
        let mut response = http::Response::builder();
        response.body(aws_smithy_http_server::Body::from(payload))?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_list_changed_blocks_error(
    error: &crate::error::ListChangedBlocksError,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let response: http::Response<aws_smithy_http_server::Body>;
        match error {
            crate::error::ListChangedBlocksError::AccessDeniedError(var_7) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_access_denied_error(
                        var_7,
                    )?;
                response = http::Response::builder()
                    .status(403)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListChangedBlocksError::InternalServerError(var_8) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_internal_server_error(
                        var_8,
                    )?;
                response = http::Response::builder()
                    .status(500)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListChangedBlocksError::RequestThrottledError(var_9) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_request_throttled_error(
                        var_9,
                    )?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListChangedBlocksError::ResourceNotFoundError(var_10) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_resource_not_found_error(
                        var_10,
                    )?;
                response = http::Response::builder()
                    .status(404)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListChangedBlocksError::ServiceQuotaExceededError(var_11) => {
                let payload = crate::operation_ser::serialize_structure_crate_error_service_quota_exceeded_error(var_11)?;
                response = http::Response::builder()
                    .status(402)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListChangedBlocksError::ValidationError(var_12) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_validation_error(var_12)?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
        };
        response
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_list_snapshot_blocks_response(
    output: &crate::output::ListSnapshotBlocksOutput,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let payload =
            crate::operation_ser::serialize_structure_crate_output_list_snapshot_blocks_output(
                output,
            )?;
        #[allow(unused_mut)]
        let mut response = http::Response::builder();
        response.body(aws_smithy_http_server::Body::from(payload))?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_list_snapshot_blocks_error(
    error: &crate::error::ListSnapshotBlocksError,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let response: http::Response<aws_smithy_http_server::Body>;
        match error {
            crate::error::ListSnapshotBlocksError::AccessDeniedError(var_13) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_access_denied_error(
                        var_13,
                    )?;
                response = http::Response::builder()
                    .status(403)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListSnapshotBlocksError::InternalServerError(var_14) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_internal_server_error(
                        var_14,
                    )?;
                response = http::Response::builder()
                    .status(500)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListSnapshotBlocksError::RequestThrottledError(var_15) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_request_throttled_error(
                        var_15,
                    )?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListSnapshotBlocksError::ResourceNotFoundError(var_16) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_resource_not_found_error(
                        var_16,
                    )?;
                response = http::Response::builder()
                    .status(404)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListSnapshotBlocksError::ServiceQuotaExceededError(var_17) => {
                let payload = crate::operation_ser::serialize_structure_crate_error_service_quota_exceeded_error(var_17)?;
                response = http::Response::builder()
                    .status(402)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::ListSnapshotBlocksError::ValidationError(var_18) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_validation_error(var_18)?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
        };
        response
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_put_snapshot_block_response(
    output: &crate::output::PutSnapshotBlockOutput,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let payload =
            crate::operation_ser::serialize_structure_crate_output_put_snapshot_block_output(
                output,
            )?;
        #[allow(unused_mut)]
        let mut response = http::Response::builder();
        response.body(aws_smithy_http_server::Body::from(payload))?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_put_snapshot_block_error(
    error: &crate::error::PutSnapshotBlockError,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let response: http::Response<aws_smithy_http_server::Body>;
        match error {
            crate::error::PutSnapshotBlockError::AccessDeniedError(var_19) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_access_denied_error(
                        var_19,
                    )?;
                response = http::Response::builder()
                    .status(403)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::PutSnapshotBlockError::InternalServerError(var_20) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_internal_server_error(
                        var_20,
                    )?;
                response = http::Response::builder()
                    .status(500)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::PutSnapshotBlockError::RequestThrottledError(var_21) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_request_throttled_error(
                        var_21,
                    )?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::PutSnapshotBlockError::ResourceNotFoundError(var_22) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_resource_not_found_error(
                        var_22,
                    )?;
                response = http::Response::builder()
                    .status(404)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::PutSnapshotBlockError::ServiceQuotaExceededError(var_23) => {
                let payload = crate::operation_ser::serialize_structure_crate_error_service_quota_exceeded_error(var_23)?;
                response = http::Response::builder()
                    .status(402)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::PutSnapshotBlockError::ValidationError(var_24) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_validation_error(var_24)?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
        };
        response
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_start_snapshot_response(
    output: &crate::output::StartSnapshotOutput,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let payload =
            crate::operation_ser::serialize_structure_crate_output_start_snapshot_output(output)?;
        #[allow(unused_mut)]
        let mut response = http::Response::builder();
        response.body(aws_smithy_http_server::Body::from(payload))?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn serialize_start_snapshot_error(
    error: &crate::error::StartSnapshotError,
) -> std::result::Result<
    http::Response<aws_smithy_http_server::Body>,
    aws_smithy_http_server::rejection::SmithyRejection,
> {
    Ok({
        let response: http::Response<aws_smithy_http_server::Body>;
        match error {
            crate::error::StartSnapshotError::AccessDeniedError(var_25) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_access_denied_error(
                        var_25,
                    )?;
                response = http::Response::builder()
                    .status(403)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::ConcurrentLimitExceededError(var_26) => {
                let payload = crate::operation_ser::serialize_structure_crate_error_concurrent_limit_exceeded_error(var_26)?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::ConflictError(var_27) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_conflict_error(var_27)?;
                response = http::Response::builder()
                    .status(503)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::InternalServerError(var_28) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_internal_server_error(
                        var_28,
                    )?;
                response = http::Response::builder()
                    .status(500)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::RequestThrottledError(var_29) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_request_throttled_error(
                        var_29,
                    )?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::ResourceNotFoundError(var_30) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_resource_not_found_error(
                        var_30,
                    )?;
                response = http::Response::builder()
                    .status(404)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::ServiceQuotaExceededError(var_31) => {
                let payload = crate::operation_ser::serialize_structure_crate_error_service_quota_exceeded_error(var_31)?;
                response = http::Response::builder()
                    .status(402)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
            crate::error::StartSnapshotError::ValidationError(var_32) => {
                let payload =
                    crate::operation_ser::serialize_structure_crate_error_validation_error(var_32)?;
                response = http::Response::builder()
                    .status(400)
                    .body(aws_smithy_http_server::Body::from(payload))?;
            }
        };
        response
    })
}

pub fn serialize_structure_crate_output_complete_snapshot_output(
    value: &crate::output::CompleteSnapshotOutput,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_output_complete_snapshot_output(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_access_denied_error(
    value: &crate::error::AccessDeniedError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_access_denied_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_internal_server_error(
    value: &crate::error::InternalServerError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_internal_server_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_request_throttled_error(
    value: &crate::error::RequestThrottledError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_request_throttled_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_resource_not_found_error(
    value: &crate::error::ResourceNotFoundError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_resource_not_found_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_service_quota_exceeded_error(
    value: &crate::error::ServiceQuotaExceededError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_service_quota_exceeded_error(
        &mut object,
        value,
    )?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_validation_error(
    value: &crate::error::ValidationError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_validation_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_output_list_changed_blocks_output(
    value: &crate::output::ListChangedBlocksOutput,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_output_list_changed_blocks_output(
        &mut object,
        value,
    )?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_output_list_snapshot_blocks_output(
    value: &crate::output::ListSnapshotBlocksOutput,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_output_list_snapshot_blocks_output(
        &mut object,
        value,
    )?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_output_put_snapshot_block_output(
    value: &crate::output::PutSnapshotBlockOutput,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_output_put_snapshot_block_output(
        &mut object,
        value,
    )?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_output_start_snapshot_output(
    value: &crate::output::StartSnapshotOutput,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_output_start_snapshot_output(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_concurrent_limit_exceeded_error(
    value: &crate::error::ConcurrentLimitExceededError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_concurrent_limit_exceeded_error(
        &mut object,
        value,
    )?;
    object.finish();
    Ok(out)
}

pub fn serialize_structure_crate_error_conflict_error(
    value: &crate::error::ConflictError,
) -> Result<String, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_error_conflict_error(&mut object, value)?;
    object.finish();
    Ok(out)
}