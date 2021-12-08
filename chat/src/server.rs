use std::collections::{HashMap, HashSet};
use std::iter;

use tokio::sync::{mpsc, Mutex, RwLock};

use crate::proto;

use crate::common::{ChatId, UserId, ADMIN_UID};

#[derive(Default)]
pub struct Service {
    // TODO: your code here.
}

/// Запускает сервер.
pub async fn serve(
    admin_token: String,
    addr: std::net::SocketAddr,
) -> Result<(), tonic::transport::Error> {
    // TODO: your code here.
}

#[tonic::async_trait]
impl proto::chat_server::Chat for Service {
    async fn create_join_codes(
        &self,
        request: tonic::Request<proto::CreateJoinCodesRequest>,
    ) -> Result<tonic::Response<proto::CreateJoinCodesResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn list_join_codes(
        &self,
        request: tonic::Request<proto::ListJoinCodesRequest>,
    ) -> Result<tonic::Response<proto::ListJoinCodesResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn join(
        &self,
        request: tonic::Request<proto::JoinRequest>,
    ) -> Result<tonic::Response<proto::JoinResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn login(
        &self,
        request: tonic::Request<proto::LoginRequest>,
    ) -> Result<tonic::Response<proto::LoginResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn list_users(
        &self,
        request: tonic::Request<proto::ListUsersRequest>,
    ) -> Result<tonic::Response<proto::ListUsersResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn get_user(
        &self,
        request: tonic::Request<proto::GetUserRequest>,
    ) -> Result<tonic::Response<proto::GetUserResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn ban_user(
        &self,
        request: tonic::Request<proto::BanUserRequest>,
    ) -> Result<tonic::Response<proto::BanUserResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn unban_user(
        &self,
        request: tonic::Request<proto::UnbanUserRequest>,
    ) -> Result<tonic::Response<proto::UnbanUserResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn create_chat_room(
        &self,
        request: tonic::Request<proto::CreateChatRoomRequest>,
    ) -> Result<tonic::Response<proto::CreateChatRoomResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn list_chat_rooms(
        &self,
        request: tonic::Request<proto::ListChatRoomsRequest>,
    ) -> Result<tonic::Response<proto::ListChatRoomsResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn get_chat_room(
        &self,
        request: tonic::Request<proto::GetChatRoomRequest>,
    ) -> Result<tonic::Response<proto::GetChatRoomResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    async fn send_message(
        &self,
        request: tonic::Request<proto::SendMessageRequest>,
    ) -> Result<tonic::Response<proto::SendMessageResponse>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }

    type StreamMessagesStream = tokio_stream::wrappers::UnboundedReceiverStream<
        Result<proto::StreamMessagesResponseEntry, tonic::Status>,
    >;

    async fn stream_messages(
        &self,
        request: tonic::Request<proto::StreamMessagesRequest>,
    ) -> Result<tonic::Response<Self::StreamMessagesStream>, tonic::Status> {
        // TODO: your code here.
        unimplemented!()
    }
}

