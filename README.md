# RustMail

A simple SMTP email sending service built with Rust and Actix-web.

## Configuration

The application is configured using environment variables:

### Server Configuration

- `BIND_ADDR` - Server bind address (default: `0.0.0.0`)
- `BIND_PORT` - Server port (default: `3333`)
- `BIND_WORKERS` - Number of worker threads (default: system CPU count)
- `RUST_LOG` - Logging level (default: `debug`)

### SMTP Configuration

- `SMTP_HOST` - SMTP server hostname (default: `localhost`)
- `SMTP_PORT` - SMTP server port (default: `25`)
- `SMTP_USE_TLS` - Use TLS/STARTTLS encryption (default: `false` for port 25, `true` for other ports)
- `SMTP_USERNAME` - SMTP authentication username (optional)
- `SMTP_PASSWORD` - SMTP authentication password (optional)

## Running the Application

```bash
# Using default settings (plain SMTP on localhost:25 without authentication)
cargo run

# With TLS SMTP (port 587 with STARTTLS)
SMTP_HOST=smtp.example.com SMTP_PORT=587 SMTP_USERNAME=user SMTP_PASSWORD=pass cargo run

# With custom bind address
BIND_ADDR=127.0.0.1 BIND_PORT=8080 cargo run

# Force plain SMTP (no TLS) even on non-standard ports
SMTP_PORT=2525 SMTP_USE_TLS=false cargo run
```

## API Endpoints

### Health Check

```http
GET /
HEAD /
```

### Send Email

```http
POST /send
Content-Type: application/json

{
  "mail": {
    "from": "sender@example.com",
    "to": ["recipient1@example.com", "recipient2@example.com"],
    "subject": "Email Subject",
    "text": "Email body text",
    "encoding": "plain"
  }
}
```

The `encoding` field can be:
- `"plain"` - Plain text
- `"base64"` - Base64 encoded text (will be decoded before sending)

## Example

```bash
curl -X POST http://localhost:3333/send \
  -H "Content-Type: application/json" \
  -d '{
    "mail": {
      "from": "sender@example.com",
      "to": ["recipient@example.com"],
      "subject": "Test Email",
      "text": "This is a test email",
      "encoding": "plain"
    }
  }'
```

## License

MIT
