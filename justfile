#!/usr/bin/env just --justfile

set shell := ["powershell.exe", "-c"]

test:
    cargo nextest run --all-features
