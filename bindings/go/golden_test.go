package wickra

// The cross-language golden invariant seen from Go: the same spec yields a
// byte-identical manifest across calls, and those bytes are what every other
// binding produces too, because the whole compiler lives once in the Rust core
// and this binding forwards its JSON verbatim.

import "testing"

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
