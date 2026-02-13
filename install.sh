#!/bin/bash

# Configuration
REPO="ronaldzav/network-monitor-connector"
INSTALL_DIR="/opt/network-monitor-connector"
SERVICE_NAME="network-monitor"
BINARY_NAME="network-monitor-connector"
DEFAULT_PORT=2141

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}      Network Monitor Installer         ${NC}"
echo -e "${BLUE}           by RonaldZav                 ${NC}"
echo -e "${BLUE}========================================${NC}"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo -e "${RED}Please run as root (sudo)${NC}"
  exit 1
fi

# 1. Install Dependencies (Minimal)
echo -e "${GREEN}[+] Installing dependencies (curl)...${NC}"
apt-get update -qq && apt-get install -y -qq curl ca-certificates

# 2. Prepare Install Directory
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${GREEN}[+] Creating installation directory: $INSTALL_DIR${NC}"
    mkdir -p "$INSTALL_DIR"
else
    echo -e "${YELLOW}[!] Installation directory already exists.${NC}"
fi

# 3. Download Latest Binary
echo -e "${GREEN}[+] Fetching latest release info...${NC}"
API_URL="https://api.github.com/repos/$REPO/releases/latest"
RELEASE_DATA=$(curl -s "$API_URL")

# Extract download URL for the binary (matches "network-monitor-connector" in the asset name)
DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep "$BINARY_NAME" | cut -d '"' -f 4 | head -n 1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: Could not find a binary asset containing '$BINARY_NAME' in the latest release.${NC}"
    echo -e "Please ensure you have uploaded the compiled binary to the GitHub Release."
    exit 1
fi

# Stop service if running to allow binary replacement
if systemctl is-active --quiet $SERVICE_NAME; then
    echo -e "${YELLOW}[!] Stopping existing service to update binary...${NC}"
    systemctl stop $SERVICE_NAME
fi

echo -e "${GREEN}[+] Downloading binary from: $DOWNLOAD_URL${NC}"
curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$DOWNLOAD_URL"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# 4. Create Config if missing
if [ ! -f "$INSTALL_DIR/config.yml" ]; then
    echo -e "${GREEN}[+] Creating default configuration...${NC}"
    cat > "$INSTALL_DIR/config.yml" <<EOF
host: 0.0.0.0
port: $DEFAULT_PORT
whitelist:
  enabled: true
  list:
    - 0.0.0.0/0
token: ""
EOF
else
    echo -e "${GREEN}[+] Config file already exists, preserving it.${NC}"
fi

# 5. Create Systemd Service (Always update to ensure correct paths)
echo -e "${GREEN}[+] Configuring Systemd service...${NC}"
cat > /etc/systemd/system/$SERVICE_NAME.service <<EOF
[Unit]
Description=Network Monitor Connector
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/$BINARY_NAME
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# 6. Enable and Start Service
echo -e "${GREEN}[+] Starting service...${NC}"
systemctl daemon-reload
systemctl enable $SERVICE_NAME
systemctl restart $SERVICE_NAME

# 7. Final Status
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Installation/Update Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo "Service status:"
systemctl status $SERVICE_NAME --no-pager | head -n 10

echo ""
echo -e "To view logs: ${BLUE}journalctl -u $SERVICE_NAME -f${NC}"
echo -e "Config file:  ${BLUE}$INSTALL_DIR/config.yml${NC}"
echo ""
echo -e "${GREEN}Enjoy!${NC}"
