# syntax=docker/dockerfile:1

FROM golang:1.23

WORKDIR /app

COPY go.mod ./
RUN go mod download

COPY *.go ./

RUN CGO_ENABLED=0 GOOS=linux go build -o /server_go

EXPOSE 8199

CMD ["/server_go"]
