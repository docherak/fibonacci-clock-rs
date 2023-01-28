# Fibonacci clock written in Rust
Everyone said the best way to learn programming is to start working on some projects, rather than following tutorials. For a long time I've wanted to create a Fibonacci clock watchface for my PineTime smartwatch. Combined with my interest in Rust, I decided to make a small Fibonacci clock app.

## Usage:
Upon running, the app prints out the current time along with verbal description of the boxes (5, 3, 2, 1, 1) and possible colors (White, Red, Green, Blue). Fibonacci clock has the following characteristics:

- it shows time in 5-minute increments using 5 square boxes (their sizes follow the Fibonacci sequence)
- to calculate the hours:
    - add up the value of the Red and Blue squares
- to calculate the minutes:
    - add up the values of the Green and Blue squares and multiply the value by 5

The app should be able to handle "borderline" times such as 12:57.

## Task list:
- [x] Create a working prototype, that adjusts time correctly and prints out colors accordingly
- [ ] Simplify repeated code based on a course taken recently
- [ ] Create docs 
- [ ] Visualize the clock using command line 
- [ ] Create GUI app
- [ ] \(Optional) Change from 1-12 to 0-11
