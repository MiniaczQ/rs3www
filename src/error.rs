use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use axum::body::Body;

use hyper::StatusCode;

use axum::response::Response;

use axum::response::IntoResponse;

use aws_sdk_s3::operation::get_object::GetObjectError;

use aws_sdk_s3::error::SdkError;

pub(crate) enum AppError {
    NotFound,
    BadGateway,
    InternalServerError,
    MethodNotAllowed,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))
                .unwrap(),
            AppError::BadGateway => Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("Bad gateway"))
                .unwrap(),
            AppError::InternalServerError => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap(),
            AppError::MethodNotAllowed => Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::from("Method not allowed"))
                .unwrap(),
        }
    }
}

impl<E> From<SdkError<GetObjectError, E>> for AppError {
    fn from(error: SdkError<GetObjectError, E>) -> Self {
        match error {
            SdkError::ServiceError(error) => {
                if error.err().is_no_such_key() {
                    AppError::NotFound
                } else {
                    AppError::BadGateway
                }
            }
            _ => AppError::InternalServerError,
        }
    }
}

impl<E> From<SdkError<ListObjectsV2Error, E>> for AppError {
    fn from(error: SdkError<ListObjectsV2Error, E>) -> Self {
        match error {
            SdkError::ServiceError(_) => AppError::BadGateway,
            _ => AppError::InternalServerError,
        }
    }
}
