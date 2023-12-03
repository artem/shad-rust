В данной задаче вам предложено написать простой web crawler.

## Описание

Web crawler - это приложение, которое обходит заданный web-сайт, посещая все его страницы
(те, ссылки на которые он смог достать). Алгоритм работы crawler'а следующий:

1. Достать очередной url из очереди;
2. Если этот url ещё не посещён - посетить его;
3. Достать из тела ответа новые url'ы и добавить их в очередь.

## Реализация

* Метод `Crawler::run` не async, но подразумевается, что он запускается в рамках
асинхронного рантайма. Он возвращает принимающий конец канала, куда будут отгружаться
структуры `Page` по мере обхода сайта.

* Параметр конфига `concurrent_requests` отвечает за то, сколько максимум одновременно
идущих запросов допускается совершать. В пределах этого ограничения вам следует
максимально распараллелить обход сайта. Т.е. после каждого запроса следует проверить размер очереди непосещённых url, и конкурентно запустить запрос первых `n` url,
где `n = concurrent_requests - число_уже_бегущих_запросов`.

* Для выполнения http-запросов используйте библиотеку reqwest, работающую поверх tokio.
Базовое использование следующее:

```rust
reqwest::get(url).await.unwrap().text().await.unwrap();
```

* Чтобы найти все ссылки в теле странцы, используйте библиотеку linkify. Базовое использование:

```rust
let mut finder = LinkFinder::new();
finder.kinds(&[LinkKind::Url]);
let links = finder.links(body).map(|l| l.as_str().to_string()).collect();
```

* У вас может быть соблазн создать сразу много worker'ов с помощью
`tokio::spawn` и общаться с ними с помощью каналов. Это будет работать, но проще
иметь одного воркера с [FuturesUnordered](https://docs.rs/futures/latest/futures/prelude/stream/struct.FuturesUnordered.html).
Пример использования:

```rust
let mut futures = FuturesUnordered::new();
futures.push(fetch_url(site));
while let Some(result) = futures.next().await {
	// ...
}
```

* Для отладки можете на своё усмотрене использовать `logging` либо `tracing`. Запустить конкретный тест и посмотреть логи можно следующими командами:
  - Для `logging`: `RUST_LOG=debug cargo test test_name -- --nocapture`
  - Для `tracing`: `RUST_LOG_SPAN_EVENTS=full cargo test test_name -- --nocapture`
