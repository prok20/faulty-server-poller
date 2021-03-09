# Faulty Server poller

[Problem definition](problem.md)

**Requirements:**
* Docker
* Rustup-configured toolchain
* psql (for database startup scripts)

**Startup:**
* `git clone https://github.com/prok20/faulty-server-poller.git && cd faulty-server-poller`
* `./scripts/init_db.sh`
* `export APP_ENVIRONMENT=development`
* `cargo test` or `cargo run`

**Configuration:**
* Environment variable `APP_ENVIRONMENT` must be set to either `development` or `production`
* .YAML files in `./configuration` folder
* Environment variables starting with `APP_` prefix and following same structure as YAML with `__` (double undercore) separators

**TODO** (что можно ещё доработать навскидку):
* Отрефакторить код в TokioBackgroundJobRunner, можно разделить на отдельно структуры/трейты Keeper (трансмиттер команд) и Runner (выполнение)
* Добавить ошибку Bad Request для get_run с невалидным id
* Перевести логирование на tracing и обмазать свои структуры и ошибки логами
* Поработать над graceful shutdown: например, сейчас треды TokioBackgroundJobRunner отваливаются некрасиво после выполнения тестов
* Завернуть в докер и воспользоваться production-окружением
