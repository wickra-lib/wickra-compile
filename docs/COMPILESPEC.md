# CompileSpec

The compiler's input. A `CompileSpec` is serde-serialisable and accepted as JSON
or TOML by the CLI (`--spec spec.json`), or inline in a command envelope through
any binding.

```json
{
  "strategy": { "...": "an opaque StrategySpec" },
  "target": { "kind": "wasm" },
  "opt_level": "release",
  "embed_data": null,
  "crate_name": "demo"
}
```

## Fields

| Field | Type | Default | Meaning |
|-------|------|---------|---------|
| `strategy` | object | — (required) | The `StrategySpec` to compile. Opaque to the compiler; it is validated by round-tripping through `wickra_backtest::StrategySpec` and embedded as data. |
| `target` | tagged object | — (required) | The artifact to emit. See [TARGETS.md](TARGETS.md). |
| `opt_level` | `"debug"` \| `"release"` \| `"size"` | `"release"` | The optimisation profile of the generated project. |
| `embed_data` | `DatasetRef` \| `null` | `null` | Optional candle data to embed into the artifact. |
| `crate_name` | string \| `null` | derived | The generated crate's name. When omitted it is derived from the strategy `symbol`, sanitised to `^[a-z][a-z0-9_]*$`. |

## `target`

Internally tagged on `kind`:

```json
{ "kind": "wasm" }
{ "kind": "binary" }
{ "kind": "no_std", "mcu": "thumbv7em-none-eabihf" }
```

`no_std` requires an `mcu` triple from the [allowlist](TARGETS.md).

## `embed_data`

Internally tagged on `kind`:

```json
{ "kind": "csv", "path": "candles.csv" }
{ "kind": "inline", "candles": [ { "...": "OHLCV rows" } ] }
```

A `csv` path is resolved relative to the process working directory when the
manifest is produced, so run the CLI from the directory that holds the file.

## The strategy

The `strategy` object is a Wickra `StrategySpec`. A minimal, valid one:

```json
{
  "symbol": "btcusdt",
  "timeframe": "1h",
  "indicators": {
    "fast": { "type": "Sma", "params": [10] },
    "slow": { "type": "Sma", "params": [30] }
  },
  "entry": { "cross_above": ["fast", "slow"] },
  "exit":  { "cross_below": ["fast", "slow"] },
  "sizing": { "type": "fixed_qty", "qty": 1 }
}
```

- **Indicators** are named; each is `{"type":"Sma|Ema|Rsi|Atr","params":[…]}`.
- **Conditions** (`entry` / `exit`) are externally tagged — `{"cross_above":[a,b]}`
  or `{"cross_below":[a,b]}` — where `a` and `b` name defined indicators. The
  reference is checked structurally.
- **Sizing** is `{"type":"fixed_qty","qty":F}` or
  `{"type":"fixed_fraction","fraction":F}`.

## Validation errors

Invalid specs fail loudly (over a binding, in-band as `{"ok":false,"error":…}`):
an unknown indicator reference, a bad crate name, an MCU triple outside the
allowlist, or a strategy that does not round-trip as a `StrategySpec`.

## See also

- [TARGETS.md](TARGETS.md) · [DETERMINISM.md](DETERMINISM.md) ·
  [Cookbook.md](Cookbook.md)
