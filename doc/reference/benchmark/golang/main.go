package main

import (
	"fmt"
	"log"
	"net/http"
	_ "net/http/pprof"
	"runtime"
	"runtime/debug"
)

func main() {
	go func() {
		log.Println(http.ListenAndServe("0.0.0.0:6060", nil))
	}()
	println("listening...")
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		bufTmp := make([]*int64, 10_000_000) // 8 byte * 10_000_000 = 80,000,000 byte = 80MB
		runtime.KeepAlive(bufTmp)
		fib(30)
		fmt.Fprintf(w, fmt.Sprintf("success"))
	})
	http.HandleFunc("/gc-stats", func(w http.ResponseWriter, r *http.Request) {
		var gcStats debug.GCStats
		debug.ReadGCStats(&gcStats)
		fmt.Fprintf(w, fmt.Sprintf("%#v", gcStats))
	})
	http.ListenAndServe(":80", nil)
}

func fib(n int) int {
	if n < 2 {
		return n
	}
	return fib(n-2) + fib(n-1)
}
