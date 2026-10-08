help:
  just --list

svr:
  cargo watch -q -c -w "src/services/redjaxk-server" \
    -x "run -p redjaxk-server"
