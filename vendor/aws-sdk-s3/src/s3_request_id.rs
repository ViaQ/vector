// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_runtime_api::http::{Headers, Response};
use aws_smithy_types::error::metadata::{Builder as ErrorMetadataBuilder, ErrorMetadata};

const EXTENDED_REQUEST_ID: &str = "s3_extended_request_id";

/// Trait to retrieve the S3-specific extended request ID
///
/// Read more at <https://aws.amazon.com/premiumsupport/knowledge-center/s3-request-id-values/>.
pub trait RequestIdExt {
    /// Returns the S3 Extended Request ID necessary when contacting AWS Support.
    fn extended_request_id(&self) -> Option<&str>;
}

impl<E> RequestIdExt for SdkError<E, Response> {
    fn extended_request_id(&self) -> Option<&str> {
        match self {
            Self::ResponseError(err) => err.raw().headers().extended_request_id(),
            Self::ServiceError(err) => err.raw().headers().extended_request_id(),
            _ => None,
        }
    }
}

impl RequestIdExt for ErrorMetadata {
    fn extended_request_id(&self) -> Option<&str> {
        self.extra(EXTENDED_REQUEST_ID)
    }
}

impl<B> RequestIdExt for Response<B> {
    fn extended_request_id(&self) -> Option<&str> {
        self.headers().extended_request_id()
    }
}

impl RequestIdExt for Headers {
    fn extended_request_id(&self) -> Option<&str> {
        self.get("x-amz-id-2")
    }
}

impl<O, E> RequestIdExt for Result<O, E>
where
    O: RequestIdExt,
    E: RequestIdExt,
{
    fn extended_request_id(&self) -> Option<&str> {
        match self {
            Ok(ok) => ok.extended_request_id(),
            Err(err) => err.extended_request_id(),
        }
    }
}

/// Applies the extended request ID to a generic error builder
pub(crate) fn apply_extended_request_id(builder: ErrorMetadataBuilder, headers: &Headers) -> ErrorMetadataBuilder {
    if let Some(extended_request_id) = headers.extended_request_id() {
        builder.custom(EXTENDED_REQUEST_ID, extended_request_id)
    } else {
        builder
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_smithy_runtime_api::client::result::SdkError;
    use aws_smithy_types::body::SdkBody;

    #[test]
    fn handle_missing_header() {
        let resp = Response::try_from(http::Response::builder().status(400).body("").unwrap()).unwrap();
        let mut builder = ErrorMetadata::builder().message("123");
        builder = apply_extended_request_id(builder, resp.headers());
        assert_eq!(builder.build().extended_request_id(), None);
    }

    #[test]
    fn test_extended_request_id_sdk_error() {
        let without_extended_request_id = || Response::try_from(http::Response::builder().body(SdkBody::empty()).unwrap()).unwrap();
        let with_extended_request_id = || {
            Response::try_from(
                http::Response::builder()
                    .header("x-amz-id-2", "some-request-id")
                    .body(SdkBody::empty())
                    .unwrap(),
            )
            .unwrap()
        };
        assert_eq!(
            None,
            SdkError::<(), _>::response_error("test", without_extended_request_id()).extended_request_id()
        );
        assert_eq!(
            Some("some-request-id"),
            SdkError::<(), _>::response_error("test", with_extended_request_id()).extended_request_id()
        );
        assert_eq!(None, SdkError::service_error((), without_extended_request_id()).extended_request_id());
        assert_eq!(
            Some("some-request-id"),
            SdkError::service_error((), with_extended_request_id()).extended_request_id()
        );
    }

    #[test]
    fn test_extract_extended_request_id() {
        let mut headers = Headers::new();
        assert_eq!(None, headers.extended_request_id());

        headers.append("x-amz-id-2", "some-request-id");
        assert_eq!(Some("some-request-id"), headers.extended_request_id());
    }

    #[test]
    fn test_apply_extended_request_id() {
        let mut headers = Headers::new();
        assert_eq!(
            ErrorMetadata::builder().build(),
            apply_extended_request_id(ErrorMetadata::builder(), &headers).build(),
        );

        headers.append("x-amz-id-2", "some-request-id");
        assert_eq!(
            ErrorMetadata::builder().custom(EXTENDED_REQUEST_ID, "some-request-id").build(),
            apply_extended_request_id(ErrorMetadata::builder(), &headers).build(),
        );
    }

    #[test]
    fn test_error_metadata_extended_request_id_impl() {
        let err = ErrorMetadata::builder().custom(EXTENDED_REQUEST_ID, "some-request-id").build();
        assert_eq!(Some("some-request-id"), err.extended_request_id());
    }
}
