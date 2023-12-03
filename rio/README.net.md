В этой задаче вам предлагается добавить поддержку UDP-сокетов в асинхронный рантайм.

> Данная папка соответствует сразу двум задачам: `rio-multi` и `rio-net`.
> К `rio-net` относится код под feature-флагом "net". Код под флагом "rt-multi-thread"
> не используется сборкой. Сборка и тестирование задачи `rio-net` устроены так: `make net` прогоняет тесты, `make submit_net` отправляет решение.
> Чтобы подружить IDE с кодом под feature-флагом "net", добавьте его в
> default features в Cargo.toml.

## 1. mio

Всё бремя взаимодействия с ОС на себя берёт библиотека mio. Она предоставляет кросс-платформенное API для неблокирующей работы с I/O. На Linux mio - тонкая обёртка над системным вызовом `epoll`.

В mio даже есть готовый тип `UdpSocket`, над которым можно проворачивать неблокирующие операции ввода-вывода. Пример базового использования `UdpSocket` (взят из документации):

```rust
// Программа Echo:
// SENDER - посылает сообщение.
// ECHOER - распечатывает входящие сообщения.

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::time::Duration;

// Токен - это идентификатор сокета, с которым связывается событие
// доступности на чтение/запись.
const SENDER: Token = Token(0);
const ECHOER: Token = Token(1);

// Создаём сокеты на разных портах.
let mut sender_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;
let mut echoer_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;

// Сахар: connect просто избавляет нас от необходиомсти указывать
// каждый раз адрес, куда надо отправить сообщение.
sender_socket.connect(echoer_socket.local_addr()?)?;

// Создаём новый Poll - это объект, через который доставляются события.
let mut poll = Poll::new()?;

// Регистрируем сокеты, чтобы Poll возвращал связанные с ними события.
poll.registry().register(&mut sender_socket, SENDER, Interest::WRITABLE)?;
poll.registry().register(&mut echoer_socket, ECHOER, Interest::READABLE)?;

let msg_to_send = [9; 9];
let mut buffer = [0; 9];

let mut events = Events::with_capacity(128);
loop {
	// .poll() заблокируется до тех пор, пока не появятся новые события,
	// или пока не пройдёт 100 миллисекунд.
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    for event in events.iter() {
        match event.token() {
            // Можно писать данные в SENDER.
			// Гарантируется, что после регистрации сокета event
			// доступности на запись обязательно вернётся из poll.
            SENDER => {
                let bytes_sent = sender_socket.send(&msg_to_send)?;
                assert_eq!(bytes_sent, 9);
                println!("sent {:?} -> {:?} bytes", msg_to_send, bytes_sent);
            },
            // Появились доступные для чтения данные.
            ECHOER => {
                let num_recv = echoer_socket.recv(&mut buffer)?;
                println!("echo {:?} -> {:?}", buffer, num_recv);
                buffer = [0; 9];
            }
            _ => unreachable!()
        }
    }
}
```

Если на mio::UdpSocket позвать операцию, которая может привести к блокировке (например, вызвать `.recv()`, когда данных ещё нет), то вернётся `io::Error` с `ErrorKind::WouldBlock`.

`mio::Poll` работает в т.н. edge-triggered режиме. Это значит, что, когда сокет станет готов на чтение, событие об этом придёт ровно один раз. Пока сокет содержит непрочитанные данные, все последующие вызовы `Poll::poll` не будут возвращать это событие снова. Сперва вы должны вычитать все данные из сокета, получить `ErrorKind::WouldBlock`, и только после этого `Poll::poll` оповестит вас о новых данных. Аналогично работает оповещение о событиях записи (у `mio::Event` есть методы `is_readable` и `is_writable` - заметьте, что могут быть выставлены оба флага, если сокет стал одновременно готов на чтение и на запись).

У `mio::UdpSocket` много методов, но в рамках задачи вам предлагается написать асинхронные обёртки только для небольшого их числа:
* bind
* connect
* local_addr
* recv
* recv_from
* send
* send_to

## 2. Склейка mio с рантаймом

Весь связанный с сетью код находится в файлах `src/network/driver.rs` и `src/network/udp.rs`.

Поддержка UDP-сокетов в рантайме состоит из двух частей:
* Обёртка над `mio::UdpSocket`, которая умеет подписываться на события готовности к чтению/записи.
* Отдельный тред, который обрабатывает приходящие от `Poll` события - в частности, пробуждает фьючи, которые этих событий ожидают.

