attribute vec4 position;
attribute vec2 aTexCoord;

varying vec2 texCoord;

void main() {
    gl_Position = position;
    texCoord = aTexCoord;
}
