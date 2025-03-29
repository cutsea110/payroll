# payroll (Case Study of Uncle Bob)

[![Rust](https://github.com/cutsea110/payroll/actions/workflows/rust.yml/badge.svg)](https://github.com/cutsea110/payroll/actions/workflows/rust.yml)
![Docker Cloud Build Status](https://img.shields.io/docker/pulls/cutsea110/payroll-app)

ref.) [アジャイルソフトウェア開発の奥義 第2版](https://www.amazon.co.jp/dp/4797347783)

This project is implementation for payroll application written in Rust.
The payroll app is described at the book above.

## Usage

```bash
$ cargo run -- -h

Usage: target/debug/payroll-app [options] FILE

Options:
    -h, --help          Print this help menu
    -q, --quiet         Don't output unnecessary information
    -f, --failopen-tx   Transaction failopen
    -s, --soft-landing  Soft landing application
    -c, --chronograph   Print the time taken to execute each transaction
    -r, --repl          Run into REPL mode
```

## Architecture (Dependent Relationship of crates)

```mermaid
graph TD

  payroll-impl --> payroll-domain
  payroll-impl --> payroll-factory

  payroll-factory --> payroll-domain

  dao --> payroll-domain

  abstract-tx --> dao
  abstract-tx --> payroll-domain

  tx-impl --> payroll-impl
  tx-impl --> dao
  tx-impl --> abstract-tx
  tx-impl --> payroll-factory
  tx-impl --> payroll-domain
  tx-impl --> tx-app
  tx-impl --> tx-factory

  tx-app --> payroll-domain
  tx-app --> app

  tx-factory --> payroll-domain
  tx-factory --> tx-app

  text-parser-tx-source --> tx-factory
  text-parser-tx-source --> tx-app
  text-parser-tx-source --> payroll-domain

  hs-db --> payroll-domain
  hs-db --> dao

  payroll-app --> hs-db
  payroll-app --> payroll-impl
  payroll-app --> tx-impl
  payroll-app --> text-parser-tx-source
  payroll-app --> tx-app
  payroll-app --> app
```

## Run on docker

See [Dockerhub cutsea110/payroll-app](https://hub.docker.com/repository/docker/cutsea110/payroll-app/general).

```bash
$ docker run -v ./:/work -it --rm cutsea110/payroll-app:0.1.2 payroll-app -h
Usage: payroll-app [options] FILE

Options:
    -h, --help          Print this help menu
    -q, --quiet         Don't output unnecessary information
    -f, --failopen-tx   Transaction failopen
    -s, --soft-landing  Soft landing application
    -c, --chronograph   Print the time taken to execute each transaction
    -r, --repl          Run into REPL mode
```

## For Developer

### How to build Docker image

You should specify the version 0.1.3, because the latest version is 0.1.2.

```bash
$ docker buildx build --load -t cutsea110/payroll-app:0.1.3 .
```
### How to run on Docker image

I suppose that you have some test programs for payroll-app in `${PWD}/script` directory.

```bash
$ docker run -v ${PWD}/script:/work -it --rm cutsea110/payroll-app:0.1.3 payroll-app /work/test1.scr
```

### Share Dockerhub

```bash
$ docker login
$ docker push cutsea110/payroll-app:0.1.3
```

### Update This README

You should update docker image version for next.
