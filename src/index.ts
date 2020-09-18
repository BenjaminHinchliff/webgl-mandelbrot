async function main() {
    const canvas = document.getElementById("screen") as HTMLCanvasElement | null;
    if (!canvas) throw new Error("failed to get canvas element!");
    const app = await import('../app/pkg');
    app.render_mandelbrot(canvas);
}

main();
