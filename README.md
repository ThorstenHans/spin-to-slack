# Spin-to-Slack

`Spin-to-Slack` is a sample application demonstrating how to use configuration data in Fermyon Spin. For detailed walk-through consider reading the corresponding blog post at: [https://www.thorsten-hans.com/master-configuration-data-in-fermyon-spin](https://www.thorsten-hans.com/master-configuration-data-in-fermyon-spin).

## How to run Spin-to-Slack

Either `Docker` is required (to run HashiCorp Vault) or you must ensure your Spin application can access an instance of HashiCorp Vault that runs somewhere else.

### HashiCorp Vault

```bash
docker run -d -e VAULT_DEV_ROOT_TOKEN_ID=foobar \
 -e VAULT_SERVER="http://127.0.0.1:8200" \
 -p 8200:8200 vault

# Install vault CLI (macOS shown here with Homebrew)
brew install vault

# Set Vault URL and token as env vars
# Alternatively, you can provide the token when using vault CLI
export VAULT_ADDR='http://0.0.0.0:8200'
export VAULT_TOKEN=foobar

# Store slack_webhook_url in vault
vault kv put secret/slack_webhook_url value="YOUR slack webhook url"
```

### Run Spin-to-Slack

```bash
export SPIN_APP_CHANNEL="#blog"
export SPIN_APP_IS_MARKDOWN="true"

# 3. Start the Spin application
spin build --up --follow-all --runtime-config-file vault.toml
```

## Send HTTP Post requests to the Spin-to-Slack

```bash
curl -X POST --json '{ "message": "Hello! This is Spin *speaking*\r\n:beverage_box:" }' http://localhost:3000
```
