# SMS Groups GUI - HATEOAS stack experimentations.

## Demoing

### Services
```sh
docker compose up
```

### API server

Seed root organization and admin:

```sh
cargo run --bin seed-sms-groups-api
```

Run API server:

```sh
cargo run --bin sms-groups-api
```
