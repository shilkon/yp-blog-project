# yp-blog-project

## blog-server

Крейт реализует веб-сервер, поддерживающий HTTP API и gRPC API. Основные настройки находятся в .env
Пример .env файла:
```
HOST=127.0.0.1
HTTP_PORT=8080
GRPC_PORT=50051
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/blog-db
JWT_SECRET=dev_super_secret_change_me_please
RUST_LOG=actix_web=info,blog_server=debug,warn
```
Крейт использует БД Postres, которая поднимается в Docker контейнере с помощью команды `docker compose up -d` (в ./crates/blog-server).
Сборка и запуск сервера осуществляется командой:
`cargo run --bin blog_server`

## blog-client
Крейт реализует библиотеку для взаимодействия с веб-сервером. По-умолчанию используется HTTP API, но можно включить gRPC API с помощью `features = ["grpc"]`. Для gRPC API используется та же Protobuf схема, что и у веб-сервера.

## blog-cli
Крейт реализует cli-утилиту для взаимодействия с веб-сервером посредством использования библиотеки `blog-client`. Токен для авторизации на сервере автоматически сохраняется в файле `.blog_token` при запросах на регистрацию и логин, а также автоматически подгружается из файла по необходимости.

Примеры команд сборки и запуска:
- `cargo run --bin blog_cli -- register --username user --email user@ya.ru --password 1234`
- `cargo run --bin --grpc blog_cli -- login --username user --password 1234`
- `cargo run --bin blog_cli -- list`
- `cargo run --bin --grpc blog_cli -- create --title title --content content`
- `cargo run --bin blog_cli -- delete --id 740878b9-d314-4fd5-8a28-a213d72d9efd`

## blog-wasm
Крейт реализует WASM-фронтенд с помощью экспортирования в JS функциональности по взаимодействию с сервером посредством использования библиотеки `blog-client`. Для сборки необходимо предварительно установить необходимые инструменты:
```
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack 
```
Сборка проекта осуществляется командой: `wasm-pack build --target web` (в ./crates/blog-wasm)

Для запуска локального сервера можно использовать команду `python -m http.server 8000`

После чего фронтенд доступен в браузере по адресу `http://localhost:8000/`