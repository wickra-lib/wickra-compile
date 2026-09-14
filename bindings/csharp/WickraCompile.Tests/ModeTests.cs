using System.IO;
using System.Text.Json;
using System.Text.Json.Nodes;
using Wickra.Compile;
using Xunit;

namespace WickraCompile.Tests;

// Operating-mode equivalence through the binding: the manifest a dry run
// describes is the manifest a real build builds. `compile` runs two ways --
// `dry_run: true` stops after codegen and returns the manifest, `dry_run:
// false` writes the project and invokes `cargo` on it -- and both must carry
// the same manifest, with `built` and `path` the only difference. The no_std
// spec is the one built for real: its generated project has no dependencies,
// so it compiles in seconds, and it needs only the `thumbv7em-none-eabihf`
// target the CI jobs install. The core pins this in Rust (operating_modes.rs);
// this checks the boundary the C# binding crosses. A missing corpus is a
// failure, not a skip.
public class ModeTests
{
    [Fact]
    public void ARealBuild_CarriesTheDryRunManifest()
    {
        string golden = GoldenTests.GoldenDir();
        string spec = File.ReadAllText(Path.Combine(golden, "specs", "no_std_blink.json"));
        JsonNode expected = JsonNode.Parse(File.ReadAllText(Path.Combine(golden, "expected", "no_std_blink.json")))!;

        using var compiler = new Compiler();
        JsonNode dry = JsonNode.Parse(compiler.Command($"{{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":{spec}}}"))!;
        Assert.False(dry["built"]!.GetValue<bool>());
        Assert.Null(dry["path"]);
        Assert.True(JsonNode.DeepEquals(expected, dry["manifest"]), "the dry-run manifest does not match the blessed golden");

        string outDir = Path.Combine(Path.GetTempPath(), $"wickra-compile-modes-{Environment.ProcessId}");
        Directory.CreateDirectory(outDir);
        try
        {
            string outJson = JsonSerializer.Serialize(outDir.Replace('\\', '/'));
            JsonNode built = JsonNode.Parse(compiler.Command($"{{\"cmd\":\"compile\",\"dry_run\":false,\"out_dir\":{outJson},\"spec\":{spec}}}"))!;
            Assert.True(built["built"]?.GetValue<bool>() == true, built.ToJsonString());
            string artifact = built["path"]!.GetValue<string>();
            Assert.True(File.Exists(artifact), $"artifact missing at {artifact}");
            Assert.True(JsonNode.DeepEquals(dry["manifest"], built["manifest"]), "the built manifest differs from the dry-run manifest");
        }
        finally
        {
            Directory.Delete(outDir, recursive: true);
        }
    }
}
