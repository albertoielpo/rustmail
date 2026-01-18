use serde::Deserialize;

/// Email payload structure containing all email details
///
/// This structure represents the actual email content and metadata
/// that will be sent through the SMTP server.
#[derive(Deserialize)]
pub struct SendMailPayload {
    /// Sender email address (e.g., "sender@example.com")
    pub from: String,

    /// List of recipient email addresses
    pub to: Vec<String>,

    /// Email subject line
    pub subject: String,

    /// Email body text (can be plain text or base64 encoded)
    pub text: String,

    /// Encoding type for the text field (e.g., "plain" or "base64")
    pub encoding: String,
}

/// Request wrapper for sending an email
///
/// This is the top-level structure received from the HTTP POST request.
#[derive(Deserialize)]
pub struct SendMailReq {
    /// The email payload containing all email details
    pub mail: SendMailPayload,
}
