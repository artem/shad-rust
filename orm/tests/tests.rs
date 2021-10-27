use orm::{data::DataType, Connection, Object, ObjectId, ObjectState, Result, Tx};

use rusqlite::params;
use tempfile::NamedTempFile;

////////////////////////////////////////////////////////////////////////////////

#[derive(Object, PartialEq, Clone, Debug)]
struct User {
    pub name: String,
    pub picture: Vec<u8>,
    pub visits: i64,
    balance: f64,
    is_admin: bool,
}

////////////////////////////////////////////////////////////////////////////////

fn assert_not_found<'a>(
    res: Result<Tx<'a, User>>,
    expected_object_id: ObjectId,
    expected_type_name: &str,
) {
    match res {
        Err(orm::Error::NotFound(err)) => {
            assert_eq!(err.object_id, expected_object_id);
            assert_eq!(err.type_name, expected_type_name);
        }
        Ok(_) => panic!("expected NotFound error, got OK"),
        Err(err) => panic!("expected NotFound error, got {}", err),
    }
}

fn fmt_res<T>(res: &Result<T>) -> String {
    match res {
        Ok(_) => "Ok".to_string(),
        Err(err) => format!("{:?}", err),
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_create() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let user = User {
        name: "John".into(),
        picture: b"sdfasdgpp9q429703"[..].into(),
        visits: 352,
        balance: 100.,
        is_admin: true,
    };
    let tx_user = tx.create(user.clone()).unwrap();
    assert_eq!(*tx_user.borrow(), user);

    let user_id = tx_user.id();
    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx.get::<User>(user_id).unwrap();
    assert_eq!(*tx_user.borrow(), user);
}

#[test]
fn test_update() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "Stacey".into(),
            picture: b"fposdasveo8ywa"[..].into(),
            visits: 837,
            balance: 345.,
            is_admin: false,
        })
        .unwrap();

    let tx_user_2 = tx.get::<User>(tx_user.id()).unwrap();
    assert_eq!(*tx_user.borrow(), *tx_user_2.borrow());

    tx_user.borrow_mut().balance = 400.;
    assert_eq!(tx_user_2.borrow().balance, 400.);

    let user_id = tx_user.id();
    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx.get::<User>(user_id).unwrap();
    assert_eq!(tx_user.borrow().balance, 400.);
}

#[test]
fn test_delete() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "Bob".into(),
            picture: b"oigwy8sdvo"[..].into(),
            visits: i64::MAX,
            balance: 220.,
            is_admin: false,
        })
        .unwrap();
    let user_id = tx_user.id();
    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx.get::<User>(user_id).unwrap();
    let tx_user_2 = tx_user.clone();

    tx_user.delete();
    assert!(matches!(tx_user_2.state(), ObjectState::Removed));

    let res = tx.get::<User>(user_id);
    assert_not_found(res, user_id, "User");

    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();
    let res = tx.get::<User>(user_id);
    assert_not_found(res, user_id, "User");
}

#[test]
fn test_create_delete() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "Tiffany".into(),
            picture: b"pygfosyadvaspph"[..].into(),
            visits: 843502,
            balance: 310.,
            is_admin: true,
        })
        .unwrap();
    let tx_user_2 = tx_user.clone();
    let user_id = tx_user.id();

    tx_user.delete();
    assert!(matches!(tx_user_2.state(), ObjectState::Removed));

    let res = tx.get::<User>(user_id);
    assert_not_found(res, user_id, "User");

    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();
    let res = tx.get::<User>(user_id);
    assert_not_found(res, user_id, "User");
}

#[test]
#[should_panic(expected = "already borrowed")]
fn test_double_borrow() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "James".into(),
            picture: b"7304gfqvwp97-gseg[8yo;hasgdpiul"[..].into(),
            visits: -12418,
            balance: 370.,
            is_admin: false,
        })
        .unwrap();
    let tx_user_2 = tx.get::<User>(tx_user.id()).unwrap();

    let _r1 = tx_user.borrow();
    let _r2 = tx_user_2.borrow_mut();
}

#[test]
#[should_panic(expected = "cannot borrow a removed object")]
fn test_borrow_created_deleted() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "James".into(),
            picture: b"sdfasdgpp9q429703"[..].into(),
            visits: i64::MIN,
            balance: 370.,
            is_admin: false,
        })
        .unwrap();
    let tx_user_2 = tx.get::<User>(tx_user.id()).unwrap();

    tx_user.delete();
    tx_user_2.borrow();
}

