use axum_extra::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agent(TypedHeader(header_from_user): TypedHeader<UserAgent>) -> String {
    header_from_user.to_string()
}
