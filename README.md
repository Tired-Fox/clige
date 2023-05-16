# CLIGE: Command Line Interface Game Engine

<!-- Header Badges -->

<div align="center">
  
<img src="assets/badges/version.svg" alt="Version"/>
<a href="https://github.com/Tired-Fox/clige/releases" alt="Release"><img src="https://img.shields.io/github/v/release/tired-fox/clige.svg?style=flat-square&color=9cf"/></a>
<a href="https://github.com/Tired-Fox/clige/blob/main/LICENSE" alt="License"><img src="assets/badges/license.svg"/></a>
<br>
<img src="assets/badges/maintained.svg" alt="Maintained"/>
<img src="assets/badges/tests.svg" alt="Tests"/>
  
</div>

<!-- End Header -->

A project to create an ansi/terminal based game engine

## Update Loop
1. **Logic**: Calculating data and states based on existing data.
- Calculates data regardless of how many fps
- Can use a delta time variable to calculate animations and events
2. **Event**: Built in event system. A seriese of events that can be subscribed to and have certain messages passed when something is triggered, or when time passes
3. **Render**: This is where each component / layer will have it's graphics computed and merged into a single canvas. This canvas then will render/draw to the screen
- Handles data -> graphics/chars and colors
- The final state of renderable characters for that frame

<!-- Footer Badges --!>

<br>
<div align="center">
  <img src="assets/badges/made_with_rust.svg" alt="Made with rust"/>
  <img src="assets/badges/built_with_love.svg" alt="Built with love"/>
</div>

<!-- End Footer -->
