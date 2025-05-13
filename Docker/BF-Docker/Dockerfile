FROM git:latest
FROM golang:latest

WORKDIR /

RUN git clone https://github.com/kgabis/brainfuck-go.git /bf/
COPY fib.bf /bf/

CMD ["go", "run", "/bf/bf.go", "/bf/fib.bf"]