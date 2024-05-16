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

### What You Need

- Rust installed. Download from [rust-lang.org](https://www.rust-lang.org/).
- Wrangler CLI installed. Install it with this command:

  ```sh
  npm install -g wrangler
  ```

- A Cloudflare account. Sign up at [Cloudflare](https://www.cloudflare.com/).

### Setup Steps

1. **Clone the Project**

   Download this project to your computer:

   ```sh
   git clone https://github.com/yourusername/your-repo-name.git
   cd your-repo-name
   ```

2. **Configure Wrangler**

   Log in to your Cloudflare account:

   ```sh
   wrangler login
   ```

3. **Update Configuration File**

   Rename `wrangler.toml.sample` to `wrangler.toml`:

   ```sh
   mv wrangler.toml.sample wrangler.toml
   ```

   Edit the `wrangler.toml` file to include your Cloudflare account details. It should look like this:

   ```toml
   name = "your-worker-name"
   main = "build/worker/shim.mjs"
   compatibility_date = "2024-04-19"
   compatibility_flags = ["formdata_parser_supports_files"]

   [build]
   command = "cargo install -q worker-build && worker-build --release"
   ```

   Replace `your-worker-name` with the name you want for your worker.

### Deploying the Worker

1. **Deploy to Cloudflare Workers**

   Deploy your worker to Cloudflare with:

   ```sh
   wrangler publish
   ```

2. **Access Your Worker**

   After deployment, you can see your worker at:

   ```plaintext
   https://your-worker-name.your-subdomain.workers.dev
   ```

   Replace `your-worker-name` and `your-subdomain` with the correct values.

### Telegram Bot Setup

1. **Set Webhooks**

   To use your worker as a Telegram bot, you need to set up webhooks. Use the following URL format to set your webhook:

   ```plaintext
   https://api.telegram.org/bot<YOUR_BOT_TOKEN>/setWebhook?url=https://your-worker-name.your-subdomain.workers.dev
   ```

   Replace `<YOUR_BOT_TOKEN>` with your actual bot token, and `your-worker-name` and `your-subdomain` with the correct values.

### Troubleshooting

If you have problems:

- Check that your `wrangler.toml` file is correct.
- Make sure you are logged in to Cloudflare with `wrangler login`.
- Verify all dependencies are installed.

For more help, visit the [Cloudflare Workers documentation](https://developers.cloudflare.com/workers/).

## License

Fire Ninja Bot is licensed under the [AGPL3 License](LICENSE).

## Support

If you find Fire Ninja Bot helpful, consider [donating](https://znano.eu.org/support) to support its development and maintenance.

