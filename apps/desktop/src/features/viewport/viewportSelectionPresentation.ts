import * as THREE from "three";
import type { EntityKey } from "../workspace/selectionState";
import type { PointPickPrimitive } from "./viewportSelection";

/** Display-only geometry. Never register this group with either picking path. */
export class ViewportSelectionPresentation {
  readonly group = new THREE.Group();
  readonly spans: THREE.LineSegments<THREE.BufferGeometry, THREE.LineBasicMaterial>;
  readonly markers: THREE.Points<THREE.BufferGeometry, THREE.ShaderMaterial>;

  constructor(private readonly primitives: readonly PointPickPrimitive[]) {
    const spans = new THREE.BufferGeometry();
    spans.setAttribute("position", new THREE.BufferAttribute(new Float32Array(primitives.length * 6), 3).setUsage(THREE.DynamicDrawUsage));
    const markers = new THREE.BufferGeometry();
    markers.setAttribute("position", new THREE.BufferAttribute(new Float32Array(primitives.length * 3), 3).setUsage(THREE.DynamicDrawUsage));
    this.spans = new THREE.LineSegments(spans, new THREE.LineBasicMaterial({
      transparent: true, depthTest: false, depthWrite: false, toneMapped: false
    }));
    this.markers = new THREE.Points(markers, new THREE.ShaderMaterial({
      transparent: true, depthTest: false, depthWrite: false, toneMapped: false,
      uniforms: {
        selectedColor: { value: new THREE.Color() },
        rimColor: { value: new THREE.Color() },
        pixelRatio: { value: 1 }
      },
      vertexShader: `
        uniform float pixelRatio;
        void main() {
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
          gl_PointSize = 11.0 * pixelRatio;
        }
      `,
      fragmentShader: `
        uniform vec3 selectedColor;
        uniform vec3 rimColor;
        void main() {
          vec2 offset = abs(gl_PointCoord - vec2(0.5));
          float diamond = offset.x + offset.y;
          if (diamond > 0.5) discard;
          gl_FragColor = vec4(diamond > 0.34 ? rimColor : selectedColor, 1.0);
          #include <colorspace_fragment>
        }
      `
    }));
    // Transparent render-list placement is deliberate: opaque geometry AND
    // incidental transparent result/load graphics must precede the cue.
    this.spans.renderOrder = 10_000;
    this.markers.renderOrder = 10_001;
    this.spans.frustumCulled = false;
    this.markers.frustumCulled = false;
    // Vertex clipping still rejects behind-camera / near/far-plane markers.
    // Disable aggregate culling because compacted dynamic buffers retain tails.
    this.group.name = "viewport-selected-authored-primitives";
    this.group.add(this.spans, this.markers);
    spans.setDrawRange(0, 0);
    markers.setDrawRange(0, 0);
    this.group.visible = false;
  }

  update(selected: ReadonlySet<EntityKey>, hidden: ReadonlySet<EntityKey>, color: number, rim: number, pixelRatio: number): void {
    const spans = this.spans.geometry.getAttribute("position") as THREE.BufferAttribute;
    const markers = this.markers.geometry.getAttribute("position") as THREE.BufferAttribute;
    let spanCount = 0;
    let markerCount = 0;
    for (const primitive of this.primitives) {
      if (!selected.has(primitive.key) || hidden.has(primitive.key)) continue;
      // Primitives already use the authored model's render-local coordinates;
      // no deformed/result geometry or second origin subtraction belongs here.
      const { center, start, end } = primitive;
      markers.setXYZ(markerCount++, center.x, center.y, center.z);
      if (start && end) {
        spans.setXYZ(spanCount++, start.x, start.y, start.z);
        spans.setXYZ(spanCount++, end.x, end.y, end.z);
      }
    }
    this.spans.geometry.setDrawRange(0, spanCount);
    this.markers.geometry.setDrawRange(0, markerCount);
    spans.needsUpdate = true;
    markers.needsUpdate = true;
    this.spans.material.color.setHex(color);
    this.markers.material.uniforms.selectedColor.value.setHex(color);
    this.markers.material.uniforms.rimColor.value.setHex(rim);
    this.markers.material.uniforms.pixelRatio.value = pixelRatio;
    this.group.visible = markerCount > 0;
  }
}
