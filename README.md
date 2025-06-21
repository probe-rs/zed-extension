# probe-rs debugger extension for Zed

Debugger extension for [Zed](https://zed.dev) which enables the use of [probe-rs](https://probe.rs) as a debugger.

Currently still in development, and not yet published on the Zed Extenion gallery.

## Debugger configuration

A minimal configuration in `.zed/debug.json`:

```json
[
  {
    "label": "Launch probe-rs debugging",
    "adapter": "probe-rs",
    "request": "launch",
    "cwd": "$ZED_WORKTREE_ROOT",
    "server": "127.0.0.1:50000  ",
    "coreConfigs": []
  }
]
```

This will attach to an already running local instance of probe-rs, listening on port 50000.

The configuration options are the same as in the vscode debugger extensions, see the [probe-rs docs](https://probe.rs/docs/tools/debugger/) for more information.


## Development setup

See the Zed documentation on extension development: <https://zed.dev/docs/extensions/developing-extensions>.
