use base64::{engine::general_purpose, Engine as _};
use rand_core::OsRng;
use rand_core::RngCore;
use warp::{http::Uri, reject::Reject, Filter};

// Custom reject type for URI parsing errors
#[derive(Debug)]
struct InvalidUri;

impl Reject for InvalidUri {}

// Generate a random URL-safe token of given length
fn generate_random_token(length: usize) -> String {
    // Generate enough random bytes to get the desired length after base64 encoding
    // Base64 encodes 3 bytes to 4 characters, so we need about length * 3/4 bytes
    let byte_count = (length * 3) / 4 + 1;
    let mut bytes = vec![0u8; byte_count];
    OsRng.fill_bytes(&mut bytes);

    // Encode to URL-safe base64 without padding
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(bytes);

    // Take exactly the requested length
    encoded.chars().take(length).collect::<String>()
}

// Handler for all routes
async fn handle_redirect(
    full_path: warp::filters::path::FullPath,
) -> Result<impl warp::Reply, warp::Rejection> {
    let random_token = generate_random_token(8);
    let redirect_url = format!("/{}", random_token);
    println!(
        "요청 수신: {} -> 리디렉션: {}",
        full_path.as_str(),
        redirect_url
    );

    // Create a Uri from the redirect url string - this should always work for our format
    let uri: Uri = redirect_url
        .parse()
        .map_err(|_| warp::reject::custom(InvalidUri))?;
    Ok(warp::redirect::found(uri))
}

#[tokio::main]
async fn main() {
    // Create route that catches all paths
    let routes = warp::path::full().and_then(handle_redirect);

    println!("Starting infinite redirect server on 0.0.0.0:80");
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
