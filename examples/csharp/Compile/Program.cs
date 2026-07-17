// A runnable C# example: compile a strategy spec (dry run) and print its
// deterministic manifest.
//
//   dotnet run --project examples/csharp/Compile
//
// Every language example uses the same spec and prints the same project_hash.
using System.Text.Json;
using Wickra.Compile;

const string spec = """
    {"strategy":{"symbol":"btcusdt","timeframe":"1h",
     "indicators":{"fast":{"type":"Sma","params":[10]},"slow":{"type":"Sma","params":[30]}},
     "entry":{"cross_above":["fast","slow"]},"exit":{"cross_below":["fast","slow"]},
     "sizing":{"type":"fixed_qty","qty":1}},"target":{"kind":"wasm"},"crate_name":"demo"}
    """;

using var compiler = new Compiler();
var raw = compiler.Command($$"""{"cmd":"compile","dry_run":true,"spec":{{spec}}}""");
using var doc = JsonDocument.Parse(raw);
var manifest = doc.RootElement.GetProperty("manifest");

Console.WriteLine($"wickra-compile {Compiler.Version()}");
Console.WriteLine($"crate: {manifest.GetProperty("crate_name").GetString()}");
Console.WriteLine($"files: {manifest.GetProperty("files").GetArrayLength()}");
Console.WriteLine($"project_hash: {manifest.GetProperty("project_hash").GetString()}");
