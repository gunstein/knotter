

curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
           "operation_id": 1,
           "object_uuid": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
           "object_data": "Sample Data"
         }' \
     http://127.0.0.1:8080/sample-globe-id

curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
           "operation_id": 1,
           "object_uuid": "f47ac10b-58cc-4372-a567-0e02b2c3d489",
           "object_data": "Sample Data"
         }' \
     http://127.0.0.1:8080/sample-globe-id


curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
           "operation_id": 1,
           "object_uuid": "f47ac10b-58cc-4372-a567-0e02b2c3d489",
           "object_data": "Sample Data"
         }' \
     http://127.0.0.1:8080/sample-globe-id


curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
               "is_fixed": false,
               "is_insert": true,
               "object_uuid": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
               "color": "red",
               "position": {
                    "x": 12.34,
                    "y": 56.78,
                    "z": 90.12
               },
               "velocity": {
                    "v_x": 0.123,
                    "v_y": 4.567,
                    "v_z": 8.910
               }
         }' \
     http://127.0.0.1:8080/globe1


     curl http://127.0.0.1:8080/globe1/0

     curl http://127.0.0.1:8080/health


curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
        "is_fixed": true,
        "is_insert": true,
        "uuid": "4d3cbd35-41e8-40be-96d2-ac0c4b9f4f26",
        "color": "#ff0000",
        "position": {
            "x": -1.05,
            "y": 0.0,
            "z": 0.0
        },
        "velocity": null
     }' \
http://127.0.0.1:8080/globe1


curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
        "is_fixed": true,
        "is_insert": true,
        "uuid": "4d3cbd35-41e8-40be-96d2-ac0c4b9f4f27",
        "color": "#ff0000",
        "position": {
            "x": 0.0,
            "y": -1.05,
            "z": 0.0
        },
        "velocity": null
     }' \
http://127.0.0.1:8080/globe1

RUST_LOG=debug cargo run
RUST_LOG=debug cargo test test_set_and_retrieve_data

cargo test -- --test-threads=1

run only integration tests, they can't be run in paralell because of stop and restart of server.
cargo test --tests -- --test-threads=1

only run unit tests
cargo test --lib

Ensure debug! is working
RUST_LOG=debug cargo test test_deserialization_insertballdto -- --nocapture

stress testing:
locust -f locustfile2.py
