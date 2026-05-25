use anyhow::{anyhow, Context};
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};

/// The single HTTP roundtrip shared by every provider. It exists to remove
/// the JSON-encode / build-request / check-status / JSON-decode boilerplate
/// that would otherwise be copy-pasted across Gemini, Anthropic, and OpenAI
/// clients.
///
/// Behaviour:
///   1. Serializes `body` to JSON and POSTs it to `url` with the supplied
///      headers.
///   2. On HTTP 4xx/5xx returns an error containing the status line plus a
///      truncated response body so callers can surface the upstream
///      message.
///   3. Otherwise deserializes the response into `T`.
pub(crate) async fn post_json<T, B>(
    url: &str,
    headers: &[(&str, &str)],
    body: &B,
) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
    B: serde::Serialize + ?Sized,
{
    let mut h = HeaderMap::with_capacity(headers.len() + 1);
    h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    for (k, v) in headers {
        let name = HeaderName::from_bytes(k.as_bytes())
            .with_context(|| format!("invalid header name: {k}"))?;
        let val =
            HeaderValue::from_str(v).with_context(|| format!("invalid header value for {k}"))?;
        h.insert(name, val);
    }

    let res = CLIENT
        .post(url)
        .headers(h)
        .json(body)
        .send()
        .await
        .map_err(|e| anyhow!("request: {e}"))?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        let snippet: String = body.chars().take(2048).collect();
        return Err(anyhow!(
            "{}: {}",
            status.canonical_reason().unwrap_or("HTTP error"),
            snippet.trim()
        ));
    }
    let parsed = res.json::<T>().await.map_err(|e| anyhow!("decode: {e}"))?;
    Ok(parsed)
}

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("rust-tut/1.0 (+https://github.com/0xm1kr)")
        .build()
        .expect("build reqwest client")
});
