export type DisplayUnitPreference = "entered" | "SI" | "US";

// Catalog symbols only. All numeric conversion, including affine temperatures,
// belongs to the Rust unit engine. Missing US symbols are explicit fallbacks.
export const DISPLAY_TARGETS: Readonly<Record<string, { SI: string; US?: string }>> = {
  dimensionless: { SI: "1", US: "1" }, slope: { SI: "1", US: "1" },
  length: { SI: "m", US: "in" }, displacement: { SI: "m", US: "in" },
  mass: { SI: "kg", US: "lb" }, time: { SI: "s", US: "s" },
  temperature: { SI: "K", US: "degF" }, temperature_interval: { SI: "K", US: "degF" },
  angle: { SI: "rad", US: "deg" }, rotation: { SI: "rad", US: "deg" },
  force: { SI: "N", US: "lbf" }, force_per_length: { SI: "N/m", US: "lbf/ft" },
  moment: { SI: "N*m", US: "lbf*ft" }, pressure: { SI: "Pa", US: "psi" },
  stress: { SI: "Pa", US: "psi" }, area: { SI: "m^2", US: "in^2" },
  volume: { SI: "m^3", US: "in^3" }, density: { SI: "kg/m^3", US: "lb/in^3" },
  linear_stiffness: { SI: "N/m", US: "lbf/in" }, rotational_stiffness: { SI: "N*m/rad" },
  velocity: { SI: "m/s" }, acceleration: { SI: "m/s^2" },
  thermal_conductivity: { SI: "W/(m*K)" }, specific_heat: { SI: "J/(kg*K)" },
  thermal_expansion_coefficient: { SI: "1/K" }, second_moment_area: { SI: "m^4", US: "in^4" },
  section_modulus: { SI: "m^3", US: "in^3" }, mass_per_length: { SI: "kg/m", US: "lb/ft" },
  volume_per_length: { SI: "m^3/m", US: "in^2" }
};
