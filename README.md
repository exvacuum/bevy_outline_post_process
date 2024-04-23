# grex_outline_post_process


A plugin for the [Bevy](https://bevyengine.org) engine which adds an outline post-processing effect.

Note: This is a full-screen post process effect and cannot be enabled/disabled for specific objects.

## Screenshots
![](./doc/screenshot.png)
![](./doc/screenshot_smooth.png)
Configuration Used:
```rs
grex_outline_post_process::components::OutlinePostProcessSettings {
    weight: 2.0,
    threshold: 0.0,
}
```
## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.1           | 0.13         |

## Installation

### Using git URL in Cargo.toml
```toml
[dependencies.grex_outline_post_process]
git = "https://github.com/exvacuum/grex_outline_post_process.git"
```

### Workspace Submodule
I recommend organizing your Bevy project as a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and pulling this repository as a git submodule:
```bash
git submodule add git@github.com:exvacuum/grex_outline_post_process.git
```

We can add a [patch override](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#overriding-repository-url) for this repository in the workspace's `Cargo.toml`:
```toml
[patch."https://github.com/exvacuum/grex_outline_post_process.git".grex_outline_post_process]
path = "./grex_outline_post_process"
```

And then, from a crate that depends on it:
```toml
[dependencies.grex_outline_post_process]
git = "https://github.com/exvacuum/grex_outline_post_process.git"
```


## Usage

In `main.rs`:
```rs
use bevy::prelude::*;
use grex_outline_post_process;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            grex_outline_post_process::OutlinePostProcessPlugin,
        ))
        .run();
}
```

When spawning a camera:
```rs
commands.spawn((
    // Camera3dBundle...
    grex_outline_post_process::components::OutlinePostProcessSettings {
        weight: 2.0,
        threshold: 0.0,
    }
));
```

This effect will only run for cameras which contain this component.

