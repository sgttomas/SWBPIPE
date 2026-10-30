import * as THREE from "three";
import type { EntityKey } from "../workspace/selectionState";
import { viewportTokenColour, type ViewportPaletteTheme } from "./viewportPalette";

/**
 * Selection and hover as halos (design system 6.6): an outline outside the silhouette of each
 * selected element, 2 CSS px in `canvas.selection`, and 1 CSS px in `canvas.hover` on a hovered
 * element that is not selected. The element keeps its own colour and its edge line.
 *
 * Display-only. Nothing here is registered with either picking path, every halo mesh refuses a
 * raycast, and picking stays analytic over the typed model index.
 *
 * How it is drawn, with no change to how the renderer is built (no stencil buffer, no render
 * target, no post-processing). The halo draws last in the main pass, in two steps:
 *
 *   1. The mask. Every haloed element is drawn again from its own geometry and its own instance
 *      matrix, with colour writes off, the depth test always passing, and its depth written just
 *      inside the near plane. Its coverage is the element's own, sample for sample, because the
 *      vertex positions are computed as the figure computes them.
 *   2. The shell. The element's outline grown in screen space by the halo's width, drawn at a
 *      depth just behind the mask with the depth test on and depth writes off. It therefore shows
 *      everywhere except inside the outline of a haloed element, and it shows over nearer
 *      geometry: a selected pipe inside a node sphere still has a halo the user can see.
 *
 * The mask leaves the near depth behind in the pixels it covers. Only what draws after the halo
 * with a depth test would meet it: the first profile's cue draws later with no depth test, and the
 * gizmo clears depth in its own scissor before it draws.
 *
 * How the shell is grown. A sphere moves each vertex along its projected normal by the width. Every
 * other shape is drawn as its crease and border edges, each a screen-space quad of the width on
 * either side with a round cap of the width at either end: the half inside the silhouette, and
 * every edge that is not on it, is under the mask, so what remains is the band outside the
 * silhouette, exact in width, round at corners, with no rule per shape and no crack at a hard
 * edge. One draw per shape and per step, however many elements are haloed.
 *
 * Halo meshes are instanced, one pair (mask, shell) per base geometry and per kind of halo, and
 * carry only the haloed instances' matrices; capacity grows geometrically and never per frame. A
 * scene with no halo owns nothing of this file. The mask shares the base geometry and never
 * disposes it; what this file creates (edge geometries, materials, instance buffers) it counts
 * into the ownership ledger itself and disposes itself, so no shared geometry is counted twice.
 */

/** What this file adds to, and takes from, the viewport's ownership ledger. */
export type HaloOwnedCounts = Readonly<{ geometries?: number; materials?: number; instanceMatrices?: number }>;
export type HaloLedger = Readonly<{
  created(counts: HaloOwnedCounts): void;
  disposed(counts: HaloOwnedCounts): void;
}>;

/** Halo widths in CSS pixels (design system 6.6). The drawn width is this times the pixel ratio. */
export const SELECTION_HALO_CSS_PX = 2;
export const HOVER_HALO_CSS_PX = 1;

// All after the figure and any transparent result or load graphic, and before the first profile's
// cue (10 000 and 10 001). Masks first, then the hover shell, then the selection shell over it.
export const HALO_MASK_RENDER_ORDER = 9_000;
export const HALO_HOVER_RENDER_ORDER = 9_001;
export const HALO_SELECTION_RENDER_ORDER = 9_002;

// Clip-space depth as a fraction of w. The mask lands at a window depth of 5e-5 and the shell at
// 2e-4: apart by thousands of steps of a 24-bit depth buffer and by several of a 16-bit one, and
// nearer than anything the camera can draw (a depth of 2e-4 is 0.002 % beyond the near plane).
const MASK_DEPTH = "0.9999";
const SHELL_DEPTH = "0.9996";

const MASK_VERTEX = /* glsl */ `
  void main() {
    #include <begin_vertex>
    #include <project_vertex>
    gl_Position.z = -gl_Position.w * ${MASK_DEPTH};
  }
`;

