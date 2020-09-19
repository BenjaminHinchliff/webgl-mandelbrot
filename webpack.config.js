const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './src/index.ts',
    output: {
        filename: 'index.bundle.js',
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, 'app'),
        }),
        new HtmlWebpackPlugin({
            title: "Web Mandelbrot",
            template: "src/index.html",
        }),
    ],
    resolve: {
        extensions: [ '.tsx', '.ts', '.jsx', '.js' ],
    },
    module: {
        rules: [
            {
                test: /\.[tj]sx?$/i,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.(vert|frag)$/i,
                use: 'raw-loader',
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
};
