#version 150

uniform vec2 position;
uniform vec2 velocity;
uniform float size;

uniform float separation;
uniform float alignement;
uniform float cohesion;

uniform float z;
uniform vec3 color ;

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

void draw_line(vec2 pos,vec2 a, vec2 b, float thickness, vec4 color){
  thickness = 1./thickness;
  vec2 ab = b - a;
  vec2 ba = -ab;
  vec2 bp = pos - b;
  vec2 ap = pos - a;
  vec2 pa = -ap;
  vec2 pd = normalize(vec2(ba.y, -ba.x));
  float proj = dot(pd, pa);
  float pr1 = dot(ba, bp);
  float pr2 = dot(ab, ap);

  if(pr1>0. && pr2 > 0. && abs(proj*thickness) <= 1.){
    fragColor = color;
    gl_FragDepth = z;
  }

}

void main(){

  vec2 invPos = vec2(position.x,resolution.y - position.y);
  vec2 oldPos = position - velocity * 0.5;
  vec2 invOldPos = vec2(oldPos.x,resolution.y - oldPos.y);

  gl_FragDepth = 0.;

  draw_ball(invPos, size, vec4(color,1.));
  // draw_line(gl_FragCoord.xy,invPos, invOldPos,size/5., vec4(0.75,0.075,1.000,1.000));
  // draw_circle(invPos, separation,1.,vec4(1.,0.,0.,0.75));
  // draw_circle(invPos, alignement,1.,vec4(0.,0.,1.,0.75));
  // draw_circle(invPos, cohesion,1.,vec4(0.,1.,0.,0.75));

}
