# Test valid URL redirect
RESPONSE_CODE=$(curl -k -s -o /dev/null -w '%{http_code}' staging.codecraft.engineering/url/B5Z)
if [ "$RESPONSE_CODE" != "307" ]; then
    echo "Error: Expected 307 redirect for valid URL, got $RESPONSE_CODE"
    exit 1
else
    echo "Valid URL redirect test passed"
fi

# Test invalid URL
RESPONSE_CODE=$(curl -k -s -o /dev/null -w '%{http_code}' staging.codecraft.engineering/url/invalid)
if [ "$RESPONSE_CODE" != "404" ]; then
    echo "Error: Expected 404 for invalid URL, got $RESPONSE_CODE"
    exit 1
else
    echo "Invalid URL redirect test passed"
fi

# Test root handler
RESPONSE_CODE=$(curl -k -s -o /dev/null -w '%{http_code}' staging.codecraft.engineering/)
if [ "$RESPONSE_CODE" != "200" ]; then
    echo "Error: Expected 200 OK response from root handler, got $RESPONSE_CODE"
    exit 1
else
    echo "Root handler test passed"
fi

# Test static file
RESPONSE_CODE=$(curl -k -s -o /dev/null -w '%{http_code}' staging.codecraft.engineering/static/CodeCraft%20Engineering%20logo.png)
if [ "$RESPONSE_CODE" != "200" ]; then
    echo "Error: Expected 200 OK response from static file handler, got $RESPONSE_CODE"
    exit 1
else
    echo "Static file handler test passed"
fi

echo "All E2E tests passed successfully!"