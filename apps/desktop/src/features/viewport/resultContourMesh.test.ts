import * as THREE from "three";
import { describe, expect, it, vi } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import { entityKey } from "../workspace/selectionState";
import { buildResultColourOverlay, RESULT_COLOUR_QUANTITIES, resultOverlayCases, type ResultColourResolution } from "./resultColourOverlay";
import { contourRings } from "./resultContour";
import {
  buildResultContourMesh,
  CONTOUR_BAND_MAX_HALF_FRACTION,
  CONTOUR_RADIAL_SEGMENTS,
  CONTOUR_RADIUS_FACTOR,
  contourBandHalfFraction,
  contourMeshMembers,
  contourRingHex,
  paintResultContourTheme
} from "./resultContourMesh";
import { resultNoValueHex, resultScaleHex, viewportRoleHex } from "./viewportPalette";
import { ViewportOwnershipLedger, ViewportResource } from "./viewportResource";

// The contour sleeves (A4; DEL-07-05 Design `result-contour.md` §3, §8, §9): which members get one,
// their rings and colours in both themes, and the resource's theme repaint. Invented data only.

// Rows in the producer's section convention, so the signed axial force blends where signs agree.
const at = (location: string) => ({
  component: "axial_force", coordinate_system: "element_local", location, basis: "invented",
  sign_convention: "positive value follows the j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction"
});
const NODES = new Map([
  ["node:A", { x: 0, y: 0, z: 0 }],
  ["node:B", { x: 1, y: 0, z: 0 }],
  ["node:C", { x: 2, y: 0, z: 0 }],
  ["node:D", { x: 2, y: 1, z: 0 }]
]);
const P1 = entityKey({ type: "pipe", id: "pipe:P-1" });
const P2 = entityKey({ type: "pipe", id: "pipe:P-2" });

function overlay(resolution: ResultColourResolution = "station", eligible = true) {
  const model = overlayModel();
  const result = overlayResult([
    row("a:1:i", "element_local_axial_force", "pipe:P-1", 0, "N", { metadata: at("end_i") }),
    row("a:1:m", "element_local_axial_force", "pipe:P-1", 5, "N", { metadata: at("midspan") }),
    row("a:1:j", "element_local_axial_force", "pipe:P-1", 10, "N", { metadata: at("end_j") }),
    row("a:2:i", "element_local_axial_force", "pipe:P-2", 2, "N", { metadata: at("end_i") }),
    row("a:2:j", "element_local_axial_force", "pipe:P-2", 2, "N", { metadata: at("end_j") }),
    row("a:3:m", "element_local_axial_force", "pipe:P-3", 4, "N", { metadata: at("midspan") })
  ]);
  return {
    model,
    overlay: buildResultColourOverlay({
      model, result, standing: healthStanding({ eligible }), resolution,
      quantity: RESULT_COLOUR_QUANTITIES.find((item) => item.id === "axial-force")!,
      overlayCase: resultOverlayCases(model, result)[0]
    })
  };
}

const members = (built = overlay(), extra: { excluded?: Set<ReturnType<typeof entityKey>>; threshold?: number | null } = {}) =>
  contourMeshMembers(built.model, built.overlay, { nodes: NODES, radii: new Map([[P2, 0.2]]), defaultRadius: 0.05, ...extra });

describe("contour sleeve members", () => {
  it("are the members with two or more stated stations, each with its own tube radius", () => {
    const drawn = members();
    expect([...drawn.keys()]).toEqual([P1, P2]);
    expect(drawn.get(P1)!.radius).toBe(0.05);
    expect(drawn.get(P2)!.radius).toBe(0.2);
    // Each station a band one sleeve diameter long: here 0.05 × 1.12 either side, on a member of length 1.
    const half = 0.05 * 1.12;
    const p1 = drawn.get(P1)!.rings;
    expect(p1.slice(0, 2).map((ring) => ring.fraction)).toEqual([0, half]);
    expect(p1.filter((ring) => Math.abs(ring.fraction - (0.5 - half)) < 1e-12 || Math.abs(ring.fraction - (0.5 + half)) < 1e-12).length).toBeGreaterThan(0);
    expect(p1).toEqual(contourRings(overlay().overlay.members.get("pipe:P-1")!.stations, { maximum: 10, bandHalfFraction: half }));
  });

  it("leave out hidden and dimmed members, one value per member, and an overlay that is held", () => {
    expect([...members(overlay(), { excluded: new Set([P1]) }).keys()]).toEqual([P2]);
    expect(members(overlay("member")).size).toBe(0);
    const held = overlay("station", false);
    expect(held.overlay.state).toBe("held");
    expect(members(held).size).toBe(0);
    expect(contourMeshMembers(held.model, null, { nodes: NODES, radii: new Map(), defaultRadius: 0.05 }).size).toBe(0);
  });

  it("carry the threshold into their rings", () => {
    const rings = members(overlay(), { threshold: 5 }).get(P1)!.rings;
    // The midspan value equals the threshold: its band is plain, and the hard edge is at the band's
    // far edge, where the blend rises above it; above it everything is coloured.
    const edge = 0.5 + 0.05 * 1.12;
    expect(rings.filter((ring) => ring.fraction < edge).every((ring) => ring.kind === "below")).toBe(true);
    expect(rings.filter((ring) => ring.fraction === edge).map((ring) => ring.kind)).toEqual(["below", "value"]);
    expect(rings.filter((ring) => ring.fraction > edge).every((ring) => ring.kind === "value")).toBe(true);
    expect(members(overlay(), { threshold: 5 }).get(P2)!.rings.every((ring) => ring.kind === "below")).toBe(true);
  });
});