#[test]
#[should_panic(expected = "cannot borrow a removed object")]
fn test_borrow_deleted() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "James".into(),
            picture: b"cto7sadupvyas8vdgivdsg"[..].into(),
            visits: 1240125,
            balance: 370.,
            is_admin: false,
        })
        .unwrap();
    let user_id = tx_user.id();
    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user_1 = tx.get::<User>(user_id).unwrap();
    let tx_user_2 = tx.get::<User>(user_id).unwrap();

    tx_user_1.delete();
    tx_user_2.borrow();
}

#[test]
#[should_panic(expected = "cannot delete a borrowed object")]
fn test_delete_borrowed() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let tx_user = tx
        .create(User {
            name: "James".into(),
            picture: b"pvyvsdasyoa8dgcsdg"[..].into(),
            visits: 120,
            balance: 370.,
            is_admin: false,
        })
        .unwrap();
    let tx_user_2 = tx.get::<User>(tx_user.id()).unwrap();

    let _r1 = tx_user.borrow();
    tx_user_2.delete();
}

#[test]
fn test_missing_column() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let sqlite_conn = rusqlite::Connection::open(&path).unwrap();
    sqlite_conn
        .execute("CREATE TABLE \"User\" (foo INTEGER)", [])
        .unwrap();
    sqlite_conn.close().unwrap();

    fn check_missing_column<T>(res: &Result<T>) -> bool {
        let columns = ["id", "name", "picture", "visits", "balance", "is_admin"];
        match res {
            Err(orm::Error::MissingColumn(err)) => {
                assert_eq!(err.type_name, "User");
                assert_eq!(err.table_name, "User");
                assert!(columns.contains(&err.attr_name));
                assert!(columns.contains(&err.column_name));
                true
            }
            _ => false,
        }
    }

    let mut orm_conn = Connection::open_sqlite_file(&path).unwrap();
    let tx = orm_conn.new_transaction().unwrap();

    let res_get = tx.get::<User>(1.into());
    assert!(
        check_missing_column(&res_get),
        "expected Error::MissingColumn at get(), got {}",
        fmt_res(&res_get),
    );

    let res_create = tx.create(User {
        name: "Henry".into(),
        picture: b"9g0q743vb8pqavs9ubv"[..].into(),
        visits: 82963052,
        balance: 38235.3,
        is_admin: true,
    });
    if !check_missing_column(&res_create) {
        let fmt_res_create = fmt_res(&res_create);
        let res_commit = tx.commit();
        if !check_missing_column(&res_commit) {
            panic!(
                "expected Error::MissingColumn at create() or commit(), \
                got '{}' for create() and '{}' for commit()",
                fmt_res_create,
                fmt_res(&res_commit),
            );
        }
    }
}

#[test]
fn test_unexpected_type() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let sqlite_conn = rusqlite::Connection::open(&path).unwrap();
    sqlite_conn
        .execute(
            "CREATE TABLE User (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                name TEXT,\
                picture BLOB,\
                visits BIGINT,\
                balance REAL,\
                is_admin TEXT\
            )",
            [],
        )
        .unwrap();
    sqlite_conn
        .execute(
            "INSERT INTO \"User\" VALUES (?, ?, ?, ?, ?, ?)",
            params![1, "Bill", &b"binary"[..], 20, 34.2, "true"],
        )
        .unwrap();
    sqlite_conn.close().unwrap();

    let mut orm_conn = Connection::open_sqlite_file(&path).unwrap();
    let tx = orm_conn.new_transaction().unwrap();

    match tx.get::<User>(1i64.into()) {
        Err(orm::Error::UnexpectedType(err)) => {
            assert_eq!(err.type_name, "User");
            assert_eq!(err.table_name, "User");
            assert_eq!(err.attr_name, "is_admin");
            assert_eq!(err.column_name, "is_admin");
            assert_eq!(err.expected_type, DataType::Bool);
            assert_eq!(err.got_type, "Text");
        }
        res => panic!(
            "expecter Error::IncorrectType at get(), got {}",
            fmt_res(&res),
        ),
    }
}

