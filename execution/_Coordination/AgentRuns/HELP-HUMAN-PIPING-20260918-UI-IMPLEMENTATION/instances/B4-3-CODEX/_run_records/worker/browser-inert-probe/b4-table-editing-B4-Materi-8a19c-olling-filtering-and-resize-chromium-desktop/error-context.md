# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Materials stable editor owns character starts, virtual scrolling, filtering and resize
- Location: e2e/b4-table-editing.spec.ts:453:1

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByTestId('material-engineering-table').getByRole('textbox')
Expected: visible
Timeout: 10000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 10000ms
  - waiting for getByTestId('material-engineering-table').getByRole('textbox')

```

```yaml
- main:
  - navigation "Application menu":
    - button "File"
    - button "Edit"
    - button "View"
    - button "Insert"
    - button "Analyze"
  - heading "SWBPIPE" [level=1]
  - paragraph: Generated UI local-render-origin precision probe
  - group "Editing tools":
    - button "Undo model edit" [disabled]
    - button "Redo model edit" [disabled]
    - button "Select" [pressed]
  - group "View":
    - button "Table" [pressed]
    - button "Model"
    - button "Both"
  - button "Run"
  - button "Issues, 2": Issues 2
  - group "Panels":
    - button "Inspector" [disabled]
    - button "Agent" [disabled]
  - combobox "Display units":
    - option "Entered" [selected]
    - option "SI"
    - option "US"
  - group
  - region "Human toolkit":
    - button "Find modeling commands": Search or command… ⌘K
  - navigation "Stages":
    - list:
      - listitem:
        - button "Model" [pressed]
      - listitem:
        - button "Loads"
      - listitem:
        - button "Results" [disabled]
      - listitem:
        - button "Review" [disabled]
    - separator
    - list:
      - listitem:
        - button "Libraries"
      - listitem:
        - button "Rules"
      - listitem:
        - button "Issues, 2": Issues
  - region "Modeling workspace":
    - group "Tables":
      - button "Model" [pressed]
      - button "Review changes"
      - button "Collapse table drawer" [disabled] [expanded]
    - text: Model
    - region "Layout grid mode":
      - button "Tree"
      - button "Grid" [pressed]
    - region "Model tree filtering":
      - text: Filter model
      - searchbox "Filter model tree"
      - text: 187 of 187 model entities visible
      - button "Clear model tree filter" [disabled]
    - region "Bulk entity grid":
      - button "Nodes"
      - button "Pipes"
      - button "Supports"
      - button "Materials" [pressed]
      - button "Sections"
      - button "Components"
      - button "Load Cases"
      - button "Combinations"
      - grid "Material fields":
        - row "Material Sort Label Sort Elastic Sort Shear Sort Thermal Sort Provenance":
          - columnheader "Material"
          - columnheader "Sort Label":
            - button "Sort Label": Label
          - columnheader "Sort Elastic":
            - button "Sort Elastic": Elastic [per-row entered unit]
          - columnheader "Sort Shear":
            - button "Sort Shear": Shear [per-row entered unit]
          - columnheader "Sort Thermal":
            - button "Sort Thermal": Thermal [per-row entered unit]
          - columnheader "Sort Provenance":
            - button "Sort Provenance": Provenance
        - rowgroup:
          - 'row "material:UIF-INVENTED-01 material:UIF-INVENTED-01 Label: Synthetic material 0 Pa Quantity readout material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [selected]':
            - rowheader "material:UIF-INVENTED-01":
              - button "material:UIF-INVENTED-01"
            - 'gridcell "material:UIF-INVENTED-01 Label: Synthetic material 0"':
              - 'button "material:UIF-INVENTED-01 Label: Synthetic material 0"': Synthetic material 0
            - gridcell "Pa Quantity readout" [selected]: Pa 200000000000 Pa
            - 'gridcell "material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:UIF-INVENTED-01 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-1 material:B4-1 Label: Synthetic material 1 material:B4-1 Elastic: 200000000000 Pa Pa Quantity readout material:B4-1 Shear: 77000000000 Pa Pa Quantity readout material:B4-1 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-1 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-1":
              - button "material:B4-1"
            - 'gridcell "material:B4-1 Label: Synthetic material 1"':
              - 'button "material:B4-1 Label: Synthetic material 1"': Synthetic material 1
            - 'gridcell "material:B4-1 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-1 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-1 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-1 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-1 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-1 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-1 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-1 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-2 material:B4-2 Label: Synthetic material 2 material:B4-2 Elastic: 200000000000 Pa Pa Quantity readout material:B4-2 Shear: 77000000000 Pa Pa Quantity readout material:B4-2 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-2 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-2":
              - button "material:B4-2"
            - 'gridcell "material:B4-2 Label: Synthetic material 2"':
              - 'button "material:B4-2 Label: Synthetic material 2"': Synthetic material 2
            - 'gridcell "material:B4-2 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-2 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-2 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-2 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-2 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-2 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-2 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-2 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-3 material:B4-3 Label: Synthetic material 3 material:B4-3 Elastic: 200000000000 Pa Pa Quantity readout material:B4-3 Shear: 77000000000 Pa Pa Quantity readout material:B4-3 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-3 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-3":
              - button "material:B4-3"
            - 'gridcell "material:B4-3 Label: Synthetic material 3"':
              - 'button "material:B4-3 Label: Synthetic material 3"': Synthetic material 3
            - 'gridcell "material:B4-3 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-3 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-3 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-3 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-3 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-3 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-3 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-3 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-4 material:B4-4 Label: Synthetic material 4 material:B4-4 Elastic: 200000000000 Pa Pa Quantity readout material:B4-4 Shear: 77000000000 Pa Pa Quantity readout material:B4-4 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-4 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-4":
              - button "material:B4-4"
            - 'gridcell "material:B4-4 Label: Synthetic material 4"':
              - 'button "material:B4-4 Label: Synthetic material 4"': Synthetic material 4
            - 'gridcell "material:B4-4 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-4 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-4 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-4 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-4 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-4 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-4 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-4 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-5 material:B4-5 Label: Synthetic material 5 material:B4-5 Elastic: 200000000000 Pa Pa Quantity readout material:B4-5 Shear: 77000000000 Pa Pa Quantity readout material:B4-5 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-5 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-5":
              - button "material:B4-5"
            - 'gridcell "material:B4-5 Label: Synthetic material 5"':
              - 'button "material:B4-5 Label: Synthetic material 5"': Synthetic material 5
            - 'gridcell "material:B4-5 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-5 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-5 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-5 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-5 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-5 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-5 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-5 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-6 material:B4-6 Label: Synthetic material 6 material:B4-6 Elastic: 200000000000 Pa Pa Quantity readout material:B4-6 Shear: 77000000000 Pa Pa Quantity readout material:B4-6 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-6 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-6":
              - button "material:B4-6"
            - 'gridcell "material:B4-6 Label: Synthetic material 6"':
              - 'button "material:B4-6 Label: Synthetic material 6"': Synthetic material 6
            - 'gridcell "material:B4-6 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-6 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-6 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-6 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-6 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-6 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-6 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-6 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-7 material:B4-7 Label: Synthetic material 7 material:B4-7 Elastic: 200000000000 Pa Pa Quantity readout material:B4-7 Shear: 77000000000 Pa Pa Quantity readout material:B4-7 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-7 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-7":
              - button "material:B4-7"
            - 'gridcell "material:B4-7 Label: Synthetic material 7"':
              - 'button "material:B4-7 Label: Synthetic material 7"': Synthetic material 7
            - 'gridcell "material:B4-7 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-7 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-7 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-7 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-7 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-7 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-7 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-7 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-8 material:B4-8 Label: Synthetic material 8 material:B4-8 Elastic: 200000000000 Pa Pa Quantity readout material:B4-8 Shear: 77000000000 Pa Pa Quantity readout material:B4-8 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-8 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-8":
              - button "material:B4-8"
            - 'gridcell "material:B4-8 Label: Synthetic material 8"':
              - 'button "material:B4-8 Label: Synthetic material 8"': Synthetic material 8
            - 'gridcell "material:B4-8 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-8 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-8 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-8 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-8 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-8 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-8 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-8 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-9 material:B4-9 Label: Synthetic material 9 material:B4-9 Elastic: 200000000000 Pa Pa Quantity readout material:B4-9 Shear: 77000000000 Pa Pa Quantity readout material:B4-9 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-9 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-9":
              - button "material:B4-9"
            - 'gridcell "material:B4-9 Label: Synthetic material 9"':
              - 'button "material:B4-9 Label: Synthetic material 9"': Synthetic material 9
            - 'gridcell "material:B4-9 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-9 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-9 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-9 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-9 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-9 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-9 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-9 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-10 material:B4-10 Label: Synthetic material 10 material:B4-10 Elastic: 200000000000 Pa Pa Quantity readout material:B4-10 Shear: 77000000000 Pa Pa Quantity readout material:B4-10 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-10 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-10":
              - button "material:B4-10"
            - 'gridcell "material:B4-10 Label: Synthetic material 10"':
              - 'button "material:B4-10 Label: Synthetic material 10"': Synthetic material 10
            - 'gridcell "material:B4-10 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-10 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-10 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-10 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-10 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-10 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-10 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-10 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-11 material:B4-11 Label: Synthetic material 11 material:B4-11 Elastic: 200000000000 Pa Pa Quantity readout material:B4-11 Shear: 77000000000 Pa Pa Quantity readout material:B4-11 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-11 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-11":
              - button "material:B4-11"
            - 'gridcell "material:B4-11 Label: Synthetic material 11"':
              - 'button "material:B4-11 Label: Synthetic material 11"': Synthetic material 11
            - 'gridcell "material:B4-11 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-11 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-11 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-11 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-11 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-11 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-11 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-11 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-12 material:B4-12 Label: Synthetic material 12 material:B4-12 Elastic: 200000000000 Pa Pa Quantity readout material:B4-12 Shear: 77000000000 Pa Pa Quantity readout material:B4-12 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-12 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-12":
              - button "material:B4-12"
            - 'gridcell "material:B4-12 Label: Synthetic material 12"':
              - 'button "material:B4-12 Label: Synthetic material 12"': Synthetic material 12
            - 'gridcell "material:B4-12 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-12 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-12 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-12 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-12 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-12 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-12 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-12 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-13 material:B4-13 Label: Synthetic material 13 material:B4-13 Elastic: 200000000000 Pa Pa Quantity readout material:B4-13 Shear: 77000000000 Pa Pa Quantity readout material:B4-13 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-13 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-13":
              - button "material:B4-13"
            - 'gridcell "material:B4-13 Label: Synthetic material 13"':
              - 'button "material:B4-13 Label: Synthetic material 13"': Synthetic material 13
            - 'gridcell "material:B4-13 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-13 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-13 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-13 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-13 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-13 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-13 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-13 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-14 material:B4-14 Label: Synthetic material 14 material:B4-14 Elastic: 200000000000 Pa Pa Quantity readout material:B4-14 Shear: 77000000000 Pa Pa Quantity readout material:B4-14 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-14 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-14":
              - button "material:B4-14"
            - 'gridcell "material:B4-14 Label: Synthetic material 14"':
              - 'button "material:B4-14 Label: Synthetic material 14"': Synthetic material 14
            - 'gridcell "material:B4-14 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-14 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-14 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-14 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-14 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-14 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-14 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-14 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-15 material:B4-15 Label: Synthetic material 15 material:B4-15 Elastic: 200000000000 Pa Pa Quantity readout material:B4-15 Shear: 77000000000 Pa Pa Quantity readout material:B4-15 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-15 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-15":
              - button "material:B4-15"
            - 'gridcell "material:B4-15 Label: Synthetic material 15"':
              - 'button "material:B4-15 Label: Synthetic material 15"': Synthetic material 15
            - 'gridcell "material:B4-15 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-15 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-15 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-15 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-15 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-15 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-15 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-15 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-16 material:B4-16 Label: Synthetic material 16 material:B4-16 Elastic: 200000000000 Pa Pa Quantity readout material:B4-16 Shear: 77000000000 Pa Pa Quantity readout material:B4-16 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-16 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-16":
              - button "material:B4-16"
            - 'gridcell "material:B4-16 Label: Synthetic material 16"':
              - 'button "material:B4-16 Label: Synthetic material 16"': Synthetic material 16
            - 'gridcell "material:B4-16 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-16 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-16 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-16 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-16 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-16 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-16 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-16 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-17 material:B4-17 Label: Synthetic material 17 material:B4-17 Elastic: 200000000000 Pa Pa Quantity readout material:B4-17 Shear: 77000000000 Pa Pa Quantity readout material:B4-17 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-17 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-17":
              - button "material:B4-17"
            - 'gridcell "material:B4-17 Label: Synthetic material 17"':
              - 'button "material:B4-17 Label: Synthetic material 17"': Synthetic material 17
            - 'gridcell "material:B4-17 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-17 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-17 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-17 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-17 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-17 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-17 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-17 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-18 material:B4-18 Label: Synthetic material 18 material:B4-18 Elastic: 200000000000 Pa Pa Quantity readout material:B4-18 Shear: 77000000000 Pa Pa Quantity readout material:B4-18 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-18 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-18":
              - button "material:B4-18"
            - 'gridcell "material:B4-18 Label: Synthetic material 18"':
              - 'button "material:B4-18 Label: Synthetic material 18"': Synthetic material 18
            - 'gridcell "material:B4-18 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-18 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-18 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-18 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-18 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-18 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-18 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-18 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-19 material:B4-19 Label: Synthetic material 19 material:B4-19 Elastic: 200000000000 Pa Pa Quantity readout material:B4-19 Shear: 77000000000 Pa Pa Quantity readout material:B4-19 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-19 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-19":
              - button "material:B4-19"
            - 'gridcell "material:B4-19 Label: Synthetic material 19"':
              - 'button "material:B4-19 Label: Synthetic material 19"': Synthetic material 19
            - 'gridcell "material:B4-19 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-19 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-19 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-19 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-19 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-19 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-19 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-19 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-20 material:B4-20 Label: Synthetic material 20 material:B4-20 Elastic: 200000000000 Pa Pa Quantity readout material:B4-20 Shear: 77000000000 Pa Pa Quantity readout material:B4-20 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-20 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-20":
              - button "material:B4-20"
            - 'gridcell "material:B4-20 Label: Synthetic material 20"':
              - 'button "material:B4-20 Label: Synthetic material 20"': Synthetic material 20
            - 'gridcell "material:B4-20 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-20 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-20 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-20 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-20 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-20 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-20 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-20 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-21 material:B4-21 Label: Synthetic material 21 material:B4-21 Elastic: 200000000000 Pa Pa Quantity readout material:B4-21 Shear: 77000000000 Pa Pa Quantity readout material:B4-21 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-21 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-21":
              - button "material:B4-21"
            - 'gridcell "material:B4-21 Label: Synthetic material 21"':
              - 'button "material:B4-21 Label: Synthetic material 21"': Synthetic material 21
            - 'gridcell "material:B4-21 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-21 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-21 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-21 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-21 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-21 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-21 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-21 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-22 material:B4-22 Label: Synthetic material 22 material:B4-22 Elastic: 200000000000 Pa Pa Quantity readout material:B4-22 Shear: 77000000000 Pa Pa Quantity readout material:B4-22 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-22 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-22":
              - button "material:B4-22"
            - 'gridcell "material:B4-22 Label: Synthetic material 22"':
              - 'button "material:B4-22 Label: Synthetic material 22"': Synthetic material 22
            - 'gridcell "material:B4-22 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-22 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-22 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-22 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-22 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-22 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-22 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-22 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-23 material:B4-23 Label: Synthetic material 23 material:B4-23 Elastic: 200000000000 Pa Pa Quantity readout material:B4-23 Shear: 77000000000 Pa Pa Quantity readout material:B4-23 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-23 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-23":
              - button "material:B4-23"
            - 'gridcell "material:B4-23 Label: Synthetic material 23"':
              - 'button "material:B4-23 Label: Synthetic material 23"': Synthetic material 23
            - 'gridcell "material:B4-23 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-23 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-23 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-23 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-23 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-23 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-23 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-23 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-24 material:B4-24 Label: Synthetic material 24 material:B4-24 Elastic: 200000000000 Pa Pa Quantity readout material:B4-24 Shear: 77000000000 Pa Pa Quantity readout material:B4-24 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-24 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-24":
              - button "material:B4-24"
            - 'gridcell "material:B4-24 Label: Synthetic material 24"':
              - 'button "material:B4-24 Label: Synthetic material 24"': Synthetic material 24
            - 'gridcell "material:B4-24 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-24 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-24 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-24 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-24 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-24 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-24 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-24 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-25 material:B4-25 Label: Synthetic material 25 material:B4-25 Elastic: 200000000000 Pa Pa Quantity readout material:B4-25 Shear: 77000000000 Pa Pa Quantity readout material:B4-25 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-25 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-25":
              - button "material:B4-25"
            - 'gridcell "material:B4-25 Label: Synthetic material 25"':
              - 'button "material:B4-25 Label: Synthetic material 25"': Synthetic material 25
            - 'gridcell "material:B4-25 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-25 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-25 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-25 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-25 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-25 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-25 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-25 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-26 material:B4-26 Label: Synthetic material 26 material:B4-26 Elastic: 200000000000 Pa Pa Quantity readout material:B4-26 Shear: 77000000000 Pa Pa Quantity readout material:B4-26 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-26 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-26":
              - button "material:B4-26"
            - 'gridcell "material:B4-26 Label: Synthetic material 26"':
              - 'button "material:B4-26 Label: Synthetic material 26"': Synthetic material 26
            - 'gridcell "material:B4-26 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-26 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-26 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-26 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-26 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-26 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-26 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-26 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-27 material:B4-27 Label: Synthetic material 27 material:B4-27 Elastic: 200000000000 Pa Pa Quantity readout material:B4-27 Shear: 77000000000 Pa Pa Quantity readout material:B4-27 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-27 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-27":
              - button "material:B4-27"
            - 'gridcell "material:B4-27 Label: Synthetic material 27"':
              - 'button "material:B4-27 Label: Synthetic material 27"': Synthetic material 27
            - 'gridcell "material:B4-27 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-27 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-27 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-27 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-27 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-27 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-27 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-27 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-28 material:B4-28 Label: Synthetic material 28 material:B4-28 Elastic: 200000000000 Pa Pa Quantity readout material:B4-28 Shear: 77000000000 Pa Pa Quantity readout material:B4-28 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-28 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-28":
              - button "material:B4-28"
            - 'gridcell "material:B4-28 Label: Synthetic material 28"':
              - 'button "material:B4-28 Label: Synthetic material 28"': Synthetic material 28
            - 'gridcell "material:B4-28 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-28 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-28 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-28 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-28 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-28 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-28 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-28 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-29 material:B4-29 Label: Synthetic material 29 material:B4-29 Elastic: 200000000000 Pa Pa Quantity readout material:B4-29 Shear: 77000000000 Pa Pa Quantity readout material:B4-29 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-29 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-29":
              - button "material:B4-29"
            - 'gridcell "material:B4-29 Label: Synthetic material 29"':
              - 'button "material:B4-29 Label: Synthetic material 29"': Synthetic material 29
            - 'gridcell "material:B4-29 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-29 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-29 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-29 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-29 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-29 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-29 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-29 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-30 material:B4-30 Label: Synthetic material 30 material:B4-30 Elastic: 200000000000 Pa Pa Quantity readout material:B4-30 Shear: 77000000000 Pa Pa Quantity readout material:B4-30 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-30 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-30":
              - button "material:B4-30"
            - 'gridcell "material:B4-30 Label: Synthetic material 30"':
              - 'button "material:B4-30 Label: Synthetic material 30"': Synthetic material 30
            - 'gridcell "material:B4-30 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-30 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-30 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-30 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-30 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-30 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-30 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-30 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-31 material:B4-31 Label: Synthetic material 31 material:B4-31 Elastic: 200000000000 Pa Pa Quantity readout material:B4-31 Shear: 77000000000 Pa Pa Quantity readout material:B4-31 Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-31 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-31":
              - button "material:B4-31"
            - 'gridcell "material:B4-31 Label: Synthetic material 31"':
              - 'button "material:B4-31 Label: Synthetic material 31"': Synthetic material 31
            - 'gridcell "material:B4-31 Elastic: 200000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-31 Elastic: 200000000000 Pa"': "200000000000"
              - text: Pa 200000000000 Pa
            - 'gridcell "material:B4-31 Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-31 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-31 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-31 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-31 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-31 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
      - group "Material fields footer":
        - text: Editing Elastic · material:UIF-INVENTED-01
        - button "Apply"
        - button "Cancel"
      - alert: Enter a finite number in the entered unit.
      - button "Review multiple changes"
  - complementary "Agent":
    - button "Agent" [disabled]
  - button "Solver · Not solved"
  - button "2 Issues"
  - text: "material: material:UIF-INVENTED-01 Entered"
  - button "About SWBPIPE…"
