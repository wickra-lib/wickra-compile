// Wickra Compile — C++ wrapper over the C ABI.
//
// Header-only, C++17, no dependency beyond the standard library and
// `wickra_compile.h` beside it. Link the same `wickra_compile` library the C
// binding does.
//
// What it adds over calling the C functions directly is the handling nobody
// wants to write twice: the handle is owned and freed, the two-call length
// protocol behind `wickra_compile_command` is done for you, the byte reader
// behind `wickra_compile_artifact_read` returns a vector, and a failure comes
// back as an exception rather than a negative integer a caller can ignore.
//
//     #include <wickra_compile.hpp>
//
//     wickra::Compiler compiler;
//     std::string artifact = compiler.command(R"({"cmd":"compile","dry_run":true,"spec":{...}})");
//
// The compiler is data-driven, so this wrapper deliberately stops at strings:
// the spec and the manifest are JSON, and which JSON library a caller uses is
// their choice, not this header's.

#ifndef WICKRA_COMPILE_HPP
#define WICKRA_COMPILE_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "wickra_compile.h"

namespace wickra {

/// Thrown when the library rejects a command or a byte handle.
class CompileError : public std::runtime_error {
 public:
  explicit CompileError(const std::string& what) : std::runtime_error(what) {}
};

/// An owning handle to a compiler.
///
/// Move-only, because the underlying handle is a unique resource: copying it
/// would free the same pointer twice.
class Compiler {
 public:
  /// Create a compiler. Throws `CompileError` if the library refuses.
  Compiler() : handle_(wickra_compile_new()) {
    if (handle_ == nullptr) {
      throw CompileError("wickra_compile_new returned null");
    }
  }

  ~Compiler() { wickra_compile_free(handle_); }

  Compiler(const Compiler&) = delete;
  Compiler& operator=(const Compiler&) = delete;

  Compiler(Compiler&& other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }

  Compiler& operator=(Compiler&& other) noexcept {
    if (this != &other) {
      wickra_compile_free(handle_);
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }

  /// Apply a command JSON and return the response JSON.
  ///
  /// The C entry point writes into a caller buffer and reports the length it
  /// needed, so this asks for the length first and then reads; the hub caches
  /// the response of a mutating command between the two calls, so a real
  /// `compile` runs once, not twice. A command the library understands but
  /// cannot carry out answers in band with `{"ok":false,"error":...}`; a
  /// negative return is a failure of the call itself and becomes an exception.
  std::string command(const std::string& cmd_json) {
    const std::int32_t needed =
        wickra_compile_command(handle_, cmd_json.c_str(), nullptr, 0);
    if (needed < 0) {
      throw CompileError("wickra_compile_command failed with code " +
                         std::to_string(needed));
    }

    std::string out(static_cast<std::size_t>(needed), '\0');
    // The C side writes a trailing NUL, so the buffer has to hold one more byte
    // than the response itself.
    const std::int32_t written = wickra_compile_command(
        handle_, cmd_json.c_str(), out.data(),
        static_cast<std::uintptr_t>(out.size()) + 1);
    if (written < 0) {
      throw CompileError("wickra_compile_command failed with code " +
                         std::to_string(written));
    }
    if (written != needed) {
      // The response changed length between the two calls, which cannot happen
      // for a handle only this thread is using. Saying so is better than
      // returning a string that is half of one answer and half of another.
      throw CompileError("wickra_compile_command length changed between calls");
    }
    return out;
  }

  /// The bytes behind a handle an `artifact_bytes` command opened.
  ///
  /// `len` is the `len` field of that command's response, so the buffer is
  /// sized once and the copy happens on the first call.
  std::vector<std::uint8_t> artifact_bytes(std::uint64_t byte_handle, std::size_t len) {
    std::vector<std::uint8_t> out(len);
    const std::int64_t got = wickra_compile_artifact_read(
        handle_, byte_handle, out.empty() ? nullptr : out.data(),
        static_cast<std::uintptr_t>(out.size()));
    if (got < 0) {
      throw CompileError("wickra_compile_artifact_read failed with code " +
                         std::to_string(got));
    }
    if (static_cast<std::size_t>(got) != len) {
      throw CompileError("wickra_compile_artifact_read: the handle holds " +
                         std::to_string(got) + " bytes, not " + std::to_string(len));
    }
    return out;
  }

  /// The library version.
  static std::string version() { return std::string(wickra_compile_version()); }

 private:
  WickraCompiler* handle_;
};

}  // namespace wickra

#endif  // WICKRA_COMPILE_HPP
