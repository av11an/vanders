# Vanders

## Overview
Vanders is a binary rust create that generates the outputs of shaders in `.ppm` files.

## Background
This is a small passion project I started working on after getting inspired from this youtube video!

https://youtu.be/xNX9H_ZkfNE?si=447XbiZNxImy7V0M

## Shaders Available
- checkerboard
- ball
- whirl

## How to compile
In the root ~/vanders directory
```bash
ffmpeg -i output/output-%02d.ppm -r 60 output.mp4
```

And then use mpv!
```bash
mpv output.mp4 -loop
```
