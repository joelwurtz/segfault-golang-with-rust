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
	localNameNotSegfault := "html"
	localNameSegfault := "segfault"

	localNameNotSegfaultCstr := C.CString(localNameNotSegfault)
	localNameSegfaultCstr := C.CString(localNameSegfault)

	// Uncommenting the next line will remove the segfault
	// C.api_do_segfault(localNameSegfaultCstr)
	signalChannel := make(chan os.Signal)
	signal.Notify(signalChannel, syscall.SIGTERM)

	C.api_do_segfault(localNameNotSegfaultCstr)
	C.api_do_segfault(localNameSegfaultCstr)
}