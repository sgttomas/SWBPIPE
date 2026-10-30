import * as THREE from "three";

/**
 * The neutral matte material of the figure: no texture and no light.
 *
 * A surface facing the camera renders exactly its colour; toward the silhouette it darkens to
 * the colour times `shadeRatio`, enough to read as a cylinder and never enough to read as lit.
 * The colour is the instance colour (when the mesh is instanced with per-instance colour)
 * times the `tint` uniform. Instancing, per-instance colour and the output colour space come
 * from three's own shader chunks, so the material follows the renderer's conventions.
 *
 * It is deliberately texture-free: a shared texture would have to enter the viewport's
 * resource ownership ledger, and this material adds nothing to it beyond the material itself.
 *
 * The edge line (design system 6.2) is an option of this material, not an object of its own:
 * a fragment within `edgeWidth` device pixels of its element's outline takes the `edge` colour.
 * It is therefore hidden, selected and repainted with its element by the code that already does
 * those things, it adds no draw, no vertex, no geometry and no texture, and nothing it draws can
 * be met by a pick. A material built without the option compiles the shader it always had.
 */

function figureVertexShader(edgePars: string, edgeMain: string): string {
  return /* glsl */ `
  #include <color_pars_vertex>
  varying vec3 vFigureViewNormal;
  varying vec3 vFigureViewPosition;
${edgePars}  void main() {
    #include <color_vertex>
    #include <beginnormal_vertex>
    #include <defaultnormal_vertex>
    #include <begin_vertex>
    #include <project_vertex>
    vFigureViewNormal = transformedNormal;
    vFigureViewPosition = -mvPosition.xyz;
${edgeMain}  }
`;
}

const FIGURE_SHADED = "mix( base * shadeRatio, base, smoothstep( 0.0, 0.75, facing ) )";

function figureFragmentShader(edgePars: string, edgeMain: string): string {
  const colour = edgeMain === "" ? FIGURE_SHADED : `mix( ${FIGURE_SHADED}, edge, figureEdgeBand )`;
  return /* glsl */ `
  uniform vec3 tint;
  uniform float opacity;
  uniform vec3 shadeRatio;
${edgePars}  #include <color_pars_fragment>
  varying vec3 vFigureViewNormal;
  varying vec3 vFigureViewPosition;
  void main() {
    vec3 base = tint;
    #if defined( USE_COLOR_ALPHA )
      base *= vColor.rgb;
    #elif defined( USE_COLOR )
      base *= vColor;
    #endif
    vec3 viewDirection = isOrthographic ? vec3( 0.0, 0.0, 1.0 ) : normalize( vFigureViewPosition );
    float facing = abs( dot( normalize( vFigureViewNormal ), viewDirection ) );
${edgeMain}    gl_FragColor = vec4( ${colour}, opacity );
    #include <colorspace_fragment>
  }
`;
}

/*
 * How the line is found. Everything is measured in device pixels by one rule: for a value `g`
 * that is zero on a line of the picture and grows away from it, `g / length( dFdx( g ), dFdy( g ) )`
 * is the distance to that line, to first order. The band is where that distance is under the
 * line's width. Every derivative is taken before any choice is made, because a derivative inside
 * non-uniform control flow is undefined; the fragment shaders below have no `if`.
 *
 * The camera's position in the element's own frame comes from the model-view matrix alone. The
 * camera is the origin of view space, and the matrix's first three columns are orthogonal (an
 * instance matrix is a rotation and a scale, and no object above a figure mesh carries a scale
 * or a shear), so the coordinate along column i is dot( -M[3], M[i] ) / dot( M[i], M[i] ) and
 * no inverse is needed. A hidden instance's matrix is all zero: the guarded division gives a
 * camera at the origin, which is inside every shape, so no plane is built, and the instance has
 * no area to draw in any case. Only a perspective camera is handled; under an orthographic one
 * the outline that depends on the eye is left out.
 */
/** What one outline adds to the figure's two shaders: declarations, and statements for main(). */
type FigureEdgeSource = Readonly<{ vertexPars: string; vertexMain: string; fragmentPars: string; fragmentMain: string }>;

