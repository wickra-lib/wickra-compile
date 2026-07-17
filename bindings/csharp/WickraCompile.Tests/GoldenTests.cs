using Wickra.Compile;
using Xunit;

namespace WickraCompile.Tests;

// The cross-language golden invariant seen from C#: the same spec yields a
// byte-identical manifest across calls, and those bytes are what every other
// binding produces too, because the whole compiler lives once in the Rust core
// and this binding forwards its JSON verbatim.
public class GoldenTests
{
    [Fact]
    public void Compile_IsByteIdenticalAcrossCalls()
    {
        using var a = new Compiler();
        using var b = new Compiler();
        Assert.Equal(a.Command(CompilerTests.CompileCmd), b.Command(CompilerTests.CompileCmd));
    }
}
