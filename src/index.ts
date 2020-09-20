import Hammer from "hammerjs";

import "./index.css";
import vertSrc from "../shaders/basic.vert";
import fragSrc from "../shaders/basic.frag";

const START_ZOOM = 0.5;
const MIN_ZOOM = 0.1;
const START_X = 1.0;
const START_Y = 0.0;
const ZOOM_INCREMENT = 1.0 / 1000.0;
const ITERATIONS = 500;
const SCROLL_SCALING = 8.0;

async function main() {
    const canvas = document.getElementById("screen") as HTMLCanvasElement | null;
    if (!canvas) throw new Error("failed to get canvas element!");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const { Mandelbrot } = await import('../app/pkg');
    const mandelbrot = new Mandelbrot(canvas, vertSrc, fragSrc, START_ZOOM, START_X, START_Y, ITERATIONS);
    let frameRequested = false;
    function throttledDraw() {
        if (!frameRequested) {
            requestAnimationFrame(() => {
                mandelbrot.draw();
                frameRequested = false;
            });
            frameRequested = true;
        }
    }
    throttledDraw();
    // browser controls
    canvas.addEventListener('resize', () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        mandelbrot.resize_viewport();
        throttledDraw();
    });
    canvas.addEventListener('wheel', (e) => {
        const change = -e.deltaY * ZOOM_INCREMENT * mandelbrot.zoom;
        mandelbrot.zoom += change;
        mandelbrot.zoom = Math.max(mandelbrot.zoom, MIN_ZOOM);
        mandelbrot.refresh_zoom();
        throttledDraw();
    });
    // mobile controls
    const hammer = new Hammer(canvas);
    const pan = new Hammer.Pan();
    const pinch = new Hammer.Pinch();
    hammer.add(pan);
    hammer.add(pinch);
    hammer.on('pan', (e) => {
        const { velocityX: dx, velocityY: dy, deltaTime: dt } = e;
        mandelbrot.x_pos += dx / window.innerWidth / mandelbrot.zoom * SCROLL_SCALING;
        mandelbrot.y_pos -= dy / window.innerHeight / mandelbrot.zoom * SCROLL_SCALING;
        mandelbrot.refresh_position();
        throttledDraw();
    });
    let lastScale: number | undefined;
    hammer.on('pinch', (e) => {
        const { scale } = e;
        if (!lastScale) {
            lastScale = scale;
        }
        const ds = scale - lastScale;
        console.log(ds);
        lastScale = scale;
        if (Math.abs(ds) < 0.5) {
            mandelbrot.zoom += ds * mandelbrot.zoom;
            mandelbrot.zoom = Math.max(mandelbrot.zoom, MIN_ZOOM);
            mandelbrot.refresh_zoom();
            throttledDraw();
        }
    });
    const iterSlider = document.getElementById('iter-slider') as HTMLInputElement;
    const iterInput = document.getElementById('iter-input') as HTMLInputElement;
    if (!iterSlider || !iterInput) throw new Error('failed to get iter slider and input');
    function iterChanged(value: number) {
        mandelbrot.iters = value;
        mandelbrot.refresh_iters();
        throttledDraw();
    }
    iterSlider.addEventListener('input', (e) => {
        const { value } = (e.target as HTMLInputElement);
        iterInput.value = value;
        iterChanged(parseInt(value));
    });
    iterInput.addEventListener('input', (e) => {
        const value = Math.max(Math.min(1000, parseInt((e.target as HTMLInputElement).value || '0')), 1);
        iterInput.value = value.toString();
        iterSlider.value = iterInput.value;
        iterChanged(value);
    });
}

main();
