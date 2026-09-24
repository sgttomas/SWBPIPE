# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> classic scrollbar allocation >> B4 classic scrollbar compact comfortable 160 preserves tracks and nonmutating pans @explicit-viewport
- Location: e2e/b4-table-editing.spec.ts:783:3

# Error details

```
TypeError: expect(received).toMatch(expected)

Matcher error: received value must be a string

Received has value: null
```

# Page snapshot

```yaml
- main [ref=e3]:
  - navigation "Application menu" [ref=e4]:
    - button "File" [ref=e6] [cursor=pointer]
    - button "Edit" [ref=e8] [cursor=pointer]
    - button "View" [ref=e10] [cursor=pointer]
    - button "Insert" [ref=e12] [cursor=pointer]
    - button "Analyze" [ref=e14] [cursor=pointer]
  - generic "Toolbar" [ref=e15]:
    - generic [ref=e16]:
      - heading "SWBPIPE" [level=1] [ref=e17]
      - paragraph [ref=e18]: Generated UI local-render-origin precision probe
    - group "Editing tools" [ref=e19]:
      - button "Undo model edit" [disabled] [ref=e20]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select (⎋)" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
    - group "View" [ref=e31]:
      - button "Table" [ref=e33] [cursor=pointer]:
        - img [ref=e34]
        - generic [ref=e36]: Table
      - button "Model" [pressed] [ref=e38] [cursor=pointer]:
        - img [ref=e39]
        - generic [ref=e42]: Model
      - button "Both" [ref=e44] [cursor=pointer]:
        - img [ref=e45]
        - generic [ref=e47]: Both
    - generic [ref=e48]:
      - button "Run" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Run
      - button "Issues, 2" [ref=e53] [cursor=pointer]:
        - img [ref=e54]
        - generic [ref=e56]: Issues
        - generic [ref=e57]: "2"
    - group "Panels" [ref=e58]:
      - button "Inspector" [disabled] [expanded] [ref=e60]:
        - img [ref=e61]
      - button "Agent" [disabled] [ref=e64]:
        - img [ref=e65]
    - generic "Display units" [ref=e68]:
      - combobox "Display units" [ref=e69]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e70]:
      - generic "Appearance" [ref=e71] [cursor=pointer]:
        - img [ref=e72]
      - option "System" [selected]
      - option "Light"
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e74]:
      - button "Find modeling commands" [ref=e75] [cursor=pointer]:
        - img [ref=e76]
        - generic [ref=e79]: Search or command…
        - generic "Command K" [ref=e80]: ⌘K
  - generic [ref=e81]:
    - navigation "Stages" [ref=e82]:
      - list [ref=e83]:
        - listitem [ref=e84]:
          - button "Model" [pressed] [ref=e85] [cursor=pointer]:
            - img [ref=e86]
            - generic [ref=e89]: Model
        - listitem [ref=e90]:
          - button "Loads" [ref=e91] [cursor=pointer]:
            - img [ref=e92]
            - generic [ref=e96]: Loads
        - listitem [ref=e97]:
          - button "Results" [disabled] [ref=e98]:
            - img [ref=e99]
            - generic [ref=e102]: Results
        - listitem [ref=e103]:
          - button "Review" [disabled] [ref=e104]:
            - img [ref=e105]
            - generic [ref=e109]: Review
      - separator [ref=e110]
      - list [ref=e111]:
        - listitem [ref=e112]:
          - button "Libraries" [ref=e113] [cursor=pointer]:
            - img [ref=e114]
            - generic [ref=e116]: Libraries
        - listitem [ref=e117]:
          - button "Rules" [ref=e118] [cursor=pointer]:
            - img [ref=e119]
            - generic [ref=e123]: Rules
        - listitem [ref=e124]:
          - button "Issues, 2" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "2"
    - region "Modeling workspace" [ref=e131]:
      - generic [ref=e132]:
        - group "Tables" [ref=e133]:
          - button "Model" [pressed] [ref=e134] [cursor=pointer]
          - button "Review changes" [ref=e135] [cursor=pointer]
          - button "Collapse table drawer" [expanded] [ref=e138] [cursor=pointer]:
            - img [ref=e139]
        - generic "Model tree" [ref=e143]:
          - generic [ref=e144]:
            - generic [ref=e147]:
              - region "Layout grid mode" [ref=e148]:
                - button "Tree" [ref=e149]
                - button "Grid" [pressed] [ref=e150]
              - region "Model tree filtering" [ref=e151]:
                - searchbox "Filter model tree" [ref=e153]
                - button "Clear model tree filter" [disabled] [ref=e154]:
                  - img [ref=e155]
              - generic [ref=e158]:
                - generic [ref=e159]:
                  - text: Family
                  - combobox "Grid family" [ref=e160]:
                    - option "Nodes"
                    - option "Pipes"
                    - option "Supports"
                    - option "Materials (160)" [selected]
                    - option "Sections"
                    - option "Components"
                    - option "Load Cases"
                    - option "Combinations"
                - button "Review changes" [ref=e161] [cursor=pointer]
                - button "Table details" [ref=e162]: Details
            - group "Pan columns" [ref=e164]:
              - button "Earlier columns" [disabled] [ref=e165]: ‹
              - button "Later columns" [ref=e166]: ›
          - region "Bulk entity grid" [ref=e168]:
            - generic [ref=e170]:
              - grid "Material fields" [ref=e171]:
                - row "Material Sort Label Sort Elastic [row unit] Sort Shear [row unit] Sort Thermal [row unit] Sort Provenance" [ref=e174]:
                  - columnheader "Material" [ref=e175]
                  - columnheader "Sort Label" [ref=e176]:
                    - button "Sort Label" [ref=e177]: Label ↕
                  - columnheader "Sort Elastic [row unit]" [ref=e178]:
                    - button "Sort Elastic [row unit]" [ref=e179]: Elastic [row unit] ↕
                  - columnheader "Sort Shear [row unit]" [ref=e180]:
                    - button "Sort Shear [row unit]" [ref=e181]: Shear [row unit] ↕
                  - columnheader "Sort Thermal [row unit]" [ref=e182]:
                    - button "Sort Thermal [row unit]" [ref=e183]: Thermal [row unit] ↕
                  - columnheader "Sort Provenance" [ref=e184]:
                    - button "Sort Provenance" [ref=e185]: Provenance ↕
                - rowgroup [ref=e187]:
                  - generic [ref=e188]:
                    - row [selected] [ref=e190]:
                      - rowheader "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity" [ref=e191]:
                        - button "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity" [ref=e192]
                      - 'gridcell "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Label: Invented scroll material 0" [ref=e193]':
                        - 'button "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Label: Invented scroll material 0" [ref=e194]': Invented scroll material 0
                      - 'gridcell "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Elastic: 200000000000 Pa Pa Quantity readout" [ref=e195]':
                        - 'button "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Elastic: 200000000000 Pa" [ref=e196]': "200000000000"
                        - generic "Pa" [ref=e197]
                        - generic "Quantity readout" [ref=e198]:
                          - generic [ref=e199]: 200000000000 Pa
                      - 'gridcell "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Shear: 77000000000 Pa Pa Quantity readout" [ref=e200]':
                        - 'button "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Shear: 77000000000 Pa" [ref=e201]': "77000000000"
                        - generic "Pa" [ref=e202]
                        - generic "Quantity readout" [ref=e203]:
                          - generic [ref=e204]: 77000000000 Pa
                      - 'gridcell "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e205]':
                        - 'button "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Thermal: 0.000012 1/degC" [ref=e206]': "0.000012"
                        - generic "1/degC" [ref=e207]
                        - generic "Quantity readout" [ref=e208]:
                          - generic [ref=e209]: 0.000012 1/degC
                      - gridcell "Retained valid draft while navigating chrome" [selected] [ref=e210]:
                        - textbox "material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity Provenance" [ref=e212]: Retained valid draft while navigating chrome
                    - 'row "material:scroll-1 material:scroll-1 Label: Invented scroll material 1 material:scroll-1 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-1 Shear: 77000000000 Pa Pa Quantity readout material:scroll-1 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-1 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e214]':
                      - rowheader "material:scroll-1" [ref=e215]:
                        - button "material:scroll-1" [ref=e216]
                      - 'gridcell "material:scroll-1 Label: Invented scroll material 1" [ref=e217]':
                        - 'button "material:scroll-1 Label: Invented scroll material 1" [ref=e218]': Invented scroll material 1
                      - 'gridcell "material:scroll-1 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e219]':
                        - 'button "material:scroll-1 Elastic: 200000000000 Pa" [ref=e220]': "200000000000"
                        - generic "Pa" [ref=e221]
                        - generic "Quantity readout" [ref=e222]:
                          - generic [ref=e223]: 200000000000 Pa
                      - 'gridcell "material:scroll-1 Shear: 77000000000 Pa Pa Quantity readout" [ref=e224]':
                        - 'button "material:scroll-1 Shear: 77000000000 Pa" [ref=e225]': "77000000000"
                        - generic "Pa" [ref=e226]
                        - generic "Quantity readout" [ref=e227]:
                          - generic [ref=e228]: 77000000000 Pa
                      - 'gridcell "material:scroll-1 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e229]':
                        - 'button "material:scroll-1 Thermal: 0.000012 1/degC" [ref=e230]': "0.000012"
                        - generic "1/degC" [ref=e231]
                        - generic "Quantity readout" [ref=e232]:
                          - generic [ref=e233]: 0.000012 1/degC
                      - 'gridcell "material:scroll-1 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e234]':
                        - 'button "material:scroll-1 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e235]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-2 material:scroll-2 Label: Invented scroll material 2 material:scroll-2 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-2 Shear: 77000000000 Pa Pa Quantity readout material:scroll-2 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-2 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e237]':
                      - rowheader "material:scroll-2" [ref=e238]:
                        - button "material:scroll-2" [ref=e239]
                      - 'gridcell "material:scroll-2 Label: Invented scroll material 2" [ref=e240]':
                        - 'button "material:scroll-2 Label: Invented scroll material 2" [ref=e241]': Invented scroll material 2
                      - 'gridcell "material:scroll-2 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e242]':
                        - 'button "material:scroll-2 Elastic: 200000000000 Pa" [ref=e243]': "200000000000"
                        - generic "Pa" [ref=e244]
                        - generic "Quantity readout" [ref=e245]:
                          - generic [ref=e246]: 200000000000 Pa
                      - 'gridcell "material:scroll-2 Shear: 77000000000 Pa Pa Quantity readout" [ref=e247]':
                        - 'button "material:scroll-2 Shear: 77000000000 Pa" [ref=e248]': "77000000000"
                        - generic "Pa" [ref=e249]
                        - generic "Quantity readout" [ref=e250]:
                          - generic [ref=e251]: 77000000000 Pa
                      - 'gridcell "material:scroll-2 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e252]':
                        - 'button "material:scroll-2 Thermal: 0.000012 1/degC" [ref=e253]': "0.000012"
                        - generic "1/degC" [ref=e254]
                        - generic "Quantity readout" [ref=e255]:
                          - generic [ref=e256]: 0.000012 1/degC
                      - 'gridcell "material:scroll-2 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e257]':
                        - 'button "material:scroll-2 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e258]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-3 material:scroll-3 Label: Invented scroll material 3 material:scroll-3 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-3 Shear: 77000000000 Pa Pa Quantity readout material:scroll-3 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-3 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e260]':
                      - rowheader "material:scroll-3" [ref=e261]:
                        - button "material:scroll-3" [ref=e262]
                      - 'gridcell "material:scroll-3 Label: Invented scroll material 3" [ref=e263]':
                        - 'button "material:scroll-3 Label: Invented scroll material 3" [ref=e264]': Invented scroll material 3
                      - 'gridcell "material:scroll-3 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e265]':
                        - 'button "material:scroll-3 Elastic: 200000000000 Pa" [ref=e266]': "200000000000"
                        - generic "Pa" [ref=e267]
                        - generic "Quantity readout" [ref=e268]:
                          - generic [ref=e269]: 200000000000 Pa
                      - 'gridcell "material:scroll-3 Shear: 77000000000 Pa Pa Quantity readout" [ref=e270]':
                        - 'button "material:scroll-3 Shear: 77000000000 Pa" [ref=e271]': "77000000000"
                        - generic "Pa" [ref=e272]
                        - generic "Quantity readout" [ref=e273]:
                          - generic [ref=e274]: 77000000000 Pa
                      - 'gridcell "material:scroll-3 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e275]':
                        - 'button "material:scroll-3 Thermal: 0.000012 1/degC" [ref=e276]': "0.000012"
                        - generic "1/degC" [ref=e277]
                        - generic "Quantity readout" [ref=e278]:
                          - generic [ref=e279]: 0.000012 1/degC
                      - 'gridcell "material:scroll-3 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e280]':
                        - 'button "material:scroll-3 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e281]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-4 material:scroll-4 Label: Invented scroll material 4 material:scroll-4 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-4 Shear: 77000000000 Pa Pa Quantity readout material:scroll-4 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-4 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e283]':
                      - rowheader "material:scroll-4" [ref=e284]:
                        - button "material:scroll-4" [ref=e285]
                      - 'gridcell "material:scroll-4 Label: Invented scroll material 4" [ref=e286]':
                        - 'button "material:scroll-4 Label: Invented scroll material 4" [ref=e287]': Invented scroll material 4
                      - 'gridcell "material:scroll-4 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e288]':
                        - 'button "material:scroll-4 Elastic: 200000000000 Pa" [ref=e289]': "200000000000"
                        - generic "Pa" [ref=e290]
                        - generic "Quantity readout" [ref=e291]:
                          - generic [ref=e292]: 200000000000 Pa
                      - 'gridcell "material:scroll-4 Shear: 77000000000 Pa Pa Quantity readout" [ref=e293]':
                        - 'button "material:scroll-4 Shear: 77000000000 Pa" [ref=e294]': "77000000000"
                        - generic "Pa" [ref=e295]
                        - generic "Quantity readout" [ref=e296]:
                          - generic [ref=e297]: 77000000000 Pa
                      - 'gridcell "material:scroll-4 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e298]':
                        - 'button "material:scroll-4 Thermal: 0.000012 1/degC" [ref=e299]': "0.000012"
                        - generic "1/degC" [ref=e300]
                        - generic "Quantity readout" [ref=e301]:
                          - generic [ref=e302]: 0.000012 1/degC
                      - 'gridcell "material:scroll-4 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e303]':
                        - 'button "material:scroll-4 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e304]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-5 material:scroll-5 Label: Invented scroll material 5 material:scroll-5 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-5 Shear: 77000000000 Pa Pa Quantity readout material:scroll-5 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-5 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e306]':
                      - rowheader "material:scroll-5" [ref=e307]:
                        - button "material:scroll-5" [ref=e308]
                      - 'gridcell "material:scroll-5 Label: Invented scroll material 5" [ref=e309]':
                        - 'button "material:scroll-5 Label: Invented scroll material 5" [ref=e310]': Invented scroll material 5
                      - 'gridcell "material:scroll-5 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e311]':
                        - 'button "material:scroll-5 Elastic: 200000000000 Pa" [ref=e312]': "200000000000"
                        - generic "Pa" [ref=e313]
                        - generic "Quantity readout" [ref=e314]:
                          - generic [ref=e315]: 200000000000 Pa
                      - 'gridcell "material:scroll-5 Shear: 77000000000 Pa Pa Quantity readout" [ref=e316]':
                        - 'button "material:scroll-5 Shear: 77000000000 Pa" [ref=e317]': "77000000000"
                        - generic "Pa" [ref=e318]
                        - generic "Quantity readout" [ref=e319]:
                          - generic [ref=e320]: 77000000000 Pa
                      - 'gridcell "material:scroll-5 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e321]':
                        - 'button "material:scroll-5 Thermal: 0.000012 1/degC" [ref=e322]': "0.000012"
                        - generic "1/degC" [ref=e323]
                        - generic "Quantity readout" [ref=e324]:
                          - generic [ref=e325]: 0.000012 1/degC
                      - 'gridcell "material:scroll-5 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e326]':
                        - 'button "material:scroll-5 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e327]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-6 material:scroll-6 Label: Invented scroll material 6 material:scroll-6 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-6 Shear: 77000000000 Pa Pa Quantity readout material:scroll-6 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-6 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e329]':
                      - rowheader "material:scroll-6" [ref=e330]:
                        - button "material:scroll-6" [ref=e331]
                      - 'gridcell "material:scroll-6 Label: Invented scroll material 6" [ref=e332]':
                        - 'button "material:scroll-6 Label: Invented scroll material 6" [ref=e333]': Invented scroll material 6
                      - 'gridcell "material:scroll-6 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e334]':
                        - 'button "material:scroll-6 Elastic: 200000000000 Pa" [ref=e335]': "200000000000"
                        - generic "Pa" [ref=e336]
                        - generic "Quantity readout" [ref=e337]:
                          - generic [ref=e338]: 200000000000 Pa
                      - 'gridcell "material:scroll-6 Shear: 77000000000 Pa Pa Quantity readout" [ref=e339]':
                        - 'button "material:scroll-6 Shear: 77000000000 Pa" [ref=e340]': "77000000000"
                        - generic "Pa" [ref=e341]
                        - generic "Quantity readout" [ref=e342]:
                          - generic [ref=e343]: 77000000000 Pa
                      - 'gridcell "material:scroll-6 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e344]':
                        - 'button "material:scroll-6 Thermal: 0.000012 1/degC" [ref=e345]': "0.000012"
                        - generic "1/degC" [ref=e346]
                        - generic "Quantity readout" [ref=e347]:
                          - generic [ref=e348]: 0.000012 1/degC
                      - 'gridcell "material:scroll-6 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e349]':
                        - 'button "material:scroll-6 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e350]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-7 material:scroll-7 Label: Invented scroll material 7 material:scroll-7 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-7 Shear: 77000000000 Pa Pa Quantity readout material:scroll-7 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-7 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e352]':
                      - rowheader "material:scroll-7" [ref=e353]:
                        - button "material:scroll-7" [ref=e354]
                      - 'gridcell "material:scroll-7 Label: Invented scroll material 7" [ref=e355]':
                        - 'button "material:scroll-7 Label: Invented scroll material 7" [ref=e356]': Invented scroll material 7
                      - 'gridcell "material:scroll-7 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e357]':
                        - 'button "material:scroll-7 Elastic: 200000000000 Pa" [ref=e358]': "200000000000"
                        - generic "Pa" [ref=e359]
                        - generic "Quantity readout" [ref=e360]:
                          - generic [ref=e361]: 200000000000 Pa
                      - 'gridcell "material:scroll-7 Shear: 77000000000 Pa Pa Quantity readout" [ref=e362]':
                        - 'button "material:scroll-7 Shear: 77000000000 Pa" [ref=e363]': "77000000000"
                        - generic "Pa" [ref=e364]
                        - generic "Quantity readout" [ref=e365]:
                          - generic [ref=e366]: 77000000000 Pa
                      - 'gridcell "material:scroll-7 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e367]':
                        - 'button "material:scroll-7 Thermal: 0.000012 1/degC" [ref=e368]': "0.000012"
                        - generic "1/degC" [ref=e369]
                        - generic "Quantity readout" [ref=e370]:
                          - generic [ref=e371]: 0.000012 1/degC
                      - 'gridcell "material:scroll-7 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e372]':
                        - 'button "material:scroll-7 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e373]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-8 material:scroll-8 Label: Invented scroll material 8 material:scroll-8 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-8 Shear: 77000000000 Pa Pa Quantity readout material:scroll-8 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-8 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e375]':
                      - rowheader "material:scroll-8" [ref=e376]:
                        - button "material:scroll-8" [ref=e377]
                      - 'gridcell "material:scroll-8 Label: Invented scroll material 8" [ref=e378]':
                        - 'button "material:scroll-8 Label: Invented scroll material 8" [ref=e379]': Invented scroll material 8
                      - 'gridcell "material:scroll-8 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e380]':
                        - 'button "material:scroll-8 Elastic: 200000000000 Pa" [ref=e381]': "200000000000"
                        - generic "Pa" [ref=e382]
                        - generic "Quantity readout" [ref=e383]:
                          - generic [ref=e384]: 200000000000 Pa
                      - 'gridcell "material:scroll-8 Shear: 77000000000 Pa Pa Quantity readout" [ref=e385]':
                        - 'button "material:scroll-8 Shear: 77000000000 Pa" [ref=e386]': "77000000000"
                        - generic "Pa" [ref=e387]
                        - generic "Quantity readout" [ref=e388]:
                          - generic [ref=e389]: 77000000000 Pa
                      - 'gridcell "material:scroll-8 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e390]':
                        - 'button "material:scroll-8 Thermal: 0.000012 1/degC" [ref=e391]': "0.000012"
                        - generic "1/degC" [ref=e392]
                        - generic "Quantity readout" [ref=e393]:
                          - generic [ref=e394]: 0.000012 1/degC
                      - 'gridcell "material:scroll-8 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e395]':
                        - 'button "material:scroll-8 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e396]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-9 material:scroll-9 Label: Invented scroll material 9 material:scroll-9 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-9 Shear: 77000000000 Pa Pa Quantity readout material:scroll-9 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-9 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e398]':
                      - rowheader "material:scroll-9" [ref=e399]:
                        - button "material:scroll-9" [ref=e400]
                      - 'gridcell "material:scroll-9 Label: Invented scroll material 9" [ref=e401]':
                        - 'button "material:scroll-9 Label: Invented scroll material 9" [ref=e402]': Invented scroll material 9
                      - 'gridcell "material:scroll-9 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e403]':
                        - 'button "material:scroll-9 Elastic: 200000000000 Pa" [ref=e404]': "200000000000"
                        - generic "Pa" [ref=e405]
                        - generic "Quantity readout" [ref=e406]:
                          - generic [ref=e407]: 200000000000 Pa
                      - 'gridcell "material:scroll-9 Shear: 77000000000 Pa Pa Quantity readout" [ref=e408]':
                        - 'button "material:scroll-9 Shear: 77000000000 Pa" [ref=e409]': "77000000000"
                        - generic "Pa" [ref=e410]
                        - generic "Quantity readout" [ref=e411]:
                          - generic [ref=e412]: 77000000000 Pa
                      - 'gridcell "material:scroll-9 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e413]':
                        - 'button "material:scroll-9 Thermal: 0.000012 1/degC" [ref=e414]': "0.000012"
                        - generic "1/degC" [ref=e415]
                        - generic "Quantity readout" [ref=e416]:
                          - generic [ref=e417]: 0.000012 1/degC
                      - 'gridcell "material:scroll-9 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e418]':
                        - 'button "material:scroll-9 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e419]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-10 material:scroll-10 Label: Invented scroll material 10 material:scroll-10 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-10 Shear: 77000000000 Pa Pa Quantity readout material:scroll-10 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-10 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e421]':
                      - rowheader "material:scroll-10" [ref=e422]:
                        - button "material:scroll-10" [ref=e423]
                      - 'gridcell "material:scroll-10 Label: Invented scroll material 10" [ref=e424]':
                        - 'button "material:scroll-10 Label: Invented scroll material 10" [ref=e425]': Invented scroll material 10
                      - 'gridcell "material:scroll-10 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e426]':
                        - 'button "material:scroll-10 Elastic: 200000000000 Pa" [ref=e427]': "200000000000"
                        - generic "Pa" [ref=e428]
                        - generic "Quantity readout" [ref=e429]:
                          - generic [ref=e430]: 200000000000 Pa
                      - 'gridcell "material:scroll-10 Shear: 77000000000 Pa Pa Quantity readout" [ref=e431]':
                        - 'button "material:scroll-10 Shear: 77000000000 Pa" [ref=e432]': "77000000000"
                        - generic "Pa" [ref=e433]
                        - generic "Quantity readout" [ref=e434]:
                          - generic [ref=e435]: 77000000000 Pa
                      - 'gridcell "material:scroll-10 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e436]':
                        - 'button "material:scroll-10 Thermal: 0.000012 1/degC" [ref=e437]': "0.000012"
                        - generic "1/degC" [ref=e438]
                        - generic "Quantity readout" [ref=e439]:
                          - generic [ref=e440]: 0.000012 1/degC
                      - 'gridcell "material:scroll-10 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e441]':
                        - 'button "material:scroll-10 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e442]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-11 material:scroll-11 Label: Invented scroll material 11 material:scroll-11 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-11 Shear: 77000000000 Pa Pa Quantity readout material:scroll-11 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-11 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e444]':
                      - rowheader "material:scroll-11" [ref=e445]:
                        - button "material:scroll-11" [ref=e446]
                      - 'gridcell "material:scroll-11 Label: Invented scroll material 11" [ref=e447]':
                        - 'button "material:scroll-11 Label: Invented scroll material 11" [ref=e448]': Invented scroll material 11
                      - 'gridcell "material:scroll-11 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e449]':
                        - 'button "material:scroll-11 Elastic: 200000000000 Pa" [ref=e450]': "200000000000"
                        - generic "Pa" [ref=e451]
                        - generic "Quantity readout" [ref=e452]:
                          - generic [ref=e453]: 200000000000 Pa
                      - 'gridcell "material:scroll-11 Shear: 77000000000 Pa Pa Quantity readout" [ref=e454]':
                        - 'button "material:scroll-11 Shear: 77000000000 Pa" [ref=e455]': "77000000000"
                        - generic "Pa" [ref=e456]
                        - generic "Quantity readout" [ref=e457]:
                          - generic [ref=e458]: 77000000000 Pa
                      - 'gridcell "material:scroll-11 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e459]':
                        - 'button "material:scroll-11 Thermal: 0.000012 1/degC" [ref=e460]': "0.000012"
                        - generic "1/degC" [ref=e461]
                        - generic "Quantity readout" [ref=e462]:
                          - generic [ref=e463]: 0.000012 1/degC
                      - 'gridcell "material:scroll-11 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e464]':
                        - 'button "material:scroll-11 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e465]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-12 material:scroll-12 Label: Invented scroll material 12 material:scroll-12 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-12 Shear: 77000000000 Pa Pa Quantity readout material:scroll-12 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-12 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e467]':
                      - rowheader "material:scroll-12" [ref=e468]:
                        - button "material:scroll-12" [ref=e469]
                      - 'gridcell "material:scroll-12 Label: Invented scroll material 12" [ref=e470]':
                        - 'button "material:scroll-12 Label: Invented scroll material 12" [ref=e471]': Invented scroll material 12
                      - 'gridcell "material:scroll-12 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e472]':
                        - 'button "material:scroll-12 Elastic: 200000000000 Pa" [ref=e473]': "200000000000"
                        - generic "Pa" [ref=e474]
                        - generic "Quantity readout" [ref=e475]:
                          - generic [ref=e476]: 200000000000 Pa
                      - 'gridcell "material:scroll-12 Shear: 77000000000 Pa Pa Quantity readout" [ref=e477]':
                        - 'button "material:scroll-12 Shear: 77000000000 Pa" [ref=e478]': "77000000000"
                        - generic "Pa" [ref=e479]
                        - generic "Quantity readout" [ref=e480]:
                          - generic [ref=e481]: 77000000000 Pa
                      - 'gridcell "material:scroll-12 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e482]':
                        - 'button "material:scroll-12 Thermal: 0.000012 1/degC" [ref=e483]': "0.000012"
                        - generic "1/degC" [ref=e484]
                        - generic "Quantity readout" [ref=e485]:
                          - generic [ref=e486]: 0.000012 1/degC
                      - 'gridcell "material:scroll-12 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e487]':
                        - 'button "material:scroll-12 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e488]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-13 material:scroll-13 Label: Invented scroll material 13 material:scroll-13 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-13 Shear: 77000000000 Pa Pa Quantity readout material:scroll-13 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-13 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e490]':
                      - rowheader "material:scroll-13" [ref=e491]:
                        - button "material:scroll-13" [ref=e492]
                      - 'gridcell "material:scroll-13 Label: Invented scroll material 13" [ref=e493]':
                        - 'button "material:scroll-13 Label: Invented scroll material 13" [ref=e494]': Invented scroll material 13
                      - 'gridcell "material:scroll-13 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e495]':
                        - 'button "material:scroll-13 Elastic: 200000000000 Pa" [ref=e496]': "200000000000"
                        - generic "Pa" [ref=e497]
                        - generic "Quantity readout" [ref=e498]:
                          - generic [ref=e499]: 200000000000 Pa
                      - 'gridcell "material:scroll-13 Shear: 77000000000 Pa Pa Quantity readout" [ref=e500]':
                        - 'button "material:scroll-13 Shear: 77000000000 Pa" [ref=e501]': "77000000000"
                        - generic "Pa" [ref=e502]
                        - generic "Quantity readout" [ref=e503]:
                          - generic [ref=e504]: 77000000000 Pa
                      - 'gridcell "material:scroll-13 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e505]':
                        - 'button "material:scroll-13 Thermal: 0.000012 1/degC" [ref=e506]': "0.000012"
                        - generic "1/degC" [ref=e507]
                        - generic "Quantity readout" [ref=e508]:
                          - generic [ref=e509]: 0.000012 1/degC
                      - 'gridcell "material:scroll-13 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e510]':
                        - 'button "material:scroll-13 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e511]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-14 material:scroll-14 Label: Invented scroll material 14 material:scroll-14 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-14 Shear: 77000000000 Pa Pa Quantity readout material:scroll-14 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-14 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e513]':
                      - rowheader "material:scroll-14" [ref=e514]:
                        - button "material:scroll-14" [ref=e515]
                      - 'gridcell "material:scroll-14 Label: Invented scroll material 14" [ref=e516]':
                        - 'button "material:scroll-14 Label: Invented scroll material 14" [ref=e517]': Invented scroll material 14
                      - 'gridcell "material:scroll-14 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e518]':
                        - 'button "material:scroll-14 Elastic: 200000000000 Pa" [ref=e519]': "200000000000"
                        - generic "Pa" [ref=e520]
                        - generic "Quantity readout" [ref=e521]:
                          - generic [ref=e522]: 200000000000 Pa
                      - 'gridcell "material:scroll-14 Shear: 77000000000 Pa Pa Quantity readout" [ref=e523]':
                        - 'button "material:scroll-14 Shear: 77000000000 Pa" [ref=e524]': "77000000000"
                        - generic "Pa" [ref=e525]
                        - generic "Quantity readout" [ref=e526]:
                          - generic [ref=e527]: 77000000000 Pa
                      - 'gridcell "material:scroll-14 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e528]':
                        - 'button "material:scroll-14 Thermal: 0.000012 1/degC" [ref=e529]': "0.000012"
                        - generic "1/degC" [ref=e530]
                        - generic "Quantity readout" [ref=e531]:
                          - generic [ref=e532]: 0.000012 1/degC
                      - 'gridcell "material:scroll-14 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e533]':
                        - 'button "material:scroll-14 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e534]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-15 material:scroll-15 Label: Invented scroll material 15 material:scroll-15 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-15 Shear: 77000000000 Pa Pa Quantity readout material:scroll-15 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-15 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e536]':
                      - rowheader "material:scroll-15" [ref=e537]:
                        - button "material:scroll-15" [ref=e538]
                      - 'gridcell "material:scroll-15 Label: Invented scroll material 15" [ref=e539]':
                        - 'button "material:scroll-15 Label: Invented scroll material 15" [ref=e540]': Invented scroll material 15
                      - 'gridcell "material:scroll-15 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e541]':
                        - 'button "material:scroll-15 Elastic: 200000000000 Pa" [ref=e542]': "200000000000"
                        - generic "Pa" [ref=e543]
                        - generic "Quantity readout" [ref=e544]:
                          - generic [ref=e545]: 200000000000 Pa
                      - 'gridcell "material:scroll-15 Shear: 77000000000 Pa Pa Quantity readout" [ref=e546]':
                        - 'button "material:scroll-15 Shear: 77000000000 Pa" [ref=e547]': "77000000000"
                        - generic "Pa" [ref=e548]
                        - generic "Quantity readout" [ref=e549]:
                          - generic [ref=e550]: 77000000000 Pa
                      - 'gridcell "material:scroll-15 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e551]':
                        - 'button "material:scroll-15 Thermal: 0.000012 1/degC" [ref=e552]': "0.000012"
                        - generic "1/degC" [ref=e553]
                        - generic "Quantity readout" [ref=e554]:
                          - generic [ref=e555]: 0.000012 1/degC
                      - 'gridcell "material:scroll-15 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e556]':
                        - 'button "material:scroll-15 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e557]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:scroll-16 material:scroll-16 Label: Invented scroll material 16 material:scroll-16 Elastic: 200000000000 Pa Pa Quantity readout material:scroll-16 Shear: 77000000000 Pa Pa Quantity readout material:scroll-16 Thermal: 0.000012 1/degC 1/degC Quantity readout material:scroll-16 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e559]':
                      - rowheader "material:scroll-16" [ref=e560]:
                        - button "material:scroll-16" [ref=e561]
                      - 'gridcell "material:scroll-16 Label: Invented scroll material 16" [ref=e562]':
                        - 'button "material:scroll-16 Label: Invented scroll material 16" [ref=e563]': Invented scroll material 16
                      - 'gridcell "material:scroll-16 Elastic: 200000000000 Pa Pa Quantity readout" [ref=e564]':
                        - 'button "material:scroll-16 Elastic: 200000000000 Pa" [ref=e565]': "200000000000"
                        - generic "Pa" [ref=e566]
                        - generic "Quantity readout" [ref=e567]:
                          - generic [ref=e568]: 200000000000 Pa
                      - 'gridcell "material:scroll-16 Shear: 77000000000 Pa Pa Quantity readout" [ref=e569]':
                        - 'button "material:scroll-16 Shear: 77000000000 Pa" [ref=e570]': "77000000000"
                        - generic "Pa" [ref=e571]
                        - generic "Quantity readout" [ref=e572]:
                          - generic [ref=e573]: 77000000000 Pa
                      - 'gridcell "material:scroll-16 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e574]':
                        - 'button "material:scroll-16 Thermal: 0.000012 1/degC" [ref=e575]': "0.000012"
                        - generic "1/degC" [ref=e576]
                        - generic "Quantity readout" [ref=e577]:
                          - generic [ref=e578]: 0.000012 1/degC
                      - 'gridcell "material:scroll-16 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e579]':
                        - 'button "material:scroll-16 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e580]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - group "Material fields footer" [ref=e581]:
                - generic [ref=e582]:
                  - generic [ref=e584]:
                    - generic [ref=e585]: Editing Provenance · material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity
                    - button "Apply" [ref=e586]
                    - button "Cancel" [ref=e587]
                  - group "Pan table status" [ref=e588]:
                    - button "Earlier table status" [ref=e589]: ‹
                    - button "Later table status" [disabled] [ref=e590]: ›
                - button "Material fields Info" [ref=e591]: Info
      - generic [ref=e593]:
        - generic [ref=e594]:
          - group "Viewport controls" [ref=e595]:
            - generic [ref=e596]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e597]:
              - generic "Deformation · unavailable" [ref=e598] [cursor=pointer]
            - group "Viewport display toggles" [ref=e599]:
              - button "Labels" [pressed] [ref=e600]
              - button "Loads" [pressed] [ref=e601]
              - button "Grid" [pressed] [ref=e602]
            - group "Viewport selection tools" [ref=e603]:
              - button "Box Select" [ref=e604]
              - generic [ref=e605]:
                - generic [ref=e606]: Selection filter
                - combobox "Selection filter" [ref=e607]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e608]
              - button "Isolate" [disabled] [ref=e609]
              - button "Show All" [disabled] [ref=e610]
              - generic [ref=e611]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e612]
              - button "Fit Visible" [ref=e613]
              - button "Fit Selection" [disabled] [ref=e614]
            - group "Viewport geometry" [ref=e615]:
              - button "Schematic" [pressed] [ref=e616]
              - button "Actual OD" [ref=e617]
              - button "Measure" [ref=e618]
          - generic "Viewport status" [ref=e619]:
            - 'generic "Selected material: material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity" [ref=e620]': "Selected: material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity"
            - status "Schematic centerline geometry" [ref=e621]
            - status "View command status" [ref=e622]: No view command dispatched.
        - generic [ref=e623]:
          - generic "Three.js pipe centerline viewport" [ref=e624]
          - generic "Viewport entity selection":
            - button "Select UI benchmark node 00010 in viewport" [ref=e626] [cursor=pointer]:
              - img [ref=e627]
              - generic [ref=e630]: UIF-PRECISION-00010
            - button "Select UI benchmark pipe 00016 in viewport" [ref=e631] [cursor=pointer]:
              - img [ref=e632]
              - generic [ref=e636]: UIF-PRECISION-00016
            - button "Select UI benchmark pipe 00004 in viewport" [ref=e637] [cursor=pointer]:
              - img [ref=e638]
              - generic [ref=e642]: UIF-PRECISION-00004
            - button "Select UI benchmark node 00020 in viewport" [ref=e643] [cursor=pointer]:
              - img [ref=e644]
              - generic [ref=e647]: UIF-PRECISION-00020
            - button "Select UI benchmark support 00020 in viewport" [ref=e648] [cursor=pointer]:
              - img [ref=e649]
              - generic [ref=e652]: UIF-PRECISION-00020
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e653]:
            - button "Front" [ref=e654] [cursor=pointer]
            - button "Top" [ref=e655] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e656] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e657]:
          - generic "Object creation tools" [ref=e658]:
            - button "Node" [ref=e659] [cursor=pointer]:
              - img [ref=e660]
              - text: Node
            - button "Pipe" [ref=e662] [cursor=pointer]:
              - img [ref=e663]
              - text: Pipe
            - button "Support" [ref=e667] [cursor=pointer]:
              - img [ref=e668]
              - text: Support
            - button "Component" [ref=e671] [cursor=pointer]:
              - img [ref=e672]
              - text: Component
            - button "Load" [ref=e675] [cursor=pointer]:
              - img [ref=e676]
              - text: Load
          - generic "Model focus" [ref=e678]: Select
          - group [ref=e679]:
            - generic "Selection & navigation" [ref=e680] [cursor=pointer]
      - separator "Resize table drawer" [ref=e681]
      - generic [ref=e682]:
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e683]:
          - 'heading "Invented scroll material 0 — material: material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity" [level=2] [ref=e684]':
            - text: Invented scroll material 0
            - generic [ref=e685]: "— material: material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity"
          - tablist "Inspector views" [ref=e686]:
            - tab "Properties" [selected] [ref=e687]
            - tab "Task" [ref=e688]
          - tabpanel [ref=e689]:
            - group [ref=e690]:
              - generic "All properties" [ref=e691] [cursor=pointer]
          - generic [ref=e692]:
            - group [ref=e693]:
              - generic "Sources and units" [ref=e694] [cursor=pointer]
            - region "Material temperature table" [ref=e695]:
              - heading "Temperature-dependent properties" [level=3] [ref=e696]
              - paragraph [ref=e697]: Enter values with their units and provenance. An incomplete point remains incomplete; load-case validation checks whether selected points can be used. Removing a referenced point may be blocked.
              - group "Invented scroll material 0" [ref=e698]:
                - generic [ref=e699]: Invented scroll material 0
                - paragraph [ref=e700]: No temperature points. Add a point to enter temperature-dependent properties.
                - button "Add temperature point" [ref=e701]
                - button "Queue temperature points" [ref=e702]
            - region "Remove selected entity" [ref=e703]:
              - heading "Remove material" [level=3] [ref=e704]
              - paragraph [ref=e705]:
                - text: Remove
                - strong [ref=e706]: Invented scroll material 0
                - text: (material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity). References from other records can block removal. No connected records will be removed automatically.
              - button "Queue removal for review" [ref=e707]
              - paragraph [ref=e708]: The queued change must be validated and explicitly applied in Pending changes.
            - group [ref=e709]:
              - generic "New section" [ref=e710] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e711]:
              - generic "New material" [ref=e712] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e713]:
              - generic "New support" [ref=e714] [cursor=pointer]
              - text: ▾
            - group [ref=e715]:
              - generic "New component" [ref=e716] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e717]:
      - button "Agent" [disabled] [ref=e719]:
        - img [ref=e720]
        - generic [ref=e723]: Agent
  - generic "Workspace status" [ref=e724]:
    - generic "Analysis statuses" [ref=e725]:
      - button "Solver · Not solved" [ref=e727] [cursor=pointer]
    - button "2 Issues" [ref=e728] [cursor=pointer]:
      - img [ref=e729]
      - text: 2 Issues
    - generic "Selection" [ref=e731]: "material: material:scroll-0-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity-retained-identity"
    - generic "Display units" [ref=e732]: Entered
    - button "About SWBPIPE…" [ref=e733] [cursor=pointer]:
      - img [ref=e734]
```

