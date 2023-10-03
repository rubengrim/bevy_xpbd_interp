**Bevy XPBD Interp** is a simple library for interpolation of [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd/) rigidbodies. It operates by interpolating between the position/rotation of the current and previous physics update based on how much time has accumulated since the last physics update. The interpolated value is then stored in the `Transform` of some separate entity that may hold meshes/cameras etc. This means perfectly smooth results even at physics update rates as low as 1hz. Interpolation usually makes a noticeable difference at rates closer to the refresh rate of your monitor as well though.

### Usage
Add `bevy_xpbd_interp` as a git dependency:
```toml
[dependencies]  
bevy_xpbd_interp = { git = "https://github.com/rubengrim/bevy_xpbd_interp", branch = "main" }
```
Alternatively, clone the repo and add it like so:
```toml
[dependencies]  
bevy_xpbd_interp = { path = "path/to/bevy_xpbd_interp" }
```
> Note: The released version of `bevy_xpbd` cannot be used with `bevy_xpbd_interp` since a value that needs to be accessed is private in the release. You need to use the main git branch in your project instead with
> `bevy_xpbd_3d = { git = "https://github.com/Jondolf/bevy_xpbd", branch = "main" }`. This also means `bevy_xpbd_interp` cannot be published to crates.io.
> 
Then add `XPBDInterpolationPlugin` to your app:
```rust
app.add_plugins(XPBDInterpolationPlugin);
```
Now you can add the `InterpolatedPosition` and/or `InterpolatedRotation` components to any entity with a `Transform`.
```rust
// The entity being transformed by bevy_xpbd
let physics_entity = commands
    .spawn((
        RigidBody::Kinematic,
        Position::default(),
        Rotation::default(),
    ))
    .id();

// Rendered box that uses the interpolated position/rotation
commands.spawn((
    PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
        transform: Transform::default(), 
        ..default()
    }, 
    InterpolatedPosition::from_source(physics_entity),
    InterpolatedRotation::from_source(physics_entity),
));
```

See `'examples/box.rs'` for a full example. Run it with `cargo run --example box`.

### Limitations
- Currently depends on the main branch of `bevy_xpbd` and not a release.
- Currently only set up for `bevy_xpbd_3d` and not `bevy_xpbd_2d`.
- Only interpolates from `bevy_xpbd` and not from any arbitrary fixed-update schedule, because `bevy_xpbd` uses a custom solution for fixed timesteps. The same principle used here can be used for any fixed-update schedule though, it's just a matter of using the right accumulator value.