# Azure client library test utilities

The types and functions in this crate help test client libraries built on `azure_core`, including the `#[recorded::test]` attribute.

We use integration tests - tests defined under a crate's `tests/` directory - for testing against provisioned resources.
Most crates use recorded tests to record (and sanitize) or play back HTTP traffic during test execution by attributing tests with `#[recorded::test]`.

If your crate does not communicate over HTTP or provisioning resources cannot be fully automated, you can also mark tests as `#[recorded::test(live)]`.
This still provides utility such as reading environment variables or other test context that might be helpful when your tests run.

## Prerequisites

- [Test Proxy]
- (Recommended) [PowerShell] to easily provision resources

## Client methods

To test client methods using our [Test Proxy] or run against live resources, you can attribute asynchronous tests
using the `#[recorded::test]` attribute:

```rust no_run
use azure_core::Result;
use azure_core_test::{recorded, TestContext};
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

#[recorded::test]
async fn get_secret(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    // Instrument your client options to hook up the test-proxy pipeline policy.
    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Get variables, credentials, and pass the instrumented client options to the client to begin.
    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    todo!()
}
```

The `TestContext` parameter is required unless your test function is attributed as `#[recorded::test(live)]` (live-only).
You can name the parameter whatever you want.
The `TestContext` parameter is used to initialize an HTTP client to play back or record tests
and provides other information to test functions that may be useful.

These tests must also return a `std::result::Result<T, E>`, which can be redefined e.g., `azure_core::Result<T>`.

### Recording features

Besides instrumenting your client and getting credentials - real credentials when running live or recording,
but mock credentials when playing back - there are a number of other helpful features on the `Recording` object returned above:

- `add_sanitizer` will add custom sanitizers. There are many pre-configured by the [Test Proxy] as well.
- `remove_sanitizers` will remove named sanitizers, like `AZSDK3430` that sanitizes all `$..id` fields and may cause playback to fail.
- `add_matcher` adds a custom matcher to match headers, path segments, and or body content.
- `random` gets random data (numbers, arrays, etc.) that is initialized from the OS when running live or recording,
  but the seed is saved with the recording and used during play back so that sequential generation of random data is deterministic.
  ChaCha20 is used to provide a deterministic, portable sequence of seeded random data.
- `var` gets a required variable with optional `ValueOptions` you can use to sanitize values.
  This function will err if the variable is not set in the environment when running live or recording, or available when playing back.
- `var_opt` gets optional variables and will not err in the aforementioned cases.

## Record tests

Like with all our other Azure SDK languages, we use a common system for provisioning resources named [Test Resources].
This uses a `test-resources.json` or `test-resources.bicep` file in the crate or service directory.

When you run this, it will output some environment variables you can set in your shell, or pass on the command line if your shell supports it.

```bash
./eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory keyvault
```

In this example, it will output a number of environment variables including `AZURE_KEYVAULT_URL`. We'll pass that in the command line for bash below.

Before you can record, though, you need to make sure you're authenticated. The script above will authenticate you in PowerShell, but the
`AzurePowerShellCredential` is not yet implemented; however, the `AzureCliCredential` is so log in with `az`:

```bash
az login
```

No you can record your tests and, after they pass successfully, check in the test recordings. You need to pass `AZURE_TEST_MODE=record` to record,
along with any other environment variables output by the [Test Resources] scripts.

```bash
AZURE_TEST_MODE=record AZURE_KEYVAULT_URL=https://my-vault.vault.azure.net cargo test -p azure_security_keyvault_secrets
test-proxy push -a sdk/keyvault/assets.json
```

Once your assets are checked in by [Test Proxy], you can commit your local changes and create a PR.

## Playing back tests

To play back tests, just run `cargo test`:

```bash
cargo test
```

If you get errors, they could indicate regressions in your tests or perhaps variables or random data wasn't saved correctly.
Review any data you generate or use not coming from the service.

## Troubleshooting

Like all Azure SDK client libraries, the `azure_core_test` crate writes information with the target rooted in the crate name
followed by any modules in which the span was created e.g., `azure_core_test::proxy`.
Because this crate is used to test Azure SDK client libraries and developers will most likely want to see fewer traces from this crate,
the log levels used are a level lower than typical client libraries:

- `INFO` includes only important information e.g., the version of `test-proxy` and on what port it's listening.
- `DEBUG` includes when a test recording or playback has started and stopped.
- `TRACE` includes details like communications with the `test-proxy` itself.

The `azure_core_test` crate also traces useful information that the `test-proxy` process itself writes to `stdout` using the `test-proxy` target and the `TRACE` logging level.

You can configure tracing using the [`RUST_LOG`](https://docs.rs/env_logger) environment variable.
For example, if you wanted to see debug information from all sources by default but see `test-proxy` messages written to `stdout` you'd run:

```bash
RUST_LOG=debug,test-proxy=trace cargo test
```

ANSI colors are written to the terminal even if redirected to a file. To disable writing ANSI color sequences, pass `NO_COLOR=1`:

```bash
NO_COLOR=1 RUST_LOG=debug,test-proxy=trace cargo test
```

[PowerShell]: https://learn.microsoft.com/powershell/scripting/install/installing-powershell
[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md
[Test Resources]: https://github.com/Azure/azure-sdk-tools/blob/main/eng/common/TestResources/README.md