const EDGE_COMMON_VERTEX = /* glsl */ `
  vec3 figureEdgeCameraLocal() {
    #ifdef USE_INSTANCING
      mat4 figureModelView = modelViewMatrix * instanceMatrix;
    #else
      mat4 figureModelView = modelViewMatrix;
    #endif
    vec3 toCamera = -figureModelView[ 3 ].xyz;
    vec3 axisX = figureModelView[ 0 ].xyz;
    vec3 axisY = figureModelView[ 1 ].xyz;
    vec3 axisZ = figureModelView[ 2 ].xyz;
    return vec3(
      dot( toCamera, axisX ) / max( dot( axisX, axisX ), 1e-30 ),
      dot( toCamera, axisY ) / max( dot( axisY, axisY ), 1e-30 ),
      dot( toCamera, axisZ ) / max( dot( axisZ, axisZ ), 1e-30 )
    );
  }
`;

const EDGE_COMMON_FRAGMENT = /* glsl */ `
  uniform vec3 edge;
  uniform float edgeWidth;
  float figureEdgePixels( float value ) {
    return value / max( length( vec2( dFdx( value ), dFdy( value ) ) ), 1e-12 );
  }
  float figureEdgeBandAt( float pixels, float width ) {
    return 1.0 - smoothstep( width - 0.5, width + 0.5, pixels );
  }
`;

/*
 * A prism: three's capped CylinderGeometry with equal radii, whose side vertex k stands at
 * x = r sin( k d ), z = r cos( k d ), d = 2 pi / sides, with its axis along y.
 *
 * Seen from outside, the outline of a prism is exactly two of its vertex lines: the two vertices
 * of the cross-section polygon at which a line from the eye is a supporting line. With the eye at
 * distance rho (in radii) and angle alpha in the cross-section plane, side face k (centred on
 * ( k + 1/2 ) d) faces the eye when rho cos( alpha - centre ) > cos( d / 2 ), that is within
 * beta = acos( cos( d / 2 ) / rho ) of alpha; the two tangent vertices bound that run of faces.
 * For each, the plane through the eye and the vertex line is a supporting plane of the prism. The
 * value g = dot( p.xz - v, n ), with n the plane's normal turned toward the axis, is affine in the
 * element's own coordinates, so as a varying it is exact across every facet; it is zero exactly on
 * the drawn outline, because a plane through the eye projects to a line; and it is positive over
 * the whole prism. Both planes are the same for every vertex of an instance.
 *
 * The rim of a cap that is open to view (a free end, a step down in diameter seen from the
 * smaller side, the sliver at an angled joint) is drawn on the cap: a cap is a fan of triangles
 * about a centre vertex, so a value of 1 at the centre and 0 at the rim is affine on each
 * triangle and zero exactly on its rim edge. A cap is told from the side by its flat normal.
 * With the eye inside the prism's radius there is no outline from the side and none is drawn.
 */
const EDGE_PRISM: FigureEdgeSource = Object.freeze({
  vertexPars: /* glsl */ `${EDGE_COMMON_VERTEX}
  varying vec4 vFigureEdge;
  float figureEdgePlane( vec2 point, vec2 eye, float theta ) {
    vec2 vertex = vec2( sin( theta ), cos( theta ) );
    vec2 along = eye - vertex;
    vec2 toAxis = normalize( vec2( -along.y, along.x ) );
    toAxis = dot( vertex, toAxis ) > 0.0 ? -toAxis : toAxis;
    return dot( point - vertex, toAxis );
  }
`,
  vertexMain: /* glsl */ `    vec2 figureEdgeEye = figureEdgeCameraLocal().xz / FIGURE_EDGE_RADIUS;
    vec2 figureEdgePoint = transformed.xz / FIGURE_EDGE_RADIUS;
    float figureEdgeRho = length( figureEdgeEye );
    vec2 figureEdgeSides = vec2( 1e4 );
    if ( figureEdgeRho > 1.0 && ! isOrthographic ) {
      float figureEdgeStep = 6.283185307179586 / FIGURE_EDGE_SIDES;
      float figureEdgeAlpha = atan( figureEdgeEye.x, figureEdgeEye.y );
      float figureEdgeBeta = acos( min( cos( 0.5 * figureEdgeStep ) / figureEdgeRho, 1.0 ) );
      figureEdgeSides = vec2(
        figureEdgePlane( figureEdgePoint, figureEdgeEye,
          ( floor( ( figureEdgeAlpha + figureEdgeBeta - 0.5 * figureEdgeStep ) / figureEdgeStep ) + 1.0 ) * figureEdgeStep ),
        figureEdgePlane( figureEdgePoint, figureEdgeEye,
          ceil( ( figureEdgeAlpha - figureEdgeBeta - 0.5 * figureEdgeStep ) / figureEdgeStep ) * figureEdgeStep )
      );
    }
    vFigureEdge = vec4( figureEdgeSides, 1.0 - length( figureEdgePoint ), step( 0.5, abs( normal.y ) ) );
`,
  fragmentPars: /* glsl */ `${EDGE_COMMON_FRAGMENT}
  varying vec4 vFigureEdge;
`,
  fragmentMain: /* glsl */ `    float figureEdgeSide = min( figureEdgePixels( vFigureEdge.x ), figureEdgePixels( vFigureEdge.y ) );
    float figureEdgeRim = mix( 1e9, figureEdgePixels( vFigureEdge.z ), step( 0.5, vFigureEdge.w ) );
    float figureEdgeBand = figureEdgeBandAt( min( figureEdgeSide, figureEdgeRim ), edgeWidth );
`
});

