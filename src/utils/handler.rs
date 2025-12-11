use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose::STANDARD};

use super::utils::{ReqType, convert_to_pdf};

pub async fn handle_conversion(Json(body): Json<ReqType>) -> impl IntoResponse {
    // Convert HTML to base64 data URL
    let data_url = format!("data:text/html;base64,{}", STANDARD.encode(body.html));

    // Convert HTML to PDF with user-specified options
    let pdf = match convert_to_pdf(data_url, body.pdf_options) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("PDF conversion error: {}", err);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "Error while converting to PDF: {}",
                    err
                )))
                .unwrap()
                .into_response();
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .body(Body::from(pdf))
        .unwrap()
        .into_response()
}
