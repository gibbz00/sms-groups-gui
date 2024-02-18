# SMS Groups GUI - HATEOAS stack experimentations.

## Demoing

### Services
```sh
docker compose up
```

### API server

Both binaries will by default write to files using their respective binary names to `/var/log`.
Make sure those files exists with write access.

```sh
# as root
touch /var/log/{seed-,}sms-groups-api.log
chmod 666 /var/log/{seed-,}sms-groups-api.log
```

Seed root organization and admin:

```sh
cargo run --bin seed-sms-groups-api
```

Run API server:

```sh
cargo run --bin sms-groups-api
```
