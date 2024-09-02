
PYTHON_SCRIPT_PATH="/path/to/your_script.py"


while true; do
    if ! python2 "$PYTHON_SCRIPT_PATH"; then
        exit_code=$?
        echo "Script crashed with exit code $exit_code. Restarting..." >&2
        sleep 5
    fi
done
