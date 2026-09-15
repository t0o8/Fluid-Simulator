# Rust Fluid Simulator

A real-time 2D fluid simulation built in Rust using the [Macroquad](https://macroquad.rs/) game engine. This project implements **Smoothed Particle Hydrodynamics (SPH)** to simulate fluid behavior, utilizing spatial hashing to optimize performance and handle high particle counts efficiently.

## Features

*   **Smoothed Particle Hydrodynamics (SPH):** Accurately models fluid mechanics using density, pressure, and viscosity calculations.
*   **Predictive Physics:** Calculates predictive forces for stable fluid compression and movement.
*   **Interactive Environments:** 
    *   Boundary collision with velocity dampening.
    *   **Mouse Interaction:** Click and hold the Left Mouse Button to act as a gravity well, pulling particles toward your cursor.
*   **Dynamic Visuals:** Particles dynamically change color based on their current density (Blue = Low Density, White = Target Density, Red = High Density).

##  Controls

*   **Left Mouse Click (Hold):** Attracts nearby fluid particles towards the mouse cursor.

*   ## Built With
* [Rust](https://www.rust-lang.org/) - The programming language
* [Macroquad](https://macroquad.rs/) - A Rust 2D game library
