#!/bin/bash
# Simple Python test runner for pymwemu

set -e

echo "Setting up pymwemu test environment..."

# Check if virtual environment exists
VENV_DIR=".env"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    VENV_PYTHON="$VENV_DIR/Scripts/python.exe"
    VENV_ACTIVATE="$VENV_DIR/Scripts/activate"
else
    VENV_PYTHON="$VENV_DIR/bin/python"
    VENV_ACTIVATE="$VENV_DIR/bin/activate"
fi

# Create venv if it doesn't exist
if [ ! -f "$VENV_PYTHON" ]; then
    echo "Creating virtual environment..."
    python3 -m venv "$VENV_DIR"
    
    # Activate and install maturin
    source "$VENV_ACTIVATE"
    pip install maturin[patchelf]
    
    # Build pymwemu
    maturin develop --release
    
    echo "Environment setup complete!"
else
    echo "Virtual environment found, activating..."
    source "$VENV_ACTIVATE"
fi

# Run simple tests first
echo "Running simple tests..."
python -m unittest tests.test_simple -v

echo ""
echo "Running all tests..."
python -m unittest discover -v tests/
