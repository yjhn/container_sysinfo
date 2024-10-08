package main

import (
	"fmt"
	"io"
	"net/http"
)

const rustServerAddr = "http://server_rs:8198/"

func getInfo(w http.ResponseWriter, r *http.Request) {
	res, err := http.Get(rustServerAddr)
	if err != nil {
		fmt.Printf("error making request to %s:\n%s", rustServerAddr, err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	info, err2 := io.ReadAll(res.Body)
	if err2 != nil {
		fmt.Printf("error reading response from %s:\n%s", rustServerAddr, err2)
		w.WriteHeader(http.StatusInternalServerError)
		return

	}
	io.WriteString(w, string(info))
}

func main() {
	http.HandleFunc("/", getInfo)
	http.ListenAndServe(":8199", nil)
}
