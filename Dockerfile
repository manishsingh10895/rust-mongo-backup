FROM mongo:4.4.3 as base

RUN apt-get update
RUN apt-get install curl -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . /mongodump

EXPOSE 27017

CMD ["mongod"]

# docker run -d -p 27018:27017 --name must -e MONGO_INITDB_ROOT_USERNAME=manish -e MONGO_INITDB_ROOT_PASSWORD=terminator 1aca2a589c3a