# rsip

[![tests](https://github.com/HMBSbige/rsip/actions/workflows/tests.yml/badge.svg)](https://github.com/HMBSbige/rsip/actions/workflows/tests.yml)
[![Release Docker Image](https://github.com/HMBSbige/rsip/actions/workflows/release-docker-image.yml/badge.svg)](https://github.com/HMBSbige/rsip/actions/workflows/release-docker-image.yml)
[![Docker](https://img.shields.io/badge/rsip-blue?label=Docker&logo=docker)](https://github.com/users/HMBSbige/packages/container/package/rsip)

Show client's IP address behind a reverse proxy.

## Docker compose example

```yml
services:
  rsip:
    image: ghcr.io/hmbsbige/rsip:latest
    container_name: rsip
    restart: unless-stopped
    networks:
      - traefik-net
    labels:
      traefik.enable: true
      traefik.http.routers.rsip.rule: Host(`rsip.example.com`)
    logging:
      driver: json-file
      options:
        max-size: 10m
        max-file: 3
    ulimits:
      nofile:
        soft: 1048576
        hard: 1048576
```
