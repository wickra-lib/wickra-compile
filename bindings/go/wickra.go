// Package wickra provides idiomatic Go bindings for wickra-compile over its C
// ABI hub: build a Compiler, drive it with command JSON (compile, targets,
// version, artifact_bytes, reset) and read back the response JSON — the same
// protocol as the CLI and every other binding, yielding the byte-identical
// manifest.
//
// The binding links the prebuilt C ABI library, staged per platform under
// ./lib/<goos>_<goarch>/, with the header vendored under ./include.
package wickra

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/lib/linux_amd64 -lwickra_compile -Wl,-rpath,${SRCDIR}/lib/linux_amd64
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/lib/linux_arm64 -lwickra_compile -Wl,-rpath,${SRCDIR}/lib/linux_arm64
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/lib/darwin_amd64 -lwickra_compile -Wl,-rpath,${SRCDIR}/lib/darwin_amd64
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/lib/darwin_arm64 -lwickra_compile -Wl,-rpath,${SRCDIR}/lib/darwin_arm64
#cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/windows_amd64 -l:wickra_compile.dll
#cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/windows_arm64 -l:wickra_compile.dll
#include <stdlib.h>
#include "wickra_compile.h"
*/
import "C"

import (
	"encoding/json"
	"fmt"
	"runtime"
	"unsafe"
)

// Compiler compiles a Wickra strategy spec into a standalone deployable and a
// deterministic manifest, driven by JSON commands.
type Compiler struct {
	handle *C.WickraCompiler
}

// New builds a compiler handle. Call Close when done (a finalizer also frees it,
// but explicit Close is preferred).
func New() *Compiler {
	handle := C.wickra_compile_new()
	c := &Compiler{handle: handle}
	runtime.SetFinalizer(c, (*Compiler).Close)
	return c
}

// Command applies a command JSON and returns the response JSON. It uses the C
// ABI's length-out protocol: a first call learns the length, then the response
// is read into a caller-owned buffer. Domain errors are reported in-band as
// {"ok":false,"error":...} JSON, not as a Go error.
func (c *Compiler) Command(cmdJSON string) (string, error) {
	ccmd := C.CString(cmdJSON)
	defer C.free(unsafe.Pointer(ccmd))

	n := C.wickra_compile_command(c.handle, ccmd, nil, 0)
	if n < 0 {
		return "", fmt.Errorf("wickra-compile: command failed (code %d)", int(n))
	}
	buf := make([]byte, int(n)+1)
	C.wickra_compile_command(
		c.handle,
		ccmd,
		(*C.char)(unsafe.Pointer(&buf[0])),
		C.uintptr_t(len(buf)),
	)
	return string(buf[:n]), nil
}

// ArtifactBytes reads the raw bytes of a file through the C ABI. It issues an
// artifact_bytes command to register the file's bytes under a handle, then
// copies them out via the byte reader. Returns an error if the file cannot be
// read.
func (c *Compiler) ArtifactBytes(path string) ([]byte, error) {
	cmd, err := json.Marshal(map[string]string{"cmd": "artifact_bytes", "path": path})
	if err != nil {
		return nil, err
	}
	resp, err := c.Command(string(cmd))
	if err != nil {
		return nil, err
	}
	var meta struct {
		Handle uint64 `json:"handle"`
		Len    int    `json:"len"`
		Error  string `json:"error"`
	}
	if err := json.Unmarshal([]byte(resp), &meta); err != nil {
		return nil, err
	}
	if meta.Error != "" {
		return nil, fmt.Errorf("wickra-compile: %s", meta.Error)
	}
	if meta.Len == 0 {
		return []byte{}, nil
	}
	buf := make([]byte, meta.Len)
	n := C.wickra_compile_artifact_read(
		c.handle,
		C.uint64_t(meta.Handle),
		(*C.uint8_t)(unsafe.Pointer(&buf[0])),
		C.uintptr_t(len(buf)),
	)
	if n < 0 {
		return nil, fmt.Errorf("wickra-compile: artifact read failed (code %d)", int(n))
	}
	return buf[:int(n)], nil
}

// Close frees the compiler handle. Safe to call more than once.
func (c *Compiler) Close() {
	if c.handle != nil {
		C.wickra_compile_free(c.handle)
		c.handle = nil
	}
	runtime.SetFinalizer(c, nil)
}

// Version returns the library version.
func Version() string {
	return C.GoString(C.wickra_compile_version())
}
