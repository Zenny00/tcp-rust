#!/bin/bash

cargo build -r
ext=$?
if [[ $ext -ne 0 ]]; then
  exit $ext
fi
sudo setcap CAP_NET_ADMIN=eip ./target/release/tcp_socket
./target/release/tcp_socket & # Run the process in the background
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid
