
**Bevy XPBD Interp** is a simple library for interpolation of [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd/) rigidbodies. It operates by interpolating between the position/rotation of the current and previous physics states based on how much time has accumulated since the last physics update. The interpolated value is stored in the `Transform` of some separate entity that may hold meshes/cameras etc. Physics objects essentially need to be split up into one entity being affected by physics, and one entity being rendered.

In a lot of cases interpolation makes a noticeable difference at normal/higher physics update frequencies, eg. 60hz, but you'll see perfectly smooth movement even at 1hz.

> Note: `bevy_xpbd` is split into a 2d and a 3d crate. This library does the same and is split up into `bevy_xpbd_2d_interp` and `bevy_xpbd_3d_interp`.
> 

### Usage
Add `bevy_xpbd` and `bevy_xpbd_interp` as dependencies:
```toml
[dependencies]  
bevy_xpbd_2d = "0.3.1"
bevy_xpbd_2d_interp = "0.1.0"
# or
[dependencies]  
bevy_xpbd_3d = "0.3.1"
bevy_xpbd_3d_interp = "0.1.0"
```

Then add `XPBDInterpolationPlugin` to your app:
```rust
app.add_plugins(XPBDInterpolationPlugin);
```
Now you can add the `InterpolatedPosition` and/or `InterpolatedRotation` components to any entity with a `Transform`:
```rust
// The entity being affected by bevy_xpbd
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

See `'crates/bevy_xpbd_2d_interp/examples/box_2d.rs'` and `'crates/bevy_xpbd_3d_interp/examples/box_3d.rs'` for full examples. Run them with `cargo run --example box_2d/box_3d`.

### Supported versions

| Bevy | Bevy XPBD | Bevy XPBD Interp |
| ---- | --------- | ---------------- |
| 0.12 | 0.3.1     | 0.1.0            |
