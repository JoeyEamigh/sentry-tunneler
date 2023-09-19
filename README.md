# sentry-tunneler

Sentry tunneler is a proxy that forwards requests to Sentry-compatible services. This was heavily inspired by <https://github.com/gbip/sentry_tunnel>.

As the [official Sentry documentation](https://docs.sentry.io/platforms/javascript/troubleshooting/#using-the-tunnel-option) notes:

> A tunnel is an HTTP endpoint that acts as a proxy between Sentry and your application. Because you control this server, there is no risk of any requests sent to it being blocked. When the endpoint lives under the same origin (although it does not have to in order for the tunnel to work), the browser will not treat any requests to the endpoint as a third-party request. As a result, these requests will have different security measures applied which, by default, don't trigger ad-blockers.

## Why not just use sentry_tunnel?

The original sentry_tunnel is also a Rust project, but it is using `gotham` which doesn't have an easy way to enable CORS for some reason. This project instead uses `axum`, which allows for a much less involved setup.

## Configuration

The following environment variables are used to configure the tunneler:

```sh
CORS_ALLOWED_ORIGINS=https://example.com
ALLOWED_SENTRY_HOSTS=https://sentry.example.com
ALLOWED_PROJECT_IDS=1,2,3
TUNNEL_PATH="/tunnel"
LISTEN_PORT=3000
```

`CORS_ALLOWED_ORIGINS`, `ALLOWED_SENTRY_HOSTS`, and `ALLOWED_PROJECT_IDS` are all comma-separated lists of values. `TUNNEL_PATH` is the path that the tunneler will listen on. `LISTEN_PORT` is the port that the tunneler will listen on.

If `CORS_ALLOWED_ORIGINS` is not set, `Access-Control-Allow-Origin` will be set to `*`. If `ALLOWED_SENTRY_HOSTS` is not set, all hosts will be allowed. If `ALLOWED_PROJECT_IDS` is not set, all projects will be allowed. This is a depature from the original `sentry_tunnel` which required all of these to be set.

## Running

This project is designed to be run as a docker container:

```sh
docker run --rm -p 3000:3000 ghcr.io/joeyeamigh/sentry-tunneler:latest
```

## Building

Running locally is easiest if you enable the `dotenv` feature and copy `.env.example` to `.env`:

```sh
cargo run --features dotenv
```
