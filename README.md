## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
./docker.sh
```

visit http://localhost:8000 and http://localhost:3000

## Test with curl

curl  -v --location --request POST 'localhost:3000/signup' \
--header 'Content-Type: application/json' \
--data '{
    "email": "vas@mail",
    "password": "pass",
    "requires2FA": true
}'


curl  -v --location --request POST 'localhost:3000/verify-2fa' \
--header 'Content-Type: application/json' \
--data '{
    "email": "vas77794@gmail.com",
    "loginAttemptId": "c0cf3f17-a544-4bd6-ad38-7677a6dc57ba",
    "2FACode": "592426"
}'