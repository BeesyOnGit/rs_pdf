use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose::STANDARD};

use super::utils::{ReqType, convert_to_pdf};

pub async fn handle_conversion(Json(body): Json<ReqType>) -> impl IntoResponse {
    // convert html to base64 dataUrl
    let data_url = format!("data:text/html;base64,{}", STANDARD.encode(body.html));

    let pdf = match convert_to_pdf(data_url) {
        Ok(file) => file,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "error while converting to pdf : {}",
                    err
                )))
                .unwrap()
                .into_response();
        }
    };

    return Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .body(Body::from(pdf))
        .unwrap()
        .into_response();
}
