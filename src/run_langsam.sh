#!/bin/bash
echo "Starting run_langsam.sh" >&2
# Activate the virtual environment
source /home/mateo/projects/code-art/lang-segment-anything/env/bin/activate

# Change to the directory containing the Python script
cd /home/mateo/projects/code-art/lang-segment-anything
echo "Current directory: $(pwd)" >&2

# Run the Python script, passing along all arguments
/home/mateo/projects/code-art/lang-segment-anything/env/bin/python api.py "$@"

echo "Finished run_langsam.sh" >&2
