# Bevy XPBD Interp

**Bevy XPBD Interp** is a simple library for interpolation of [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd/) rigidbodies. It operates by interpolating between the position/rotation of the current and previous physics update based on how much time has accumulated since the last physics update. The interpolated value is then stored in the `Transform` of some separate entity that may hold meshes/cameras etc. This means perfectly smooth results even at physics update frequencies as low as 1hz. Interpolation makes a noticeable difference at frequencies closer to 60hz as well though.

### Usage
Add `bevy_xpbd_interp` as a dependency in your `Cargo.toml`:
```toml
[dependencies]  
bevy_xpbd_interp = "0.1.0"
```
> Note: The released version of `bevy_xpbd` cannot be used with `bevy_xpbd_interp` since a value I need to access is private in the release. Use the main git branch instead with
> `bevy_xpbd_3d = { git = "https://github.com/Jondolf/bevy_xpbd", branch = "main" }`
> 
Then add `XPBDInterpolationPlugin` to your app:
```rust
app.add_plugins(XPBDInterpolationPlugin)
```
Now you can add the `InterpolatedPosition` and/or `InterpolatedRotation` components to any entity with a `Transform`, like so:
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

See `examples/box.rs` for a full example. Run it with `cargo run --example box`.

### Limitations
- Currently depends on the main branch of `bevy_xpbd` and not the released crate.
- Currently only supports `bevy_xpbd_3d` and not `bevy_xpbd_2d`. 
- Only interpolates from `bevy_xpbd` and not from an arbitrary fixed-update schedule, because `bevy_xpbd` uses a custom solution for fixed timesteps. The same principle can be used for any fixed-update schedule though, it's just a matter of using the right accumulator value.