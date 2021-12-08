use std::collections::HashSet;
use std::sync::atomic;
use std::time::Duration;

use futures::join;

use pretty_assertions::{assert_eq, assert_ne};

use tokio_stream::StreamExt;

const ADMIN_TOKEN: &str = "8931a63a84126797b7fc8344cb0e2f5f";

async fn serve() -> String {
    static PORT: atomic::AtomicU16 = atomic::AtomicU16::new(8000);

    let port = PORT.fetch_add(1, atomic::Ordering::SeqCst);
    let addr_str = format!("127.0.0.1:{}", port);
    let addr = addr_str.parse().expect("failed to parse SERVER_ADDR");

    tokio::spawn(async move {
        chat::serve(ADMIN_TOKEN.to_string(), addr)
            .await
            .expect("failed to start the server");
    });

    // Wait for 200*50ms = 10s.
    for _ in 0..200 {
        if tokio::net::TcpStream::connect(&addr_str).await.is_ok() {
            return format!("http://{}", addr_str);
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    unreachable!("server startup timed out");
}

async fn serve_and_connect_admin() -> (String, chat::Client) {
    let server_addr = serve().await;
    let client = chat::Client::connect(Some(ADMIN_TOKEN.to_string()), server_addr.clone())
        .await
        .expect("failed to connect to server");
    (server_addr, client)
}

#[tokio::test]
async fn test_connect() {
    serve_and_connect_admin().await;
}

#[tokio::test]
async fn test_create_list_join_codes() {
    let (_, mut client) = serve_and_connect_admin().await;

    let num_join_codes = 4;

    let join_codes_vec = client
        .create_join_codes(num_join_codes)
        .await
        .expect("failed to create join codes");
    assert_eq!(join_codes_vec.len(), num_join_codes as usize);

    let join_codes: HashSet<_> = join_codes_vec.into_iter().into_iter().collect();

    let listed_join_codes: HashSet<_> = client
        .list_join_codes()
        .await
        .expect("failed to list join codes")
        .into_iter()
        .collect();

    assert_eq!(join_codes, listed_join_codes);
    assert_eq!(join_codes.len(), num_join_codes as usize);
}

#[tokio::test]
async fn test_join() {
    let (server_addr, mut client) = serve_and_connect_admin().await;

    let mut join_codes = client
        .create_join_codes(1)
        .await
        .expect("failed to create join codes");
    assert_eq!(join_codes.len(), 1);

    let mut user_client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect");

    user_client
        .join(
            join_codes.pop().unwrap(),
            "alice".to_string(),
            "2D154cE3bc!".to_string(),
        )
        .await
        .expect("failed to join");

    assert!(user_client.token.len() > 0);
}

macro_rules! cant_join_with_weak_password_tests {
    ($($name:ident: $password:expr,)*) => { $(
        #[tokio::test]
        async fn $name() {
            let (server_addr, mut client) = serve_and_connect_admin().await;
            join(
                &server_addr, &mut client,
                "alice", $password,
            ).await
                .expect_err("managed to join with a weak password");
        }
    )* }
}

cant_join_with_weak_password_tests! {
    test_cant_join_with_short_password: "We4kPass!",
    test_cant_join_with_password_without_digits: "WeakPass!!",
    test_cant_join_with_password_without_punct: "WeakPassss",
    test_cant_join_with_password_without_alphas: "12345678!!",
}

#[tokio::test]
async fn test_cant_join_with_duplicate_user_name() {
    let (server_addr, mut client) = serve_and_connect_admin().await;

    join(&server_addr, &mut client, "alice", "tops3cret!")
        .await
        .expect("failed to join");

    join(&server_addr, &mut client, "alice", "t0psecret!")
        .await
        .expect_err("managed to join with a duplicate user name");
}

macro_rules! cant_join_with_invalid_user_name_tests {
    ($($name:ident: $user_name:expr,)*) => { $(
        #[tokio::test]
        async fn $name() {
            let (server_addr, mut client) = serve_and_connect_admin().await;
            join(
                &server_addr, &mut client,
                $user_name, "t0psecret!",
            ).await
                .expect_err("managed to join with an invalid user name");
        }
    )* }
}

cant_join_with_invalid_user_name_tests! {
    test_cant_join_with_whitespace_user_name: "al ice",
    test_cant_join_with_punct_user_name_1: "al!ice",
    test_cant_join_with_punct_user_name_2: "alice##",
}

#[tokio::test]
async fn test_login() {
    let (server_addr, mut client) = serve_and_connect_admin().await;

    join(&server_addr, &mut client, "alice", "t0psecret!")
        .await
        .expect("failed to join");

    let mut user_client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect");

    user_client
        .login("alice".to_string(), "t0psecret!".to_string())
        .await
        .expect("failed to login");

    assert!(user_client.token.len() > 0);
}

#[tokio::test]
async fn test_login_invalidates_old_token() {
    let (server_addr, mut client) = serve_and_connect_admin().await;

    join(&server_addr, &mut client, "alice", "t0psecret!")
        .await
        .expect("failed to join");

    let mut user_client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect");

    user_client
        .login("alice".to_string(), "t0psecret!".to_string())
        .await
        .expect("failed to login");

    assert!(user_client.token.len() > 0);

    let old_token = user_client.token.clone();

    user_client
        .login("alice".to_string(), "t0psecret!".to_string())
        .await
        .expect("failed to login");

    assert_ne!(old_token, user_client.token);

    user_client.token = old_token;
    user_client
        .list_users()
        .await
        .expect_err("managed to get the list of users with old token");
}

#[tokio::test]
async fn test_cant_login_with_wrong_password() {
    let (server_addr, mut client) = serve_and_connect_admin().await;

    join(&server_addr, &mut client, "alice", "t0psecret!")
        .await
        .expect("failed to join");

    let mut eve_client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect eve");

    eve_client
        .login("alice".to_string(), "f4kepassword#".to_string())
        .await
        .expect_err("managed to login with wrong password");
}

#[tokio::test]
async fn test_cant_login_with_nonexistent_user_name() {
    let server_addr = serve().await;
    let mut user_client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect");

    user_client
        .login("nonexistent_user".to_string(), "f4kepassword#".to_string())
        .await
        .expect_err("managed to login as a nonexistent user");
}

#[tokio::test]
async fn test_cant_login_if_banned() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let eve_uid = join(&server_addr, &mut admin_client, "eve", "t0psecret!")
        .await
        .expect("failed to join");

    admin_client
        .ban_user(eve_uid)
        .await
        .expect("failed to ban eve");

    let mut user_client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect");

    user_client
        .login("eve".to_string(), "t0psecret!".to_string())
        .await
        .expect_err("managed to log in despite being banned");
}

#[tokio::test]
async fn test_list_users() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let user_names = &["alice", "bob", "eve", "jack"];
    let mut eve_uid = None;

    for &user_name in user_names {
        let uid = join(&server_addr, &mut admin_client, user_name, "t0psecret!")
            .await
            .expect("failed to join");

        if user_name == "eve" {
            eve_uid = Some(uid);
        }
    }

    admin_client
        .ban_user(eve_uid.unwrap())
        .await
        .expect("failed to ban eve");

    let mut client = chat::Client::connect(None, server_addr.clone())
        .await
        .expect("failed to connect");

    client
        .login("alice".to_string(), "t0psecret!".to_string())
        .await
        .expect("failed to login");

    let listed_users = client.list_users().await.expect("failed to list users");

    let mut listed_user_names: Vec<_> = listed_users.iter().map(|user| &user.name).collect();
    listed_user_names.sort();
    assert_eq!(&listed_user_names, &user_names);

    let banned_user_names: Vec<_> = listed_users
        .iter()
        .filter(|user| user.banned)
        .map(|user| &user.name)
        .collect();
    assert_eq!(banned_user_names.len(), 1);
    assert_eq!(banned_user_names[0], "eve");
}

