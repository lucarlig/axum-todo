FROM rust:1.67

WORKDIR /usr/src/todo
COPY . .

RUN cargo install --path .

CMD ["todo"]