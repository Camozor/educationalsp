default:
  @just --list

build:
  cargo build

watch:
  watchexec just build
