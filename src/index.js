async function main() {
    const app = await import('../app/pkg');
    app.render_mandelbrot();
}

main();
