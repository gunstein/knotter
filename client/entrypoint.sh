#!/bin/sh

# Replace API_URL in index.html with the environment variable value
sed -i "s|window.API_URL =.*|window.API_URL = \"$API_URL\";|" /usr/share/nginx/html/index.html

# Start Nginx
exec nginx -g 'daemon off;'
