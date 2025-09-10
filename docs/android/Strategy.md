# Android Strategy (Scaffold)

## Constraints
- Unrooted devices: no raw block access; factory reset is the supported path

## Consumer Path
- Guided factory reset + app data wipe
- Attestation: Play Integrity/SafetyNet proof of reset (store token)

## Enterprise Path
- Device Owner app via Android Management API / EMM
- Policies: wipe/lock, FRP bypass handling, audit callbacks
- OEM integrations (MDM/OEM APIs) for deeper wipe where supported

## Certificates
- Mark Android resets with attestation level in certificate
- Include device policy state and management domain
