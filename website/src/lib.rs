use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod wit {
    wit_bindgen::generate!({
        inline: r#"
        package qr:website;
        world website {
            import example:qr/qr@1.0.0;
        }
        "#,
        path: "../wit",
    });
}

use wit::example::qr::qr;

#[derive(serde::Deserialize)]
struct QRQuery {
    url: String,
    size: u32,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_website(req: Request) -> anyhow::Result<impl IntoResponse> {
    // To avoid writing a *shudder* Web site, get the info from the query string
    let qr_query: QRQuery = serde_querystring::from_str(req.query(), serde_querystring::ParseMode::Duplicate)?;

    let qr_string = qr::get_qr_code(&qr_query.url, qr_query.size);

    let totally_real_web_page = format!("SVG for QR code:\n{qr_string}\n");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(totally_real_web_page)
        .build())
}