# Test source

```ts
  728 |     const details = page.getByRole("dialog", { name: "Table details", exact: true });
  729 |     const drawer = page.getByTestId("toggle-tree"), table = page.getByTestId("engineering-table");
  730 |     const observations: unknown[] = [];
  731 |     for (const traversal of ["Tab", "Shift+Tab"]) {
  732 |       await trigger.click(); await expect(details).toBeFocused(); await page.keyboard.press(traversal);
  733 |       await expect(details).toBeVisible(); await expect(details).not.toBeFocused();
  734 |       observations.push({ traversal, focusBeforeEscape: await page.evaluate(() => document.activeElement?.outerHTML) });
  735 |       await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused(); await expect(drawer).toHaveAttribute("aria-expanded", "true");
  736 |     }
  737 |     await trigger.click(); await expect(details).toBeFocused(); await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused();
  738 |     // Native typeahead changes the family through an actual key, while the
  739 |     // platform popup itself is outside headless Chromium keyboard control.
  740 |     await family.focus(); await page.keyboard.press("p"); await expect(family).toHaveValue("pipes");
  741 |     await family.selectOption("nodes"); await expect(family).toHaveValue("nodes");
  742 |     const node = model.nodes[0], cell = page.getByTestId(`table-cell-${node.id}-x`);
  743 |     await cell.dblclick(); const input = table.getByRole("textbox"); await input.fill("invalid retained"); await input.press("Enter");
  744 |     await trigger.click(); await expect(details).toBeVisible(); await input.focus();
  745 |     // Descendant editor Escape runs first; it must not be stolen by Details.
  746 |     await page.keyboard.press("Escape"); await expect(input).toHaveCount(0); await expect(details).toBeVisible(); await expect(cell).toHaveText(String(node.position.x));
  747 |     await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused(); await expect(drawer).toHaveAttribute("aria-expanded", "true");
  748 |     await family.selectOption("sections"); const section = page.getByTestId("section-engineering-table"), type = page.getByTestId("table-cell-section:repair-type");
  749 |     await type.dblclick(); const enumeration = section.getByRole("combobox"); await enumeration.fill("p"); await trigger.click(); await expect(details).toBeVisible();
  750 |     await enumeration.focus(); const popup = page.getByRole("listbox", { name: "Supported values" }); await expect(popup).toBeVisible();
  751 |     await page.keyboard.press("Escape"); await expect(popup).toHaveCount(0); await expect(enumeration).toHaveValue("p"); await expect(details).toBeVisible();
  752 |     await page.keyboard.press("Escape"); await expect(enumeration).toHaveCount(0); await expect(type).toHaveText("pipe"); await expect(details).toBeVisible();
  753 |     await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused(); await expect(drawer).toHaveAttribute("aria-expanded", "true");
  754 |     await family.selectOption("nodes"); await cell.dblclick(); await input.fill("retained transition"); await input.press("Enter"); const originalInput = await input.elementHandle();
  755 |     await trigger.click(); await expect(details).toBeFocused();
  756 |     // True focus exit closes Details without stealing focus from the destination.
  757 |     const outside = page.getByTestId("workspace-undo"); const destination = page.getByTestId("view-switch-table");
  758 |     await destination.focus(); await expect(details).toBeHidden(); await expect(destination).toBeFocused();
  759 |     await destination.click(); await expect(family).toHaveCount(0); await expect(page.locator(".compact-table-details")).toHaveCount(0); await expect(input).toHaveValue("retained transition"); expect(await originalInput!.evaluate((element) => element.isConnected)).toBe(true);
  760 |     await input.focus(); await page.keyboard.press("Escape"); await expect(input).toHaveCount(0); await expect(cell).toBeFocused(); await expect(outside).toBeDisabled();
  761 |     await page.getByTestId(`view-switch-${view}`).click(); await expect(family).toBeVisible();
  762 |     await cell.dblclick(); await input.fill("retained tree"); await input.press("Enter"); const treeInput = await input.elementHandle();
  763 |     await trigger.click(); await page.getByTestId("layout-mode-tree").click(); await expect(page.locator(".compact-table-details")).toHaveCount(0); await expect(family).toHaveCount(0);
  764 |     await page.getByTestId("layout-mode-grid").click(); await expect(input).toHaveValue("retained tree"); expect(await treeInput!.evaluate((element) => element.isConnected)).toBe(true);
  765 |     // The previous editor was deliberately cancelled, not unmounted by view change.
  766 |     expect(await originalInput!.evaluate((element) => element.isConnected)).toBe(false);
  767 |     await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(outside).toBeDisabled();
  768 |     if (view === "both") {
  769 |       await family.focus(); await page.keyboard.press("Escape"); await expect(drawer).toHaveAttribute("aria-expanded", "false");
  770 |       await drawer.click(); await expect(family).toBeVisible(); await expect(page.locator(".compact-table-details:popover-open")).toHaveCount(0);
  771 |     }
  772 |     await info.attach("details-escape-regression", { body: JSON.stringify({ view, observations }, null, 2), contentType: "application/json" });
  773 |   });
  774 | }
  775 | 
  776 | const classicTest = test.extend({ browser: [async ({ playwright }, use) => {
  777 |   const chrome = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
  778 |   const browser = await playwright.chromium.launch({ ignoreDefaultArgs: ["--hide-scrollbars"], executablePath: existsSync(chrome) ? chrome : undefined });
  779 |   try { await use(browser); } finally { await browser.close(); }
  780 | }, { scope: "worker" }] });
  781 | classicTest.describe("classic scrollbar allocation", () => {
  782 | for (const density of ["comfortable", "compact"] as const) for (const count of [1, 5, 160]) {
  783 |   classicTest(`B4 classic scrollbar compact ${density} ${count} preserves tracks and nonmutating pans @explicit-viewport`, async ({ page, browser }, info) => {
  784 |     await attachBrowserIdentity(browser, info); await page.setViewportSize({ width: 1280, height: 800 });
  785 |     await page.addInitScript((density) => localStorage.setItem("chirality.desktop.ui-preferences.v1", JSON.stringify({ version: 1, density, tableDrawerPx: 180 })), density);
  786 |     const { model } = await readFixture("precision-origin-base.model.json"), material = model.materials[0];
  787 |     model.materials = Array.from({ length: count }, (_, i) => ({ ...material, id: `material:scroll-${i}${i === 0 ? "-retained-identity".repeat(8) : ""}`, label: `Invented scroll material ${i}` }));
  788 |     const firstId = model.materials[0].id;
  789 |     model.pipe_segments = model.pipe_segments.map((pipe: any) => ({ ...pipe, material: pipe.material === material.id ? firstId : pipe.material }));
  790 |     model.sections = Array.from({ length: count }, (_, i) => ({ id: `section:scroll-${i}`, name: `Invented scroll section ${i}`, section_type: "pipe", properties: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 10, unit: "mm" } }, provenance: "invented scrollbar fixture" }));
  791 |     await gotoModel(page, model);
  792 |     // Force an actual Chromium classic allocation, then measure it. This is
  793 |     // browser evidence with nonzero gutters, never a native WebKit witness.
  794 |     await page.addStyleTag({ content: "*::-webkit-scrollbar { width: 18px; height: 18px; } *::-webkit-scrollbar-thumb { background: #777; } *::-webkit-scrollbar-track { background: #eee; }" });
  795 |     const probe = await page.evaluate(() => { const e = document.createElement("div"); e.style.cssText = "position:fixed;width:100px;height:100px;overflow:scroll"; e.innerHTML = '<div style="width:200px;height:200px"></div>'; document.body.append(e); const result = { vertical: e.offsetWidth - e.clientWidth, horizontal: e.offsetHeight - e.clientHeight }; e.remove(); return result; });
  796 |     expect(probe.vertical).toBeGreaterThan(0); expect(probe.horizontal).toBeGreaterThan(0);
  797 |     await page.getByTestId("view-switch-model").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  798 |     const family = page.getByRole("combobox", { name: "Grid family" }); await family.selectOption("materials");
  799 |     const table = page.getByTestId("material-engineering-table"), rows = table.getByRole("rowgroup"), later = page.getByRole("button", { name: "Later columns", exact: true }), earlier = page.getByRole("button", { name: "Earlier columns", exact: true });
  800 |     const observations: unknown[] = [];
  801 |     let expectedHost: number | null = 127;
  802 |     async function measure(surface = table) {
  803 |       const value = await surface.evaluate((root) => {
  804 |         const rows = root.querySelector<HTMLElement>('[role="rowgroup"]')!, header = root.querySelector<HTMLElement>(".engineering-table-header")!, row = rows.querySelector<HTMLElement>('[role="row"]')!;
  805 |         const bounds = (e: Element) => { const r = e.getBoundingClientRect(); return { x: r.x, right: r.right, y: r.y, bottom: r.bottom, width: r.width, height: r.height }; };
  806 |         const toolbar = root.closest(".model-tree")!.querySelector<HTMLElement>(".model-grid-toolbar")!, footer = root.querySelector<HTMLElement>(".engineering-table-footer")!;
  807 |         return { body: bounds(rows), clientWidth: rows.clientWidth, clientHeight: rows.clientHeight, gutter: rows.offsetWidth - rows.clientWidth,
  808 |           horizontal: rows.offsetHeight - rows.clientHeight, toolbar: bounds(toolbar), footer: bounds(footer), host: bounds(root.closest(".shell-tree-host")!),
  809 |           columns: [...header.children].map(bounds), cells: [...row.children].map(bounds),
  810 |           headingHeights: [...header.querySelectorAll("button")].map((button) => { const range = document.createRange(); range.selectNodeContents(button); return range.getBoundingClientRect().height; }),
  811 |           rails: [...root.closest(".model-tree")!.querySelectorAll<HTMLElement>(".table-overflow-rail.enabled > .table-overflow-viewport")].filter((e) => e.getBoundingClientRect().height > 0).map((e) => ({ client: e.clientHeight, height: e.offsetHeight, scrollWidth: e.scrollWidth, width: e.clientWidth })) };
  812 |       }); observations.push(value);
  813 |       if (expectedHost === null) expect(value.host.height).toBeGreaterThan(227); else expect(value.host.height).toBe(expectedHost); expect(value.toolbar.height).toBe(30); expect(value.footer.height).toBe(32);
  814 |       expect(value.clientHeight).toBeGreaterThanOrEqual(density === "comfortable" ? 36 : 30); expect(value.horizontal).toBe(0);
  815 |       expect(value.body.right).toBeCloseTo(value.host.right, 0);
  816 |       value.columns.forEach((column, i) => { expect(value.cells[i].x).toBeCloseTo(column.x, 0); expect(value.cells[i].right).toBeCloseTo(column.right, 0); });
  817 |       value.headingHeights.forEach((height) => expect(height).toBeLessThanOrEqual(28)); value.rails.forEach((rail) => { expect(rail.client).toBe(28); expect(rail.height).toBe(28); });
  818 |       return value;
  819 |     }
  820 |     let geometry = await measure(); expect(geometry.gutter).toBe(count > 1 ? probe.vertical : 0);
  821 |     await expect(table.getByRole("button", { name: "Sort Elastic [row unit]", exact: true })).toHaveAccessibleName("Sort Elastic [row unit]");
  822 |     while (await later.isEnabled()) await later.click(); geometry = await measure();
  823 |     expect(geometry.cells.at(-1)!.right).toBeLessThanOrEqual(geometry.body.x + geometry.clientWidth + 1);
  824 |     const last = table.getByTestId(`table-cell-${firstId}-provenance`); await last.click(); await last.press("Enter"); const editor = table.getByRole("textbox");
  825 |     await editor.fill("Retained valid draft while navigating chrome"); const retained = await editor.elementHandle();
  826 |     const statusLater = table.getByRole("button", { name: "Later table status", exact: true }); await expect(statusLater).toBeVisible(); await statusLater.click();
  827 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
> 828 |     await earlier.click(); expect(await page.evaluate(() => document.activeElement?.getAttribute("aria-label"))).toMatch(/^(Earlier|Later) columns$/); await expect(editor).toHaveValue("Retained valid draft while navigating chrome"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
      |                                                                                                                  ^ TypeError: expect(received).toMatch(expected)
  829 |     while (await later.isEnabled()) await later.click(); await measure(); await expect(editor).toHaveValue("Retained valid draft while navigating chrome");
  830 |     const editorBounds = await editor.boundingBox(), rowBounds = await rows.boundingBox(); expect(editorBounds!.x).toBeGreaterThanOrEqual(rowBounds!.x - 1); expect(editorBounds!.x + editorBounds!.width).toBeLessThanOrEqual(rowBounds!.x + geometry.clientWidth + 1);
  831 |     const infoButton = table.getByRole("button", { name: "Material fields Info", exact: true }); await infoButton.click(); const dialog = table.getByRole("dialog", { name: "Material fields Info", exact: true }); await expect(dialog).toBeVisible();
  832 |     await expect(dialog).toContainText("unit as entered in each row"); await expect(dialog).toContainText(firstId); await expect(editor).toHaveValue("Retained valid draft while navigating chrome"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  833 |     await page.keyboard.press("Tab"); await expect(dialog.getByRole("button", { name: "Close Info", exact: true })).toBeFocused();
  834 |     await page.keyboard.press("Tab"); await expect(dialog).toBeHidden(); await expect(infoButton).not.toBeFocused();
  835 |     await infoButton.click(); await expect(dialog).toBeFocused(); await page.keyboard.press("Escape"); await expect(infoButton).toBeFocused();
  836 |     const toolsLater = page.getByRole("button", { name: "Later table controls", exact: true }); if (await toolsLater.count()) { await toolsLater.click(); await expect(editor).toHaveValue("Retained valid draft while navigating chrome"); }
  837 |     await page.getByRole("button", { name: "Table details", exact: true }).focus(); await page.keyboard.press("Enter"); await expect(page.getByRole("dialog", { name: "Table details", exact: true })).toBeVisible(); await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.keyboard.press("Escape");
  838 |     await table.getByRole("button", { name: "Cancel", exact: true }).focus(); await page.keyboard.press("Enter"); expect(await retained!.evaluate((e) => e.isConnected)).toBe(false); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  839 |     const firstCell = table.getByTestId(`table-cell-${firstId}-label`); await firstCell.focus(); geometry = await measure(); expect(geometry.cells[1].x).toBeGreaterThanOrEqual(geometry.body.x - 1);
  840 |     await last.focus(); geometry = await measure(); expect(geometry.cells.at(-1)!.right).toBeLessThanOrEqual(geometry.body.x + geometry.clientWidth + 1);
  841 |     if (count > 1) { const r = await rows.boundingBox(); await page.mouse.move(r!.x + 40, r!.y + 12); await page.mouse.wheel(0, 100000); await expect.poll(() => rows.evaluate((e) => e.scrollTop)).toBeGreaterThan(0); await expect(table.getByRole("rowheader").getByRole("button", { name: `material:scroll-${count - 1}`, exact: true })).toBeVisible(); }
  842 |     await page.getByTestId("model-tree-filter-input").fill("scroll-0"); geometry = await measure(); expect(geometry.gutter).toBe(0);
  843 |     await page.getByTestId("model-tree-filter-input").fill("no matching invented entity"); await expect(rows.locator('[role="row"]')).toHaveCount(0); await expect(page.getByRole("group", { name: "Pan columns", exact: true })).toHaveCount(1);
  844 |     await page.getByTestId("model-tree-filter-input").fill(""); await family.selectOption("sections"); const section = page.getByTestId("section-engineering-table"); await measure(section); await expect(page.getByRole("group", { name: "Pan columns", exact: true })).toHaveCount(1);
  845 |     await page.getByTestId("layout-mode-tree").click(); await expect(page.getByRole("group", { name: "Pan columns", exact: true })).toHaveCount(0); await page.getByTestId("layout-mode-grid").click(); await family.selectOption("sections"); await measure(section);
  846 |     const resize = page.getByRole("separator", { name: "Resize table drawer" }); const resizeBox = await resize.boundingBox();
  847 |     await page.mouse.move(resizeBox!.x + resizeBox!.width / 2, resizeBox!.y + resizeBox!.height / 2); await page.mouse.down();
  848 |     await page.mouse.move(resizeBox!.x + resizeBox!.width / 2, resizeBox!.y + resizeBox!.height / 2 - 100); await page.mouse.up();
  849 |     await expect(resize).toHaveAttribute("aria-valuenow", "280"); expectedHost = 227; await measure(section);
  850 |     await resize.focus(); for (let i = 0; i < 10; i++) await page.keyboard.press("ArrowUp"); expectedHost = null; await measure(section);
  851 |     await page.setViewportSize({ width: 1024, height: 768 }); await measure(section);
  852 |     await page.getByTestId("view-switch-table").click();
  853 |     const full = await section.evaluate((root) => {
  854 |       const body = root.querySelector<HTMLElement>('[role="rowgroup"]')!, header = root.querySelector('.engineering-table-header')!, row = body.querySelector('[role="row"]')!;
  855 |       return { gutter: body.offsetWidth - body.clientWidth, header: [...header.children].map((e) => e.getBoundingClientRect().right), row: [...row.children].map((e) => e.getBoundingClientRect().right) };
  856 |     }); observations.push({ noncompact: full }); full.header.forEach((right, index) => expect(full.row[index]).toBeCloseTo(right, 0));
  857 |     if (count === 160) expect(full.gutter).toBe(probe.vertical);
  858 |     await info.attach("classic-scrollbar-observations", { body: JSON.stringify({ density, count, probe, observations }, null, 2), contentType: "application/json" });
  859 |   });
  860 | }
  861 | 
  862 | });
  863 | 
```