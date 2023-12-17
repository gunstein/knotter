# Use an official Nginx image
FROM nginx:alpine

# Copy the static content (HTML, JS, CSS, WASM, etc.) to the Nginx web directory
COPY web_test /usr/share/nginx/html

# Override the default Nginx configuration if necessary (optional)
# COPY nginx.conf /etc/nginx/nginx.conf

# Expose port 80
EXPOSE 80

# Copy the entrypoint script to the container
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Use the entrypoint script to start the container
ENTRYPOINT ["/entrypoint.sh"]