#[tokio::test]
async fn test_get_user() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let users = &[("alice", false), ("eve", true)];
    let mut uids = vec![];

    for &(user_name, banned) in users {
        let uid = join(&server_addr, &mut admin_client, user_name, "t0psecret!")
            .await
            .expect("failed to join");

        if banned {
            admin_client.ban_user(uid).await.expect("failed to ban");
        }

        uids.push(uid);
    }

    for (&(user_name, banned), &uid) in users.iter().zip(&uids) {
        let user = admin_client
            .get_user(uid)
            .await
            .expect("failed to get user");
        assert_eq!(user.name, user_name);
        assert_eq!(user.banned, banned);
    }
}

#[tokio::test]
async fn test_vox_populi_ban() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let num_users = 6;
    let user_names: Vec<_> = (0..num_users).map(|i| format!("user{}", i)).collect();
    let mut banned_uid = None;

    for (i, user_name) in user_names.iter().enumerate() {
        let uid = join(&server_addr, &mut admin_client, user_name, "t0psecret!")
            .await
            .expect("failed to join");

        if i == 0 {
            banned_uid = Some(uid);
        }
    }

    let banned_uid = banned_uid.unwrap();

    for (i, user_name) in user_names
        .iter()
        .skip(1)
        .take(num_users / 2 + 1)
        .enumerate()
    {
        let mut client = chat::Client::connect(None, server_addr.clone())
            .await
            .expect("failed to connect");

        client
            .login(user_name.clone(), "t0psecret!".to_string())
            .await
            .expect("failed to login");

        client
            .ban_user(banned_uid)
            .await
            .expect("failed to ban user");

        let banned_user = admin_client
            .get_user(banned_uid)
            .await
            .expect("failed to get user");
        assert_eq!(banned_user.banned, i + 1 > num_users / 2);
    }
}

