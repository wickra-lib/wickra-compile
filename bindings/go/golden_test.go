package wickra

// The cross-language golden invariant seen from Go: the same spec yields a
// byte-identical manifest across calls, and every golden spec reproduces the
// exact project hash pinned in golden/expected — those bytes are what every
// binding produces, because the whole compiler lives once in the Rust core and
// this binding forwards its JSON verbatim.

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

func TestCompileByteIdenticalAcrossCalls(t *testing.T) {
	a := New()
	defer a.Close()
	b := New()
	defer b.Close()

	ra, err := a.Command(compileCmd)
	if err != nil {
		t.Fatal(err)
	}
	rb, err := b.Command(compileCmd)
	if err != nil {
		t.Fatal(err)
	}
	if ra != rb {
		t.Fatalf("expected byte-identical output, got:\n a: %s\n b: %s", ra, rb)
	}
}

// binary_daemon embeds a CSV whose path is resolved relative to the working
// directory, so it is covered by the Rust golden (which controls the cwd) rather
// than here.
var goldenSpecs = []string{"sma_cross", "ema_trend", "rsi_reversion", "no_std_blink"}

func TestGoldenSpecsReproduceExpectedProjectHash(t *testing.T) {
	golden := filepath.Join("..", "..", "golden")
	c := New()
	defer c.Close()

	for _, name := range goldenSpecs {
		spec, err := os.ReadFile(filepath.Join(golden, "specs", name+".json"))
		if err != nil {
			t.Fatal(err)
		}
		expectedRaw, err := os.ReadFile(filepath.Join(golden, "expected", name+".json"))
		if err != nil {
			t.Fatal(err)
		}
		var expected struct {
			ProjectHash string `json:"project_hash"`
		}
		if err := json.Unmarshal(expectedRaw, &expected); err != nil {
			t.Fatal(err)
		}

		cmd := `{"cmd":"compile","dry_run":true,"spec":` + string(spec) + `}`
		resp, err := c.Command(cmd)
		if err != nil {
			t.Fatal(err)
		}
		var got struct {
			Manifest struct {
				ProjectHash string `json:"project_hash"`
			} `json:"manifest"`
		}
		if err := json.Unmarshal([]byte(resp), &got); err != nil {
			t.Fatalf("unmarshal %s response: %v (%s)", name, err, resp)
		}
		if got.Manifest.ProjectHash != expected.ProjectHash {
			t.Fatalf("project_hash mismatch for %s: got %s, want %s",
				name, got.Manifest.ProjectHash, expected.ProjectHash)
		}
	}
}
