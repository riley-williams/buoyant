# Animation

View subtrees can be animated by attaching the `.animated()` modifier to a view. This modifier
creates smooth transitions between instances of the view.

```rust,ignore
pub fn animated<T>(self, animation: Animation, value: T) -> Animated<Self, T>
where
    T: PartialEq + Clone
```

## Triggering Animation

When the value provided to the `.animated()` modifier changes, the animation render tree node
drives an animation factor using the provided curve that its children use to interpolate
their properties.

> Any changes to the view's properties that occur without changing the value passed to
> `.animated()` will not animate.