const MASK_FRAGMENT = /* glsl */ `
  void main() {
    gl_FragColor = vec4( 0.0 );
  }
`;

/*
 * The normal shell. The vertex is projected twice, as it stands and moved a little along its normal
 * in view space; the difference on the screen is the direction it is moved in, by the halo's width
 * in device pixels. A normal that points at the eye has no direction on the screen and the vertex
 * stays where it is, inside the outline.
 */
const NORMAL_SHELL_VERTEX = /* glsl */ `
  uniform vec2 haloViewport;
  uniform float haloWidth;
  void main() {
    #include <beginnormal_vertex>
    #include <defaultnormal_vertex>
    #include <begin_vertex>
    #include <project_vertex>
    vec3 haloNormal = normalize( transformedNormal );
    float haloStep = max( -mvPosition.z, 1e-6 ) * 0.01;
    vec4 haloMoved = projectionMatrix * vec4( mvPosition.xyz + haloNormal * haloStep, 1.0 );
    vec2 haloDirection = ( haloMoved.xy / max( haloMoved.w, 1e-9 ) - gl_Position.xy / max( gl_Position.w, 1e-9 ) ) * haloViewport;
    float haloLength = length( haloDirection );
    haloDirection = haloLength > 1e-6 ? haloDirection / haloLength : vec2( 0.0 );
    gl_Position.xy += haloDirection * ( 2.0 * haloWidth / haloViewport ) * gl_Position.w;
    gl_Position.z = -gl_Position.w * ${SHELL_DEPTH};
  }
`;

/*
 * The edge shell. Each edge of the shape is four vertices: two at either end, one for either side.
 * A vertex knows its own end (position), the other end (haloOther), and its side and which end it
 * is (haloSide: each +1 or -1). Both
 * ends are taken to view space; an end behind the near plane is brought to it along the edge, so a
 * projection never divides by a w that is not in front of the eye. On the screen the vertex moves
 * away from the other end by the width (room for the cap) and to its side by the width. The
 * position is written already divided, with w = 1, so the varying is interpolated in the screen's
 * own plane: it carries the fragment's place along the edge, its place across it and the edge's
 * length, all in device pixels, and the fragment shader keeps only what is within the width of the
 * edge, which rounds the cap. Every outline is then the silhouette grown by a disc of the width.
 */
const EDGE_SHELL_VERTEX = /* glsl */ `
  attribute vec3 haloOther;
  attribute vec2 haloSide;
  uniform vec2 haloViewport;
  uniform float haloWidth;
  varying vec3 vHaloCap;
  void main() {
    vHaloCap = vec3( 0.0 );
    #ifdef USE_INSTANCING
      mat4 haloModelView = modelViewMatrix * instanceMatrix;
    #else
      mat4 haloModelView = modelViewMatrix;
    #endif
    vec4 haloOwn = haloModelView * vec4( position, 1.0 );
    vec4 haloFar = haloModelView * vec4( haloOther, 1.0 );
    float haloNear = -1.0001 * projectionMatrix[ 3 ][ 2 ] / ( projectionMatrix[ 2 ][ 2 ] - 1.0 );
    bool haloOwnBehind = haloOwn.z > haloNear;
    bool haloFarBehind = haloFar.z > haloNear;
    if ( haloOwnBehind && haloFarBehind ) {
      gl_Position = vec4( 0.0, 0.0, 2.0, 1.0 );
      return;
    }
    if ( haloOwnBehind ) haloOwn.xyz = mix( haloFar.xyz, haloOwn.xyz, ( haloNear - haloFar.z ) / ( haloOwn.z - haloFar.z ) );
    if ( haloFarBehind ) haloFar.xyz = mix( haloOwn.xyz, haloFar.xyz, ( haloNear - haloOwn.z ) / ( haloFar.z - haloOwn.z ) );
    vec4 haloOwnClip = projectionMatrix * haloOwn;
    vec4 haloFarClip = projectionMatrix * haloFar;
    vec2 haloOwnScreen = haloOwnClip.xy / haloOwnClip.w;
    vec2 haloAlong = ( haloOwnScreen - haloFarClip.xy / haloFarClip.w ) * 0.5 * haloViewport;
    float haloLength = length( haloAlong );
    // An edge seen end on has no direction: its two ends then face apart along x, and the caps make a disc.
    haloAlong = haloLength > 1e-4 ? haloAlong / haloLength : vec2( haloSide.y, 0.0 );
    vec2 haloAcross = vec2( -haloAlong.y, haloAlong.x ) * haloSide.x;
    gl_Position = vec4( haloOwnScreen + ( haloAlong + haloAcross ) * ( 2.0 * haloWidth / haloViewport ), -${SHELL_DEPTH}, 1.0 );
    // The place along the edge from its middle (signed by the end), the place across it (in the
    // first end's frame, which the second end's side is the reverse of), and the edge's length.
    vHaloCap = vec3( haloSide.y * ( 0.5 * haloLength + haloWidth ), haloSide.x * haloSide.y * haloWidth, haloLength );
  }
`;

