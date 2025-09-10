# Audit Logging & Retention (Scaffold)

## Station Logs
- Operation logs with timestamps, device, mode, result
- Signed log bundles uploaded to server

## Server Logs
- API access logs, job lifecycle, auth events
- Correlation IDs per job/station

## Centralization
- Ship to SIEM (Splunk/ELK), structured JSON
- Retention policies: 1–7 years per org policy

## Privacy
- PII minimization; role-based access
