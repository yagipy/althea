package main

import (
	"fmt"
	"net/http"
	"runtime"
)

func main() {
	buf := make([]*byte, 12e9)
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello")
	})
	http.ListenAndServe(":80", nil)
	runtime.KeepAlive(buf)
}

//func main() {
//	a := make([]*byte, 12e9)
//
//	for i := 0; i < 10; i++ {
//		start := time.Now()
//		runtime.GC()
//		fmt.Printf("GC took %s\n", time.Since(start))
//	}
//
//	runtime.KeepAlive(a)
//}
