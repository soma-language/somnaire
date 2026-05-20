use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::either::Either;
use serde::Serialize;
use std::ops::FromResidual;

pub struct ApiResponse<T>(pub (StatusCode, Either<Json<T>, Json<ApiError>>));

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum ApiErrorCode {
    InternalError,
    InvalidJson,
    InvalidCredentials,
    InvalidVerification,
    AccountAlreadyExists,
    CaptchaVerificationFailed,
    UnauthorizedToken,
    DomainAlreadyInUse,
    InvalidDomain,
    UnauthorizedDomain,
    InvalidAccountDomain,
    DomainAccountMismatch,
    InvalidReferralCode,
    AlreadyExists,
    NotFound,
    InvalidInviteCode,
    NotBusinessMember,
    NotInfluencer,
    AlreadySwiped,
    MissingField,
    CannotSwipeOnYourOwnProfile,
    NotConversationParticipant,
    InvalidMessage,
    BusinessNotFound,
    SwipeLimitReached,
    SuperLikeLimitReached,
    PremiumRequired,
    StripeError,
    InvalidWebhookSignature,
    SeedingCreditAlreadyUsed,
    ContractTooSmall,
    InvalidContractState,
    InvalidFileType,
    FileTooLarge,
}

impl ApiErrorCode {
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            ApiErrorCode::InternalError => "INTERNAL_ERROR",
            ApiErrorCode::InvalidJson => "INVALID_JSON",
            ApiErrorCode::InvalidCredentials => "INVALID_CREDENTIALS",
            ApiErrorCode::InvalidVerification => "INVALID_VERIFICATION",
            ApiErrorCode::AccountAlreadyExists => "ACCOUNT_ALREADY_EXISTS",
            ApiErrorCode::CaptchaVerificationFailed => "CAPTCHA_VERIFICATION_FAILED",
            ApiErrorCode::UnauthorizedToken => "UNAUTHORIZED_TOKEN",
            ApiErrorCode::DomainAlreadyInUse => "DOMAIN_IN_USE",
            ApiErrorCode::InvalidDomain => "INVALID_DOMAIN",
            ApiErrorCode::UnauthorizedDomain => "UNAUTHORIZED_DOMAIN",
            ApiErrorCode::InvalidAccountDomain => "INVALID_ACCOUNT_DOMAIN",
            ApiErrorCode::DomainAccountMismatch => "DOMAIN_ACCOUNT_MISMATCH",
            ApiErrorCode::InvalidReferralCode => "INVALID_REFERRAL_CODE",
            ApiErrorCode::AlreadyExists => "ALREADY_EXISTS",
            ApiErrorCode::NotFound => "NOT_FOUND",
            ApiErrorCode::InvalidInviteCode => "INVALID_INVITE_CODE",
            ApiErrorCode::NotBusinessMember => "NOT_BUSINESS_MEMBER",
            ApiErrorCode::NotInfluencer => "NOT_INFLUENCER",
            ApiErrorCode::AlreadySwiped => "ALREADY_SWIPED",
            ApiErrorCode::MissingField => "MISSING_FIELD",
            ApiErrorCode::CannotSwipeOnYourOwnProfile => "SWIPING_ON_YOUR_PROFILE",
            ApiErrorCode::NotConversationParticipant => "NOT_CONVERSATION_PARTICIPANT",
            ApiErrorCode::InvalidMessage => "INVALID_MESSAGE",
            ApiErrorCode::BusinessNotFound => "BUSINESS_NOT_FOUND",
            ApiErrorCode::SwipeLimitReached => "SWIPE_LIMIT_REACHED",
            ApiErrorCode::SuperLikeLimitReached => "SUPER_LIKE_LIMIT_REACHED",
            ApiErrorCode::PremiumRequired => "PREMIUM_REQUIRED",
            ApiErrorCode::StripeError => "STRIPE_ERROR",
            ApiErrorCode::InvalidWebhookSignature => "INVALID_WEBHOOK_SIGNATURE",
            ApiErrorCode::SeedingCreditAlreadyUsed => "SEEDING_CREDIT_ALREADY_USED",
            ApiErrorCode::ContractTooSmall => "CONTRACT_TOO_SMALL",
            ApiErrorCode::InvalidContractState => "INVALID_CONTRACT_STATE",
            ApiErrorCode::InvalidFileType => "INVALID_FILE_TYPE",
            ApiErrorCode::FileTooLarge => "FILE_TOO_LARGE",
        }
    }
}

pub fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse((StatusCode::OK, Either::E1(Json(data))))
}

#[must_use]
pub fn no_content() -> ApiResponse<()> {
    ApiResponse((StatusCode::NO_CONTENT, Either::E1(Json(()))))
}

#[must_use]
pub fn internal_error<T: Serialize>() -> ApiResponse<T> {
    error(
        StatusCode::INTERNAL_SERVER_ERROR,
        ApiErrorCode::InternalError,
        "An internal server error occurred",
    )
}

#[must_use]
pub fn error<T: Serialize>(
    status: StatusCode,
    code: ApiErrorCode,
    message: &str,
) -> ApiResponse<T> {
    ApiResponse((
        status,
        Either::E2(Json(ApiError {
            code: code.code(),
            message: String::from(message),
            details: None,
        })),
    ))
}

pub fn api_error_with_details<T: Serialize>(
    status: StatusCode,
    code: &'static str,
    message: &str,
    details: serde_json::Value,
) -> ApiResponse<T> {
    ApiResponse((
        status,
        Either::E2(Json(ApiError {
            code,
            message: String::from(message),
            details: Some(details),
        })),
    ))
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

impl<T: Serialize, E> FromResidual<Result<std::convert::Infallible, E>> for ApiResponse<T> {
    fn from_residual(_residual: Result<std::convert::Infallible, E>) -> Self {
        internal_error()
    }
}

impl<T> From<ApiError> for ApiResponse<T> {
    fn from(error: ApiError) -> Self {
        ApiResponse((StatusCode::BAD_REQUEST, Either::E2(Json(error))))
    }
}
