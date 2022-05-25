# UI

This repository shows one way to build an interactive UI with Vulkan and GLFW.

This repository is NOT a general purpose UI toolkit. Applications which need
truly full-fledged UI should consider using ImGUI or something like Iced.rs.
This is meant as a learning exercise and a building-block for other experiments
which need a basic UI.

## Contents

#### Vulkan Basics

1. [Hello World](./examples/e0)
   - First triangle with a passthrough renderer
1. [Ortho Transform](./examples/e1)
   - Use a Uniform Buffer and a Descriptor Set to provide the shader with an
     orthographic projection matrix.
1. [First Texture](./examples/e2)
   - Use the asset loader class to read multiple textures into GPU memory then
     render textured vertices

#### UI

1. [UI State](./examples/e3)
   - Create an App with a pseudo-retained mode UI

