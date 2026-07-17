using System.Text.Json;
using Wickra.Compile;
using Xunit;

namespace WickraCompile.Tests;

public class CompilerTests
{
    // A dry-run compile: pure codegen + manifest, no toolchain. The strategy is a
    // valid wickra_backtest::StrategySpec so the compiler accepts it.
    internal const string CompileCmd =
        "{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":{" +
        "\"strategy\":{\"symbol\":\"x\",\"timeframe\":\"1h\"," +
        "\"indicators\":{\"f\":{\"type\":\"Ema\",\"params\":[3]}}," +
        "\"entry\":{\"cross_above\":[\"f\",\"f\"]},\"exit\":{\"cross_below\":[\"f\",\"f\"]}," +
        "\"sizing\":{\"type\":\"fixed_qty\",\"qty\":1}}," +
        "\"target\":{\"kind\":\"wasm\"},\"crate_name\":\"demo\"}}";

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Compiler.Version()));
    }

    [Fact]
    public void Compile_DryRun_ReturnsManifest()
    {
        using var compiler = new Compiler();
        JsonElement outp = JsonDocument.Parse(compiler.Command(CompileCmd)).RootElement;

        Assert.False(outp.GetProperty("built").GetBoolean());
        string hash = outp.GetProperty("manifest").GetProperty("project_hash").GetString()!;
        Assert.False(string.IsNullOrEmpty(hash));
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var compiler = new Compiler();
        // The C ABI hub folds a domain error into {"ok":false,...} JSON, so an
        // unknown command surfaces in-band rather than as an exception.
        string raw = compiler.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }

    [Fact]
    public void ArtifactBytes_ReadsAFile()
    {
        string path = Path.GetTempFileName();
        byte[] want = [1, 2, 3, 4, 5, 42, 200, 0, 255];
        try
        {
            File.WriteAllBytes(path, want);
            using var compiler = new Compiler();
            byte[] got = compiler.ArtifactBytes(path);
            Assert.Equal(want, got);
        }
        finally
        {
            File.Delete(path);
        }
    }

    [Fact]
    public void ArtifactBytes_MissingFile_Throws()
    {
        using var compiler = new Compiler();
        Assert.Throws<InvalidOperationException>(
            () => compiler.ArtifactBytes("does-not-exist-9e3a.bin"));
    }
}
