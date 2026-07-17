package wickra

import (
	"encoding/json"
	"os"
	"strings"
	"testing"
)

// A dry-run compile: pure codegen + manifest, no toolchain. The strategy is a
// valid wickra_backtest::StrategySpec so the compiler accepts it.
const compileCmd = `{"cmd":"compile","dry_run":true,"spec":{` +
	`"strategy":{"symbol":"x","timeframe":"1h",` +
	`"indicators":{"f":{"type":"Ema","params":[3]}},` +
	`"entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},` +
	`"sizing":{"type":"fixed_qty","qty":1}},` +
	`"target":{"kind":"wasm"},"crate_name":"demo"}}`

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestCompileDryRunReturnsManifest(t *testing.T) {
	c := New()
	defer c.Close()
	raw, err := c.Command(compileCmd)
	if err != nil {
		t.Fatal(err)
	}
	var out struct {
		Built    bool `json:"built"`
		Manifest struct {
			ProjectHash string `json:"project_hash"`
		} `json:"manifest"`
	}
	if err := json.Unmarshal([]byte(raw), &out); err != nil {
		t.Fatalf("unmarshal: %v (%s)", err, raw)
	}
	if out.Built {
		t.Fatal("expected built=false for a dry run")
	}
	if out.Manifest.ProjectHash == "" {
		t.Fatalf("expected a project_hash, got: %s", raw)
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	c := New()
	defer c.Close()
	raw, err := c.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}

func TestArtifactBytesReadsAFile(t *testing.T) {
	c := New()
	defer c.Close()
	want, err := os.ReadFile("go.mod")
	if err != nil {
		t.Fatal(err)
	}
	got, err := c.ArtifactBytes("go.mod")
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != string(want) {
		t.Fatalf("bytes mismatch: got %d bytes, want %d", len(got), len(want))
	}
}

func TestArtifactBytesMissingFileIsError(t *testing.T) {
	c := New()
	defer c.Close()
	if _, err := c.ArtifactBytes("does-not-exist.bin"); err == nil {
		t.Fatal("expected an error for a missing file")
	}
}
