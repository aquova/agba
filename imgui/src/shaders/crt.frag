#version 150 core

// Details on implementation from here:
// https://www.gamasutra.com/blogs/SvyatoslavCherkasov/20140531/218753/Shader_tutorial_CRT_emulation.php

in vec2 tex_coord;
uniform sampler2D tex;
uniform int scale;
out vec4 color;

void main() {
    vec4 orig = texture(tex, tex_coord);
    color = vec4(0.0, 0.0, 0.0, 1.0);
    int pp = int(gl_FragCoord.x) % 3;
    color[pp] = orig[pp];

    vec4 mul = vec4(0.0, 0.0, 0.0, 1.0);
    switch (pp) {
        case 0:
            mul.x = 1.0;
            mul.y = 0.25;
            break;

        case 1:
            mul.y = 1.0;
            mul.x = 0.25;
            break;

        case 2:
            mul.z = 1.0;
            mul.x = 0.25;
            break;

        default:
            break;
    }

    color *= mul;
}
