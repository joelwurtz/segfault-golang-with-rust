package main

/*
#cgo LDFLAGS: -lsegfaulthtml5evergolang -Ltarget/x86_64-unknown-linux-musl/debug
#include <stdlib.h>
#include "target/segfaulthtml5evergolang.h"
*/
import "C"
import (
	"os"
	"os/signal"
	"syscall"
)

func main() {
	html := "<html><head><title>yolo</title><meta property=\"og:title\" content=\"yolo\" /></head>"
	htmlCstr := C.CString(html)

	// Uncommenting the next line will remove the segfault
	//C.html5ever_parse_data(htmlCstr)
	signalChannel := make(chan os.Signal)
	signal.Notify(signalChannel, syscall.SIGTERM)

	C.html5ever_parse_data(htmlCstr)
}