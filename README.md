# gnam-api

## Running with docker-compose

```yaml
version: "2"
services:
    gnam-api:
        image: ghcr.io/fcl-lm/gnam-api:main
        container_name: gnam-api
        ports:
            - "9090:9090"
        environment:
            S3_ENDPOINT: http://127.0.0.1:8333
            AWS_ACCESS_KEY_ID: adminadmin
            AWS_SECRET_ACCESS_KEY: adminadmin
        volumes:
            - <source_document>:/data
```

## API
The rest API is described below.

### Get health status
#### Request
`GET /health`
```bash
curl -i http://localhost:9090/health
```

#### Response
```
HTTP/1.1 200 OK
content-length: 40
content-type: application/json
date: Sat, 17 Jun 2023 20:46:46 GMT

{"message":"The gnam-api service is up"}
```

### Ingest new data
#### Request
`GET /gnam`
```bash
curl -i -F file=@./file http://localhost:9090/gnam
```

#### Response
```
HTTP/1.1 100 Continue

HTTP/1.1 200 OK
content-length: 22
content-type: application/json
date: Sat, 17 Jun 2023 20:48:53 GMT

{"message":"Success."}
```