#[test]
fn test_null_value() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let mut orm_conn = Connection::open_sqlite_file(&path).unwrap();
    let tx = orm_conn.new_transaction().unwrap();

    tx.create(User {
        name: "Henry".into(),
        picture: b"9g0q743vb8pqavs9ubv"[..].into(),
        visits: 82963052,
        balance: 38235.3,
        is_admin: true,
    })
    .unwrap();

    tx.commit().unwrap();

    let sqlite_conn = rusqlite::Connection::open(&path).unwrap();
    sqlite_conn
        .execute(
            "INSERT INTO User(id, name, picture, visits, balance, is_admin) \
            VALUES (?, ?, ?, ?, ?, ?)",
            params![
                100,
                "Jill",
                rusqlite::types::Value::Null,
                45,
                12415.31,
                false
            ],
        )
        .unwrap();
    sqlite_conn.close().unwrap();

    let tx = orm_conn.new_transaction().unwrap();

    match tx.get::<User>(100i64.into()) {
        Err(orm::Error::UnexpectedType(err)) => {
            assert_eq!(err.type_name, "User");
            assert_eq!(err.attr_name, "picture");
            assert_eq!(err.column_name, "picture");
            assert_eq!(err.expected_type, DataType::Bytes);
            assert_eq!(err.got_type, "Null");
        }
        res => panic!(
            "expected Error::IncorrectType at get(), got {}",
            fmt_res(&res),
        ),
    }
}

#[test]
fn test_conflict() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let mut conn_one = Connection::open_sqlite_file(&path).unwrap();
    let tx_one = conn_one.new_transaction().unwrap();

    tx_one
        .create(User {
            name: "Rupert".into(),
            picture: b"pasubvp23vgvesapghd"[..].into(),
            visits: 2352,
            balance: 32355.3,
            is_admin: false,
        })
        .unwrap();

    let mut conn_two = Connection::open_sqlite_file(&path).unwrap();
    let tx_two = conn_two.new_transaction().unwrap();

    let res_create = tx_two.create(User {
        name: "Holly".into(),
        picture: b"sdvgawdvgw0vwg0vas78agvd"[..].into(),
        visits: 235235,
        balance: 84762.3,
        is_admin: false,
    });

    if !matches!(res_create, Err(orm::Error::LockConflict)) {
        panic!("expected Error::LockConflict, got {}", fmt_res(&res_create));
    }
}

#[test]
fn test_empty_struct() {
    #[derive(Object)]
    struct Empty {}

    #[derive(Object)]
    struct Void;

    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    let empty_id = tx.create::<Empty>(Empty {}).unwrap().id();
    let void_id = tx.create::<Void>(Void).unwrap().id();

    tx.get::<Empty>(empty_id).unwrap();
    tx.get::<Void>(void_id).unwrap();

    tx.commit().unwrap();

    let tx = conn.new_transaction().unwrap();

    tx.get::<Empty>(empty_id).unwrap();
    tx.get::<Void>(void_id).unwrap();

    assert!(matches!(
        tx.get::<Empty>(125.into()),
        Err(orm::Error::NotFound(_))
    ));

    assert!(matches!(
        tx.get::<Void>(125.into()),
        Err(orm::Error::NotFound(_))
    ));
}

#[test]
fn test_sql_injection() {
    let names = ["\"; DROP TABLE user --", "'; DROP TABLE user --"];

    let mut conn = Connection::open_in_memory().unwrap();

    for &name in names.iter() {
        let tx = conn.new_transaction().unwrap();

        let user = User {
            name: name.into(),
            picture: name.into(),
            visits: 100,
            balance: 200.,
            is_admin: false,
        };
        let user_id = tx.create(user.clone()).unwrap().id();

        let user_2 = tx.get::<User>(user_id).unwrap();
        assert_eq!(user, *user_2.borrow());

        user_2.borrow_mut().name.push_str("--");

        tx.commit().unwrap();

        let tx = conn.new_transaction().unwrap();

        tx.get::<User>(user_id).unwrap().delete();
        tx.commit().unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Object)]
#[table_name("order_table")]
struct Order {
    #[column_name("IsTall")]
    is_tall: bool,
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_table_column_names() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let mut orm_conn = Connection::open_sqlite_file(&path).unwrap();
    let tx = orm_conn.new_transaction().unwrap();

    let order_id = tx.create(Order { is_tall: true }).unwrap().id().into_i64();
    tx.commit().unwrap();

