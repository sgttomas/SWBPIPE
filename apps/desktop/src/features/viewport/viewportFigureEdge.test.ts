import * as THREE from "three";
import { describe, expect, it, vi } from "vitest";
import { entityKey } from "../workspace/selectionState";
import {
  createFigureMaterial,
  figureEdgeOutlineFor,
  hasFigureEdge,
  isFigureMaterial,
  setFigureEdge,
  type FigureEdgeOutline,
  type FigureMaterial
} from "./viewportFigureMaterial";
import { viewportRoleHex, viewportShadeRatio, viewportTokenColour } from "./viewportPalette";
import {
  applyVisibilityPresentation,
  currentOwnedViewportResourceSnapshot,
  registerInstancedRolePresentation,
  registerPaletteRole,
  ViewportOwnershipLedger,
  ViewportResource
} from "./viewportResource";

// The figure's shader as it stood before the edge line existed. A material built without the
// line must compile exactly this, byte for byte.
const SHADER_WITHOUT_EDGE = {
  vertex: `
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
`,
  fragment: `
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
`
};

const PIPE_OUTLINE: FigureEdgeOutline = { form: "prism", sides: 10, radius: 1 };
const OUTLINES: readonly (readonly [string, FigureEdgeOutline])[] = [
  ["the pipe's prism", PIPE_OUTLINE],
  ["a placeholder prism", { form: "prism", sides: 12, radius: 0.11 }],
  ["a box", { form: "box", halfExtents: [0.12, 0.12, 0.12] }],
  ["a torus", { form: "torus", sides: 8, ringRadius: 0.24, tubeRadius: 0.027 }]
];

const edgeHex = (material: FigureMaterial) => (material.uniforms.edge.value as THREE.Color).getHex();
const tokenHex = (theme: "light" | "dark") => viewportTokenColour(theme, "canvas.edge").hex;

