using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Wickra.Compile;

/// <summary>Raw P/Invoke surface for the wickra-compile C ABI.</summary>
internal static partial class Native
{
    internal const string Lib = "wickra_compile";

    /// <summary>Construct a compiler handle. Never null except on allocation failure.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_compile_new();

    /// <summary>Free a compiler handle.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial void wickra_compile_free(IntPtr handle);

    /// <summary>
    /// Apply a command JSON (NUL-terminated UTF-8), writing the response into a
    /// caller-owned buffer. Returns the response length, or a negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial int wickra_compile_command(IntPtr handle, byte[] cmdUtf8, byte[]? outBuf, nuint cap);

    /// <summary>
    /// Copy the bytes of an open artifact handle (from an <c>artifact_bytes</c>
    /// response) into a caller-owned buffer. Returns the total length, or a
    /// negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial long wickra_compile_artifact_read(IntPtr handle, ulong byteHandle, byte[]? outBuf, nuint cap);

    /// <summary>The library version as a static NUL-terminated string.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_compile_version();
}
