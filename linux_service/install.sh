# Specify the service name and file path
SERVICE_NAME="bt_ai_ui"
SERVICE_FILE="./bt_ai_ui.service"
SOURCE_FILE="./ai_ui"
DESTINATION_PGM_FILE="/usr/local/bin/bt_ai_ui"
DESTINATION_FOLDER="/usr/local/share/bt_ai_ui"
USER_NAME="bt.ai_ui.service"
GROUP_NAME="bt.ai.service"

# Check if the service is running
if systemctl is-active --quiet "$SERVICE_NAME"; then
    echo "Service $SERVICE_NAME is running. Stopping the service..."
    # Stop the service
    sudo systemctl stop "$SERVICE_NAME"
else
    echo "Service $SERVICE_NAME is not running."
fi

# Check if the group exists
if getent group "$GROUP_NAME" &>/dev/null; then
    echo "Group $GROUP_NAME exists."
else
    echo "Group $GROUP_NAME does not exist. Creating group..."
    sudo groupadd "$GROUP_NAME"
    # sudo groupadd bt.ai.service
fi

# Check if the user exists
if id "$USER_NAME" &>/dev/null; then
    echo "User $USER_NAME exists."
else
    echo "User $USER_NAME does not exist. Creating user..."
    sudo useradd -r -g "$GROUP_NAME" -s /usr/sbin/nologin -d /nonexistent "$USER_NAME"
    #sudo useradd -r -g bt.ai.service -s /usr/sbin/nologin -d /nonexistent bt.ai_ui.service
fi

echo "Installing Bachuetech AI UI application"
echo "Moving file $SOURCE_FILE to $DESTINATION_PGM_FILE..."
sudo cp -f "$SOURCE_FILE" "$DESTINATION_PGM_FILE"
#sudo mv ai_ui /usr/local/bin/bt_ai_ui
sudo chmod +x "$DESTINATION_PGM_FILE"
sudo chown "root:$GROUP_NAME" "$DESTINATION_PGM_FILE"
#sudo chown bt.ai_ui.service:bt.ai.service /usr/local/bin/bt_ai_ui

# Check if the directory exists, if not, create it
if [ ! -d "$DESTINATION_FOLDER" ]; then
    echo "Directory $DESTINATION_FOLDER does not exist. Creating directory..."
    sudo mkdir -p "$DESTINATION_FOLDER"
    #sudo mkdir /usr/local/share/bt_ai_ui
else
    echo "Directory $DESTINATION_FOLDER already exists."
fi

#Moving Config and Site folder
sudo cp -fr site "$DESTINATION_FOLDER"
sudo cp -fr config "$DESTINATION_FOLDER"

sudo chown -R "root:$GROUP_NAME" "$DESTINATION_FOLDER"
sudo chmod -R 770 "$DESTINATION_FOLDER"

#Moving Service File
sudo cp -f "$SERVICE_FILE" /etc/systemd/system/
sudo chown root:root /etc/systemd/system/bt_ai_ui.service

sudo systemctl daemon-reload
sudo systemctl start "$SERVICE_NAME"
if systemctl is-active --quiet "$SERVICE_NAME"; then
    echo "Service $SERVICE_NAME is running!"
    # Stop the service
    sudo systemctl status "$SERVICE_NAME"
    sudo systemctl enable "$SERVICE_NAME"
else
    echo "ERROR: Service $SERVICE_NAME is not running."
    sudo journalctl -u "$SERVICE_NAME" -f
fi


