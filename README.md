# Stridespot Email System

This service exposes HTTP endpoints for sending transactional and marketing email notifications through the `lettre` mailer. The server is built with `ntex` and accepts JSON payloads for each endpoint.

## Server

- Host: `127.0.0.1`
- Port: `8080`
- Base URL: `http://127.0.0.1:8080`
- Content type: `application/json`

## Running

```bash
cargo run
```

## Endpoints

All routes are `POST` endpoints.

### 1. `/delivery`

Sends a delivery confirmation email.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com",
  "delivery_id": "DL123456"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/delivery \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","delivery_id":"DL123456"}'
```

Response:

- `200 OK` on success
- Empty body

### 2. `/login`

Sends a login alert email when a user signs in.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com",
  "device": "Android",
  "location": "Uyo",
  "time": "2026-05-05T12:00:00Z"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/login \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","device":"Android","location":"Uyo","time":"2026-05-05T12:00:00Z"}'
```

Response:

- `200 OK` on success
- Empty body

### 3. `/marketing`

Sends a marketing or promotional email.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com",
  "topic": "Special Offer",
  "body": "Enjoy 20% off your next delivery with Spotrider!"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/marketing \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","topic":"Special Offer","body":"Enjoy 20% off your next delivery with Spotrider!"}'
```

Response:

- `200 OK` on success
- Empty body

### 4. `/otp`

Generates and sends a verification OTP email.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/otp \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com"}'
```

Response:

- `200 OK` on success
- Body contains generated OTP:

```json
{
  "OTP": "123456"
}
```

### 5. `/payment`

Sends a deposit confirmation email for a processed payment.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com",
  "sender_name": "John Doe",
  "amount": "USD 99.00",
  "order_id": "ORD987654"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/payment \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","sender_name":"John Doe","amount":"USD 99.00","order_id":"ORD987654"}'
```

Response:

- `200 OK` on success
- Empty body

### 6. `/registration`

Sends a welcome email after successful registration.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/registration \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com"}'
```

Response:

- `200 OK` on success
- Empty body

### 7. `/resetpassword`

Generates a temporary password and sends it by email.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/resetpassword \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com"}'
```

Response:

- `200 OK` on success
- Body contains temporary password:

```json
{
  "tempoary_password": "Ab12Cd34Ef56"
}
```

### 8. `/updates`

Sends feature or product update announcements.

Request body schema:

```json
{
  "name": "Jane Doe",
  "email": "jane@example.com",
  "feature_topic": "New Dashboard Features",
  "body": "We added delivery tracking, driver ETA, and account security improvements.",
  "site_link": "https://example.com/updates"
}
```

Example curl:

```bash
curl -X POST http://127.0.0.1:8080/updates \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","feature_topic":"New Dashboard Features","body":"We added delivery tracking, driver ETA, and account security improvements.","site_link":"https://example.com/updates"}'
```

Response:

- `200 OK` on success
- Empty body

## Notes

- All endpoints accept JSON and return `200 OK` when the email dispatch function completes successfully.
- The `/otp` endpoint returns a generated OTP string.
- The `/resetpassword` endpoint returns a temporary password string.
- The current implementation does not require authentication headers.
- If you want to extend the service, add new request structs and register the route in `src/main.rs`.