describe("figure material: the edge line is an option", () => {
  it("compiles the shader it always had, byte for byte, when no line is asked for", () => {
    for (const material of [
      createFigureMaterial(),
      createFigureMaterial({ opacity: 0.82, transparent: true }),
      createFigureMaterial({ edge: null })
    ]) {
      expect(material.vertexShader).toBe(SHADER_WITHOUT_EDGE.vertex);
      expect(material.fragmentShader).toBe(SHADER_WITHOUT_EDGE.fragment);
      expect(Object.keys(material.uniforms).sort()).toEqual(["opacity", "shadeRatio", "tint"]);
      expect(material.defines).toEqual({});
      expect(hasFigureEdge(material)).toBe(false);
      for (const word of ["edge", "dFdx", "dFdy", "FIGURE_EDGE"]) {
        expect(`${material.vertexShader}${material.fragmentShader}`).not.toContain(word);
      }
      material.dispose();
    }
  });

  it.each(OUTLINES)("adds a colour and a width uniform, and nothing else the ledger could own, for %s", (_name, outline) => {
    const material = createFigureMaterial({ edge: outline });
    expect(isFigureMaterial(material)).toBe(true);
    expect(hasFigureEdge(material)).toBe(true);
    expect(Object.keys(material.uniforms).sort()).toEqual(["edge", "edgeWidth", "opacity", "shadeRatio", "tint"]);
    expect(material.uniforms.edge.value).toBeInstanceOf(THREE.Color);
    expect(material.uniforms.edgeWidth.value).toBe(1);
    const values = [...Object.values(material), ...Object.values(material.uniforms).map((uniform) => uniform.value)];
    expect(values.filter((value) => value instanceof THREE.Texture)).toEqual([]);
    // Still the same unlit, untoned, opaque figure.
    expect(material.lights).toBe(false);
    expect(material.toneMapped).toBe(false);
    expect(material.transparent).toBe(false);
    expect(material.side).toBe(THREE.FrontSide);
    // The line is mixed over the same shaded colour the figure always drew.
    expect(material.fragmentShader).toContain("mix( base * shadeRatio, base, smoothstep( 0.0, 0.75, facing ) )");
    expect(material.fragmentShader).toContain("uniform vec3 edge;");
    expect(material.fragmentShader).toContain("uniform float edgeWidth;");
    const includes = [...`${material.vertexShader}\n${material.fragmentShader}`.matchAll(/#include <([a-z_]+)>/g)];
    expect(includes.length).toBeGreaterThanOrEqual(8);
    for (const [, chunk] of includes) expect(THREE.ShaderChunk).toHaveProperty(chunk);
    material.dispose();
  });

  it.each(OUTLINES)("takes every screen-space derivative outside control flow for %s", (_name, outline) => {
    // A derivative inside non-uniform control flow is undefined. The fragment shader has no `if`,
    // no loop and no early exit; a choice is made by arithmetic or a selection after the derivatives.
    const { fragmentShader } = createFigureMaterial({ edge: outline });
    expect(fragmentShader).toMatch(/dFdx/);
    const body = fragmentShader.slice(fragmentShader.indexOf("void main()"));
    for (const flow of [/\bif\s*\(/, /\bfor\s*\(/, /\bwhile\s*\(/, /\bdiscard\b/, /\breturn\b/]) {
      expect(body).not.toMatch(flow);
    }
    // The helpers before main() hold no flow either: each is straight-line arithmetic.
    const helpers = fragmentShader.slice(0, fragmentShader.indexOf("void main()"));
    for (const flow of [/\bif\s*\(/, /\bfor\s*\(/, /\bwhile\s*\(/, /\bdiscard\b/]) expect(helpers).not.toMatch(flow);
  });

  it("states each kind's constants as defines, so each kind compiles a program of its own", () => {
    const defines = OUTLINES.map(([, outline]) => createFigureMaterial({ edge: outline }).defines);
    expect(defines[0]).toEqual({ FIGURE_EDGE_SIDES: "10.0", FIGURE_EDGE_RADIUS: "1.0" });
    expect(defines[1]).toEqual({ FIGURE_EDGE_SIDES: "12.0", FIGURE_EDGE_RADIUS: "0.11" });
    expect(defines[2]).toEqual({ FIGURE_EDGE_HALF_EXTENTS: "vec3( 0.12, 0.12, 0.12 )" });
    expect(defines[3]).toEqual({ FIGURE_EDGE_SIDES: "8.0", FIGURE_EDGE_RING_RADIUS: "0.24", FIGURE_EDGE_TUBE_RADIUS: "0.027" });
    // The two prisms share their source and differ in their defines alone.
    const [pipe, placeholder] = [OUTLINES[0][1], OUTLINES[1][1]].map((edge) => createFigureMaterial({ edge }));
    expect(pipe.vertexShader).toBe(placeholder.vertexShader);
    expect(pipe.fragmentShader).toBe(placeholder.fragmentShader);
  });

  it("refuses an outline that is not a shape", () => {
    const bad: FigureEdgeOutline[] = [
      { form: "prism", sides: 2, radius: 1 },
      { form: "prism", sides: 10.5, radius: 1 },
      { form: "prism", sides: 10, radius: 0 },
      { form: "prism", sides: 10, radius: Number.NaN },
      { form: "box", halfExtents: [0.12, 0, 0.12] },
      { form: "torus", sides: 8, ringRadius: 0.02, tubeRadius: 0.027 },
      { form: "torus", sides: 8, ringRadius: 0.24, tubeRadius: Number.POSITIVE_INFINITY }
    ];
    for (const edge of bad) expect(() => createFigureMaterial({ edge })).toThrow(/edge outline/i);
  });

  it("reads an outline from the geometry it will be drawn on, and none from a shape it has no rule for", () => {
    expect(figureEdgeOutlineFor(new THREE.CylinderGeometry(1, 1, 1, 10, 1, false))).toEqual(PIPE_OUTLINE);
    expect(figureEdgeOutlineFor(new THREE.CylinderGeometry(0.11, 0.11, 0.34, 12))).toEqual(
      { form: "prism", sides: 12, radius: 0.11 }
    );
    expect(figureEdgeOutlineFor(new THREE.BoxGeometry(0.24, 0.24, 0.24))).toEqual(
      { form: "box", halfExtents: [0.12, 0.12, 0.12] }
    );
    expect(figureEdgeOutlineFor(new THREE.TorusGeometry(0.24, 0.027, 8, 18, Math.PI * 0.75))).toEqual(
      { form: "torus", sides: 8, ringRadius: 0.24, tubeRadius: 0.027 }
    );
    // A cone, a tapered tube, an open tube, a tube in rings, a subdivided box and a sphere have no rule.
    expect(figureEdgeOutlineFor(new THREE.ConeGeometry(0.18, 0.34, 4))).toBeNull();
    expect(figureEdgeOutlineFor(new THREE.CylinderGeometry(1, 0.5, 1, 10))).toBeNull();
    expect(figureEdgeOutlineFor(new THREE.CylinderGeometry(1, 1, 1, 10, 1, true))).toBeNull();
    expect(figureEdgeOutlineFor(new THREE.CylinderGeometry(1, 1, 1, 10, 1, false, 0, Math.PI))).toBeNull();
    expect(figureEdgeOutlineFor(new THREE.BoxGeometry(1, 1, 1, 2, 1, 1))).toBeNull();
    expect(figureEdgeOutlineFor(new THREE.SphereGeometry(0.095, 12, 8))).toBeNull();
  });

  it("paints the line's colour and width in place", () => {
    const material = createFigureMaterial({ edge: PIPE_OUTLINE });
    const colour = material.uniforms.edge.value;
    setFigureEdge(material, tokenHex("dark"), 2);
    expect(material.uniforms.edge.value).toBe(colour);
    expect(edgeHex(material)).toBe(tokenHex("dark"));
    expect(material.uniforms.edgeWidth.value).toBe(2);
    // The colour is held as the tint is: in three's working space, so the drawn pixel is the token.
    const asTint = new THREE.Color().setHex(tokenHex("dark"));
    expect((colour as THREE.Color).toArray()).toEqual(asTint.toArray());
    // A material without the line is left exactly as it was.
    const plain = createFigureMaterial();
    setFigureEdge(plain, tokenHex("dark"), 2);
    expect(Object.keys(plain.uniforms).sort()).toEqual(["opacity", "shadeRatio", "tint"]);
    material.dispose();
    plain.dispose();
  });

  it.each(OUTLINES)("a clone of a material with the line for %s is a whole one, with a colour of its own", (_name, outline) => {
    const source = createFigureMaterial({ edge: outline });
    setFigureEdge(source, tokenHex("dark"), 2);
    const clone = source.clone();
    expect(isFigureMaterial(clone)).toBe(true);
    expect(hasFigureEdge(clone)).toBe(true);
    expect(clone.vertexShader).toBe(source.vertexShader);
    expect(clone.fragmentShader).toBe(source.fragmentShader);
    expect(clone.defines).toEqual(source.defines);
    expect(clone.defines).not.toBe(source.defines);
    expect(clone.uniforms.edge.value).not.toBe(source.uniforms.edge.value);
    expect(edgeHex(clone)).toBe(tokenHex("dark"));
    expect(clone.uniforms.edgeWidth.value).toBe(2);
    setFigureEdge(clone, tokenHex("light"), 1);
    expect(edgeHex(source)).toBe(tokenHex("dark"));
    expect(source.uniforms.edgeWidth.value).toBe(2);
    expect(clone.color).toBe(clone.uniforms.tint.value);
    source.dispose();
    clone.dispose();
  });

  it("constructs and clones every kind without a console warning or error", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => undefined);
    const error = vi.spyOn(console, "error").mockImplementation(() => undefined);
    try {
      for (const [, edge] of OUTLINES) {
        const material = createFigureMaterial({ edge });
        const clone = material.clone();
        material.dispose();
        clone.dispose();
      }
      expect(warn).not.toHaveBeenCalled();
      expect(error).not.toHaveBeenCalled();
    } finally {
      warn.mockRestore();
      error.mockRestore();
    }
  });
});

describe("viewport edge line: repaint, ownership, hiding and selection", () => {
  const instanceHex = (mesh: THREE.InstancedMesh, index: number) =>
    new THREE.Color().fromBufferAttribute(mesh.instanceColor!, index).getHex();

  function edgeResource(theme: "light" | "dark", figureEdgeWidth: number | undefined, selectedKeys: readonly string[] = []) {
    const layers = {
      modelLayer: new THREE.Group(),
      authoredLoadLayer: new THREE.Group(),
      resultLayer: new THREE.Group(),
      diagnosticLayer: new THREE.Group(),
      routingLayer: new THREE.Group()
    };
    const invalidate = vi.fn();
    const ownership = new ViewportOwnershipLedger(31);
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      ownership,
      resourceGeneration: 31,
      contextLostCount: 0,
      contextRestoredCount: 0,
      scheduler: { pendingCount: 0 },
      renderer: { domElement: { isConnected: true } },
      options: {},
      invalidate,
      scene: new THREE.Scene(),
      gizmoScene: new THREE.Scene(),
      selectionPresentation: null,
      ...layers,
      selectedKeys: new Set(selectedKeys),
      hiddenKeys: new Set(),
      themePresentation: theme,
      gridVisible: true,
      authoredLoadsVisible: true,
      ...(figureEdgeWidth === undefined ? {} : { figureEdgeWidth })
    }) as ViewportResource;
    return { resource, ownership, invalidate, ...layers };
  }

  const KEYS = [entityKey({ type: "pipe", id: "p:1" }), entityKey({ type: "pipe", id: "p:2" })];

  function edgedPipes(keys: readonly ReturnType<typeof entityKey>[] = KEYS) {
    const geometry = new THREE.CylinderGeometry(1, 1, 1, 10, 1, false);
    const mesh = new THREE.InstancedMesh(geometry, createFigureMaterial({ edge: figureEdgeOutlineFor(geometry) }), keys.length);
    const matrix = new THREE.Matrix4();
    keys.forEach((_key, index) => {
      matrix.compose(new THREE.Vector3(index, 0, 0), new THREE.Quaternion(), new THREE.Vector3(0.052, 3, 0.052));
      mesh.setMatrixAt(index, matrix);
    });
    registerInstancedRolePresentation(mesh, keys, "pipe");
    return mesh;
  }

  it("paints the line for the default theme at one device pixel when a mesh is registered", () => {
    const mesh = edgedPipes();
    const material = mesh.material as FigureMaterial;
    expect(edgeHex(material)).toBe(tokenHex("light"));
    expect(material.uniforms.edgeWidth.value).toBe(1);
  });

  it("repaints the line with the theme, at the resource's pixel ratio, in place, creating and disposing nothing", () => {
    const { resource, ownership, invalidate, modelLayer, diagnosticLayer } = edgeResource("light", 2);
    const pipes = edgedPipes();
    const material = pipes.material as FigureMaterial;
    const node = new THREE.Mesh(new THREE.SphereGeometry(0.1, 8, 6), createFigureMaterial());
    registerPaletteRole(node, "node");
    resource.replaceLayer(modelLayer, [pipes]);
    resource.replaceLayer(diagnosticLayer, [node]);
    expect(tokenHex("dark")).not.toBe(tokenHex("light"));
    // A rebuilt layer is painted for the resource's theme and ratio without a theme change.
    expect(edgeHex(material)).toBe(tokenHex("light"));
    expect(material.uniforms.edgeWidth.value).toBe(2);

    const uniforms = { edge: material.uniforms.edge, colour: material.uniforms.edge.value, width: material.uniforms.edgeWidth };
    const disposals = [pipes, node].flatMap((object) => [
      vi.spyOn(object.geometry, "dispose"),
      vi.spyOn(object.material as THREE.Material, "dispose")
    ]);
    const ledgerBefore = ownership.snapshot();
    const resourceSnapshotBefore = currentOwnedViewportResourceSnapshot();
    const versionBefore = material.version;
    invalidate.mockClear();

    for (const theme of ["dark", "light", "dark", "light"] as const) {
      resource.setThemePresentation(theme);
      expect(edgeHex(material)).toBe(tokenHex(theme));
      expect(material.uniforms.edgeWidth.value).toBe(2);
      expect((material.uniforms.shadeRatio.value as THREE.Color).toArray()).toEqual([...viewportShadeRatio(theme)]);
      expect(instanceHex(pipes, 0)).toBe(viewportRoleHex(theme, "pipe"));
    }
    expect(invalidate).toHaveBeenCalledTimes(4);
    expect(material.uniforms.edge).toBe(uniforms.edge);
    expect(material.uniforms.edge.value).toBe(uniforms.colour);
    expect(material.uniforms.edgeWidth).toBe(uniforms.width);
    // No recompilation was asked for: a repaint is uniform values alone.
    expect(material.version).toBe(versionBefore);
    expect(pipes.material).toBe(material);
    expect(ownership.snapshot()).toEqual(ledgerBefore);
    expect(currentOwnedViewportResourceSnapshot()).toBe(resourceSnapshotBefore);
    for (const dispose of disposals) expect(dispose).not.toHaveBeenCalled();
    // The material without the line was repainted as before and gained nothing.
    expect(Object.keys((node.material as FigureMaterial).uniforms).sort()).toEqual(["opacity", "shadeRatio", "tint"]);
  });

  it("draws one device pixel when the resource states no ratio", () => {
    const { resource, modelLayer } = edgeResource("dark", undefined);
    const pipes = edgedPipes();
    resource.replaceLayer(modelLayer, [pipes]);
    expect(edgeHex(pipes.material as FigureMaterial)).toBe(tokenHex("dark"));
    expect((pipes.material as FigureMaterial).uniforms.edgeWidth.value).toBe(1);
  });

  it("adds no object, no geometry, no attribute and no texture: the line is the element's own drawing", () => {
    const plainGeometry = new THREE.CylinderGeometry(1, 1, 1, 10, 1, false);
    const plain = new THREE.InstancedMesh(plainGeometry, createFigureMaterial(), KEYS.length);
    registerInstancedRolePresentation(plain, KEYS, "pipe");
    const edged = edgedPipes();
    expect(edged.children).toEqual([]);
    expect(Object.keys(edged.geometry.attributes).sort()).toEqual(Object.keys(plainGeometry.attributes).sort());
    expect(Object.keys(edged.userData).sort()).toEqual(Object.keys(plain.userData).sort());

    const counts = [plain, edged].map((mesh) => {
      const ledger = new ViewportOwnershipLedger(32);
      ledger.createObjects([mesh]);
      return ledger.snapshot().live;
    });
    expect(counts[1]).toEqual(counts[0]);
    expect(counts[1]).toMatchObject({ pipeMeshes: 1, geometries: 1, materials: 1, textures: 0, instanceMatrices: 1, instanceColors: 1 });

    const { resource, modelLayer } = edgeResource("light", 2);
    resource.replaceLayer(modelLayer, [edged]);
    const drawn: THREE.Object3D[] = [];
    modelLayer.traverse((object) => drawn.push(object));
    // The layer and the one mesh it was given: nothing a pick, a box or a filter could newly meet.
    expect(drawn).toEqual([modelLayer, edged]);
  });

  it("hides a line with its element by the same zero matrix, and by nothing else", () => {
    const edged = edgedPipes();
    const before = new THREE.Matrix4();
    edged.getMatrixAt(1, before);
    applyVisibilityPresentation([edged], new Set([KEYS[0]]));
    const hidden = new THREE.Matrix4();
    edged.getMatrixAt(0, hidden);
    expect(hidden.elements).toEqual(new THREE.Matrix4().makeScale(0, 0, 0).elements);
    const kept = new THREE.Matrix4();
    edged.getMatrixAt(1, kept);
    expect(kept.elements).toEqual(before.elements);
    expect(edged.visible).toBe(true);
    expect(edged.count).toBe(KEYS.length);
    applyVisibilityPresentation([edged], new Set());
    edged.getMatrixAt(0, hidden);
    expect(hidden.elements).toEqual((edged.userData.instanceBaseMatrices as THREE.Matrix4[])[0].elements);
  });

  it("keeps the line in canvas.edge over a selected element's held colour", () => {
    const { resource, modelLayer } = edgeResource("light", 1, [KEYS[0]]);
    const edged = edgedPipes();
    resource.replaceLayer(modelLayer, [edged]);
    for (const theme of ["dark", "light"] as const) {
      resource.setThemePresentation(theme);
      expect(instanceHex(edged, 0)).not.toBe(viewportRoleHex(theme, "pipe"));
      expect(instanceHex(edged, 1)).toBe(viewportRoleHex(theme, "pipe"));
      expect(edgeHex(edged.material as FigureMaterial)).toBe(tokenHex(theme));
    }
    resource.setSelectionPresentation([]);
    expect(instanceHex(edged, 0)).toBe(viewportRoleHex("light", "pipe"));
    expect(edgeHex(edged.material as FigureMaterial)).toBe(tokenHex("light"));
  });
});
