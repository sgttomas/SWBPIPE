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
 */

const FIGURE_VERTEX_SHADER = /* glsl */ `
  #include <color_pars_vertex>
  varying vec3 vFigureViewNormal;
  varying vec3 vFigureViewPosition;
  void main() {
    #include <color_vertex>
    #include <beginnormal_vertex>
    #include <defaultnormal_vertex>
    #include <begin_vertex>
    #include <project_vertex>
    vFigureViewNormal = transformedNormal;
    vFigureViewPosition = -mvPosition.xyz;
  }
`;

const FIGURE_FRAGMENT_SHADER = /* glsl */ `
  uniform vec3 tint;
  uniform float opacity;
  uniform vec3 shadeRatio;
  #include <color_pars_fragment>
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
    gl_FragColor = vec4( mix( base * shadeRatio, base, smoothstep( 0.0, 0.75, facing ) ), opacity );
    #include <colorspace_fragment>
  }
`;

export type FigureMaterialOptions = Readonly<{
  /** Draw in the transparent pass with the `opacity` uniform as alpha. */
  transparent?: boolean;
  opacity?: number;
}>;

export type FigureMaterial = THREE.ShaderMaterial & {
  /** The `tint` uniform's colour, so colour-setting code treats this material like a basic one. */
  readonly color: THREE.Color;
};

export function createFigureMaterial(options: FigureMaterialOptions = {}): FigureMaterial {
  const tint = new THREE.Color();
  const material = new THREE.ShaderMaterial({
    uniforms: {
      tint: { value: tint },
      opacity: { value: options.opacity ?? 1 },
      // A ratio per channel in linear terms, not a colour in any colour space.
      shadeRatio: { value: new THREE.Color() }
    },
    vertexShader: FIGURE_VERTEX_SHADER,
    fragmentShader: FIGURE_FRAGMENT_SHADER,
    transparent: options.transparent ?? false,
    lights: false,
    toneMapped: false
  });
  material.userData.viewportFigureMaterial = true;
  Object.defineProperty(material, "color", { value: tint, enumerable: true, writable: false });
  return material as FigureMaterial;
}

export function isFigureMaterial(material: THREE.Material): material is FigureMaterial {
  return material instanceof THREE.ShaderMaterial && material.userData.viewportFigureMaterial === true;
}

/** Sets the silhouette darkening; the three numbers are per-channel ratios in linear terms. */
export function setFigureShadeRatio(material: FigureMaterial, ratio: readonly [number, number, number]): void {
  const value = material.uniforms.shadeRatio.value as THREE.Color;
  value.r = ratio[0];
  value.g = ratio[1];
  value.b = ratio[2];
}