/*
 * A box: every face that is seen is outlined at its border, by the distance along each of the
 * face's two in-plane axes to the nearer border. A border whose other face is also seen is a
 * crease drawn from both sides, so each side draws half the width and the crease is one line.
 */
const EDGE_BOX: FigureEdgeSource = Object.freeze({
  vertexPars: /* glsl */ `${EDGE_COMMON_VERTEX}
  varying vec3 vFigureEdgePoint;
  varying vec3 vFigureEdgeEye;
  varying vec3 vFigureEdgeNormal;
`,
  vertexMain: /* glsl */ `    vFigureEdgePoint = transformed;
    vFigureEdgeEye = figureEdgeCameraLocal();
    vFigureEdgeNormal = normal;
`,
  fragmentPars: /* glsl */ `${EDGE_COMMON_FRAGMENT}
  varying vec3 vFigureEdgePoint;
  varying vec3 vFigureEdgeEye;
  varying vec3 vFigureEdgeNormal;
`,
  fragmentMain: /* glsl */ `    vec3 figureEdgeRate = max( vec3(
      length( vec2( dFdx( vFigureEdgePoint.x ), dFdy( vFigureEdgePoint.x ) ) ),
      length( vec2( dFdx( vFigureEdgePoint.y ), dFdy( vFigureEdgePoint.y ) ) ),
      length( vec2( dFdx( vFigureEdgePoint.z ), dFdy( vFigureEdgePoint.z ) ) )
    ), vec3( 1e-12 ) );
    vec3 figureEdgeToBorder = ( FIGURE_EDGE_HALF_EXTENTS - abs( vFigureEdgePoint ) ) / figureEdgeRate;
    vec3 figureEdgeOwnAxis = step( vec3( 0.5 ), abs( vFigureEdgeNormal ) );
    vec3 figureEdgeCrease = step( FIGURE_EDGE_HALF_EXTENTS, vFigureEdgeEye * sign( vFigureEdgePoint ) );
    vec3 figureEdgeWidths = edgeWidth * mix( vec3( 1.0 ), vec3( 0.5 ), figureEdgeCrease );
    vec3 figureEdgeBands = ( vec3( 1.0 ) - smoothstep( figureEdgeWidths - 0.5, figureEdgeWidths + 0.5, figureEdgeToBorder ) )
      * ( vec3( 1.0 ) - figureEdgeOwnAxis );
    float figureEdgeBand = max( figureEdgeBands.x, max( figureEdgeBands.y, figureEdgeBands.z ) );
`
});

/*
 * A torus: three's TorusGeometry, a polygon swept about z, whose section vertex j stands at
 * ( cos( j d ), sin( j d ) ) in the section's own plane (outward, z). A torus's normal lies in its
 * section plane, so whether a face is seen depends only on the eye's position projected into that
 * plane, and the prism's rule applies section by section. The tangent vertex changes along the
 * sweep, so the planes are found per fragment; the distance takes the derivatives of the section
 * coordinates, which are continuous, and never of g, which steps where the vertex changes.
 */
