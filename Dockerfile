FROM debian:buster

COPY ./target/release/pizza-platz-farm /usr/local/bin/

CMD ["/usr/local/bin/pizza-platz-farm"]
