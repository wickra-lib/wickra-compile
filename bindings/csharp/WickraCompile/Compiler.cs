using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;

namespace Wickra.Compile;

/// <summary>
/// Compiles a Wickra strategy spec into a standalone deployable and a
/// deterministic manifest, driven by JSON commands over the Wickra C ABI.
/// Construct one, drive it with command JSON (<c>compile</c>, <c>targets</c>,
/// <c>version</c>, <c>artifact_bytes</c>, <c>reset</c>) and read back the
/// response JSON — the same protocol as the CLI and every other binding.
/// </summary>
public sealed class Compiler : IDisposable
{
    private readonly CompilerHandle _handle;

    /// <summary>Construct a compiler handle.</summary>
    public Compiler()
    {
        IntPtr ptr = Native.wickra_compile_new();
        if (ptr == IntPtr.Zero)
        {
            throw new InvalidOperationException("wickra-compile: allocation failed");
        }
        _handle = new CompilerHandle(ptr);
    }

    /// <summary>Apply a command JSON and return the response JSON.</summary>
    /// <remarks>
    /// Uses the C ABI's length-out protocol: a first call learns the length, then
    /// the response is read into a caller-owned buffer. Domain errors (a bad
    /// command, an unknown command, an invalid spec) come back in-band as
    /// <c>{"ok":false,...}</c> JSON, not as an exception.
    /// </remarks>
    /// <exception cref="InvalidOperationException">A required argument was unusable or a panic was caught.</exception>
    public string Command(string cmdJson)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid, this);

        byte[] cmd = Utf8(cmdJson);
        IntPtr h = _handle.DangerousGetHandle();
        int n = Native.wickra_compile_command(h, cmd, null, 0);
        if (n < 0)
        {
            throw new InvalidOperationException($"wickra-compile: command failed (code {n})");
        }
        var buf = new byte[n + 1];
        Native.wickra_compile_command(h, cmd, buf, (nuint)buf.Length);
        return Encoding.UTF8.GetString(buf, 0, n);
    }

    /// <summary>Read the raw bytes of a file through the C ABI byte reader.</summary>
    /// <exception cref="InvalidOperationException">The file could not be read.</exception>
    public byte[] ArtifactBytes(string path)
    {
        string cmd = JsonSerializer.Serialize(new Dictionary<string, string>
        {
            ["cmd"] = "artifact_bytes",
            ["path"] = path,
        });
        using JsonDocument doc = JsonDocument.Parse(Command(cmd));
        JsonElement root = doc.RootElement;
        if (root.TryGetProperty("error", out JsonElement err))
        {
            throw new InvalidOperationException($"wickra-compile: {err.GetString()}");
        }
        ulong byteHandle = root.GetProperty("handle").GetUInt64();
        int len = root.GetProperty("len").GetInt32();
        if (len == 0)
        {
            return [];
        }
        var buf = new byte[len];
        IntPtr h = _handle.DangerousGetHandle();
        long n = Native.wickra_compile_artifact_read(h, byteHandle, buf, (nuint)buf.Length);
        if (n < 0)
        {
            throw new InvalidOperationException($"wickra-compile: artifact read failed (code {n})");
        }
        return buf;
    }

    /// <summary>The library version.</summary>
    public static string Version() =>
        Marshal.PtrToStringUTF8(Native.wickra_compile_version()) ?? string.Empty;

    /// <summary>Free the native compiler handle.</summary>
    public void Dispose() => _handle.Dispose();

    /// <summary>Encode a string as NUL-terminated UTF-8 for the C ABI.</summary>
    private static byte[] Utf8(string s)
    {
        int len = Encoding.UTF8.GetByteCount(s);
        var buf = new byte[len + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buf, 0);
        return buf;
    }
}

/// <summary>A safe handle owning a native compiler pointer.</summary>
internal sealed class CompilerHandle : SafeHandle
{
    public CompilerHandle(IntPtr handle)
        : base(IntPtr.Zero, ownsHandle: true) => SetHandle(handle);

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        Native.wickra_compile_free(handle);
        return true;
    }
}
