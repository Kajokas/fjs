package main

import (
    "net/http"
)

func serve_page(path string, folder string) {
    http.Handle(path, http.FileServer(http.Dir(folder)))
}

func main() {
    @@GENERATE_CODE
}
