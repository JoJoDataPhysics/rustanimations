# procedural animation of linked nodes
<img src="https://github.com/JoJoDataPhysics/rustanimations/blob/main/Screenshot.png" alt="fishes" width="450"/>

## Project Overview

This Rust project provides functionality for simulating and manipulating circular chains of nodes in a 2D space. Each circular node has properties like position, radius, and direction, and can be aligned with other nodes to create chains. The project is built using Rust's standard libraries along with the [Bevy game engine](https://bevyengine.org/) for rendering and animation, and the [rand crate](https://docs.rs/rand/latest/rand/) for generating random values. This simulation includes the ability to create circular chains, position and align nodes, and visualize them.

The project also includes a `ContourNode` system, which allows the generation of a contour made up of nodes, and uses Bevy's rendering capabilities to animate and display various elements, such as circles and contour points.

### Key Features:
- **Circular Nodes**: Each node represents a circle in 2D space, with adjustable properties such as radius and direction.
- **Circle Chains**: Multiple nodes can be linked into a chain, allowing you to align and move the nodes together.
- **Visualization**: Using Bevy’s rendering system, the project animates circular nodes and chains with customizable visibility options for circles, nodes, indices, and contour dots.
- **Contouring**: The project supports contour generation for defining special geometric relationships between nodes.

## Project Structure

### Modules

1. **CircularNode**: Defines a circular node with properties such as position, radius, and direction. Includes methods for creating and aligning nodes.
2. **CircleChain**: Handles the logic of managing a chain of circular nodes, with functions for adding, moving, and aligning nodes in the chain.
3. **ContourNode**: Represents contour points that interact with circular chains, allowing for more complex geometric shapes.
4. **Testing**: Functions like `test_circle`, `test_circle_chain`, and `test_contour` are provided to validate the behavior of nodes and chains.

### Core Components

- **`CircularNode`**: A structure that represents a circle in 2D space, with properties like position `(x, y)`, radius, and direction (angle in radians). Key methods include:
  - **`allign_nodes`**: Aligns one circular node relative to a reference node at a specified distance.
  - **`new`**: Creates a new circular node.

- **`CircleChain`**: A collection of circular nodes connected in a chain. Key methods include:
  - **`new`**: Creates a new circle chain with a specified head node and distance between nodes.
  - **`add_circle`**: Adds a new circular node to the chain.
  - **`move_head`**: Moves the head of the chain by a specified offset.
  - **`position_head`**: Repositions the head of the chain at a specific coordinate.
  - **`allign_nodes`**: Aligns all the nodes in the chain based on the head node.

- **`seven_chain`**: A utility function that creates a specific chain of seven circular nodes, used to demonstrate chaining and alignment.

- **`ContourNode`**: Defines nodes that represent a contour, with methods to generate a specific seven-node contour.

## Animation and Visualization

The project uses Bevy’s `App` system to animate the circular chains, positioning them in an orbit around a central point and rendering them with customizable visual elements. Bevy's ECS (Entity Component System) manages the behavior of the animated chains, and `Gizmos` are used to visualize the nodes, circles, and other elements.

Key Bevy Systems:

- **`setup()`**: Initializes the Bevy app, spawns a 2D camera, and generates multiple circular chains.
- **`animate_circles()`**: This system updates the position and orientation of each circle in the chain and renders them using Bevy’s gizmo system.

## Setup and Requirements

### Prerequisites

- **Rust**: Make sure to have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Bevy**: This project depends on the Bevy game engine. Bevy and other dependencies are managed through Cargo.
- **Rand**: The `rand` crate is used for generating random numbers in the animation.

### Installing Dependencies

To install the necessary dependencies for this project, run the following command:

```bash
cargo build
```

### Running the Project

To run the project and view the animated circular chains in action, execute the following command:

```bash
cargo run
```

This will start the application, and a window will open displaying the animated chains in 2D space.

### Testing

The project includes several test functions that validate the behavior of individual circular nodes and chains. These tests are invoked in the `main()` function and their output is printed to the console.

- **`test_circle()`**: Tests the behavior of aligning a single circular node with respect to a reference node.
- **`test_circle_chain()`**: Tests the creation, alignment, and movement of a chain of circles.

### Customization

- **Change Node Visibility**: The visibility of circles, nodes, indices, and contour dots in the chain can be customized by modifying the boolean flags in `CircleChain`, such as `is_visible_circles`, `is_visible_nodes`, and so on.
- **Orbit Configuration**: You can adjust the number of chains, speed, and orbit radius by modifying the random values generated in the `setup()` function.

## License

This project is licensed under the MIT License.

## Contributing

Feel free to open issues or submit pull requests if you'd like to improve this project. Contributions for performance improvements, feature extensions, or bug fixes are always welcome.

---

Enjoy exploring and experimenting with circular nodes, chains, and contours in this 2D space simulation!
