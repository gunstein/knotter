# knotter

<img src="/knotter.webp" >

Collaborative editing of a sphere surface. Client made with Rust, Bevy and Rapier and deployd to web as WASM. Server made with rust and actix web.

[Demo](https://gunstein.vatnar.no)

Build server:
docker build -f Dockerfile_server -t knotter_api_server .
docker tag knotter_api_server gunstein/knotter_api_server:ver_2
docker login
docker push gunstein/knotter_api_server:ver_2

sudo docker run -v knotter_data_volume:/data -p 8080:8080 knotter_api_server

Restart docker compose (builds everything)
sudo docker compose down
sudo docker compose up -d

NB! If changes in client. Client volume must be deleted with this command
docker volume rm knotter_client-data

docker compose up -d --build