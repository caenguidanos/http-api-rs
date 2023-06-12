# HTTP API Rust template

Template for production ready HTTP servers in Rust.

- **CI**: Dagger
- **Database**: Postgres
- **HTTP Server**: Axum
- **Security**: OAuth2 RBAC
- **Telemetry**: OpenTelemetry/Jaeger
- **OpenApi 3.0**: Swagger

### Required dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/engine/install/)

### Environment

- Create `.env` from `.env.example` and fulfill values.
- Create `static/environments/.env.jaeger` from `static/environments/.env.jaeger.example` and fulfill values.
- Create `static/environments/.env.postgres` from `static/environments/.env.postgres.example` and fulfill values.
- Create `static/environments/.env.swagger` from `static/environments/.env.swagger.example` and fulfill values.

| Environment variable | Description                                                    | Example                  | Documentation                                                                 |
|----------------------|----------------------------------------------------------------|--------------------------|-------------------------------------------------------------------------------|
| `OAUTH_CLIENT_ID`    | The client_id of the application from your **oauth2** provider | sdf82yufuysdfusdy28      | [Auth0](https://auth0.com/docs/get-started/applications/application-settings) |
| `OAUTH_DOMAIN`       | The domain of your **oauth2** provider                         | https://auth.example.com | [Auth0](https://auth0.com/docs/customize/custom-domains)                      |
| `OAUTH_AUDIENCE`     | Your API identifier as **oauth2** resource                     | https://api.example.com  | [Auth0](https://auth0.com/docs/get-started/apis/api-settings)                 |

#### Replace variables on OpenApi

```yaml
components:
  securitySchemes:
    Identity:
      type: oauth2
      flows:
        authorizationCode:
          authorizationUrl: %OAUTH_DOMAIN%/authorize?audience=%OAUTH_AUDIENCE%
          tokenUrl: %OAUTH_DOMAIN%/oauth/token
          scopes: # permissions
            ecommerce.product:read: Read products
            ecommerce.product:create: Create products
            ecommerce.product:delete: Delete products
```

### Install

Node dependencies are required for **Dagger**.

```shell
npm install
```

### Run

#### Start server

```shell
cargo run --release
```

- Axum HTTP Server will run on: `:8080`

#### Start infrastructure

```shell
docker compose up
```

Database **seed** is injected with `ci/init/pg_init.sh`.

- Postgres will run on: `:5432`
- Postgres GUI will run on: `:5433`
- Jaeger will run on: `:16686`
- Swagger will run on: `:9000`

### CI Test

It's required [Node](https://nodejs.org/en) installed on host for executing the pipeline.

```shell
export DATABASE_TEMPLATE=ecommerce_template

make test
```

### Build

```shell
cargo build --release
```

### Manual test

#### Start DB

```shell
docker compose up -V --force-recreate postgres
```

#### Run tests

```shell
export DATABASE_TEMPLATE=ecommerce_template

cargo test --release
```
