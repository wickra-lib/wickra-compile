using System.IO;
using System.Text.Json;
using Wickra.Compile;
using Xunit;

namespace WickraCompile.Tests;

// The cross-language golden invariant seen from C#: the same spec yields a
// byte-identical manifest across calls, and every golden spec reproduces the
// exact project hash pinned in golden/expected — those bytes are what every
// binding produces, because the whole compiler lives once in the Rust core and
// this binding forwards its JSON verbatim.
public class GoldenTests
{
    // binary_daemon embeds a CSV resolved relative to the working directory, so
    // it is covered by the Rust golden, not here.
    private static readonly string[] Specs = ["sma_cross", "ema_trend", "rsi_reversion", "no_std_blink"];

    [Fact]
    public void Compile_IsByteIdenticalAcrossCalls()
    {
        using var a = new Compiler();
        using var b = new Compiler();
        Assert.Equal(a.Command(CompilerTests.CompileCmd), b.Command(CompilerTests.CompileCmd));
    }

    [Fact]
    public void EveryGoldenSpec_ReproducesItsExpectedProjectHash()
    {
        string golden = GoldenDir();
        using var compiler = new Compiler();

        foreach (string name in Specs)
        {
            string spec = File.ReadAllText(Path.Combine(golden, "specs", $"{name}.json"));
            using JsonDocument expectedDoc =
                JsonDocument.Parse(File.ReadAllText(Path.Combine(golden, "expected", $"{name}.json")));
            string expected = expectedDoc.RootElement.GetProperty("project_hash").GetString()!;

            string cmd = $"{{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":{spec}}}";
            using JsonDocument response = JsonDocument.Parse(compiler.Command(cmd));
            string got = response.RootElement.GetProperty("manifest").GetProperty("project_hash").GetString()!;

            Assert.Equal(expected, got);
        }
    }

    // Walk up from the test assembly directory to the repository's `golden/`.
    private static string GoldenDir()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            string candidate = Path.Combine(dir.FullName, "golden");
            if (Directory.Exists(candidate))
            {
                return candidate;
            }
            dir = dir.Parent;
        }
        throw new DirectoryNotFoundException("could not locate the golden/ directory");
    }
}
