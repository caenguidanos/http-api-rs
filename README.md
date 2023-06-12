# HTTP API Rust template

Template for production ready HTTP servers in Rust.

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

| Environment variable      | Description                                                   | Example                                            | Documentation                                                                 |
|---------------------------|---------------------------------------------------------------|----------------------------------------------------|-------------------------------------------------------------------------------|
| `OAUTH_CLIENT_ID`         | The client_id of the application from your oauth provider     | sdf82yufuysdfusdy28                                | [Auth0](https://auth0.com/docs/get-started/applications/application-settings) |
| `OAUTH_DOMAIN`            | Your domain from your oauth provider                          | https://auth.example.com                           | [Auth0](https://auth0.com/docs/customize/custom-domains)                      |
| `OAUTH_AUDIENCE`          | Your API identifier as oauth resource                         | https://api.example.com                            | [Auth0](https://auth0.com/docs/get-started/apis/api-settings)                 |
| `OAUTH_AUTHORIZATION_URL` | OAuth Authorization URL of your application with the audience | `OAUTH_DOMAIN`/authorize?audience=`OAUTH_AUDIENCE` | [Auth0](https://auth0.com/docs/get-started/apis/api-settings)                 |
| `OAUTH_TOKEN_URL`         | OAuth Token URL of your application with the audience         | `OAUTH_DOMAIN`/oauth/token                         | [Auth0](https://auth0.com/docs/get-started/apis/api-settings)                 |

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

### Build

```shell
cargo build --release
```

### Local Test

#### Start infrastructure

```shell
docker compose up
```

#### Execute Rust tests with Postgres Template Database

More info [here](https://www.postgresql.org/docs/current/manage-ag-templatedbs.html).

```shell
export DATABASE_TEMPLATE=ecommerce_template

cargo test --release
```

### Screenshots

![swagger](./static/img/swagger.png)
![jaeger](./static/img/jaeger.png)
![pgweb](./static/img/pgweb.png)