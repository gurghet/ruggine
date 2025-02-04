#!/bin/bash

# Exit on error
set -e

# Function to check response
check_response() {
    local url=$1
    local expected_status=$2
    local expected_body=$3
    local description=$4

    echo "Testing $description..."
    response=$(curl -s -w "\n%{http_code}" $url)
    status=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')

    if [ "$status" != "$expected_status" ]; then
        echo "❌ Failed: Expected status $expected_status but got $status"
        exit 1
    fi

    if [ ! -z "$expected_body" ] && [[ ! "$body" =~ $expected_body ]]; then
        echo "❌ Failed: Response body doesn't match expected pattern"
        echo "Expected: $expected_body"
        echo "Got: $body"
        exit 1
    fi

    echo "✅ Passed!"
    echo
}

# Cleanup function
cleanup() {
    echo "Cleaning up..."
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
    pkill -f url_shortener 2>/dev/null || true
}

# Set up cleanup on script exit
trap cleanup EXIT

# Kill any existing server
echo "Ensuring no existing server is running..."
pkill -f url_shortener 2>/dev/null || true
sleep 1

# Start the server in the background
echo "Starting server..."
cargo run &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to start..."
for i in {1..10}; do
    if curl -s http://localhost:3000/healthz >/dev/null; then
        echo "Server is ready!"
        break
    fi
    if [ $i -eq 10 ]; then
        echo "Server failed to start"
        exit 1
    fi
    sleep 1
done

# Run tests
check_response "http://localhost:3000/" "200" "<!DOCTYPE html>" "Root path returns index.html"
check_response "http://localhost:3000/static/index.html" "404" "Not Found" "Direct access to index.html returns 404"
check_response "http://localhost:3000/static/images/cce-logo.png" "200" "" "Access to existing image returns 200"
check_response "http://localhost:3000/static/images/nonexistent.png" "404" "File not found" "Access to non-existent image returns 404"
check_response "http://localhost:3000/static/random.txt" "404" "Not Found" "Access to non-image file returns 404"

echo "All tests passed! 🎉"
