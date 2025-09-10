# Central Dashboard & API (Scaffold)

Coordinates wiping stations, provides batch jobs, stores certificates, and exposes REST APIs.

## Architecture
- **API**: REST/JSON (OpenAPI)
- **Auth**: OIDC (admin) + service accounts (agents)
- **Storage**: Postgres (metadata), S3-compatible object store (PDF/JSON certs)
- **Messaging**: Redis/NATS for jobs and heartbeat

## Endpoints (initial)
- POST /v1/stations/register
- POST /v1/jobs (create batch)
- GET  /v1/jobs/:id
- POST /v1/jobs/:id/assign
- POST /v1/results (upload certificate & logs)

Run: `cargo run` (listens on 0.0.0.0:8080)