/*
 * Past either end of the edge a fragment is kept only within the width of that end: a round cap.
 * Between the ends the quad is already exactly the width to either side.
 */
const EDGE_SHELL_FRAGMENT = /* glsl */ `
  uniform vec3 haloColour;
  uniform float haloWidth;
  varying vec3 vHaloCap;
  void main() {
    float haloPast = max( abs( vHaloCap.x ) - 0.5 * vHaloCap.z, 0.0 );
    if ( haloPast * haloPast + vHaloCap.y * vHaloCap.y > haloWidth * haloWidth ) discard;
    gl_FragColor = vec4( haloColour, 1.0 );
    #include <colorspace_fragment>
  }
`;

const SHELL_FRAGMENT = /* glsl */ `
  uniform vec3 haloColour;
  void main() {
    gl_FragColor = vec4( haloColour, 1.0 );
    #include <colorspace_fragment>
  }
`;

export type HaloKind = "selection" | "hover";
type ShellForm = "normal" | "edge";

/** The shape's growth rule: a sphere by its normals, anything else by its edges. */
export function haloShellFormFor(geometry: THREE.BufferGeometry): ShellForm {
  return geometry instanceof THREE.SphereGeometry ? "normal" : "edge";
}

/**
 * The edge-quad geometry of a shape: its crease edges (faces more than one degree apart) and its
 * border edges, four vertices and two triangles each. Edges inside a flat face are left out: they
 * are never on a silhouette.
 */
export function haloEdgeGeometryFor(source: THREE.BufferGeometry): THREE.BufferGeometry {
  const edges = new THREE.EdgesGeometry(source, 1);
  const ends = edges.getAttribute("position") as THREE.BufferAttribute;
  const edgeCount = ends.count / 2;
  const position = new Float32Array(edgeCount * 12);
  const other = new Float32Array(edgeCount * 12);
  const side = new Float32Array(edgeCount * 8);
  const index = new Uint32Array(edgeCount * 6);
  for (let edge = 0; edge < edgeCount; edge += 1) {
    // Vertex order round the quad: own end A on either side, then end B on either side. B's own
    // across-direction is A's reversed, so B's first vertex is on A's second vertex's side.
    for (let corner = 0; corner < 4; corner += 1) {
      const own = edge * 2 + (corner < 2 ? 0 : 1);
      const far = edge * 2 + (corner < 2 ? 1 : 0);
      const at = (edge * 4 + corner) * 3;
      position[at] = ends.getX(own); position[at + 1] = ends.getY(own); position[at + 2] = ends.getZ(own);
      other[at] = ends.getX(far); other[at + 1] = ends.getY(far); other[at + 2] = ends.getZ(far);
      side[(edge * 4 + corner) * 2] = corner % 2 === 0 ? 1 : -1;
      side[(edge * 4 + corner) * 2 + 1] = corner < 2 ? 1 : -1;
    }
    const first = edge * 4;
    index.set([first, first + 1, first + 2, first, first + 2, first + 3], edge * 6);
  }
  edges.dispose();
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(position, 3));
  geometry.setAttribute("haloOther", new THREE.BufferAttribute(other, 3));
  geometry.setAttribute("haloSide", new THREE.BufferAttribute(side, 2));
  geometry.setIndex(new THREE.BufferAttribute(index, 1));
  return geometry;
}

