# Backend test layout

Default backend tests should not require live Postgres, Redis, or Meilisearch. Keep API boundary tests on lazy clients or mocked state where possible, and keep pure logic tests in the module that owns the logic.

Service integration tests live in `service_integration.rs` and are ignored by default. Run them only against disposable local services:

```sh
SOMES_INTEGRATION_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/somes_test \
SOMES_INTEGRATION_REDIS_URL=redis://127.0.0.1:6379/15 \
SOMES_INTEGRATION_MEILISEARCH_URL=http://127.0.0.1:7700 \
SOMES_INTEGRATION_MEILISEARCH_KEY=masterKey \
cargo test -p somes-api --test service_integration -- --ignored
```

Recommended service setup:

- Postgres: start a fresh database, apply migrations, then load a small deterministic fixture set.
- Redis: use an isolated DB number or container and only touch keys prefixed with `somes:test:`.
- Meilisearch: start a fresh instance, create test-only indexes, insert a few representative documents, and delete the data directory after the run.

The next step is to wrap that service setup in Docker Compose or Testcontainers once the workspace dependency layout is reproducible in CI.
