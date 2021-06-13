FROM mongo:4.4.3 as base

RUN apt-get update
RUN apt-get install curl -y
RUN apt-get install gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

VOLUME [ "/data" ]

EXPOSE 27017

CMD ["mongod"]

# docker run -d -p 27018:27017 -v C:\Users\manis\Documents\practice\rust\mongodump:/dumper --name must -e MONGO_INITDB_ROOT_USERNAME=manish -e MONGO_INITDB_ROOT_PASSWORD=terminator b6fa32cbce59