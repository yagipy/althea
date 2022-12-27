package main

import (
	"fmt"
	"github.com/coreos/go-systemd/activation"
	"log"
	"net/http"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello")
	})
	server := &http.Server{
		Handler: mux,
	}

	listener, err := activation.Listeners()
	if err != nil {
		log.Printf("%s\n", err)
		return
	}
	if len(listener) == 0 {
		log.Printf("can't find any listeners\n")
		return
	}

	server.Serve(listener[0])
}
