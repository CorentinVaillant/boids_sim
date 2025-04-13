#version 150

uniform vec2 position;
uniform vec2 velocity;
uniform float size;

uniform float separation;
uniform float alignement;
uniform float cohesion;

uniform float z;

uniform float canva_z;
uniform vec2 canva_pos;
uniform vec2 canva_size;
uniform uvec2 resolution;



in vec4 gl_FragCoord;

out vec4 fragColor;


void draw_ball(vec2 pos, float radius, vec4 color){
    if (length(pos-gl_FragCoord.xy) <=(radius)){
    fragColor = color;
    gl_FragDepth = z;
  }
}

void draw_circle(vec2 pos, float radius, float thickness, vec4 color){
  if ((length(gl_FragCoord.xy - pos) < radius + thickness) && (length(gl_FragCoord.xy - pos) > radius - thickness)){
    fragColor = color;
    gl_FragDepth = z;
  }
}

void main(){

  vec2 inv_position = vec2(position.x,resolution.y - position.y);
  gl_FragDepth = 0.;

  draw_ball(inv_position, size, vec4(1.,1.,1.,1.));
  draw_circle(inv_position, separation,1.,vec4(1.,0.,0.,0.75));
  draw_circle(inv_position, alignement,1.,vec4(0.,0.,1.,0.75));
  draw_circle(inv_position, cohesion,1.,vec4(0.,1.,0.,0.75));

}
