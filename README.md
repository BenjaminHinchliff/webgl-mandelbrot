# An Explorable WebGL Mandelbrot Renderer

This one has been a bit of a headache to make. Not the actual rendering of the set, but making it so you can drag and zoom was a bit trickly. Oh and sorry mobile users, because you can't zoom in (yet). I'll get onto that once I figure out an intuitive, cross-platform way to do it. Maybe hammer.js.

Anyway, it's a renderer that lets you explore the mandelbrot set by dragging around and scrolling, accelerated by WebGL, meaning so long as you have semi-decent GPU hardware, you can get a very smooth experience. Oh also I wrote the core of the code in Rust.

It's not deployed anywhere as of now, but I'm working on getting it deployed to github pages really soon.

## Testing/Compilation
To run the development server use:
```
npm run serve
```

To compile the code use:
```
npm run build
```

## TODO(s):

- Iterations slider (right now statically set to 50)
- Better mobile support? I'm not sure about the performance of WebGL on mobile right now so I'm not sure it it's worth the time
- Testing? I would really like to have it but I really have no idea how to test WebGL code without something stupid like image comparison or a really massive mock that just makes you write the code twice.
