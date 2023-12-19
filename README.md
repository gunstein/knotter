# knotter
Build server:
docker build -f Dockerfile_server -t knotter_api_server .
docker tag knotter_api_server gunstein/knotter_api_server:ver_1
docker login
docker push gunstein/knotter_api_server:ver_1

sudo docker run -v knotter_data_volume:/data -p 8080:8080 knotter_api_server

Restart docker compose (builds everything)
sudo docker compose down
sudo docker compose up -d

NB! If changes in client. Client volumen must be deleted with this command
docker volume rm knotter_client-data

docker compose up -d --build