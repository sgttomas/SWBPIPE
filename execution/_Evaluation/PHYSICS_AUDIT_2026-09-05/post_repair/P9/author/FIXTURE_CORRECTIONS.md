# Preintegration fixture/interface corrections

V1 compiled but every model blocked because per-record invented provenance was missing. V2 sets explicit invented-original provenance on consumed fixture records. No numerical input or expected quantity changed.

V2 exposed eight expected numerical defects plus two fixture issues: moment directions require RX/RZ (global_x/global_z are translation); source-edge test needs an actual derived component result (ordinary rows have no source edges). V3 corrects rotational DOF labels and adds a metadata-only bend marker at root with invented unit multipliers, geometry-only consumption, and no changed numerical assertion. Existing source-link invariant is unchanged. This marker supplies observation reachability; it is not a new physical benchmark or code-derived value.

Frozen EXPECTED_BEFORE_RUN and supplemental equations remain byte-identical. Original source versions and every log are retained.
