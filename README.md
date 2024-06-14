# bevy_outline_post_process

![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
![Tag](https://img.shields.io/github/v/tag/exvacuum/bevy_outline_post_process)
![Build](https://img.shields.io/github/actions/workflow/status/exvacuum/bevy_outline_post_process/rust.yml)
[![Docs](https://img.shields.io/website?url=https%3A%2F%2Fexvacuum.github.io%2Fbevy_outline_post_process%2F&label=docs)](https://exvacuum.github.io/bevy_outline_post_process)

A plugin for the [Bevy](https://bevyengine.org) engine which adds an outline post-processing effect. Optionally supports adaptive outlining, so darker areas are outlined in white rather than black, based on luminance.

Note: This is a full-screen post process effect and cannot be enabled/disabled for specific objects.

## Screenshots
![](./doc/screenshot.png)
![](./doc/screenshot_smooth.png)
Configuration Used:
```rs
bevy_outline_post_process::components::OutlinePostProcessSettings::new(2.0, 0.0, false);
```
## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.2           | 0.13         |

## Installation

### Using git URL in Cargo.toml
```toml
[dependencies.bevy_outline_post_process]
git = "https://github.com/exvacuum/bevy_outline_post_process.git"
```

## Usage

In `main.rs`:
```rs
use bevy::prelude::*;
use bevy_outline_post_process;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_outline_post_process::OutlinePostProcessPlugin,
        ))
        .insert_resource(Msaa::Off)
        .run();
}
```

When spawning a camera:
```rs
commands.spawn((
    // Camera3dBundle...
    NormalPrepass,
    bevy_outline_post_process::components::OutlinePostProcessSettings::new(2.0, 0.0, false);
));
```

This effect will only run for cameras which contain this component.

