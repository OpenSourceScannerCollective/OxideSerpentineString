#!/bin/bash
python3 -m venv .env
source .env/bin/activate
cd ./python/oxide_serpentine_string
python test.py