package wickra

// Operating-mode equivalence through the binding: the manifest a dry run
// describes is the manifest a real build builds. `compile` runs two ways --
// `dry_run: true` stops after codegen and returns the manifest, `dry_run:
// false` writes the project and invokes `cargo` on it -- and both must carry
// the same manifest, with `built` and `path` the only difference. The no_std
// spec is the one built for real: its generated project has no dependencies,
// so it compiles in seconds, and it needs only the `thumbv7em-none-eabihf`
// target the CI jobs install. The core pins this in Rust (operating_modes.rs);
// this checks the boundary the Go binding crosses. A missing corpus is a
// failure, not a skip.

import (
	"bytes"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type artifact struct {
	Built    bool            `json:"built"`
	Path     string          `json:"path"`
	Manifest json.RawMessage `json:"manifest"`
}

// canonical re-marshals a JSON value so two documents compare by content.
func canonical(t *testing.T, raw []byte) []byte {
	t.Helper()
	var v any
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatalf("unmarshal: %v (%s)", err, raw)
	}
	out, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return out
}

func TestARealBuildCarriesTheDryRunManifest(t *testing.T) {
	golden := filepath.Join("..", "..", "golden")
	spec, err := os.ReadFile(filepath.Join(golden, "specs", "no_std_blink.json"))
	if err != nil {
		t.Fatal("golden corpus not found: ", err)
	}
	expected, err := os.ReadFile(filepath.Join(golden, "expected", "no_std_blink.json"))
	if err != nil {
		t.Fatal(err)
	}

	c := New()
	defer c.Close()
	dryRaw, err := c.Command(`{"cmd":"compile","dry_run":true,"spec":` + string(spec) + `}`)
	if err != nil {
		t.Fatal(err)
	}
	var dry artifact
	if err := json.Unmarshal([]byte(dryRaw), &dry); err != nil {
		t.Fatalf("unmarshal dry run: %v (%s)", err, dryRaw)
	}
	if dry.Built || dry.Path != "" {
		t.Fatalf("a dry run must not build: %s", dryRaw)
	}
	if !bytes.Equal(canonical(t, dry.Manifest), canonical(t, expected)) {
		t.Fatal("the dry-run manifest does not match the blessed golden")
	}

	outDir := t.TempDir()
	outJSON, _ := json.Marshal(strings.ReplaceAll(outDir, `\`, "/"))
	builtRaw, err := c.Command(`{"cmd":"compile","dry_run":false,"out_dir":` + string(outJSON) + `,"spec":` + string(spec) + `}`)
	if err != nil {
		t.Fatal(err)
	}
	var built artifact
	if err := json.Unmarshal([]byte(builtRaw), &built); err != nil {
		t.Fatalf("unmarshal build: %v (%s)", err, builtRaw)
	}
	if !built.Built {
		t.Fatalf("the build did not run: %s", builtRaw)
	}
	if _, err := os.Stat(built.Path); err != nil {
		t.Fatalf("artifact missing at %s: %v", built.Path, err)
	}
	if !bytes.Equal(canonical(t, built.Manifest), canonical(t, dry.Manifest)) {
		t.Fatal("the built manifest differs from the dry-run manifest")
	}
}
