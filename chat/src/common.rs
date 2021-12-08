pub const ADMIN_UID: UserId = UserId(0);

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct UserId(pub u32);

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ChatId(pub u32);
