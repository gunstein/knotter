Bygge wasm
~/koding/knotter/client   cargo build --release --target wasm32-unknown-unknown

Bygge katalog med javascript og rigging
~/koding/knotter$ wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/knotter.wasm 

Optimalisere wasm (gjøre mindre)

~/koding/knotter/target/wasm32-unknown-unknown/release$ wasm-opt -Oz -o output2.wasm knotter.wasm

//Run local web server with wasm
basic-http-server web_test
//to test mobile. ifconfig to get address to use in browser
basic-http-server -a 0.0.0.0:9090 web_test


RUST_LOG=off cargo run --release

Docker build client:
sudo docker build -t knotter_client .
docker run -e API_URL='http://localhost:8080' -p 80:80 knotter_client

Stop all containers, Reomove all containers and Remove all docker images:
docker stop $(docker ps -aq)
docker rm $(docker ps -aq)
docker rmi $(docker images -q)
maybe:
docker image prune -a --force

