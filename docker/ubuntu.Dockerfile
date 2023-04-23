FROM ubuntu:20.04

WORKDIR /app
COPY ./scripts/install_python.sh /app
RUN chmod +x /app/install_python.sh
RUN /app/install_python.sh

CMD ["/lib/systemd/systemd"]
