# Network Monitor Connector

A lightweight, high-performance system monitoring API written in Rust. It provides real-time CPU and RAM usage statistics via a secure REST API.

## Features

*   **CPU Monitoring**: Global usage and per-core breakdown.
*   **RAM Monitoring**: Used and total memory.
*   **Secure**: Token-based authentication and IP whitelisting.
*   **Fast**: Built with Rust and Actix-web.
*   **Easy Deployment**: Downloads the pre-compiled binary directly from GitHub Releases.

## Quick Install (Ubuntu/Debian)
ONLY SUPPORT FOR x86_64 CPU, IF YOU USE ARM CPU, DOWNLOAD SOURCE CODE AND COMPILE.

Run the following command in your terminal to install and start the service automatically:

```bash
curl -sSL https://raw.githubusercontent.com/ronaldzav/network-monitor-connector/main/install.sh | sudo bash
```

This script will:
1. Fetch the latest binary release from GitHub.
2. Install it to `/opt/network-monitor-connector`.
3. Set up a Systemd service (`network-monitor`) to keep it running in the background.

## Configuration

The configuration file is located at:
`/opt/network-monitor-connector/config.yml`

```yaml
host: 0.0.0.0
port: 2141
whitelist:
  enabled: true
  list:
    - 0.0.0.0/0  # Change this to your trusted IP range (e.g., 192.168.1.0/24)
token: "YOUR_GENERATED_TOKEN"
```

After changing the configuration, restart the service:
```bash
sudo systemctl restart network-monitor
```

## API Usage

### Authentication
Pass the token in the header or as a query parameter.

*   **Header**: `Authorization: Bearer <TOKEN>`
*   **Query**: `?token=<TOKEN>`

### Endpoints

#### 1. Get All Metrics
```http
GET /v1/monitor
```
Response:
```json
{
  "cpu": {
    "cpu": 15.5,
    "cores": { "0": 10.2, "1": 20.5 }
  },
  "ram": {
    "used_ram": 4500000000,
    "total_ram": 16000000000
  }
}
```

#### 2. Get CPU Only
```http
GET /v1/monitor/cpu
```

#### 3. Get RAM Only
```http
GET /v1/monitor/ram
```

## Logs

To view the service logs (including the startup banner and token):

```bash
sudo journalctl -u network-monitor -f
```

## Uninstallation

To remove the service and files:

```bash
sudo systemctl stop network-monitor
sudo systemctl disable network-monitor
sudo rm /etc/systemd/system/network-monitor.service
sudo rm -rf /opt/network-monitor-connector
sudo systemctl daemon-reload
```

---
by RonaldZav