```

# Test source

```ts
  379 |   }
  380 |   return result;
  381 | }
  382 | ` });
  383 |   });
  384 |   await page.addInitScript(() => { (window as any).__b4MaterialConversionGate = { hold: false, pending: [] }; });
  385 |   await gotoModel(page, model);
  386 |   await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  387 |   const direct = page.getByTestId("material-engineering-table"); const elastic = direct.getByTestId(`table-cell-${firstId}-elastic`);
  388 |   await expect(elastic.locator("..")).toHaveAttribute("aria-readonly", "false");
  389 |   await expect(elastic.locator("..").locator(".engineering-table-unit")).toHaveText("MPa");
  390 |   await direct.getByRole("button", { name: "Sort Elastic", exact: true }).click();
  391 |   await expect(direct.getByRole("rowheader").first()).toHaveText(secondId);
  392 |   await elastic.dblclick(); const edit = direct.getByRole("textbox"); await edit.fill("0"); await direct.getByRole("button", { name: "Apply", exact: true }).click();
  393 |   await expect(edit).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  394 |   await edit.fill("210000"); await direct.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(elastic).toHaveText("200000");
  395 |   await elastic.dblclick(); await edit.fill("210000"); await direct.getByRole("button", { name: "Apply", exact: true }).click(); await expect(elastic).toHaveText("210000");
  396 |   await expect(page.getByTestId("project-edited")).toBeVisible(); await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000");
  397 |   await page.getByTestId("workspace-redo").click(); await expect(elastic).toHaveText("210000");
  398 |   await page.getByTestId("material-grid-review-disclosure").click(); const review = page.getByTestId("material-engineering-table-review");
  399 |   await review.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await review.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await expect(review.getByRole("rowheader").first()).toHaveText(firstId);
  400 |   const reviewCell = review.getByTestId(`review-cell-${firstId}-elastic`); await reviewCell.dblclick(); const input = review.getByRole("textbox");
  401 |   const retainedInput = await input.elementHandle();
  402 |   await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  403 |   await input.press("ControlOrMeta+a"); await input.press("5"); await input.press("0"); await input.press("ArrowLeft");
  404 |   await expect(input).toHaveValue("50"); await expect(input).toBeFocused();
  405 |   await expect(review.getByRole("status")).toContainText("sort unavailable"); await expect(review.getByRole("rowheader").first()).toHaveText(firstId);
  406 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  407 |   expect(await input.evaluate((node, original) => node === original, retainedInput)).toBe(true);
  408 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([1, 1]);
  409 |   // Descending completion physically moves the active row while its native
  410 |   // editor, selection and text-Undo stack must remain owned and intact.
  411 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  412 |   await expect(review.getByRole("rowheader").first()).toHaveText(secondId); await expect(input).toBeFocused();
  413 |   expect(await input.evaluate((node, original) => node === original, retainedInput)).toBe(true);
  414 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([1, 1]);
  415 |   await input.press("3"); await expect(input).toHaveValue("530"); await input.press("ControlOrMeta+z"); await expect(input).toHaveValue("50");
  416 |   await expect(elastic).toHaveText("210000");
  417 |   await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await expect(review.getByText("Draft retained; model unchanged.")).toBeVisible();
  418 |   await page.getByTestId("entity-grid-type-nodes").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(reviewCell).toHaveText("50");
  419 |   await review.getByTestId(`review-cell-${secondId}-label`).dblclick(); await review.getByRole("textbox").fill("retained second"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
  420 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(firstId);
  421 |   await page.getByTestId("queue-entity-grid-intents").click(); await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText('"unit":"MPa"');
  422 |   await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText("2 applied");
  423 |   await showModelTree(page); await filter.fill(""); await expect(review.getByTestId(`review-cell-${secondId}-label`)).toHaveText("retained second");
  424 |   await page.getByTestId("clear-entity-grid-drafts").click(); await expect(review.getByTestId(`review-cell-${secondId}-label`)).toHaveText("Synthetic second material");
  425 |   await page.getByTestId("material-grid-review-disclosure").click(); await expect(elastic).toHaveText("50");
  426 |   const geometry = await direct.evaluate((root) => {
  427 |     const box = (element: Element) => { const r = element.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height, bottom: r.bottom }; };
  428 |     return { pane: box(root.closest(".shell-table-pane")!), header: box(root.querySelector(".engineering-table-header")!), body: box(root.querySelector(".engineering-table-body-slot")!), footer: box(root.querySelector(".engineering-table-footer")!), units: [...root.querySelectorAll(".engineering-table-unit")].map((unit) => ({ text: unit.textContent, ...box(unit), client: unit.clientWidth, scroll: unit.scrollWidth })) };
  429 |   });
  430 |   expect(geometry.body.height).toBeGreaterThan(0); expect(geometry.footer.bottom).toBeLessThanOrEqual(geometry.pane.bottom); expect(geometry.header.bottom).toBeLessThanOrEqual(geometry.body.y);
  431 |   for (const unit of geometry.units) { expect(unit.width).toBeGreaterThan(0); expect(unit.scroll).toBeLessThanOrEqual(unit.client); }
  432 |   await info.attach("materials-contained-units", { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  433 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  434 |   await showModelTree(page); await page.getByTestId("material-grid-review-disclosure").click();
  435 |   await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  436 |   await reviewCell.dblclick(); await review.getByRole("textbox").fill("70");
  437 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  438 |   await page.getByTestId("entity-grid-type-nodes").click();
  439 |   const nodeLabel = page.getByTestId(`table-cell-${model.nodes[0].id}-label`); await nodeLabel.dblclick(); const nodeEditor = page.getByTestId("engineering-table").getByRole("textbox"); await nodeEditor.fill(" ");
  440 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  441 |   await expect(nodeEditor).toBeFocused(); await expect(nodeEditor).toHaveValue(" "); await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click();
  442 |   await page.getByTestId("entity-grid-type-materials").click(); await reviewCell.dblclick(); await review.getByRole("textbox").fill("80");
  443 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  444 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  445 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(elastic).toHaveText("50"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  446 |   if (await page.getByTestId("material-grid-review-disclosure").getAttribute("aria-expanded") === "true") await page.getByTestId("material-grid-review-disclosure").click();
  447 |   await direct.getByTestId(`table-cell-${firstId}-label`).dblclick(); const newEditor = direct.getByRole("textbox"); await newEditor.press("R"); await newEditor.press("S");
  448 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  449 |   await expect(newEditor).toHaveValue("RS"); await expect(newEditor).toBeFocused(); await direct.getByRole("button", { name: "Cancel", exact: true }).click();
  450 |   await page.screenshot({ path: info.outputPath("b4-materials-reopened.png") });
  451 | });
  452 | 
  453 | test("B4 Materials stable editor owns character starts, virtual scrolling, filtering and resize", async ({ page, browser }, info) => {
  454 |   await attachBrowserIdentity(browser, info);
  455 |   const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0];
  456 |   model.materials = Array.from({ length: 140 }, (_, index) => ({ ...material, id: index === 0 ? material.id : `material:B4-${index}`, label: `Synthetic material ${index}` }));
  457 |   await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  458 |   const table = page.getByTestId("material-engineering-table"); const cell = table.getByTestId(`table-cell-${material.id}-elastic`);
  459 |   await expect(cell.locator("..")).toHaveAttribute("aria-readonly", "false");
  460 |   await cell.focus(); await page.keyboard.press("x"); const editor = table.getByRole("textbox"); await expect(editor).toBeFocused();
  461 |   await page.keyboard.press("y"); await expect(editor).toHaveValue("xy"); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
  462 |   await page.keyboard.press("ArrowLeft"); const retained = await editor.elementHandle();
  463 |   const originalViewport = page.viewportSize()!; await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 80 });
  464 |   await expect(editor).toBeFocused(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([1, 1]);
  465 |   const aligned = async () => editor.evaluate((input) => {
  466 |     const anchor = input.closest(".engineering-table")!.querySelector("[data-editor-anchor]")!; const a = anchor.getBoundingClientRect(), e = input.getBoundingClientRect();
  467 |     return { dx: e.x - a.x, dy: e.y - a.y, dw: e.width - a.width, dh: e.height - a.height };
  468 |   });
  469 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  470 |   await openWorkspaceSection(page, "libraries");
  471 |   await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 120 });
  472 |   await page.getByTestId("workspace-dock-close").click();
  473 |   const pageReturn = await table.evaluate((root) => {
  474 |     const input = root.querySelector<HTMLInputElement>("input")!; const anchor = root.querySelector("[data-editor-anchor]")!;
  475 |     return { inputVisibility: getComputedStyle(input).visibility, value: input.value, owner: document.activeElement?.outerHTML.slice(0, 300),
  476 |       input: input.getBoundingClientRect().toJSON(), anchor: anchor.getBoundingClientRect().toJSON(), ancestorInert: Boolean(root.closest("[inert]")) };
  477 |   });
  478 |   await info.attach("material-page-inert-return", { body: JSON.stringify(pageReturn, null, 2), contentType: "application/json" });
> 479 |   await expect(editor).toBeVisible(); await expect(editor).toHaveValue("xy");
      |                        ^ Error: expect(locator).toBeVisible() failed
  480 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  481 |   expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true);
  482 |   await expect(editor).not.toBeFocused(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  483 |   const rows = page.getByTestId("material-engineering-table-rows"); await hoverTableBody(page, rows); await page.mouse.wheel(0, 2200);
  484 |   await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  485 |   expect(await retained!.evaluate((input) => input.isConnected)).toBe(true);
  486 |   await expect(editor).toHaveValue("xy");
  487 |   // The clipped editor cannot intercept header/footer pointer controls.
  488 |   const hits = await table.evaluate((root) => [".engineering-table-header", ".engineering-table-footer"].map((selector) => {
  489 |     const element = root.querySelector(selector)!; const r = element.getBoundingClientRect(); const hit = document.elementFromPoint(r.left + 12, r.top + r.height / 2);
  490 |     return Boolean(hit && element.contains(hit));
  491 |   })); expect(hits).toEqual([true, true]);
  492 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("material:B4-139"); await expect(rows.locator('[role="row"]')).toHaveCount(2);
  493 |   expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true); await expect(editor).toHaveValue("xy");
  494 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  495 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(editor).toBeHidden(); await page.getByTestId("entity-grid-type-materials").click(); await expect(editor).toHaveValue("xy");
  496 |   await expect(page.getByTestId("entity-grid-type-materials")).toBeFocused();
  497 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(editor).toHaveCount(0); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  498 |   await filter.fill(""); await cell.focus(); await page.keyboard.press("Enter"); await expect(editor).toBeFocused();
  499 |   expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  500 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await cell.dblclick(); await expect(editor).toBeFocused();
  501 |   expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  502 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await page.setViewportSize(originalViewport);
  503 |   await info.attach("material-editor-clipping", { body: JSON.stringify({ headerFooterHitOwnership: hits }), contentType: "application/json" });
  504 | });
  505 | 
```