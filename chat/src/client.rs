use crate::common::{ChatId, UserId};
use crate::proto;

use tokio_stream::StreamExt;

type InnerClient = proto::chat_client::ChatClient<tonic::transport::Channel>;

pub struct Client {
    pub token: String,
    // TODO: your code here.
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub banned: bool,
}

#[derive(Clone, Debug)]
pub struct Chat {
    pub id: ChatId,
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamMessagesResponseEntry {
    pub user_id: UserId,
    pub user_name: String,
    pub content: String,
}

#[derive(Debug)]
pub enum ConnectLoginError {
    Connect(tonic::transport::Error),
    Login(tonic::Status),
}

impl Client {
    /// Подключает к серверу. ```token == None``` обозначает отсутствие какого-либо залогина.
    pub async fn connect(
        token: Option<String>,
        dst: String,
    ) -> Result<Self, tonic::transport::Error> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Подключает к серверу, затем осущствляет логин с заданными юзернеймом и паролем.
    pub async fn connect_login(
        user_name: String,
        password: String,
        dst: String,
    ) -> Result<Self, ConnectLoginError> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn new(token: Option<String>, inner: InnerClient) -> Client {
        // TODO: your code here.
        unimplemented!()
    }

    /// Создаёт новые коды, необходимые для регистрации пользователя.
    ///
    /// # Arguments
    ///
    /// * num_codes - Число новых кодов
    pub async fn create_join_codes(
        &mut self,
        num_codes: u32,
    ) -> Result<Vec<String>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Возвращает список всех неиспользованных кодов.
    pub async fn list_join_codes(&mut self) -> Result<Vec<String>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Использует код для регистрации. Каждый код можно использовать только один раз.
    ///
    /// Метод должен заполнить поле ```token```.
    ///
    /// Требования к юзернеймам:
    /// * не должны содержать ничего, кроме латинских букв, цифр и нижних подчёркиваний
    ///
    /// Требования к паролям:
    /// * должны содержать хотя одну латинскую букву, цифру и символ пунктуации
    /// * должны иметь длину не меньше 10 символов
    pub async fn join(
        &mut self,
        join_code: String,
        user_name: String,
        password: String,
    ) -> Result<UserId, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Логин с заданными юзернеймом и паролем.
    ///
    /// Метод должен заполнить поле ```token```.
    pub async fn login(
        &mut self,
        user_name: String,
        password: String,
    ) -> Result<(), tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Возвращает список всех пользователей.
    pub async fn list_users(&mut self) -> Result<Vec<User>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Возвращает информацию о пользователе.
    pub async fn get_user(&mut self, user_id: UserId) -> Result<User, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Отправляет голос за бан пользователя. Если голосующий является администратором, то
    /// пользователь сразу же банится. В противном случае пользователь банится при превышении
    /// половины числа всех зарегистрированных пользователей числом проголосовавщих за бан.
    pub async fn ban_user(&mut self, user_id: UserId) -> Result<bool, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Отзывает голос за бан пользователя. Если голосующий является администратором, то
    /// пользователь сразу же разбанивается. В противном случае, если пользователя ещё не забанили,
    /// голос удаляется из множества голосов, иначе не происходит ничего – отозвать свершившийся бан
    /// может только администратор.
    pub async fn unban_user(&mut self, user_id: UserId) -> Result<(), tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Создаёт комнату с чатом.
    pub async fn create_chat_room(&mut self, name: String) -> Result<ChatId, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Возвращает список всех комнат.
    pub async fn list_chat_rooms(&mut self) -> Result<Vec<Chat>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Возвращает информацию о комнате.
    pub async fn get_chat_room(&mut self, chat_id: ChatId) -> Result<Chat, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Посылает сообщение в комнату.
    pub async fn send_message(
        &mut self,
        chat_id: ChatId,
        content: String,
    ) -> Result<(), tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Возвращает поток с сообщениями, которые пришли после ответа на этот запрос.
    pub async fn stream_messages(
        &mut self,
        chat_id: ChatId,
    ) -> Result<
        impl futures::Stream<Item = Result<StreamMessagesResponseEntry, tonic::Status>>,
        tonic::Status,
    > {
        // TODO: your code here.
        unimplemented!()
    }
}