type HaloPair = {
  mask: THREE.InstancedMesh;
  shell: THREE.InstancedMesh;
  capacity: number;
  count: number;
};

const ignoreRaycast = (): void => {};
const scratchMatrix = new THREE.Matrix4();
const scratchSize = new THREE.Vector2();

function isIdentity(matrix: THREE.Matrix4): boolean {
  const e = matrix.elements;
  return e[0] === 1 && e[5] === 1 && e[10] === 1 && e[15] === 1 &&
    e[1] === 0 && e[2] === 0 && e[3] === 0 && e[4] === 0 && e[6] === 0 && e[7] === 0 &&
    e[8] === 0 && e[9] === 0 && e[11] === 0 && e[12] === 0 && e[13] === 0 && e[14] === 0;
}

function capacityFor(count: number, current: number): number {
  let capacity = Math.max(current, 1);
  while (capacity < count) capacity *= 2;
  return capacity;
}

export class ViewportHaloPresentation {
  readonly group = new THREE.Group();

  private readonly pairs: Record<HaloKind, Map<THREE.BufferGeometry, HaloPair>> = {
    selection: new Map(),
    hover: new Map()
  };
  private readonly edgeGeometries = new Map<THREE.BufferGeometry, THREE.BufferGeometry>();
  private maskMaterial: THREE.ShaderMaterial | null = null;
  private readonly shellMaterials = new Map<string, THREE.ShaderMaterial>();
  private theme: ViewportPaletteTheme = "light";
  private pixelRatio = 1;
  private readonly viewport = new THREE.Vector2(1, 1);
  private disposed = false;

  constructor(private readonly ledger: HaloLedger | null = null) {
    this.group.name = "viewport-halo";
  }

  /** The halo meshes now drawn or kept: for tests and for the resource's own bookkeeping. */
  pairFor(kind: HaloKind, geometry: THREE.BufferGeometry): Readonly<HaloPair> | null {
    return this.pairs[kind].get(geometry) ?? null;
  }

  /** How many halo instances of a kind are drawn, over every shape. */
  count(kind: HaloKind): number {
    let total = 0;
    for (const pair of this.pairs[kind].values()) total += pair.count;
    return total;
  }

  /** Repaints both halo colours in place. Nothing is created or disposed. */
  setTheme(theme: ViewportPaletteTheme): void {
    this.theme = theme;
    for (const [name, material] of this.shellMaterials) this.paintShell(material, name.startsWith("selection") ? "selection" : "hover");
  }

  /**
   * The drawing buffer's size in device pixels and the renderer's pixel ratio. The halo's width is
   * its CSS width times this ratio. One rule: both are read where the renderer is sized, by the
   * caller, and never per frame.
   */
  setViewport(widthDevicePixels: number, heightDevicePixels: number, pixelRatio: number): void {
    this.viewport.set(Math.max(1, widthDevicePixels), Math.max(1, heightDevicePixels));
    this.pixelRatio = pixelRatio > 0 ? pixelRatio : 1;
    for (const [name, material] of this.shellMaterials) this.paintShell(material, name.startsWith("selection") ? "selection" : "hover");
  }

  /** Reads size and ratio from a renderer; the resource calls it where it sizes the renderer. */
  setViewportFromRenderer(renderer: Pick<THREE.WebGLRenderer, "getDrawingBufferSize" | "getPixelRatio">): void {
    const size = renderer.getDrawingBufferSize(scratchSize);
    this.setViewport(size.x, size.y, renderer.getPixelRatio());
  }

