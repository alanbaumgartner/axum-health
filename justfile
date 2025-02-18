#!/usr/bin/env just --justfile

set shell := ["powershell.exe", "-c"]

test:
    cargo nextest run --features=diesel-r2d2,diesel-mobc,diesel-deadpool,diesel-bb8,sqlx,sea-orm