Рантайм запускает сетевую подсистему вызовом `NetworkDriver::start()`. Вызов создаёт `NetworkDriver` со всем необходимым состоянием и запускает цикл обработки событий в отдельном потоке. Этот вызов возвращает `NetworkHandle`, который впредь будет доступен из контекста рантайма как `RuntimeHandle::current().state().network_handle` (но только коду рантайма - это непубличное API). Когда рантайм будет уничтожен, позовётся деструктор `NetworkHandle`, который должен остановить связанный поток.

Жизненный цикл `UdpSocket` состоит из трёх фаз:
1. Создание сокета - в этот момент сокет должен подписаться на `read` и `write` события нижележащего `mio::UdpSocket` (т.к. мы заранее не знаем, что будет делать пользователь - только читать, только писать или всё вместе). За это отвечает метод `Registry::register`.
2. Обработка асинхронной операции чтения (записи) - надо проверить, приходил ли сигнал от `Poll`, что сокет готов к чтению (записи). Если приходил, можно попробовать операцию чтения (записи); если операция возвращает `ErrorKind::WouldBlock` - надо запомнить, что до получения нового сигнала от `Poll` этот сокет не надо читать (писать). Важно убедиться, что у вас нет гонки между операциями над сокетом и нотификациями от `Poll` (см. 4.1).
3. Уничтожение сокета - надо отписать `Poll` от событий этого сокета, позвав `Registry::deregister`.

## 3. Требования к решению

* Сокет должен адекватно переживать перемещение между рантаймами. Для этого вам придётся сохранить внутри него `RuntimeHandle`, ссылающийся на тот рантайм, где сокет изначально зарегистрирован.
* По контракту `Future`, в момент готовности фьючи рантайм должен позвать waker, переданный в **последний** вызов `Future::poll`.
* Одному и тому же сокету могут соответствовать несколько фьюч - а значит, несколько waker'ов:
    - Когда сокет становится доступен на чтение, разбудите все waker'ы, подписанные на чтение;
    - Когда сокет становится доступен на запись, разбудите все waker'ы, подписанные на запись.
* Не делайте лишних системных вызовов в методе `poll`: если вам не приходило событие, что из сокета можно читать (писать), то не стоит читать (писать).

## 4. Советы по реализации

### 4.1. Как избежать гонки между `Future::poll` и `mio::Poll::poll`

Наивный алгоритм поллинга фьючи может выглядеть так:

1. Проверить, приходила ли информация про данный сокет из mio, что он доступен на чтение (запись).
2. Если да - сделать попытку чтения (записи).
3. Если попытка чтения (записи) завершилась с `ErrorKind::WouldBlock`, то пометить сокет как недоступный на чтение (запись).

В этом алгоритме есть гонка: между шагами 2 и 3 может прийти новое событие из mio, что сокет вновь доступен на чтение (запись).
Тогда поток драйвера пометит сокет как доступный на чтение (запись), и сразу после этого шаг 3 алгоритма сбросит эту пометку.
Таким образом, мы получим неактуальную информацию о состоянии сокета и почти точно deadlock.

Чтобы избежать гонки, воспользуйтесь следующим алгоритмом.

1. Для каждого сокета будем поддерживать 64-битный дескриптор. Младшие 2 бита отвечают за готовность сокета на чтение и запись.
Старшие 62 бита являются "эпохой". При создании сокета эпоха равна 0.
2. Когда mio возвращает новое событие о данном сокете, мы записываем новый статус готовности сокета на чтение/запись в дескриптор,
а также увеличиваем эпоху на единицу.
3. В реализации `.poll` фьючи мы:
    1. Запоминаем текущее значение дескриптора сокета.
    2. Если в дескрипторе установлен бит доступности на чтение (запись), то делаем соответствуюий вызов на сокете.
    3. Если вызов завершился с `ErrorKind::WouldBlock`, то мы берём значение дескриптора из 3.1, сбрасываем соответствующий бит
    и делаем compare exchange с текущим значением дескриптора.

Таким образом, если сокет вновь стал доступен на чтение/запись между шагами 3.2 и 3.3, мы не сбросим эту информацию из дескриптора.

### 4.2. Другие советы

* Чтобы генерировать токены для `mio`, воспользуйтесь `AtomicUsize`. Бывают другие схемы, но эта самая простая и кросс-платформенная.
* Внутри `UdpSocket` стоит завести отдельный метод `async_io`, принимающий `impl FnMut() -> io::Result<T>`. Этот метод подписывается на готовность сокета и пытается провести указанную операцию до тех пор, пока она не вернёт что-то отличное от `ErrorKind::WouldBlock`.
* Можете пользоваться логированием (макросы `log::{trace, debug, info, warn, error}`). Запустить конкретный тест и посмотреть логи можно следующей командой:
  - `RUST_LOG=debug cargo test test_name -- --nocapture`
* Для продвиутых есть `tracing`, но его настраивайте сами :)