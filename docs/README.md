# SourceWarden

[![Check](https://github.com/Eastwooder/source-warden/actions/workflows/check.yaml/badge.svg)](https://github.com/Eastwooder/source-warden/actions/workflows/check.yaml)

A GitHub Application meant to watch over your source code.

## Getting started

Either you install all required packages and tools, or you just use the
provided developer environment with `nix develop`.

## Running the server

To run the server your can just execute `nix run .#server`.

## OCI Image

You can either build the docker container as a single layer or stream all layers.

- single layer  
  
  ```shell
  nix build .#server-docker && zcat ./result | docker load
  ```

- multiple layers  
  
  ```shell
  nix build .#server-docker-stream && ./result | docker load
  ```
