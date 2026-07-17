// A runnable Go example: compile a strategy spec (dry run) and print its
// deterministic manifest.
//
//   cd examples/go && go run compile.go
//
// Every language example uses the same spec and prints the same project_hash.
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-compile/bindings/go"
)

const spec = `{"strategy":{"symbol":"btcusdt","timeframe":"1h",` +
	`"indicators":{"fast":{"type":"Sma","params":[10]},"slow":{"type":"Sma","params":[30]}},` +
	`"entry":{"cross_above":["fast","slow"]},"exit":{"cross_below":["fast","slow"]},` +
	`"sizing":{"type":"fixed_qty","qty":1}},"target":{"kind":"wasm"},"crate_name":"demo"}`

func main() {
	compiler := wickra.New()
	defer compiler.Close()

	resp, err := compiler.Command(`{"cmd":"compile","dry_run":true,"spec":` + spec + `}`)
	if err != nil {
		panic(err)
	}
	var out struct {
		Manifest struct {
			CrateName   string            `json:"crate_name"`
			Files       []json.RawMessage `json:"files"`
			ProjectHash string            `json:"project_hash"`
		} `json:"manifest"`
	}
	if err := json.Unmarshal([]byte(resp), &out); err != nil {
		panic(err)
	}

	fmt.Printf("wickra-compile %s\n", wickra.Version())
	fmt.Printf("crate: %s\n", out.Manifest.CrateName)
	fmt.Printf("files: %d\n", len(out.Manifest.Files))
	fmt.Printf("project_hash: %s\n", out.Manifest.ProjectHash)
}
