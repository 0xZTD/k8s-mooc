package main

import (
	"log"
	"net/http"
	"os"
)

func main() {
	port := os.Getenv("PORT")
	port = ":" + port

	log.Printf("Server started in port %v", port)
	http.ListenAndServe(port, nil)
}