  /**
   * Brings the halos in line with the figure under `roots`: a selection halo instance for every
   * selected element that is not hidden, and one hover halo instance for the hovered element when
   * it is neither selected nor hidden. Pairs whose base geometry is no longer under the roots (a
   * replaced layer) are disposed. Allocates per shape, never per element.
   */
  sync(
    roots: readonly THREE.Object3D[],
    selected: ReadonlySet<EntityKey>,
    hidden: ReadonlySet<EntityKey>,
    hovered: EntityKey | null
  ): void {
    if (this.disposed) return;
    const sources = this.sources(roots);
    this.retire(sources);
    this.fill("selection", sources, (key) => selected.has(key) && !hidden.has(key), selected.size === 0);
    this.syncHoverFrom(sources, selected, hidden, hovered);
  }

  /** The hover halo alone: one scan for the key, one instance written, nothing else touched. */
  syncHover(
    roots: readonly THREE.Object3D[],
    selected: ReadonlySet<EntityKey>,
    hidden: ReadonlySet<EntityKey>,
    hovered: EntityKey | null
  ): void {
    if (this.disposed) return;
    this.syncHoverFrom(this.sources(roots), selected, hidden, hovered);
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    for (const kind of ["selection", "hover"] as const) {
      for (const pair of this.pairs[kind].values()) this.disposePair(pair);
      this.pairs[kind].clear();
    }
    for (const geometry of this.edgeGeometries.values()) geometry.dispose();
    this.ledger?.disposed({ geometries: this.edgeGeometries.size });
    this.edgeGeometries.clear();
    const materials = this.shellMaterials.size + (this.maskMaterial ? 1 : 0);
    this.maskMaterial?.dispose();
    for (const material of this.shellMaterials.values()) material.dispose();
    this.maskMaterial = null;
    this.shellMaterials.clear();
    if (materials > 0) this.ledger?.disposed({ materials });
    this.group.clear();
    this.group.removeFromParent();
  }

  private sources(roots: readonly THREE.Object3D[]): THREE.Mesh[] {
    const sources: THREE.Mesh[] = [];
    for (const root of roots) {
      root.traverse((object) => {
        if (!(object instanceof THREE.Mesh)) return;
        if (object instanceof THREE.InstancedMesh
          ? Array.isArray(object.userData.instanceEntityKeys)
          : typeof object.userData.selectionEntityKey === "string") sources.push(object);
      });
    }
    return sources;
  }

  private retire(sources: readonly THREE.Mesh[]): void {
    const live = new Set<THREE.BufferGeometry>();
    for (const source of sources) live.add(source.geometry);
    for (const kind of ["selection", "hover"] as const) {
      for (const [geometry, pair] of this.pairs[kind]) {
        if (live.has(geometry)) continue;
        this.disposePair(pair);
        this.pairs[kind].delete(geometry);
      }
    }
    for (const [geometry, edges] of this.edgeGeometries) {
      if (live.has(geometry)) continue;
      edges.dispose();
      this.ledger?.disposed({ geometries: 1 });
      this.edgeGeometries.delete(geometry);
    }
  }

  private syncHoverFrom(
    sources: readonly THREE.Mesh[],
    selected: ReadonlySet<EntityKey>,
    hidden: ReadonlySet<EntityKey>,
    hovered: EntityKey | null
  ): void {
    const shown = hovered !== null && !selected.has(hovered) && !hidden.has(hovered) ? hovered : null;
    this.fill("hover", sources, (key) => key === shown, shown === null);
  }