#[tokio::test]
async fn test_vox_populi_cant_unban() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let num_users = 6;
    let user_names: Vec<_> = (0..num_users).map(|i| format!("user{}", i)).collect();
    let mut banned_uid = None;

    for (i, user_name) in user_names.iter().enumerate() {
        let uid = join(&server_addr, &mut admin_client, user_name, "t0psecret!")
            .await
            .expect("failed to join");

        if i == 0 {
            banned_uid = Some(uid);
        }
    }

    let banned_uid = banned_uid.unwrap();
    let mut user_clients = vec![];

    for user_name in user_names.iter().skip(1).take(num_users / 2 + 1) {
        let mut client = chat::Client::connect(None, server_addr.clone())
            .await
            .expect("failed to connect");

        client
            .login(user_name.clone(), "t0psecret!".to_string())
            .await
            .expect("failed to login");

        client
            .ban_user(banned_uid)
            .await
            .expect("failed to ban user");

        user_clients.push(client);
    }

    for user_client in &mut user_clients {
        user_client
            .unban_user(banned_uid)
            .await
            .expect("failed to unban user");
    }

    let banned_user = admin_client
        .get_user(banned_uid)
        .await
        .expect("failed to get user");
    assert!(banned_user.banned);
}

#[tokio::test]
async fn test_ban_vote_withdrawing() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let num_users = 6;
    let user_names: Vec<_> = (0..num_users).map(|i| format!("user{}", i)).collect();
    let mut banned_uid = None;

    for (i, user_name) in user_names.iter().enumerate() {
        let uid = join(&server_addr, &mut admin_client, user_name, "t0psecret!")
            .await
            .expect("failed to join");

        if i == 0 {
            banned_uid = Some(uid);
        }
    }

    let banned_uid = banned_uid.unwrap();

    let mut user_clients: Vec<_> = futures::future::join_all(user_names.iter().map(|user_name| {
        let server_addr = server_addr.clone();
        async move {
            let mut client = chat::Client::connect(None, server_addr.clone())
                .await
                .expect("failed to connect");

            client
                .login(user_name.clone(), "t0psecret!".to_string())
                .await
                .expect("failed to login");

            client
        }
    }))
    .await;

    for client in user_clients.iter_mut().take(num_users / 2) {
        client
            .ban_user(banned_uid)
            .await
            .expect("failed to ban user");
    }

    user_clients[num_users / 2 - 1]
        .unban_user(banned_uid)
        .await
        .expect("failed to unban user");

    user_clients[num_users / 2]
        .ban_user(banned_uid)
        .await
        .expect("failed to ban user");

    let unbanned_user = admin_client
        .get_user(banned_uid)
        .await
        .expect("failed to get user");
    assert!(!unbanned_user.banned);
}

#[tokio::test]
async fn test_admin_unban() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let unbanned_uid = join(&server_addr, &mut admin_client, "alice", "t0psecret!")
        .await
        .expect("failed to join");

    admin_client
        .ban_user(unbanned_uid)
        .await
        .expect("failed to ban");
    admin_client
        .unban_user(unbanned_uid)
        .await
        .expect("failed to unban");

    let unbanned_user = admin_client
        .get_user(unbanned_uid)
        .await
        .expect("failed to get user");
    assert!(!unbanned_user.banned);
}

