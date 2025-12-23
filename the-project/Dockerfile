FROM golang:alpine

WORKDIR /app
COPY . .

RUN go build ./src/main/main.go
CMD [ "./main" ]
