use actix_web::cookie::Key;
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;

pub fn set_up_flash_messages() -> FlashMessagesFramework {
    let secret_key =
        Key::from(b"0123456789012345678901234567890123456789012345678901234567890123456789");

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    FlashMessagesFramework::builder(message_store).build()
}
