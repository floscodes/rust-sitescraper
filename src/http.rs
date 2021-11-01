use reqwest;

/// Sends a http-request to the given URL and returns the response as a [`Result`] that can be unwrapped to a [`String`] if the request was successful.
/// 
/// # Example
/// 
/// ```
/// use sitescraper;
///
/// let html = sitescraper::http::get("http://example.com/).unwrap();
/// ```
pub async fn get(url: &str) -> Result<String, reqwest::Error> {
    let out = reqwest::get(url).await?;
    let out = out.text().await?;
    Ok(out)
}