const EDGE_TORUS: FigureEdgeSource = Object.freeze({
  vertexPars: /* glsl */ `${EDGE_COMMON_VERTEX}
  varying vec3 vFigureEdgePoint;
  varying vec3 vFigureEdgeEye;
`,
  vertexMain: /* glsl */ `    vFigureEdgePoint = transformed;
    vFigureEdgeEye = figureEdgeCameraLocal();
`,
  fragmentPars: /* glsl */ `${EDGE_COMMON_FRAGMENT}
  varying vec3 vFigureEdgePoint;
  varying vec3 vFigureEdgeEye;
  float figureEdgeSectionPixels( vec2 point, vec2 eye, float theta, vec2 rateX, vec2 rateY ) {
    vec2 vertex = vec2( cos( theta ), sin( theta ) );
    vec2 along = eye - vertex;
    vec2 toAxis = normalize( vec2( -along.y, along.x ) );
    toAxis = dot( vertex, toAxis ) > 0.0 ? -toAxis : toAxis;
    return dot( point - vertex, toAxis ) / max( length( toAxis.x * rateX + toAxis.y * rateY ), 1e-12 );
  }
`,
  fragmentMain: /* glsl */ `    vec2 figureEdgeOutward = normalize( vFigureEdgePoint.xy );
    vec2 figureEdgePoint = vec2( dot( vFigureEdgePoint.xy, figureEdgeOutward ) - FIGURE_EDGE_RING_RADIUS, vFigureEdgePoint.z )
      / FIGURE_EDGE_TUBE_RADIUS;
    vec2 figureEdgeEye = vec2( dot( vFigureEdgeEye.xy, figureEdgeOutward ) - FIGURE_EDGE_RING_RADIUS, vFigureEdgeEye.z )
      / FIGURE_EDGE_TUBE_RADIUS;
    vec2 figureEdgeRateX = vec2( dFdx( figureEdgePoint.x ), dFdy( figureEdgePoint.x ) );
    vec2 figureEdgeRateY = vec2( dFdx( figureEdgePoint.y ), dFdy( figureEdgePoint.y ) );
    float figureEdgeRho = length( figureEdgeEye );
    float figureEdgeStep = 6.283185307179586 / FIGURE_EDGE_SIDES;
    float figureEdgeAlpha = atan( figureEdgeEye.y, figureEdgeEye.x + ( figureEdgeRho > 0.0 ? 0.0 : 1.0 ) );
    float figureEdgeBeta = acos( min( cos( 0.5 * figureEdgeStep ) / max( figureEdgeRho, 1.0 ), 1.0 ) );
    float figureEdgeSection = min(
      figureEdgeSectionPixels( figureEdgePoint, figureEdgeEye,
        ( floor( ( figureEdgeAlpha + figureEdgeBeta - 0.5 * figureEdgeStep ) / figureEdgeStep ) + 1.0 ) * figureEdgeStep,
        figureEdgeRateX, figureEdgeRateY ),
      figureEdgeSectionPixels( figureEdgePoint, figureEdgeEye,
        ceil( ( figureEdgeAlpha - figureEdgeBeta - 0.5 * figureEdgeStep ) / figureEdgeStep ) * figureEdgeStep,
        figureEdgeRateX, figureEdgeRateY )
    );
    float figureEdgeBand = figureEdgeBandAt( figureEdgeRho > 1.0 && ! isOrthographic ? figureEdgeSection : 1e9, edgeWidth );
`
});

/**
 * The shape whose outline the line follows. Each is one of three's own geometries as the
 * viewport builds it; `figureEdgeOutlineFor` reads it from the geometry, so the constants are
 * stated once, where the geometry is made.
 */
export type FigureEdgeOutline =
  | Readonly<{ form: "prism"; sides: number; radius: number }>
  | Readonly<{ form: "box"; halfExtents: readonly [number, number, number] }>
  | Readonly<{ form: "torus"; sides: number; ringRadius: number; tubeRadius: number }>;

export type FigureMaterialOptions = Readonly<{
  /** Draw in the transparent pass with the `opacity` uniform as alpha. */
  transparent?: boolean;
  opacity?: number;
  /** Draw the edge line along this outline. Absent or null: no line, and the shader it always had. */
  edge?: FigureEdgeOutline | null;
}>;

/** A GLSL float literal: always with a decimal point or an exponent. */
function glslFloat(value: number): string {
  const text = String(value);
  return /[.e]/.test(text) ? text : `${text}.0`;
}