    let sqlite_conn = rusqlite::Connection::open(&path).unwrap();
    sqlite_conn
        .prepare("SELECT IsTall FROM order_table WHERE id = ?")
        .unwrap()
        .query_row([order_id], |_| Ok(()))
        .unwrap();
}

#[test]
fn test_not_found() {
    let mut conn = Connection::open_in_memory().unwrap();

    let tx = conn.new_transaction().unwrap();
    match tx.get::<Order>(3523.into()) {
        Err(orm::Error::NotFound(err)) => {
            assert_eq!(err.type_name, "Order");
            assert_eq!(err.object_id, 3523.into());
        }
        res => panic!("Expected NotFound, got {}", fmt_res(&res)),
    }
}

#[test]
fn test_unexpected_type_renamed() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let sqlite_conn = rusqlite::Connection::open(&path).unwrap();
    sqlite_conn
        .execute(
            "CREATE TABLE order_table (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                IsTall TEXT\
            )",
            [],
        )
        .unwrap();
    sqlite_conn
        .execute("INSERT INTO order_table VALUES (?, ?)", params![1, "FALSE"])
        .unwrap();
    sqlite_conn.close().unwrap();

    let mut orm_conn = Connection::open_sqlite_file(&path).unwrap();
    let tx = orm_conn.new_transaction().unwrap();

    match tx.get::<Order>(1.into()) {
        Err(orm::Error::UnexpectedType(err)) => {
            assert_eq!(err.type_name, "Order");
            assert_eq!(err.table_name, "order_table");
            assert_eq!(err.attr_name, "is_tall");
            assert_eq!(err.column_name, "IsTall");
            assert_eq!(err.expected_type, DataType::Bool);
            assert_eq!(err.got_type, "Text");
        }
        res => panic!(
            "expecter Error::IncorrectType at get(), got {}",
            fmt_res(&res),
        ),
    }
}

#[test]
fn test_missing_column_renamed() {
    let path = NamedTempFile::new().unwrap().into_temp_path();

    let sqlite_conn = rusqlite::Connection::open(&path).unwrap();
    sqlite_conn
        .execute(
            "CREATE TABLE order_table (id INTEGER PRIMARY KEY AUTOINCREMENT)",
            [],
        )
        .unwrap();
    sqlite_conn
        .execute("INSERT INTO order_table VALUES (?)", [1])
        .unwrap();
    sqlite_conn.close().unwrap();

    fn check_missing_column<T>(res: &Result<T>) -> bool {
        match res {
            Err(orm::Error::MissingColumn(err)) => {
                assert_eq!(err.type_name, "Order");
                assert_eq!(err.table_name, "order_table");
                assert_eq!(err.attr_name, "is_tall");
                assert_eq!(err.column_name, "IsTall");
                true
            }
            _ => false,
        }
    }

    let mut orm_conn = Connection::open_sqlite_file(&path).unwrap();
    let tx = orm_conn.new_transaction().unwrap();

    let res_get = tx.get::<Order>(1.into());
    assert!(
        check_missing_column(&res_get),
        "expected Error::MissingColumn at get(), got {}",
        fmt_res(&res_get),
    );

    let res_create = tx.create(Order { is_tall: false });
    if !check_missing_column(&res_create) {
        let fmt_res_create = fmt_res(&res_create);
        let res_commit = tx.commit();
        if !check_missing_column(&res_commit) {
            panic!(
                "expected Error::MissingColumn at create() or commit(), \
                got '{}' for create() and '{}' for commit()",
                fmt_res_create,
                fmt_res(&res_commit),
            );
        }
    }
}

#[cfg(feature = "test_lifetimes_create")]
#[test]
fn test_lifetimes_create() {
    let mut conn = Connection::open_in_memory().unwrap();
    let tx = conn.new_transaction().unwrap();

    let order = tx.create(Order { is_tall: false }).unwrap();
    tx.commit();

    eprintln!("is_tall: {}", order.borrow().is_tall);
}

#[cfg(feature = "test_lifetimes_get")]
#[test]
fn test_lifetimes_create() {
    let mut conn = Connection::open_in_memory().unwrap();
    let tx = conn.new_transaction().unwrap();

    let order_id = tx.create(Order { is_tall: false }).unwrap().id();
    tx.commit();

    let tx = conn.new_transaction().unwrap();
    let order = tx.get::<Order>(order_id).unwrap();
    tx.commit();

    eprintln!("is_tall: {}", order.borrow().is_tall);
}
