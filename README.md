# somes

The source code of the somes repository.

## The base structure is as follows:
- **`somes-api`**

    Is the backend of somes. It depends mainly on the [`axum`] web framework.

- **`sommes-common-lib`**

    A rust library containing common structures and code that should be used interoperability between the backend and the outdated rust frontend. (e.g. `LoginInfo` is probably the same for both ends.)

- **`somes-frontend`**
    The somes frontend, written with Svelte.

[`axum`]: https://github.com/tokio-rs/axum

## Building and running of the development suite

- **[`dataservice`]** different repo - somes software for scraping and storing gathered data (and do some dl with it) 

[`dataservice`]: https://github.com/som-es/dataservice
