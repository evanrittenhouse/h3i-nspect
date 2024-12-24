use h3i::{client::connection_summary::ConnectionSummary, quiche::h3::{Header, WireErrorCode}};
use std::iter::once;

/// Host name for a request
#[derive(Clone, Debug)]
pub struct Host(pub String);

impl From<String> for Host {
    fn from(s: String) -> Self {
        match s.strip_prefix("https://") {
            Some(s) => Self(s.to_owned()),
            None => Self(s),
        }
    }
}

impl AsRef<str> for Host {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub fn assert_connection_error(
    is_app: bool,
    error_code: WireErrorCode,
) -> Box<dyn Fn(&ConnectionSummary) -> bool + Send + Sync> {
    Box::new(move |cs: &ConnectionSummary| {
        cs.conn_close_details
            .peer_error()
            .is_some_and(|e| e.is_app == is_app && e.error_code == error_code as u64)
    })
}

pub fn default_headers(host: &str) -> Vec<Header> {
    println!("host: {host:?}");
    vec![
        Header::new(b":method", b"GET"),
        Header::new(b":scheme", b"https"),
        Header::new(b":authority", host.as_bytes()),
        Header::new(b":path", b"/"),
    ]
}

pub fn default_headers_plus(host: &str, header: Header) -> Vec<Header> {
    default_headers(host)
        .into_iter()
        .chain(once(header))
        .collect::<Vec<Header>>()
}
