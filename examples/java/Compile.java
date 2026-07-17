// A runnable Java example: compile a strategy spec (dry run) through the binding.
//
//   cargo build -p compile-c
//   mvn -f bindings/java/pom.xml -q package -DskipTests
//   javac -cp bindings/java/target/classes examples/java/Compile.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes;examples/java/out" Compile
//
// Every language example uses the same spec and prints the same project_hash.
import org.wickra.compile.Compiler;

public final class Compile {
    private static final String SPEC =
            "{\"strategy\":{\"symbol\":\"btcusdt\",\"timeframe\":\"1h\","
                    + "\"indicators\":{\"fast\":{\"type\":\"Sma\",\"params\":[10]},"
                    + "\"slow\":{\"type\":\"Sma\",\"params\":[30]}},"
                    + "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
                    + "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
                    + "\"sizing\":{\"type\":\"fixed_qty\",\"qty\":1}},"
                    + "\"target\":{\"kind\":\"wasm\"},\"crate_name\":\"demo\"}";

    public static void main(String[] args) {
        try (Compiler compiler = new Compiler()) {
            String response =
                    compiler.command("{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":" + SPEC + "}");
            System.out.println("wickra-compile " + Compiler.version());
            System.out.println(response);
        }
    }
}
