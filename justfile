default:
  @just --list

build:
  cargo build

watch:
  watchexec just build

nvim:
  XDG_CONFIG_HOME=. nvim example/example.md

