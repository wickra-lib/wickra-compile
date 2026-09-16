# Wickra Compile examples — Java

Runnable Java examples for the [Wickra Compile Java binding](../../bindings/java). The binding reaches the C ABI
through the Foreign Function & Memory API (JDK 22+), so build the library once
and point the JVM at it with `-Dnative.lib.dir`:

```bash
cargo build -p wickra-compile-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q package -DskipTests
javac -cp bindings/java/target/classes examples/java/Compile.java -d examples/java/out
java --enable-native-access=ALL-UNNAMED  -Dnative.lib.dir="$PWD/target/release"  -cp "bindings/java/target/classes:examples/java/out" Compile
```

## The examples

| Example | What it does |
|---------|--------------|
| `Compile.java` | A runnable Java example: compile a strategy spec (dry run) through the binding. |