describe("contour sleeve mesh", () => {
  it("has one ring of vertices per contour ring, outside the member's tube, and is never picked", () => {
    const drawn = [...members().values()];
    const mesh = buildResultContourMesh(drawn, "light")!;
    const ringCount = drawn.reduce((count, member) => count + member.rings.length, 0);
    const position = mesh.geometry.getAttribute("position");
    expect(position.count).toBe(ringCount * CONTOUR_RADIAL_SEGMENTS);
    expect(mesh.geometry.getIndex()!.count).toBe(drawn.reduce((count, member) => count + (member.rings.length - 1) * CONTOUR_RADIAL_SEGMENTS * 6, 0));
    // P-1 lies on x: every vertex of its first ring is at its tube radius times the factor from the axis.
    for (let vertex = 0; vertex < CONTOUR_RADIAL_SEGMENTS; vertex += 1) {
      expect(Math.hypot(position.getY(vertex), position.getZ(vertex))).toBeCloseTo(0.05 * CONTOUR_RADIUS_FACTOR, 6);
      expect(position.getX(vertex)).toBeCloseTo(0, 6);
    }
    const hits: THREE.Intersection[] = [];
    mesh.raycast(new THREE.Raycaster(new THREE.Vector3(0.5, 5, 0), new THREE.Vector3(0, -1, 0)), hits);
    expect(hits).toEqual([]);
    expect(mesh.userData.viewportContourMembers).toEqual([P1, P2]);
    expect(buildResultContourMesh([], "light")).toBeNull();
  });

  it("colours a ring by its scale position, the pipe colour below the threshold and the no-value colour in a gap", () => {
    for (const theme of ["light", "dark"] as const) {
      expect(contourRingHex(theme, { fraction: 0, kind: "value", position: 0.4 })).toBe(resultScaleHex(theme, 0.4));
      expect(contourRingHex(theme, { fraction: 0, kind: "below", position: 0 })).toBe(viewportRoleHex(theme, "pipe"));
      expect(contourRingHex(theme, { fraction: 0, kind: "gap", position: 0 })).toBe(resultNoValueHex(theme));
    }
  });

  it("holds both themes and repaints in place, through the resource's own palette path", () => {
    const drawn = [...members().values()];
    const mesh = buildResultContourMesh(drawn, "light")!;
    const colour = mesh.geometry.getAttribute("color") as THREE.BufferAttribute;
    const lastVertex = drawn[0].rings.length * CONTOUR_RADIAL_SEGMENTS - 1;
    const hexAt = (index: number) => new THREE.Color().fromBufferAttribute(colour, index).getHex();
    expect(hexAt(0)).toBe(resultScaleHex("light", 0));
    expect(hexAt(lastVertex)).toBe(resultScaleHex("light", 1));
    paintResultContourTheme(mesh, "dark");
    expect(hexAt(0)).toBe(resultScaleHex("dark", 0));
    expect(hexAt(lastVertex)).toBe(resultScaleHex("dark", 1));
    expect(mesh.geometry.getAttribute("color")).toBe(colour);

    // The resource's theme change reaches the result layer's sleeve.
    const layers = { modelLayer: new THREE.Group(), authoredLoadLayer: new THREE.Group(), resultLayer: new THREE.Group(), diagnosticLayer: new THREE.Group(), routingLayer: new THREE.Group() };
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      ownership: new ViewportOwnershipLedger(7), resourceGeneration: 7, contextLostCount: 0, contextRestoredCount: 0, scheduler: { pendingCount: 0 }, invalidate: vi.fn(), scene: new THREE.Scene(), gizmoScene: new THREE.Scene(),
      selectionPresentation: null, ...layers, selectedKeys: new Set(), hiddenKeys: new Set(), dimmedKeys: new Set(),
      themePresentation: "dark", gridVisible: true, authoredLoadsVisible: true, resultColours: null, figureEdgeWidth: 1, halo: null,
      renderer: { domElement: { isConnected: true }, getPixelRatio: () => 1 }, options: {}
    }) as ViewportResource;
    resource.replaceLayer(resource.resultLayer, [buildResultContourMesh(drawn, "light")!]);
    const inLayer = resource.resultLayer.children[0] as THREE.Mesh;
    // Taken into the layer, it is painted for the resource's theme.
    expect(new THREE.Color().fromBufferAttribute(inLayer.geometry.getAttribute("color") as THREE.BufferAttribute, 0).getHex()).toBe(resultScaleHex("dark", 0));
    resource.setThemePresentation("light");
    expect(new THREE.Color().fromBufferAttribute(inLayer.geometry.getAttribute("color") as THREE.BufferAttribute, 0).getHex()).toBe(resultScaleHex("light", 0));
  });

  it("gives every stated station triangles of non-zero area in its own value colour, an isolated station between two gaps included", () => {
    const model = overlayModel();
    const sign = (location: string, convention: string) => ({ ...at(location), sign_convention: convention });
    const SECTION = "positive value follows the j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction";
    const I_END = "positive value follows the element-local DOF at the i-end force vector";
    const J_END = "positive value follows the element-local DOF at the j-end force vector";
    // Section signs −, +, − in both fixtures: both intervals are gaps, and the midspan station sits
    // between them. The first states every row in the section convention (signs as stated); the
    // second states the end rows as node-on-element end actions (converted).
    const fixtures: Array<[string, Array<[string, number, string]>]> = [
      ["section rows", [["end_i", -4, SECTION], ["midspan", 2, SECTION], ["end_j", -3, SECTION]]],
      ["end-action rows", [["end_i", 4, I_END], ["midspan", 2, SECTION], ["end_j", -3, J_END]]]
    ];
    for (const [name, stations] of fixtures) {
      const result = overlayResult(stations.map(([location, value, convention]) =>
        row(`${name}:${location}`, "element_local_axial_force", "pipe:P-1", value, "N", { metadata: sign(location, convention) })));
      const built = buildResultColourOverlay({
        model, result, standing: healthStanding({ eligible: true }), resolution: "station",
        quantity: RESULT_COLOUR_QUANTITIES.find((item) => item.id === "axial-force")!, overlayCase: resultOverlayCases(model, result)[0]
      });
      const drawn = [...contourMeshMembers(model, built, { nodes: NODES, radii: new Map(), defaultRadius: 0.05 }).values()];
      const mesh = buildResultContourMesh(drawn, "light")!;
      const position = mesh.geometry.getAttribute("position");
      const colour = mesh.geometry.getAttribute("color") as THREE.BufferAttribute;
      const index = mesh.geometry.getIndex()!;
      const hex = (vertex: number) => new THREE.Color().fromBufferAttribute(colour, vertex).getHex();
      const point = (vertex: number) => new THREE.Vector3().fromBufferAttribute(position, vertex);
      const areaInColour = (target: number) => {
        let area = 0;
        for (let at = 0; at < index.count; at += 3) {
          const [a, b, c] = [index.getX(at), index.getX(at + 1), index.getX(at + 2)];
          if (hex(a) !== target || hex(b) !== target || hex(c) !== target) continue;
          area += new THREE.Triangle(point(a), point(b), point(c)).getArea();
        }
        return area;
      };
      // Behaviour first: every stated station has drawn area in its own value colour.
      for (const [location, value] of stations) {
        expect(areaInColour(resultScaleHex("light", Math.abs(value) / 4)), `${name} ${location}`).toBeGreaterThan(0);
      }
      // And the two intervals are gaps: area in the no-value colour.
      expect(areaInColour(resultNoValueHex("light")), name).toBeGreaterThan(0);
      expect(built.signGapIntervalCount, name).toBe(2);
    }
  });

  it("makes a band one sleeve diameter long, at most a sixteenth of the member either side", () => {
    expect(contourBandHalfFraction(0.05, 1)).toBeCloseTo(0.05 * CONTOUR_RADIUS_FACTOR, 12);
    expect(contourBandHalfFraction(1, 1)).toBe(CONTOUR_BAND_MAX_HALF_FRACTION);
  });

  it("leaves any other object alone when repainting", () => {
    const plain = new THREE.Mesh(new THREE.BufferGeometry(), new THREE.MeshBasicMaterial());
    expect(() => paintResultContourTheme(plain, "dark")).not.toThrow();
  });
});
