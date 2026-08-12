# probe-rs debugger extension for Zed

Debugger extension for [Zed](https://zed.dev) which enables the use of [probe-rs](https://probe.rs) as a debugger.

Currently still in development.

## Debugger configuration

A minimal configuration in `.zed/debug.json`:

```json
[
  {
    "label": "Launch probe-rs debugging",
    "adapter": "probe-rs",
    "request": "launch",
    "cwd": "$ZED_WORKTREE_ROOT",
    "server": "127.0.0.1:50000",
    "coreConfigs": []
  }
]
```

This will attach to an already running local instance of probe-rs, listening on port 50000.

The configuration options are the same as in the vscode debugger extensions, see the [probe-rs docs](https://probe.rs/docs/tools/debugger/) for more information.

## Minimal debugging walkthrough

The following workflow starts the probe-rs Debug Adapter Protocol (DAP) server and connects Zed to it. It assumes that the project already builds for the target board and that a compatible probe is connected.

1. Build the firmware for the board target, for example:

   ```bash
   cargo build --target thumbv7em-none-eabihf
   ```

2. Start the probe-rs DAP server from the project directory:

   ```bash
   probe-rs dap-server --port 50000
   ```

   Keep this process running while debugging. If port `50000` is already in use, choose another port and use the same value in both commands and `.zed/debug.json`.

3. Install and enable the probe-rs extension in Zed.

4. Create `.zed/debug.json` in the project. A configuration that flashes the freshly built binary is:

   ```json
   [
     {
       "label": "Debug firmware with probe-rs",
       "adapter": "probe-rs",
       "request": "launch",
       "cwd": "$ZED_WORKTREE_ROOT",
       "server": "127.0.0.1:50000",
       "flashingConfig": {
         "flashingEnabled": true,
         "haltAfterReset": true
       },
       "coreConfigs": [
         {
           "programBinary": "$ZED_WORKTREE_ROOT/target/thumbv7em-none-eabihf/debug/your-binary"
         }
       ]
     }
   ]
   ```

   Replace `thumbv7em-none-eabihf` and `your-binary` with the target triple and binary produced by the project. If the firmware is already loaded, the `flashingConfig` and `programBinary` entries can be omitted.

5. In Zed, choose **Run → Start Debugger** (or press `F4`) and select **Debug firmware with probe-rs**. Breakpoints, stepping, and target registers are then provided by the probe-rs DAP connection.

6. After changing the firmware, rebuild it and restart the debugger so the new binary is loaded.


## Development setup

See the Zed documentation on extension development: <https://zed.dev/docs/extensions/developing-extensions>.
