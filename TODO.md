`set_canvas_size` should be implemented at client level

```rust
fn set_canvas_size(&self, ctx: &Option<C>) {
    // Scale the drawing canvas by [dpr] to ensure sharp rendering on
    // high pixel density displays.
    if let Some(ctx) = ctx {
        // ctx.canvas
        //   ..style.width = "${w}px"
        //   ..style.height = "${h}px"
        //   ..width = scaledW
        //   ..height = scaledH;
        // ctx.set_transform(dpr, 0, 0, dpr, 0, 0);
    }
}
```
