# procedural animation of linked nodes
<img src="https://github.com/JoJoDataPhysics/rustanimations/blob/main/Screenshot.png" alt="fishes" width="450"/>

## Project Overview

This Rust project is a graphical simulation built using the [Bevy game engine](https://bevyengine.org/) and the [rand crate](https://docs.rs/rand/latest/rand/). The project demonstrates the movement of circular chains and the visualization of different elements (like nodes, indices, and contour dots) as they orbit around a central point. The animation is created using Bevy's entity-component-system (ECS) and Bevy's real-time rendering capabilities.

The main components of the project are:

- **`mod movements`**: Contains the logic for animating circular objects and orchestrating their motion.
- **`mod shapes`**: Defines different geometric shapes, such as circular chains, and how these shapes are constructed, aligned, and displayed.
- **`mod testing`**: Provides testing utilities for verifying the behavior of circular chains and contours.

## Project Structure

### Modules

- **`mod movements`**: Handles the movement logic for animated shapes, such as orbiting around a central point.
- **`mod shapes`**: Defines the geometric structures like circular chains and contours. The key sub-modules in `shapes` are:
  - `circle`: Contains the `CircleChain` struct for managing a chain of connected circles.
  - `combi_shapes`: Contains utility functions like `seven_chain()` for creating specific combinations of shapes.
  - `contour`: Defines structures like `seven_node_contour()` for node-based shapes.
- **`mod testing`**: Contains test functions to verify that circles, chains, and contours are behaving as expected.

### Key Functions

- **`test_circle()`**: Runs a test to verify the behavior of a single circle.
- **`test_circle_chain()`**: Tests the creation and alignment of a chain of circles.
- **`test_contour()`**: Tests the creation and positioning of contour nodes.

### `main()` Function

The main entry point of the program, responsible for:

1. Running the tests (`test_circle()`, `test_circle_chain()`, and `test_contour()`).
2. Initializing the Bevy app.
3. Spawning entities like the camera and animated circular chains in the 2D space.
4. Defining systems that run in the Bevy ECS loop, including the `animate_circles` system for animating the circular chains.

### Animation Logic

The `animate_circles()` system is responsible for:

- Calculating the updated position of each circle in the chain.
- Rendering circles, nodes, and other indicators (like contour dots).
- Orbiting the circles around a central point and aligning them based on their respective angles and radii.

## Setup and Requirements

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can download and install it from [rust-lang.org](https://www.rust-lang.org/).
- **Bevy**: This project relies on Bevy for game engine features. Bevy and its dependencies are included via `Cargo.toml`.
- **Rand**: The `rand` crate is used for generating random values, such as circle speed and orbit radius.

### Installing Dependencies

After cloning the repository, run the following command to fetch and install dependencies:

```bash
cargo build
```

### Running the Project

To run the project, execute the following command:

```bash
cargo run
```

This will start the application and display the animated circular chains in a 2D window.

### Testing

The testing functions are called in the `main()` function, and the results will be outputted to the console when the application starts. These test the behavior of circular shapes and chains, ensuring correct alignment and positioning.

## Customization

- You can customize the number of animated circular chains by modifying the `for _ in 0..10` loop in the `setup()` function.
- Modify the visualization by adjusting visibility flags in `CircleChain` (e.g., `is_visible_circles`, `is_visible_nodes`, `is_visible_contour_dots`, etc.).
  
## License

This project is open-source and available under the MIT License.

## Contributing

Feel free to open issues or submit pull requests to improve the functionality, performance, or features of this project.

---

Enjoy experimenting with geometric shapes and animations!
