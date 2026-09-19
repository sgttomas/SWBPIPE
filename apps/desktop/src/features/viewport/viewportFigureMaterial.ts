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

/**
 * The figure material is a class of its own because three's `clone()` is
 * `new this.constructor().copy(this)`: a clone is built by this constructor with no argument,
 * and `ShaderMaterial.copy` then replaces `uniforms` with clones of the source's. A clone is
 * therefore a whole figure material, and `isFigureMaterial` trusts the class alone: a marker in
 * `userData` survives `copy` into a material that is not one.
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
    super({
      uniforms: {
        tint: { value: new THREE.Color() },
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
