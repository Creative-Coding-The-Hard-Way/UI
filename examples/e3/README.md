# First Texture

This example uses the AssetLoader to read an image from disk and use it as a
texture.

NOTE: the implementation uses the DescriptorIndexing feature from Vulkan 1.2
to enable variable-length descriptor set bindings and non-uniform sampler
indexing. This lets the implementation bind all of the textures once, then
decide which texture to use per-triangle.

## Usage

```
cargo run --example e2
```

## Keybinds

* `Esc` - exit
* `Space + Ctrl` - toggle fullscreen

## Screenshot

![screenshot](./screenshot.PNG)
