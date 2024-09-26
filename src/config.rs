/// BOT Token.
pub const BOT_TOKEN: &str = include_str!("../BOT_TOKEN.txt");

/// D1 Database Name.
pub const D1_DB_NAME: &str = "DB_FNB";

/// Sources for MTProto proxies.
pub const MTPROTO_SOURCES: &[&str] = &["https://t.me/s/NextGenProxy", "https://t.me/s/MTP_roto"];

/// Sources for Shadowsocks proxies.
pub const SHADOWSOCKS_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

/// Sources for VMess proxies.
pub const VMESS_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

/// Sources for VLess proxies.
pub const VLESS_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

/// Sources for Trojan proxies.
pub const TROJAN_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

/// Sources for Hysteria proxies.
pub const HYSTERIA_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

/// Sources for TUIC proxies.
pub const TUIC_SOURCES: &[&str] =
    &["https://raw.githubusercontent.com/barry-far/V2ray-Configs/main/All_Configs_Sub.txt"];

/// Help message providing information about available commands.
pub const HELP_MESSAGE: &str = "Fire Ninja Bot allows you to access proxies to bypass firewalls and access blocked content. Currently, only the following commands are available:

- /help: Shows this message.
- /support: If you want to keep the project alive.
- /mtproxy: Fetches and provides a list of MTProto proxies.
- /shadowsocks: Fetches and provides a list of Shadowsocks proxies.
- /vmess: Fetches and provides a list of VMess proxies.
- /vless: Fetches and provides a list of VLess proxies.
- /trojan: Fetches and provides a list of Trojan proxies.
- /hysteria: Fetches and provides a list of Hysteria(version 1 and 2) proxies.
- /tuic: Fetches and provides a list of TUIC proxies.
";

/// Support message providing information on how to support the project.
pub const SUPPORT_MESSAGE: &str = "
If you find this bot helpful, please consider donating to keep the project alive, or simply give it a star on my GitHub: 

Monero (XMR): `8AF4Lybz7QwiucdYW2szsgiqTHdBp5kjZSSRm6ddzd5363S6n4jixpkACGMLx5JWZnUR5MnGF7cMoidjppruAvLvMe2ovHZ`

**Ethereum** (And any BEP20 token): `0x9E00DC6bE0d07bDB5Ff8B62593a0193913c9B595`

**TRON** (And any TRC20 token): `TUT762nFQQRoXvDe1Z72p3kKH9uY3XZCg9`

- __Github Repository__: https://github.com/zolagonano/fire-ninja-bot
";

/// Message displayed for an invalid command.
pub const INVALID_COMMAND_MESSAGE: &str =
    "Invalid command, use /help to get list of available commands.";

/// Message displayed when user successfully subscribes for proxies.
pub const SUBSCRIPTION_MESSAGE: &str =
    "You successfully subscribed! You'd recive your daily proxies at UTC+000 15:30";

/// Message displayed when user fails to subscribe for proxies.
pub const SUBSCRIPTION_FAILED_MESSAGE: &str =
    "Please provide a proxy type to subscribe, for example: `/subscribe mtproxy, vless`";

/// Message displayed when adding new user to database fails.
pub const FAILED_TO_ADD_USER_MESSAGE: &str = "DB_ERR: Failed to add new user to D1 Database.";

/// Message displayed when removing user from the database fails.
pub const FAILED_TO_DELETE_USER_MESSAGE: &str = "DB_ERR: Failed to delete user from D1 Database.";

/// Message displayed when user unsubscribes from daily proxies.
pub const UNSUBSCRIBE_MESSAGE: &str = "You successfully canceled your daily proxy subscription.";

/// Message displayed when no proxies are found.
pub const NO_PROXIES_FOUND_MESSAGE: &str = "No proxies were found.";

/// Separator used to separate lines in the output.
pub const LINE_SEPERATOR: &str = "\n﹌﹌﹌\n";

/// Command string for MTProto proxies.
pub const MTPROXY_COMMAND: &str = "/mtproxy";

/// Command string for Shadowsocks proxies.
pub const SHADOWSOCKS_COMMAND: &str = "/shadowsocks";

/// Command string for VMess proxies.
pub const VMESS_COMMAND: &str = "/vmess";

/// Command string for VLess proxies.
pub const VLESS_COMMAND: &str = "/vless";

/// Command string for Trojan proxies.
pub const TROJAN_COMMAND: &str = "/trojan";

/// Command string for Hysteria proxies.
pub const HYSTERIA_COMMAND: &str = "/hysteria";

/// Command string for TUIC proxies.
pub const TUIC_COMMAND: &str = "/tuic";

/// Command string for displaying help message.
pub const HELP_COMMAND: &str = "/help";

/// Command string for starting the bot.
pub const START_COMMAND: &str = "/start";

/// Command string for displaying support message.
pub const SUPPORT_COMMAND: &str = "/support";

/// Command string for subscriptions
pub const SUBSCRIBE_COMMAND: &str = "/subscribe";

/// Command string to unsubscribe
pub const UNSUBSCRIBE_COMMAND: &str = "/unsubscribe";