#[tokio::test]
async fn test_communicate_in_chat_rooms() {
    let (server_addr, mut admin_client) = serve_and_connect_admin().await;

    let alice_uid = join(&server_addr, &mut admin_client, "alice", "t0psecret!")
        .await
        .expect("failed to join");

    let mut alice_client = chat::Client::connect_login(
        "alice".to_string(),
        "t0psecret!".to_string(),
        server_addr.clone(),
    )
    .await
    .expect("failed to login");

    let bob_uid = join(&server_addr, &mut admin_client, "bob", "t0psecret!")
        .await
        .expect("failed to join");

    let mut bob_client = chat::Client::connect_login(
        "bob".to_string(),
        "t0psecret!".to_string(),
        server_addr.clone(),
    )
    .await
    .expect("failed to login");

    let general_cid = alice_client
        .create_chat_room("general".to_string())
        .await
        .expect("failed to create chat room");

    let memes_cid = bob_client
        .create_chat_room("memes".to_string())
        .await
        .expect("failed to create chat room");

    let alice_general_msgs_stream = stream_messages(&mut alice_client, general_cid, 2).await;
    let alice_memes_msgs_stream = stream_messages(&mut alice_client, memes_cid, 2).await;

    let bob_general_msgs_stream = stream_messages(&mut bob_client, general_cid, 2).await;
    let bob_memes_msgs_stream = stream_messages(&mut bob_client, memes_cid, 2).await;

    let general_messages = &[
        (general_cid, alice_uid, "hello, is this working?"),
        (general_cid, bob_uid, "hey, yes it does!"),
    ];

    let memes_messages = &[
        (memes_cid, bob_uid, "hello there"),
        (memes_cid, alice_uid, "general kenobi"),
    ];

    for &(cid, uid, msg) in general_messages.iter().chain(memes_messages) {
        let client = if uid == alice_uid {
            &mut alice_client
        } else {
            &mut bob_client
        };
        client
            .send_message(cid, msg.to_string())
            .await
            .expect("failed to send message");
    }

    let (alice_general_msgs, alice_memes_msgs, bob_general_msgs, bob_memes_msgs) = join!(
        alice_general_msgs_stream,
        alice_memes_msgs_stream,
        bob_general_msgs_stream,
        bob_memes_msgs_stream,
    );

    for (alice_messages, bob_messages, expected_messages) in [
        (&alice_general_msgs, &bob_general_msgs, general_messages),
        (&alice_memes_msgs, &bob_memes_msgs, memes_messages),
    ] {
        assert_eq!(alice_messages, bob_messages);

        let messages: Vec<_> = alice_messages
            .iter()
            .cloned()
            .map(|msg| (msg.user_id, msg.user_name, msg.content))
            .collect();

        let expected_messages: Vec<_> = expected_messages
            .iter()
            .map(|&(_, uid, content)| {
                (
                    uid,
                    if uid == alice_uid {
                        "alice".to_string()
                    } else {
                        "bob".to_string()
                    },
                    content.to_string(),
                )
            })
            .collect();

        assert_eq!(messages, expected_messages);
    }
}

async fn join(
    server_addr: &str,
    admin_client: &mut chat::Client,
    user: &str,
    password: &str,
) -> Result<chat::UserId, tonic::Status> {
    let mut join_codes = admin_client
        .create_join_codes(1)
        .await
        .expect("failed to create join codes");
    assert_eq!(join_codes.len(), 1);

    let mut user_client = chat::Client::connect(None, server_addr.to_string())
        .await
        .expect("failed to connect");

    user_client
        .join(
            join_codes.pop().unwrap(),
            user.to_string(),
            password.to_string(),
        )
        .await
}

async fn stream_messages(
    client: &mut chat::Client,
    cid: chat::ChatId,
    num_messages: usize,
) -> impl futures::Future<Output = Vec<chat::client::StreamMessagesResponseEntry>> {
    client
        .stream_messages(cid)
        .await
        .expect("failed to stream messages")
        .take(num_messages)
        .map(|msg| msg.expect("failed to receive message"))
        .collect()
}
