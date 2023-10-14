Start ScyllaDb
docker run --name scylladb -d -p 9042:9042 scylladb/scylla
or
docker run --rm --name scylladb -it -p 9042:9042 scylladb/scylla --smp 2

Connect to scylla container
docker exec -it scylladb cqlsh

Create table
CREATE KEYSPACE IF NOT EXISTS mykeyspace WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

USE mykeyspace;

CREATE TABLE IF NOT EXISTS transactions (
    globe_id text,
    transaction_id TIMEUUID,
    operation_id int,
    object_uuid UUID,
    object_data text,
    PRIMARY KEY (globe_id, transaction_id)
);


INSERT INTO transactions (globe_id, transaction_id, operation_id, object_uuid, object_data) 
VALUES ('globe123', 9a3227ec-8e82-43aa-bc1a-31e9780f90d8, 1, a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15, 'first data');

INSERT INTO transactions (globe_id, transaction_id, operation_id, object_uuid, object_data) 
VALUES ('globe123', a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12, 2, a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a16, 'second data');

INSERT INTO transactions (globe_id, transaction_id, operation_id, object_uuid, object_data) 
VALUES ('globe456', a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13, 1, a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a17, 'another data');

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
           "object_data": "Sample Data"
         }' \
     http://127.0.0.1:8080/globe1


     curl http://127.0.0.1:8080/globe1

     curl http://127.0.0.1:8080/health