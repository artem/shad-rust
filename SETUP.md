## Настройка окружения

Инструкция приведена для Ubuntu 22.04. Тем не менее, настройка под Mac OS и Windows будет мало
отличаться (под Windows вам, вероятно, понадобится WSL или Cygwin). В теории курс
кросс-платформенный - все задачи можно делать на Linux, Mac и Windows - однако
мы тестировали только под Ubuntu 22.04, так что на других платформах следует ожидать непредвиденных проблем.
В случае их возникновения, пишите в чат курса - чем сможем, поможем :)

### Регистрация в системе

1. Зарегистрируйтесь в [тестовой системе](https://rust.manytask.org). Секретный код: `safe-and-sound`.
1. Сгенерируйте ssh ключ, если у вас его еще нет.

	```
	ssh-keygen -N "" -f ~/.ssh/id_rsa
	```

1. Скопируйте содержимое файла id_rsa.pub (`cat ~/.ssh/id_rsa.pub`) в https://gitlab.manytask.org/-/profile/keys
1. Проверьте, что ssh ключ работает. Выполните команду `ssh git@gitlab.manytask.org`. Вы должны увидеть такое приветствие:

	```
	$ ssh git@gitlab.manytask.org
	PTY allocation request failed on channel 0
	Welcome to GitLab, Fedor Korotkiy!
	Connection to gitlab.manytask.org closed.
	```

### Настройка репозитория

1. Склонируйте репозиторий с задачами.

	```
	git clone https://gitlab.manytask.org/rust-ysda/public-2022-fall.git shad-rust
	```

   Команда `git clone` создаст директорию `shad-rust` и запишет туда все файлы из этого репозитория.
1. Каждую неделю после занятий вам надо будет обновлять репозиторий, чтобы у вас появились условия
   новых задач:

	```
	git pull --rebase
	```

1. Для отправки решения на сервер, необходимо, чтобы у вас были заданы имя и email в git:

	```
	git config --global user.name "Vasya Pupkin"
	git config --global user.email vasya@pupkin.ru
	```

1. Откройте страницу своего репозитория в браузере. Перейдите по ссылке MY REPO на [странице с задачами](https://rust.manytask.org).
1. Добавьте в git свой приватный репозиторий. Для этого запустите из директории репозитория команду:

	```
	git remote add student $ADDRESS
	```

   `$ADDRESS` нужно скопировать со страницы репозитория. Синяя кнопка Clone -> Clone with SSH.

### Настройка IDE

Официально поддерживаемой средой разработки является VS Code, однако вы вольны использовать любые редакторы/IDE, которые вам нравятся.

1. Установите Rust, следуя [официальному руководству](https://www.rust-lang.org/tools/install).
1. Установите форматтер, линтер и утилиту бенчмаркинга:

    ```
    rustup component add rustfmt
    rustup component add clippy
    cargo install cargo-criterion
    ```

1. Установите [VS Code](https://code.visualstudio.com).
1. Установите расширения для VS Code:
   * [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
   * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

1. В VS Code нажмите "File" -> "Open Folder", откройте директорию, куда вы склонировали репозиторий курса.

### Отправка решения

Чтобы проверить работоспособность окружения, решите первую тестовую задачу:

1. Откройте `add/src/lib.rs`. Убедитесь, что у вас работают базовые вещи: подсветка ошибок компиляции, автокомплит, go to definition.
1. Откройте `add/tests/tests.rs`. Нажмите `Debug` над `fn test_add()`, убедитесь, что тест падает и вы оказываетесь в дебагере в момент его падения.
1. Напишите правильную реализацию функции `add` в `add/src/lib.rs`.
1. Находясь в директории `add`, запустите локальные тесты командой `make`. Убедитесь, что они проходят.
1. Закомитьте изменения:

    ```
	git add .
	git commit -m 'Solve task: add'  # сообщение может быть произвольным
    ```

1. Отправьте своё решение на сервер командой `make submit`. Ваш сабмит должен появиться по ссылке "SUBMITS" на [rust.manytask.org](https://rust.manytask.org).
После успешного прохождения тестов вам должно начислиться 0 баллов в
[таблице с баллами](https://docs.google.com/spreadsheets/d/1X18wt5mls0qbZiiZIXqrbB1ejQHAwpDLZNKpRpTRao0).

Если на каком-то этапе у вас возникли проблемы - пишите в чат курса.
