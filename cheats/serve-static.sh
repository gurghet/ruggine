#!/bin/bash
# establish a working directory as the directory this script is in
cd $(dirname "$0")
# run caddy
caddy run --config ./Caddyfile
