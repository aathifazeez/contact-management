# 📇 Contact Management API

A **RESTful Contact Management service** built with **Rust** and
**Axum**.\
The API allows managing **persons, mobile numbers, and email addresses**
with validation and business rules.

The service uses **in-memory storage**, making it simple to run and test
locally.

------------------------------------------------------------------------

# 🚀 Features

-   Create and manage **persons**
-   Attach **multiple mobile numbers** to a person
-   Attach **multiple email addresses** to a person
-   **Validation rules** for inputs
-   **Business rule enforcement**
-   **Error handling with structured responses**
-   **Health check endpoints**
-   **RESTful API design**
-   **Async architecture using Tokio**

------------------------------------------------------------------------

# 🧰 Tech Stack

-   Rust
-   Axum
-   Tokio
-   Serde / Serde JSON
-   REST API

------------------------------------------------------------------------

# 📂 Project Structure

    contact-management
    │
    ├── src
    │   ├── app
    │   │   ├── dto
    │   │   │   └── person.rs
    │   │   │
    │   │   ├── handler
    │   │   │   ├── persons.rs
    │   │   │   ├── mobiles.rs
    │   │   │   ├── emails.rs
    │   │   │   └── health.rs
    │   │   │
    │   │   ├── repository
    │   │   │   ├── person_repository.rs
    │   │   │   ├── mobile_repository.rs
    │   │   │   └── email_repository.rs
    │   │   │
    │   │   ├── service
    │   │   │   ├── person_service.rs
    │   │   │   ├── mobile_service.rs
    │   │   │   └── email_service.rs
    │   │   │
    │   │   ├── route
    │   │   │   └── router.rs
    │   │   │
    │   │   └── state.rs
    │   │
    │   ├── pkg
    │   │   ├── config
    │   │   │   └── app_config.rs
    │   │   │
    │   │   ├── error
    │   │   │   └── error.rs
    │   │   │
    │   │   └── web
    │   │       └── web.rs
    │   │
    │   └── main.rs
    │
    ├── Cargo.toml
    └── README.md

------------------------------------------------------------------------

# ⚙️ Installation

### 1. Clone the repository

``` bash
git clone https://github.com/<your-username>/contact-management-api.git
cd contact-management-api
```

### 2. Install Rust

If Rust is not installed:

https://rustup.rs

### 3. Run the application

``` bash
cargo run
```

Server output:

    Contact Management Service running on http://0.0.0.0:3000
    Health check → http://0.0.0.0:3000/livez
    API base → http://0.0.0.0:3000/api/v1

------------------------------------------------------------------------

# 🌐 Base URL

    http://localhost:3000/api/v1

------------------------------------------------------------------------

# ❤️ Health Endpoints

### Liveness

    GET /livez

Response

``` json
{
  "status": "alive"
}
```

### Readiness

    GET /readyz

Response

``` json
{
  "status": "ready"
}
```

------------------------------------------------------------------------

# 👤 Persons API

### Create Person

    POST /api/v1/persons

Request

``` json
{
  "name": "Alice",
  "display_name": "Al"
}
```

Response

``` json
{
  "id": 1,
  "name": "Alice",
  "display_name": "Al"
}
```

------------------------------------------------------------------------

### List Persons

    GET /api/v1/persons

Response

``` json
[
  {
    "id": 1,
    "name": "Alice",
    "display_name": "Al"
  }
]
```

------------------------------------------------------------------------

### Get Person

    GET /api/v1/persons/{id}

Example

    GET /api/v1/persons/1

------------------------------------------------------------------------

### Update Person

    PUT /api/v1/persons/{id}

Request

``` json
{
  "name": "Alice Smith"
}
```

------------------------------------------------------------------------

### Delete Person

    DELETE /api/v1/persons/{id}

------------------------------------------------------------------------

# 📱 Mobile API

### Add Mobile

    POST /api/v1/persons/{person_id}/mobiles

Request

``` json
{
  "number": "+1-555-0101",
  "label": "work"
}
```

------------------------------------------------------------------------

### List Mobiles

    GET /api/v1/persons/{person_id}/mobiles

------------------------------------------------------------------------

### Get Mobile

    GET /api/v1/mobiles/{id}

------------------------------------------------------------------------

### Update Mobile

    PUT /api/v1/mobiles/{id}

------------------------------------------------------------------------

### Delete Mobile

    DELETE /api/v1/mobiles/{id}

------------------------------------------------------------------------

# 📧 Email API

### Add Email

    POST /api/v1/persons/{person_id}/emails

Request

``` json
{
  "address": "alice@example.com",
  "label": "personal"
}
```

------------------------------------------------------------------------

### List Emails

    GET /api/v1/persons/{person_id}/emails

------------------------------------------------------------------------

### Get Email

    GET /api/v1/emails/{id}

------------------------------------------------------------------------

### Update Email

    PUT /api/v1/emails/{id}

------------------------------------------------------------------------

### Delete Email

    DELETE /api/v1/emails/{id}

------------------------------------------------------------------------

# 🔒 Business Rules

A person **cannot be deleted** if they still have:

-   Mobile numbers
-   Email addresses

Example response

``` json
{
  "error": "Cannot delete person 1: they still have mobile numbers. Delete all mobiles first."
}
```

------------------------------------------------------------------------

# ⚠️ Error Responses

Example validation error

``` json
{
  "error": "Person name cannot be empty"
}
```

Example not found

``` json
{
  "error": "Person with id 999 not found"
}
```

------------------------------------------------------------------------

# 🧪 Testing

You can test the API using:

-   Postman
-   curl
-   any HTTP client

Example:

``` bash
curl -X POST http://localhost:3000/api/v1/persons \
-H "Content-Type: application/json" \
-d '{"name":"Alice"}'
```

------------------------------------------------------------------------

# 📌 Notes

-   Storage is **in-memory**
-   Data resets when the server restarts
-   IDs are generated automatically

------------------------------------------------------------------------

# 👨‍💻 Author

Musahhil Rifas

------------------------------------------------------------------------

# 📄 License

This project is for educational purposes.
