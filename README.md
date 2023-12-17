# knotter
Build server:
sudo docker build -f Dockerfile_server -t knotter_api_server .
sudo docker run -v knotter_data_volume:/data -p 8080:8080 knotter_api_server

Restart docker compose
sudo docker compose down
sudo docker compose up -d