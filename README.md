# Fire Ninja Bot

Fire Ninja Bot is a Telegram bot implemented as a Cloudflare Worker. It provides users with access to a variety of proxies, allowing them to bypass firewalls and access blocked content.

## Features

- Fetches proxies from specified sources.
- Responds to Telegram commands to provide proxy information.
- TODO: Support for various types of proxies:
    - [x] MTPROTO
    - [x] SS
    - [x] VMESS
    - [x] VLESS
    - [x] TROJAN
    - [x] HYSTERIA1
    - [x] HYSTERIA2
    - [x] TUIC
    - [ ] SSR
    - [ ] HTTP
    - [ ] HTTPS
    - [ ] SOCKS4
    - [ ] SOCKS5

## Usage

To use Fire Ninja Bot, simply add it to your Telegram group or chat and interact with it using the available commands.

Available commands:

- `/help`: Shows Help message.
- `/support`: Shows donation URLs.
- `/mtproxy`: Fetches and provides a list of available proxies.
- `/shadowsocks`: Fetches and provides a list of shadowsocks proxies.
- `/vmess`: Fetches and provides a list of VMess proxies. 
- `/vless`: Fetches and provides a list of VLess proxies. 
- `/trojan`: Fetches and provides a list of Trojan proxies. 
- `/hysteria`: Fetches and provides a list of Hysteria(v1 and v2) proxies. 
- `/tuic`: Fetches and provides a list of TUIC proxies. 


## Setup

1. Clone this repository.
2. Deploy the Cloudflare Worker with your Cloudflare account.
3. Add the deployed Worker URL as a webhook in your Telegram bot settings.
4. Interact with your bot on Telegram using the available commands.

## License

Fire Ninja Bot is licensed under the [AGPL3 License](LICENSE).

## Support

If you find Fire Ninja Bot helpful, consider [donating](https://znano.eu.org/support) to support its development and maintenance.

