import "./index.css";
import vertSrc from "../shaders/basic.vert";
import fragSrc from "../shaders/basic.frag";

async function main() {
    const canvas = document.getElementById("screen") as HTMLCanvasElement | null;
    if (!canvas) throw new Error("failed to get canvas element!");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const { Mandelbrot } = await import('../app/pkg');
    const mandelbrot = new Mandelbrot(canvas, vertSrc, fragSrc, 0.5);
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
        const change = -e.deltaY / 1000;
        mandelbrot.zoom += change;
        mandelbrot.zoom = Math.max(mandelbrot.zoom, 0.1);
        mandelbrot.refresh_zoom();
        throttledDraw();
    });
}

main();
