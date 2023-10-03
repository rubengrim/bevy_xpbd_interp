**bevy_xpbd_interp** is a simple library for interpolation of [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd/) rigidbodies. It operates by interpolating between the position/rotation of the current and previous physics update, and passing the interpolated values to the `Transform` of some separate entity that may hold any meshes/cameras etc. This means perfectly smooth results even at physics update frequencies as low as 1hz. Interpolation makes a noticeable difference at frequencies closer to 60hz too though.

### Usage
Add `bevy_xpbd_interp` as a dependency in your `Cargo.toml`:
```toml
[dependencies]  
bevy_xpbd_interp = "0.1.0"
```
Then add the `XPBDInterpolationPlugin` to your app:
```rust
app.add_plugins(XPBDInterpolationPlugin)
```
Now you can add the `InterpolatedPosition` and `InterpolatedRotation` components to any entity with a `Transform` to interpolate and store the result in that same `Transform`, like so:
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
        material: material_assets.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.8, 0.6),
            ..default()
        }),
        transform: Transform::default(),
        ..default()
    },
    InterpolatedPosition::new(physics_entity),
    InterpolatedRotation::new(physics_entity),
));
```

See `examples/box.rs` for the full example. Run it with `cargo run --example box`.