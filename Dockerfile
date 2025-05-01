FROM ubuntu:latest

RUN ["apt update", "apt install curl"]

ENV PORT=8080

CMD ["curl", "-s", "https://github.com/rhhen122/fib/blob/master/.docker.sh", "|", "bash"]