  private fill(kind: HaloKind, sources: readonly THREE.Mesh[], wanted: (key: EntityKey) => boolean, none: boolean): void {
    const pairs = this.pairs[kind];
    for (const pair of pairs.values()) pair.count = 0;
    if (!none) {
      // First the counts, so that capacity grows once per shape; then the matrices.
      const counts = new Map<THREE.BufferGeometry, number>();
      for (const source of sources) {
        let count = 0;
        if (source instanceof THREE.InstancedMesh) {
          const keys = source.userData.instanceEntityKeys as readonly EntityKey[];
          for (let index = 0; index < keys.length; index += 1) if (wanted(keys[index])) count += 1;
        } else if (source.visible && wanted(source.userData.selectionEntityKey as EntityKey)) {
          count = 1;
        }
        if (count > 0) counts.set(source.geometry, (counts.get(source.geometry) ?? 0) + count);
      }
      for (const [geometry, count] of counts) this.ensurePair(kind, geometry, count);
      for (const source of sources) {
        const pair = pairs.get(source.geometry);
        if (!pair || !counts.has(source.geometry)) continue;
        this.write(pair, source, wanted);
      }
    }
    for (const pair of pairs.values()) {
      for (const mesh of [pair.mask, pair.shell]) {
        mesh.count = pair.count;
        mesh.visible = pair.count > 0;
        mesh.instanceMatrix.clearUpdateRanges();
        if (pair.count > 0) mesh.instanceMatrix.addUpdateRange(0, pair.count * 16);
        mesh.instanceMatrix.needsUpdate = pair.count > 0;
      }
    }
  }

  private write(pair: HaloPair, source: THREE.Mesh, wanted: (key: EntityKey) => boolean): void {
    const mask = pair.mask.instanceMatrix.array as Float32Array;
    const shell = pair.shell.instanceMatrix.array as Float32Array;
    const world = isIdentity(source.matrixWorld) ? null : source.matrixWorld;
    const put = (elements: ArrayLike<number>, from: number): void => {
      const at = pair.count * 16;
      if (world) {
        scratchMatrix.fromArray(elements, from).premultiply(world);
        mask.set(scratchMatrix.elements, at);
        shell.set(scratchMatrix.elements, at);
      } else {
        for (let cell = 0; cell < 16; cell += 1) {
          const value = elements[from + cell];
          mask[at + cell] = value;
          shell[at + cell] = value;
        }
      }
      pair.count += 1;
    };
    if (!(source instanceof THREE.InstancedMesh)) {
      if (source.visible && wanted(source.userData.selectionEntityKey as EntityKey)) {
        scratchMatrix.identity();
        put(scratchMatrix.elements, 0);
      }
      return;
    }
    const keys = source.userData.instanceEntityKeys as readonly EntityKey[];
    // The base matrix, never the drawn one: a hidden instance's drawn matrix is zero, and a hidden
    // element is refused by `wanted` before it gets here.
    const base = source.userData.instanceBaseMatrices as readonly THREE.Matrix4[] | undefined;
    const drawn = source.instanceMatrix.array;
    for (let index = 0; index < keys.length; index += 1) {
      if (!wanted(keys[index])) continue;
      if (base) put(base[index].elements, 0);
      else put(drawn, index * 16);
    }
  }

  private ensurePair(kind: HaloKind, geometry: THREE.BufferGeometry, count: number): void {
    const pairs = this.pairs[kind];
    const existing = pairs.get(geometry);
    if (existing && existing.capacity >= count) return;
    const capacity = capacityFor(count, existing?.capacity ?? 0);
    if (existing) this.disposePair(existing);
    const form = haloShellFormFor(geometry);
    const shellGeometry = form === "normal" ? geometry : this.edgeGeometry(geometry);
    const mask = this.haloMesh(geometry, this.mask(), capacity, HALO_MASK_RENDER_ORDER);
    const shell = this.haloMesh(
      shellGeometry, this.shell(kind, form), capacity,
      kind === "selection" ? HALO_SELECTION_RENDER_ORDER : HALO_HOVER_RENDER_ORDER
    );
    mask.name = `viewport-halo-${kind}-mask`;
    shell.name = `viewport-halo-${kind}-shell`;
    this.group.add(mask, shell);
    this.ledger?.created({ instanceMatrices: 2 });
    pairs.set(geometry, { mask, shell, capacity, count: 0 });
  }

