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
    response=$(curl -k -s -w "\n%{http_code}" $url)
    status=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')

    if [ "$status" != "$expected_status" ]; then
        echo " Failed: Expected status $expected_status but got $status"
        exit 1
    fi

    if [ ! -z "$expected_body" ] && [[ ! "$body" =~ $expected_body ]]; then
        echo " Failed: Response body doesn't match expected pattern"
        echo "Expected: $expected_body"
        echo "Got: $body"
        exit 1
    fi

    echo " Passed!"
    echo
}

# Test URL shortener functionality
check_response "staging.codecraft.engineering/url/B5Z" "307" "" "URL shortener redirect"
check_response "staging.codecraft.engineering/url/invalid" "404" "URL shortcode not found" "Invalid URL redirect"

# Test static file serving
check_response "staging.codecraft.engineering/" "200" "<!DOCTYPE html>" "Root path returns index.html"
check_response "staging.codecraft.engineering/static/index.html" "404" "Not Found" "Direct access to index.html returns 404"
check_response "staging.codecraft.engineering/static/images/cce-logo.png" "200" "" "Access to existing image returns 200"
check_response "staging.codecraft.engineering/static/images/nonexistent.png" "404" "File not found" "Access to non-existent image returns 404"
check_response "staging.codecraft.engineering/static/random.txt" "404" "Not Found" "Access to non-image file returns 404"

# Test utility endpoints
check_response "staging.codecraft.engineering/healthz" "200" "OK" "Health check endpoint"
check_response "staging.codecraft.engineering/version" "200" "" "Version endpoint"

echo "All tests passed! "