#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Colors for output ---
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[0;33m'
COLOR_NC='\033[0m' # No Color

echo -e "${COLOR_YELLOW}Starting AtCoder environment setup...${COLOR_NC}"

# --- Check for Python 3 ---
if ! command -v python3 &> /dev/null
then
    echo "Error: python3 is not installed. Please install Python 3 and try again."
    exit 1
fi

# --- Create Python virtual environment ---
VENV_DIR="atcoder-env"
if [ ! -d "$VENV_DIR" ]; then
    echo "Creating Python virtual environment in './${VENV_DIR}'..."
    python3 -m venv $VENV_DIR
else
    echo "Virtual environment './${VENV_DIR}' already exists. Skipping creation."
fi

# --- Install required packages ---
echo "Installing required packages (atcodercli, online-judge-tools, setuptools)..."
# Upgrade pip first and run installs quietly for a cleaner output
"$VENV_DIR/bin/pip" install --quiet --upgrade pip
"$VENV_DIR/bin/pip" install --quiet atcodercli online-judge-tools setuptools

echo -e "\n${COLOR_GREEN}Setup complete!${COLOR_NC}"
echo "The virtual environment is ready in the './${VENV_DIR}' directory."
echo "You can now use 'task new -- <contest_name>' to set up a new contest." 