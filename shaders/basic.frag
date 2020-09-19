precision mediump float;
varying vec2 texCoord;

uniform float aspect;
uniform int max_iter;
//uniform sampler1D palette;

const int MAX_MAX_ITER = 10000;

void square_plus_c(inout vec2 z, in vec2 c)
{
	float tmpX = z.x;
	z.x = z.x * z.x - z.y * z.y;
	z.y = 2.0 * tmpX * z.y;
	z += c;
}

int mandelbrot(in vec2 point, out vec2 z)
{
	z = vec2(0.0, 0.0);
	for (int i = 0; i < MAX_MAX_ITER; ++i)
	{
        if (i >= max_iter) break;
		if (length(z) > 20.0)
		{
			return i;
		}
		square_plus_c(z, point);
	}
	return max_iter;
}

const float ln_2_0 = log(2.0); 

float mandelbrot_renorm(in vec2 point)
{
	vec2 z;
	int iters = mandelbrot(point, z);
	// short circut if it's within set
	if (iters == max_iter) return float(iters);
	// steps to narrow errors
	square_plus_c(z, point);
	square_plus_c(z, point);
	float mu = float(iters) + 1.0 - log(log(length(z))) / ln_2_0;
	return mu;
}

void main()
{
	vec2 position;
	position.x = aspect * texCoord.x;
	position.y = texCoord.y;
	position *= 2.0;
	position -= vec2(1.5, 1.0);
	position += vec2(-0.5, 0.0);
	// position /= 1.0 / 2.0;
	float mu = mandelbrot_renorm(position);
	float iter = mu / float(max_iter);
	gl_FragColor = vec4(vec3(iter), 1.0);
}