function positive(value: number): boolean {
  return Number.isFinite(value) && value > 0;
}

type FigureEdgeProgram = Readonly<{
  vertexShader: string;
  fragmentShader: string;
  defines: Record<string, string>;
}>;

function figureEdgeProgram(outline: FigureEdgeOutline): FigureEdgeProgram {
  const refuse = (): never => {
    throw new Error(`Unsupported figure edge outline: ${JSON.stringify(outline)}`);
  };
  const sidesOk = (sides: number): boolean => Number.isInteger(sides) && sides >= 3;
  let source: FigureEdgeSource;
  let defines: Record<string, string>;
  if (outline.form === "prism") {
    if (!sidesOk(outline.sides) || !positive(outline.radius)) refuse();
    source = EDGE_PRISM;
    defines = { FIGURE_EDGE_SIDES: glslFloat(outline.sides), FIGURE_EDGE_RADIUS: glslFloat(outline.radius) };
  } else if (outline.form === "box") {
    if (outline.halfExtents.length !== 3 || !outline.halfExtents.every(positive)) refuse();
    source = EDGE_BOX;
    defines = { FIGURE_EDGE_HALF_EXTENTS: `vec3( ${outline.halfExtents.map(glslFloat).join(", ")} )` };
  } else if (outline.form === "torus") {
    if (!sidesOk(outline.sides) || !positive(outline.tubeRadius) || !positive(outline.ringRadius) ||
        outline.ringRadius <= outline.tubeRadius) refuse();
    source = EDGE_TORUS;
    defines = {
      FIGURE_EDGE_SIDES: glslFloat(outline.sides),
      FIGURE_EDGE_RING_RADIUS: glslFloat(outline.ringRadius),
      FIGURE_EDGE_TUBE_RADIUS: glslFloat(outline.tubeRadius)
    };
  } else {
    return refuse();
  }
  return {
    vertexShader: figureVertexShader(source.vertexPars, source.vertexMain),
    fragmentShader: figureFragmentShader(source.fragmentPars, source.fragmentMain),
    defines
  };
}

const FIGURE_VERTEX_SHADER = figureVertexShader("", "");
const FIGURE_FRAGMENT_SHADER = figureFragmentShader("", "");

/**
 * The figure material is a class of its own because three's `clone()` is
 * `new this.constructor().copy(this)`: a clone is built by this constructor with no argument,
 * and `ShaderMaterial.copy` then replaces `uniforms` with clones of the source's. A clone is
 * therefore a whole figure material, and `isFigureMaterial` trusts the class alone: a marker in
 * `userData` survives `copy` into a material that is not one. `copy` carries the shaders, the
 * defines and the uniforms across, so a clone of a material with the edge line has the line.
 *
 * `color` is the `tint` uniform's colour, so colour-setting code treats this material like a
 * basic one. It is read from the live `uniforms` on every access and never stored, because
 * `copy` swaps the uniform objects after the constructor has run. It is an own accessor defined
 * in the constructor, with `declare` for its type: a class field is emitted as a define
 * (ES2022 with `useDefineForClassFields`) and would hold one colour object for good. A uniform's
 * value is untyped in three's declarations; the accessor's return type says what this file put
 * there.
 *
 * `opacity` is the `opacity` uniform's value in the same way: reading either gives the other
 * and writing either changes both, from construction on, through `setValues({ opacity })` and
 * across `clone()` and `copy()`. The drawn alpha is the uniform's, so a material whose `opacity`
 * property disagreed with it would draw one thing and report another. It is an own accessor
 * for the same reason `color` is; `Material`'s constructor assigns `this.opacity = 1` before the
 * uniforms exist, and the accessor replaces that data property once they do. Opacity and
 * `transparent` stay independent, as in three: an alpha below 1 is drawn only when the material
 * is in the transparent pass, so a later dimming sets both.
 *
 * Only properties of `THREE.ShaderMaterial` pass through `super(parameters)`: three's
 * `setValues` warns about any other. `type` and `isShaderMaterial` stay as three sets them,
 * because its renderer reads both to take the custom-shader path.
 */
export class FigureMaterial extends THREE.ShaderMaterial {
  declare readonly color: THREE.Color;