  private haloMesh(geometry: THREE.BufferGeometry, material: THREE.Material, capacity: number, renderOrder: number): THREE.InstancedMesh {
    const mesh = new THREE.InstancedMesh(geometry, material, capacity);
    mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    mesh.count = 0;
    mesh.visible = false;
    mesh.renderOrder = renderOrder;
    // One mesh holds a shape's haloed instances from every spatial chunk, so it has no useful
    // bounds of its own; vertex clipping still rejects what is off the screen.
    mesh.frustumCulled = false;
    mesh.matrixAutoUpdate = false;
    mesh.raycast = ignoreRaycast;
    mesh.userData.viewportHalo = true;
    return mesh;
  }

  private disposePair(pair: HaloPair): void {
    this.group.remove(pair.mask, pair.shell);
    // The instance buffers only: the mask's geometry is the figure's, and an edge geometry and the
    // materials are shared between pairs and disposed with this presentation.
    pair.mask.dispose();
    pair.shell.dispose();
    this.ledger?.disposed({ instanceMatrices: 2 });
  }

  private edgeGeometry(source: THREE.BufferGeometry): THREE.BufferGeometry {
    let edges = this.edgeGeometries.get(source);
    if (!edges) {
      edges = haloEdgeGeometryFor(source);
      this.edgeGeometries.set(source, edges);
      this.ledger?.created({ geometries: 1 });
    }
    return edges;
  }

  private mask(): THREE.ShaderMaterial {
    if (!this.maskMaterial) {
      this.maskMaterial = new THREE.ShaderMaterial({
        vertexShader: MASK_VERTEX,
        fragmentShader: MASK_FRAGMENT,
        colorWrite: false,
        // With the depth test off WebGL writes no depth, so the test stays on and always passes.
        depthTest: true,
        depthFunc: THREE.AlwaysDepth,
        depthWrite: true,
        transparent: true,
        blending: THREE.NoBlending,
        lights: false,
        toneMapped: false
      });
      this.maskMaterial.name = "viewport-halo-mask";
      this.ledger?.created({ materials: 1 });
    }
    return this.maskMaterial;
  }

  private shell(kind: HaloKind, form: ShellForm): THREE.ShaderMaterial {
    const name = `${kind}-${form}`;
    let material = this.shellMaterials.get(name);
    if (!material) {
      material = new THREE.ShaderMaterial({
        uniforms: {
          haloColour: { value: new THREE.Color() },
          haloWidth: { value: 1 },
          haloViewport: { value: new THREE.Vector2(1, 1) }
        },
        vertexShader: form === "normal" ? NORMAL_SHELL_VERTEX : EDGE_SHELL_VERTEX,
        fragmentShader: form === "normal" ? SHELL_FRAGMENT : EDGE_SHELL_FRAGMENT,
        depthTest: true,
        depthFunc: THREE.LessEqualDepth,
        depthWrite: false,
        transparent: true,
        blending: THREE.NoBlending,
        // An edge quad faces either way, and a grown sphere is closed: neither needs culling.
        side: form === "edge" ? THREE.DoubleSide : THREE.FrontSide,
        lights: false,
        toneMapped: false
      });
      material.name = `viewport-halo-${name}`;
      this.paintShell(material, kind);
      this.shellMaterials.set(name, material);
      this.ledger?.created({ materials: 1 });
    }
    return material;
  }

  private paintShell(material: THREE.ShaderMaterial, kind: HaloKind): void {
    const token = kind === "selection" ? "canvas.selection" : "canvas.hover";
    (material.uniforms.haloColour.value as THREE.Color).setHex(viewportTokenColour(this.theme, token).hex);
    material.uniforms.haloWidth.value = (kind === "selection" ? SELECTION_HALO_CSS_PX : HOVER_HALO_CSS_PX) * this.pixelRatio;
    (material.uniforms.haloViewport.value as THREE.Vector2).copy(this.viewport);
  }
}
