import "./index.css";
import vertSrc from "../shaders/basic.vert";
import fragSrc from "../shaders/basic.frag";

async function main() {
    const canvas = document.getElementById("screen") as HTMLCanvasElement | null;
    if (!canvas) throw new Error("failed to get canvas element!");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const { Mandelbrot } = await import('../app/pkg');
    const mandelbrot = new Mandelbrot(canvas, vertSrc, fragSrc, 0.5, 1.0, 0.0);
    let frameRequested = false;
    const throttledDraw = () => {
        if (!frameRequested) {
            requestAnimationFrame(() => {
                mandelbrot.draw();
                frameRequested = false;
            });
            frameRequested = true;
        }
    }
    throttledDraw();
    window.addEventListener('resize', () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        mandelbrot.resize_viewport();
        throttledDraw();
    });
    window.addEventListener('wheel', (e) => {
        const change = -e.deltaY / 1000 * mandelbrot.zoom;
        mandelbrot.zoom += change;
        mandelbrot.zoom = Math.max(mandelbrot.zoom, 0.1);
        mandelbrot.refresh_zoom();
        throttledDraw();
    });
    let mouseIsDown = false;
    window.addEventListener('mousedown', () => {
        mouseIsDown = true;
    });
    window.addEventListener('mouseup', () => {
        mouseIsDown = false;
    });
    window.addEventListener('mousemove', (e) => {
        if (mouseIsDown) {
            const { movementX: dx, movementY: dy } = e;
            mandelbrot.x_pos += dx / window.innerWidth / mandelbrot.zoom;
            mandelbrot.y_pos -= dy / window.innerHeight / mandelbrot.zoom;
            mandelbrot.refresh_position();
            throttledDraw();
        }
    });
}

main();
