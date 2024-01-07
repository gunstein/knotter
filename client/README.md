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
./build_wasm_test.sh
docker build -t knotter_client .
docker tag knotter_client gunstein/knotter_client:ver_1
docker push gunstein/knotter_client:ver_1

docker run -e API_URL='http://localhost:8080' -p 80:80 knotter_client

Stop all containers, Reomove all containers and Remove all docker images:
docker stop $(docker ps -aq)
docker rm $(docker ps -aq)
docker rmi $(docker images -q)
maybe:
docker image prune -a --force

For the demo version:
When starting client in browser remember the globe parameter:
http://192.168.86.40/?globe=gvtest123