  constructor(options: FigureMaterialOptions = {}) {
    const edge = options.edge ? figureEdgeProgram(options.edge) : null;
    super({
      uniforms: {
        tint: { value: new THREE.Color() },
        opacity: { value: options.opacity ?? 1 },
        // A ratio per channel in linear terms, not a colour in any colour space.
        shadeRatio: { value: new THREE.Color() },
        ...(edge
          ? {
              // The line's colour, held as the tint is; and its width in device pixels, which is
              // the renderer's pixel ratio for a line of one CSS pixel.
              edge: { value: new THREE.Color() },
              edgeWidth: { value: 1 }
            }
          : {})
      },
      vertexShader: edge ? edge.vertexShader : FIGURE_VERTEX_SHADER,
      fragmentShader: edge ? edge.fragmentShader : FIGURE_FRAGMENT_SHADER,
      ...(edge ? { defines: edge.defines } : {}),
      transparent: options.transparent ?? false,
      lights: false,
      toneMapped: false
    });
    Object.defineProperty(this, "color", {
      get: (): THREE.Color => this.uniforms.tint.value,
      enumerable: true,
      configurable: true
    });
    Object.defineProperty(this, "opacity", {
      get: (): number => this.uniforms.opacity.value,
      set: (value: number): void => {
        this.uniforms.opacity.value = value;
      },
      enumerable: true,
      configurable: true
    });
  }
}

export function createFigureMaterial(options: FigureMaterialOptions = {}): FigureMaterial {
  return new FigureMaterial(options);
}

export function isFigureMaterial(material: THREE.Material): material is FigureMaterial {
  return material instanceof FigureMaterial;
}

/** Sets the silhouette darkening; the three numbers are per-channel ratios in linear terms. */
export function setFigureShadeRatio(material: FigureMaterial, ratio: readonly [number, number, number]): void {
  const value = material.uniforms.shadeRatio.value as THREE.Color;
  value.r = ratio[0];
  value.g = ratio[1];
  value.b = ratio[2];
}

/** Whether the material draws the edge line. Read from the uniforms, which a clone carries. */
export function hasFigureEdge(material: FigureMaterial): boolean {
  return "edge" in material.uniforms && "edgeWidth" in material.uniforms;
}

/**
 * Paints the edge line in place: its colour as 0xRRGGBB in sRGB, as `Color.setHex` takes it, and
 * its width in device pixels. A material without the line is left as it is.
 */
export function setFigureEdge(material: FigureMaterial, hex: number, widthDevicePixels: number): void {
  if (!hasFigureEdge(material)) return;
  (material.uniforms.edge.value as THREE.Color).setHex(hex);
  material.uniforms.edgeWidth.value = widthDevicePixels;
}

/**
 * The outline of a geometry the viewport draws, or null when there is no rule for its shape:
 * a capped, untapered, whole cylinder in one ring is a prism; a box of one segment a side is a
 * box; a torus is a torus. three keeps a geometry's constructor arguments in `parameters`.
 */
export function figureEdgeOutlineFor(geometry: THREE.BufferGeometry): FigureEdgeOutline | null {
  // A cone is a cylinder by class, with parameters of its own and no radius at its tip.
  if (geometry instanceof THREE.ConeGeometry) return null;
  if (geometry instanceof THREE.CylinderGeometry) {
    const { radiusTop, radiusBottom, radialSegments, heightSegments, openEnded, thetaLength } = geometry.parameters;
    const whole = positive(radiusTop) && radiusTop === radiusBottom && !openEnded && heightSegments === 1 &&
      thetaLength === Math.PI * 2;
    return whole ? { form: "prism", sides: Math.floor(radialSegments), radius: radiusTop } : null;
  }
  if (geometry instanceof THREE.BoxGeometry) {
    const { width, height, depth, widthSegments, heightSegments, depthSegments } = geometry.parameters;
    const plain = widthSegments === 1 && heightSegments === 1 && depthSegments === 1;
    return plain ? { form: "box", halfExtents: [width / 2, height / 2, depth / 2] } : null;
  }
  if (geometry instanceof THREE.TorusGeometry) {
    const { radius, tube, radialSegments } = geometry.parameters;
    return { form: "torus", sides: Math.floor(radialSegments), ringRadius: radius, tubeRadius: tube };
  }
  return null;
}
