use zed_extension_api as zed;

struct ProbeRsDebugger {
    // ... state
}

impl zed::Extension for ProbeRsDebugger {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    // ...
}

zed::register_extension!(ProbeRsDebugger);
