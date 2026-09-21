# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 virtual Grid confines body scrolling and boundary wheel without moving chrome
- Location: e2e/b4-table-editing.spec.ts:208:1

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 1

@@ -24,11 +24,11 @@
      "y": 692,
    },
    "header": Object {
      "height": 33,
      "width": 700,
-     "x": 68,
+     "x": -308,
      "y": 369.1875,
    },
    "host": Object {
      "height": 667,
      "width": 348,

Call Log:
- Timeout 10000ms exceeded while waiting on the predicate
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
      - paragraph [ref=e18]: Generated UI foundation 1000-pipe model
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
      - button "Model" [ref=e38] [cursor=pointer]:
        - img [ref=e39]
        - generic [ref=e42]: Model
      - button "Both" [pressed] [ref=e44] [cursor=pointer]:
        - img [ref=e45]
        - generic [ref=e47]: Both
    - generic [ref=e48]:
      - button "Run" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Run
      - button "Issues, 3" [ref=e53] [cursor=pointer]:
        - img [ref=e54]
        - generic [ref=e56]: Issues
        - generic [ref=e57]: "3"
    - group "Panels" [ref=e58]:
      - button "Inspector" [expanded] [ref=e60] [cursor=pointer]:
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
          - button "Issues, 3" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "3"
    - region "Modeling workspace" [ref=e131]:
      - generic [ref=e132]:
        - group "Tables" [ref=e133]:
          - button "Model" [pressed] [ref=e134] [cursor=pointer]
          - button "Review changes" [ref=e135] [cursor=pointer]
          - button "Collapse table drawer" [disabled] [expanded] [ref=e138]:
            - img [ref=e139]
        - generic "Model tree" [ref=e143]:
          - generic [ref=e144]: Model
          - region "Layout grid mode" [ref=e145]:
            - button "Tree" [ref=e146]:
              - img [ref=e147]
              - text: Tree
            - button "Grid" [active] [pressed] [ref=e150]:
              - img [ref=e151]
              - text: Grid
          - region "Model tree filtering" [ref=e153]:
            - generic [ref=e154]:
              - img [ref=e155]
              - generic [ref=e158]: Filter model
              - searchbox "Filter model tree" [ref=e159]
            - generic [ref=e160]: 2069 of 2069 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e161]:
              - img [ref=e162]
          - region "Bulk entity grid" [ref=e166]:
            - generic "Grid entity type" [ref=e167]:
              - button "Nodes" [pressed] [ref=e168]
              - button "Pipes" [ref=e169]
              - button "Supports" [ref=e170]
              - button "Materials" [ref=e171]
              - button "Sections" [ref=e172]
              - button "Components" [ref=e173]
              - button "Load Cases" [ref=e174]
              - button "Combinations" [ref=e175]
            - generic [ref=e177]:
              - grid "Node fields" [ref=e178]:
                - row "Node Sort Label Sort X Sort Y Sort Z Sort Provenance" [ref=e179]:
                  - columnheader "Node" [ref=e180]
                  - columnheader "Sort Label" [ref=e181]:
                    - button "Sort Label" [ref=e182]: Label ↕
                  - columnheader "Sort X" [ref=e183]:
                    - button "Sort X" [ref=e184]: X [m] ↕
                  - columnheader "Sort Y" [ref=e185]:
                    - button "Sort Y" [ref=e186]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e187]:
                    - button "Sort Z" [ref=e188]: Z [m] ↕
                  - columnheader "Sort Provenance" [ref=e189]:
                    - button "Sort Provenance" [ref=e190]: Provenance ↕
                - rowgroup [ref=e192]:
                  - generic [ref=e193]:
                    - 'row "node:UIF-00000 node:UIF-00000 Label: UI benchmark node 00000 node:UIF-00000 X: 0 m node:UIF-00000 Y: 0 m node:UIF-00000 Z: 0 m node:UIF-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e195]':
                      - rowheader "node:UIF-00000" [ref=e196]:
                        - button "node:UIF-00000" [ref=e197]
                      - 'gridcell "node:UIF-00000 Label: UI benchmark node 00000" [ref=e198]':
                        - 'button "node:UIF-00000 Label: UI benchmark node 00000" [ref=e199]': UI benchmark node 00000
                      - 'gridcell "node:UIF-00000 X: 0 m" [ref=e200]':
                        - 'button "node:UIF-00000 X: 0 m" [ref=e201]': "0"
                      - 'gridcell "node:UIF-00000 Y: 0 m" [ref=e202]':
                        - 'button "node:UIF-00000 Y: 0 m" [ref=e203]': "0"
                      - 'gridcell "node:UIF-00000 Z: 0 m" [ref=e204]':
                        - 'button "node:UIF-00000 Z: 0 m" [ref=e205]': "0"
                      - 'gridcell "node:UIF-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e206]':
                        - 'button "node:UIF-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e207]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00011 node:UIF-00011 Label: UI benchmark node 00011 node:UIF-00011 X: 1.1 m node:UIF-00011 Y: -0.00614 m node:UIF-00011 Z: -0.000131 m node:UIF-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e209]':
                      - rowheader "node:UIF-00011" [ref=e210]:
                        - button "node:UIF-00011" [ref=e211]
                      - 'gridcell "node:UIF-00011 Label: UI benchmark node 00011" [ref=e212]':
                        - 'button "node:UIF-00011 Label: UI benchmark node 00011" [ref=e213]': UI benchmark node 00011
                      - 'gridcell "node:UIF-00011 X: 1.1 m" [ref=e214]':
                        - 'button "node:UIF-00011 X: 1.1 m" [ref=e215]': "1.1"
                      - 'gridcell "node:UIF-00011 Y: -0.00614 m" [ref=e216]':
                        - 'button "node:UIF-00011 Y: -0.00614 m" [ref=e217]': "-0.00614"
                      - 'gridcell "node:UIF-00011 Z: -0.000131 m" [ref=e218]':
                        - 'button "node:UIF-00011 Z: -0.000131 m" [ref=e219]': "-0.000131"
                      - 'gridcell "node:UIF-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e220]':
                        - 'button "node:UIF-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e221]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00012 node:UIF-00012 Label: UI benchmark node 00012 node:UIF-00012 X: 1.2 m node:UIF-00012 Y: 0.006592 m node:UIF-00012 Z: -0.003488 m node:UIF-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e223]':
                      - rowheader "node:UIF-00012" [ref=e224]:
                        - button "node:UIF-00012" [ref=e225]
                      - 'gridcell "node:UIF-00012 Label: UI benchmark node 00012" [ref=e226]':
                        - 'button "node:UIF-00012 Label: UI benchmark node 00012" [ref=e227]': UI benchmark node 00012
                      - 'gridcell "node:UIF-00012 X: 1.2 m" [ref=e228]':
                        - 'button "node:UIF-00012 X: 1.2 m" [ref=e229]': "1.2"
                      - 'gridcell "node:UIF-00012 Y: 0.006592 m" [ref=e230]':
                        - 'button "node:UIF-00012 Y: 0.006592 m" [ref=e231]': "0.006592"
                      - 'gridcell "node:UIF-00012 Z: -0.003488 m" [ref=e232]':
                        - 'button "node:UIF-00012 Z: -0.003488 m" [ref=e233]': "-0.003488"
                      - 'gridcell "node:UIF-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e234]':
                        - 'button "node:UIF-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e235]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00013 node:UIF-00013 Label: UI benchmark node 00013 node:UIF-00013 X: 1.3 m node:UIF-00013 Y: 0.000269 m node:UIF-00013 Z: -0.005106 m node:UIF-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e237]':
                      - rowheader "node:UIF-00013" [ref=e238]:
                        - button "node:UIF-00013" [ref=e239]
                      - 'gridcell "node:UIF-00013 Label: UI benchmark node 00013" [ref=e240]':
                        - 'button "node:UIF-00013 Label: UI benchmark node 00013" [ref=e241]': UI benchmark node 00013
                      - 'gridcell "node:UIF-00013 X: 1.3 m" [ref=e242]':
                        - 'button "node:UIF-00013 X: 1.3 m" [ref=e243]': "1.3"
                      - 'gridcell "node:UIF-00013 Y: 0.000269 m" [ref=e244]':
                        - 'button "node:UIF-00013 Y: 0.000269 m" [ref=e245]': "0.000269"
                      - 'gridcell "node:UIF-00013 Z: -0.005106 m" [ref=e246]':
                        - 'button "node:UIF-00013 Z: -0.005106 m" [ref=e247]': "-0.005106"
                      - 'gridcell "node:UIF-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e248]':
                        - 'button "node:UIF-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e249]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00014 node:UIF-00014 Label: UI benchmark node 00014 node:UIF-00014 X: 1.4 m node:UIF-00014 Y: 0.003938 m node:UIF-00014 Z: -0.008687 m node:UIF-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e251]':
                      - rowheader "node:UIF-00014" [ref=e252]:
                        - button "node:UIF-00014" [ref=e253]
                      - 'gridcell "node:UIF-00014 Label: UI benchmark node 00014" [ref=e254]':
                        - 'button "node:UIF-00014 Label: UI benchmark node 00014" [ref=e255]': UI benchmark node 00014
                      - 'gridcell "node:UIF-00014 X: 1.4 m" [ref=e256]':
                        - 'button "node:UIF-00014 X: 1.4 m" [ref=e257]': "1.4"
                      - 'gridcell "node:UIF-00014 Y: 0.003938 m" [ref=e258]':
                        - 'button "node:UIF-00014 Y: 0.003938 m" [ref=e259]': "0.003938"
                      - 'gridcell "node:UIF-00014 Z: -0.008687 m" [ref=e260]':
                        - 'button "node:UIF-00014 Z: -0.008687 m" [ref=e261]': "-0.008687"
                      - 'gridcell "node:UIF-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e262]':
                        - 'button "node:UIF-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e263]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00015 node:UIF-00015 Label: UI benchmark node 00015 node:UIF-00015 X: 1.5 m node:UIF-00015 Y: 0.007479 m node:UIF-00015 Z: -0.003472 m node:UIF-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e265]':
                      - rowheader "node:UIF-00015" [ref=e266]:
                        - button "node:UIF-00015" [ref=e267]
                      - 'gridcell "node:UIF-00015 Label: UI benchmark node 00015" [ref=e268]':
                        - 'button "node:UIF-00015 Label: UI benchmark node 00015" [ref=e269]': UI benchmark node 00015
                      - 'gridcell "node:UIF-00015 X: 1.5 m" [ref=e270]':
                        - 'button "node:UIF-00015 X: 1.5 m" [ref=e271]': "1.5"
                      - 'gridcell "node:UIF-00015 Y: 0.007479 m" [ref=e272]':
                        - 'button "node:UIF-00015 Y: 0.007479 m" [ref=e273]': "0.007479"
                      - 'gridcell "node:UIF-00015 Z: -0.003472 m" [ref=e274]':
                        - 'button "node:UIF-00015 Z: -0.003472 m" [ref=e275]': "-0.003472"
                      - 'gridcell "node:UIF-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e276]':
                        - 'button "node:UIF-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e277]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00016 node:UIF-00016 Label: UI benchmark node 00016 node:UIF-00016 X: 1.6 m node:UIF-00016 Y: -0.007864 m node:UIF-00016 Z: 0.0085 m node:UIF-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e279]':
                      - rowheader "node:UIF-00016" [ref=e280]:
                        - button "node:UIF-00016" [ref=e281]
                      - 'gridcell "node:UIF-00016 Label: UI benchmark node 00016" [ref=e282]':
                        - 'button "node:UIF-00016 Label: UI benchmark node 00016" [ref=e283]': UI benchmark node 00016
                      - 'gridcell "node:UIF-00016 X: 1.6 m" [ref=e284]':
                        - 'button "node:UIF-00016 X: 1.6 m" [ref=e285]': "1.6"
                      - 'gridcell "node:UIF-00016 Y: -0.007864 m" [ref=e286]':
                        - 'button "node:UIF-00016 Y: -0.007864 m" [ref=e287]': "-0.007864"
                      - 'gridcell "node:UIF-00016 Z: 0.0085 m" [ref=e288]':
                        - 'button "node:UIF-00016 Z: 0.0085 m" [ref=e289]': "0.0085"
                      - 'gridcell "node:UIF-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e290]':
                        - 'button "node:UIF-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e291]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00017 node:UIF-00017 Label: UI benchmark node 00017 node:UIF-00017 X: 1.7 m node:UIF-00017 Y: -0.007896 m node:UIF-00017 Z: -0.00007 m node:UIF-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e293]':
                      - rowheader "node:UIF-00017" [ref=e294]:
                        - button "node:UIF-00017" [ref=e295]
                      - 'gridcell "node:UIF-00017 Label: UI benchmark node 00017" [ref=e296]':
                        - 'button "node:UIF-00017 Label: UI benchmark node 00017" [ref=e297]': UI benchmark node 00017
                      - 'gridcell "node:UIF-00017 X: 1.7 m" [ref=e298]':
                        - 'button "node:UIF-00017 X: 1.7 m" [ref=e299]': "1.7"
                      - 'gridcell "node:UIF-00017 Y: -0.007896 m" [ref=e300]':
                        - 'button "node:UIF-00017 Y: -0.007896 m" [ref=e301]': "-0.007896"
                      - 'gridcell "node:UIF-00017 Z: -0.00007 m" [ref=e302]':
                        - 'button "node:UIF-00017 Z: -0.00007 m" [ref=e303]': "-0.00007"
                      - 'gridcell "node:UIF-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e304]':
                        - 'button "node:UIF-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e305]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00018 node:UIF-00018 Label: UI benchmark node 00018 node:UIF-00018 X: 1.8 m node:UIF-00018 Y: 0.004735 m node:UIF-00018 Z: 0.007458 m node:UIF-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e307]':
                      - rowheader "node:UIF-00018" [ref=e308]:
                        - button "node:UIF-00018" [ref=e309]
                      - 'gridcell "node:UIF-00018 Label: UI benchmark node 00018" [ref=e310]':
                        - 'button "node:UIF-00018 Label: UI benchmark node 00018" [ref=e311]': UI benchmark node 00018
                      - 'gridcell "node:UIF-00018 X: 1.8 m" [ref=e312]':
                        - 'button "node:UIF-00018 X: 1.8 m" [ref=e313]': "1.8"
                      - 'gridcell "node:UIF-00018 Y: 0.004735 m" [ref=e314]':
                        - 'button "node:UIF-00018 Y: 0.004735 m" [ref=e315]': "0.004735"
                      - 'gridcell "node:UIF-00018 Z: 0.007458 m" [ref=e316]':
                        - 'button "node:UIF-00018 Z: 0.007458 m" [ref=e317]': "0.007458"
                      - 'gridcell "node:UIF-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e318]':
                        - 'button "node:UIF-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e319]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00019 node:UIF-00019 Label: UI benchmark node 00019 node:UIF-00019 X: 1.9 m node:UIF-00019 Y: 0.008608 m node:UIF-00019 Z: 0.007815 m node:UIF-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e321]':
                      - rowheader "node:UIF-00019" [ref=e322]:
                        - button "node:UIF-00019" [ref=e323]
                      - 'gridcell "node:UIF-00019 Label: UI benchmark node 00019" [ref=e324]':
                        - 'button "node:UIF-00019 Label: UI benchmark node 00019" [ref=e325]': UI benchmark node 00019
                      - 'gridcell "node:UIF-00019 X: 1.9 m" [ref=e326]':
                        - 'button "node:UIF-00019 X: 1.9 m" [ref=e327]': "1.9"
                      - 'gridcell "node:UIF-00019 Y: 0.008608 m" [ref=e328]':
                        - 'button "node:UIF-00019 Y: 0.008608 m" [ref=e329]': "0.008608"
                      - 'gridcell "node:UIF-00019 Z: 0.007815 m" [ref=e330]':
                        - 'button "node:UIF-00019 Z: 0.007815 m" [ref=e331]': "0.007815"
                      - 'gridcell "node:UIF-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e332]':
                        - 'button "node:UIF-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e333]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00020 node:UIF-00020 Label: UI benchmark node 00020 node:UIF-00020 X: 2 m node:UIF-00020 Y: -0.002018 m node:UIF-00020 Z: 0.004189 m node:UIF-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e335]':
                      - rowheader "node:UIF-00020" [ref=e336]:
                        - button "node:UIF-00020" [ref=e337]
                      - 'gridcell "node:UIF-00020 Label: UI benchmark node 00020" [ref=e338]':
                        - 'button "node:UIF-00020 Label: UI benchmark node 00020" [ref=e339]': UI benchmark node 00020
                      - 'gridcell "node:UIF-00020 X: 2 m" [ref=e340]':
                        - 'button "node:UIF-00020 X: 2 m" [ref=e341]': "2"
                      - 'gridcell "node:UIF-00020 Y: -0.002018 m" [ref=e342]':
                        - 'button "node:UIF-00020 Y: -0.002018 m" [ref=e343]': "-0.002018"
                      - 'gridcell "node:UIF-00020 Z: 0.004189 m" [ref=e344]':
                        - 'button "node:UIF-00020 Z: 0.004189 m" [ref=e345]': "0.004189"
                      - 'gridcell "node:UIF-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e346]':
                        - 'button "node:UIF-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e347]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00021 node:UIF-00021 Label: UI benchmark node 00021 node:UIF-00021 X: 2.1 m node:UIF-00021 Y: 0.007456 m node:UIF-00021 Z: 0.00827 m node:UIF-00021 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e349]':
                      - rowheader "node:UIF-00021" [ref=e350]:
                        - button "node:UIF-00021" [ref=e351]
                      - 'gridcell "node:UIF-00021 Label: UI benchmark node 00021" [ref=e352]':
                        - 'button "node:UIF-00021 Label: UI benchmark node 00021" [ref=e353]': UI benchmark node 00021
                      - 'gridcell "node:UIF-00021 X: 2.1 m" [ref=e354]':
                        - 'button "node:UIF-00021 X: 2.1 m" [ref=e355]': "2.1"
                      - 'gridcell "node:UIF-00021 Y: 0.007456 m" [ref=e356]':
                        - 'button "node:UIF-00021 Y: 0.007456 m" [ref=e357]': "0.007456"
                      - 'gridcell "node:UIF-00021 Z: 0.00827 m" [ref=e358]':
                        - 'button "node:UIF-00021 Z: 0.00827 m" [ref=e359]': "0.00827"
                      - 'gridcell "node:UIF-00021 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e360]':
                        - 'button "node:UIF-00021 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e361]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00022 node:UIF-00022 Label: UI benchmark node 00022 node:UIF-00022 X: 2.2 m node:UIF-00022 Y: 0.005498 m node:UIF-00022 Z: -0.000413 m node:UIF-00022 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e363]':
                      - rowheader "node:UIF-00022" [ref=e364]:
                        - button "node:UIF-00022" [ref=e365]
                      - 'gridcell "node:UIF-00022 Label: UI benchmark node 00022" [ref=e366]':
                        - 'button "node:UIF-00022 Label: UI benchmark node 00022" [ref=e367]': UI benchmark node 00022
                      - 'gridcell "node:UIF-00022 X: 2.2 m" [ref=e368]':
                        - 'button "node:UIF-00022 X: 2.2 m" [ref=e369]': "2.2"
                      - 'gridcell "node:UIF-00022 Y: 0.005498 m" [ref=e370]':
                        - 'button "node:UIF-00022 Y: 0.005498 m" [ref=e371]': "0.005498"
                      - 'gridcell "node:UIF-00022 Z: -0.000413 m" [ref=e372]':
                        - 'button "node:UIF-00022 Z: -0.000413 m" [ref=e373]': "-0.000413"
                      - 'gridcell "node:UIF-00022 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e374]':
                        - 'button "node:UIF-00022 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e375]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00023 node:UIF-00023 Label: UI benchmark node 00023 node:UIF-00023 X: 2.3 m node:UIF-00023 Y: 0.000262 m node:UIF-00023 Z: 0.005477 m node:UIF-00023 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e377]':
                      - rowheader "node:UIF-00023" [ref=e378]:
                        - button "node:UIF-00023" [ref=e379]
                      - 'gridcell "node:UIF-00023 Label: UI benchmark node 00023" [ref=e380]':
                        - 'button "node:UIF-00023 Label: UI benchmark node 00023" [ref=e381]': UI benchmark node 00023
                      - 'gridcell "node:UIF-00023 X: 2.3 m" [ref=e382]':
                        - 'button "node:UIF-00023 X: 2.3 m" [ref=e383]': "2.3"
                      - 'gridcell "node:UIF-00023 Y: 0.000262 m" [ref=e384]':
                        - 'button "node:UIF-00023 Y: 0.000262 m" [ref=e385]': "0.000262"
                      - 'gridcell "node:UIF-00023 Z: 0.005477 m" [ref=e386]':
                        - 'button "node:UIF-00023 Z: 0.005477 m" [ref=e387]': "0.005477"
                      - 'gridcell "node:UIF-00023 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e388]':
                        - 'button "node:UIF-00023 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e389]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00024 node:UIF-00024 Label: UI benchmark node 00024 node:UIF-00024 X: 2.4 m node:UIF-00024 Y: -0.002368 m node:UIF-00024 Z: -0.001285 m node:UIF-00024 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e391]':
                      - rowheader "node:UIF-00024" [ref=e392]:
                        - button "node:UIF-00024" [ref=e393]
                      - 'gridcell "node:UIF-00024 Label: UI benchmark node 00024" [ref=e394]':
                        - 'button "node:UIF-00024 Label: UI benchmark node 00024" [ref=e395]': UI benchmark node 00024
                      - 'gridcell "node:UIF-00024 X: 2.4 m" [ref=e396]':
                        - 'button "node:UIF-00024 X: 2.4 m" [ref=e397]': "2.4"
                      - 'gridcell "node:UIF-00024 Y: -0.002368 m" [ref=e398]':
                        - 'button "node:UIF-00024 Y: -0.002368 m" [ref=e399]': "-0.002368"
                      - 'gridcell "node:UIF-00024 Z: -0.001285 m" [ref=e400]':
                        - 'button "node:UIF-00024 Z: -0.001285 m" [ref=e401]': "-0.001285"
                      - 'gridcell "node:UIF-00024 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e402]':
                        - 'button "node:UIF-00024 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e403]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00025 node:UIF-00025 Label: UI benchmark node 00025 node:UIF-00025 X: 2.5 m node:UIF-00025 Y: -0.004322 m node:UIF-00025 Z: 0.005539 m node:UIF-00025 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e405]':
                      - rowheader "node:UIF-00025" [ref=e406]:
                        - button "node:UIF-00025" [ref=e407]
                      - 'gridcell "node:UIF-00025 Label: UI benchmark node 00025" [ref=e408]':
                        - 'button "node:UIF-00025 Label: UI benchmark node 00025" [ref=e409]': UI benchmark node 00025
                      - 'gridcell "node:UIF-00025 X: 2.5 m" [ref=e410]':
                        - 'button "node:UIF-00025 X: 2.5 m" [ref=e411]': "2.5"
                      - 'gridcell "node:UIF-00025 Y: -0.004322 m" [ref=e412]':
                        - 'button "node:UIF-00025 Y: -0.004322 m" [ref=e413]': "-0.004322"
                      - 'gridcell "node:UIF-00025 Z: 0.005539 m" [ref=e414]':
                        - 'button "node:UIF-00025 Z: 0.005539 m" [ref=e415]': "0.005539"
                      - 'gridcell "node:UIF-00025 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e416]':
                        - 'button "node:UIF-00025 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e417]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00026 node:UIF-00026 Label: UI benchmark node 00026 node:UIF-00026 X: 2.6 m node:UIF-00026 Y: -0.002013 m node:UIF-00026 Z: 0.005263 m node:UIF-00026 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e419]':
                      - rowheader "node:UIF-00026" [ref=e420]:
                        - button "node:UIF-00026" [ref=e421]
                      - 'gridcell "node:UIF-00026 Label: UI benchmark node 00026" [ref=e422]':
                        - 'button "node:UIF-00026 Label: UI benchmark node 00026" [ref=e423]': UI benchmark node 00026
                      - 'gridcell "node:UIF-00026 X: 2.6 m" [ref=e424]':
                        - 'button "node:UIF-00026 X: 2.6 m" [ref=e425]': "2.6"
                      - 'gridcell "node:UIF-00026 Y: -0.002013 m" [ref=e426]':
                        - 'button "node:UIF-00026 Y: -0.002013 m" [ref=e427]': "-0.002013"
                      - 'gridcell "node:UIF-00026 Z: 0.005263 m" [ref=e428]':
                        - 'button "node:UIF-00026 Z: 0.005263 m" [ref=e429]': "0.005263"
                      - 'gridcell "node:UIF-00026 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e430]':
                        - 'button "node:UIF-00026 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e431]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00027 node:UIF-00027 Label: UI benchmark node 00027 node:UIF-00027 X: 2.7 m node:UIF-00027 Y: 0.003404 m node:UIF-00027 Z: 0.002876 m node:UIF-00027 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e433]':
                      - rowheader "node:UIF-00027" [ref=e434]:
                        - button "node:UIF-00027" [ref=e435]
                      - 'gridcell "node:UIF-00027 Label: UI benchmark node 00027" [ref=e436]':
                        - 'button "node:UIF-00027 Label: UI benchmark node 00027" [ref=e437]': UI benchmark node 00027
                      - 'gridcell "node:UIF-00027 X: 2.7 m" [ref=e438]':
                        - 'button "node:UIF-00027 X: 2.7 m" [ref=e439]': "2.7"
                      - 'gridcell "node:UIF-00027 Y: 0.003404 m" [ref=e440]':
                        - 'button "node:UIF-00027 Y: 0.003404 m" [ref=e441]': "0.003404"
                      - 'gridcell "node:UIF-00027 Z: 0.002876 m" [ref=e442]':
                        - 'button "node:UIF-00027 Z: 0.002876 m" [ref=e443]': "0.002876"
                      - 'gridcell "node:UIF-00027 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e444]':
                        - 'button "node:UIF-00027 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e445]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00028 node:UIF-00028 Label: UI benchmark node 00028 node:UIF-00028 X: 2.8 m node:UIF-00028 Y: -0.000109 m node:UIF-00028 Z: 0.004767 m node:UIF-00028 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e447]':
                      - rowheader "node:UIF-00028" [ref=e448]:
                        - button "node:UIF-00028" [ref=e449]
                      - 'gridcell "node:UIF-00028 Label: UI benchmark node 00028" [ref=e450]':
                        - 'button "node:UIF-00028 Label: UI benchmark node 00028" [ref=e451]': UI benchmark node 00028
                      - 'gridcell "node:UIF-00028 X: 2.8 m" [ref=e452]':
                        - 'button "node:UIF-00028 X: 2.8 m" [ref=e453]': "2.8"
                      - 'gridcell "node:UIF-00028 Y: -0.000109 m" [ref=e454]':
                        - 'button "node:UIF-00028 Y: -0.000109 m" [ref=e455]': "-0.000109"
                      - 'gridcell "node:UIF-00028 Z: 0.004767 m" [ref=e456]':
                        - 'button "node:UIF-00028 Z: 0.004767 m" [ref=e457]': "0.004767"
                      - 'gridcell "node:UIF-00028 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e458]':
                        - 'button "node:UIF-00028 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e459]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00029 node:UIF-00029 Label: UI benchmark node 00029 node:UIF-00029 X: 2.9 m node:UIF-00029 Y: -0.008207 m node:UIF-00029 Z: 0.008929 m node:UIF-00029 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e461]':
                      - rowheader "node:UIF-00029" [ref=e462]:
                        - button "node:UIF-00029" [ref=e463]
                      - 'gridcell "node:UIF-00029 Label: UI benchmark node 00029" [ref=e464]':
                        - 'button "node:UIF-00029 Label: UI benchmark node 00029" [ref=e465]': UI benchmark node 00029
                      - 'gridcell "node:UIF-00029 X: 2.9 m" [ref=e466]':
                        - 'button "node:UIF-00029 X: 2.9 m" [ref=e467]': "2.9"
                      - 'gridcell "node:UIF-00029 Y: -0.008207 m" [ref=e468]':
                        - 'button "node:UIF-00029 Y: -0.008207 m" [ref=e469]': "-0.008207"
                      - 'gridcell "node:UIF-00029 Z: 0.008929 m" [ref=e470]':
                        - 'button "node:UIF-00029 Z: 0.008929 m" [ref=e471]': "0.008929"
                      - 'gridcell "node:UIF-00029 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e472]':
                        - 'button "node:UIF-00029 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e473]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00030 node:UIF-00030 Label: UI benchmark node 00030 node:UIF-00030 X: 3 m node:UIF-00030 Y: 0.004987 m node:UIF-00030 Z: -0.006719 m node:UIF-00030 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e475]':
                      - rowheader "node:UIF-00030" [ref=e476]:
                        - button "node:UIF-00030" [ref=e477]
                      - 'gridcell "node:UIF-00030 Label: UI benchmark node 00030" [ref=e478]':
                        - 'button "node:UIF-00030 Label: UI benchmark node 00030" [ref=e479]': UI benchmark node 00030
                      - 'gridcell "node:UIF-00030 X: 3 m" [ref=e480]':
                        - 'button "node:UIF-00030 X: 3 m" [ref=e481]': "3"
                      - 'gridcell "node:UIF-00030 Y: 0.004987 m" [ref=e482]':
                        - 'button "node:UIF-00030 Y: 0.004987 m" [ref=e483]': "0.004987"
                      - 'gridcell "node:UIF-00030 Z: -0.006719 m" [ref=e484]':
                        - 'button "node:UIF-00030 Z: -0.006719 m" [ref=e485]': "-0.006719"
                      - 'gridcell "node:UIF-00030 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e486]':
                        - 'button "node:UIF-00030 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e487]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00031 node:UIF-00031 Label: UI benchmark node 00031 node:UIF-00031 X: 3.1 m node:UIF-00031 Y: 0.006932 m node:UIF-00031 Z: 0.004364 m node:UIF-00031 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e489]':
                      - rowheader "node:UIF-00031" [ref=e490]:
                        - button "node:UIF-00031" [ref=e491]
                      - 'gridcell "node:UIF-00031 Label: UI benchmark node 00031" [ref=e492]':
                        - 'button "node:UIF-00031 Label: UI benchmark node 00031" [ref=e493]': UI benchmark node 00031
                      - 'gridcell "node:UIF-00031 X: 3.1 m" [ref=e494]':
                        - 'button "node:UIF-00031 X: 3.1 m" [ref=e495]': "3.1"
                      - 'gridcell "node:UIF-00031 Y: 0.006932 m" [ref=e496]':
                        - 'button "node:UIF-00031 Y: 0.006932 m" [ref=e497]': "0.006932"
                      - 'gridcell "node:UIF-00031 Z: 0.004364 m" [ref=e498]':
                        - 'button "node:UIF-00031 Z: 0.004364 m" [ref=e499]': "0.004364"
                      - 'gridcell "node:UIF-00031 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e500]':
                        - 'button "node:UIF-00031 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e501]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00032 node:UIF-00032 Label: UI benchmark node 00032 node:UIF-00032 X: 3.2 m node:UIF-00032 Y: 0.005898 m node:UIF-00032 Z: 0.006945 m node:UIF-00032 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e503]':
                      - rowheader "node:UIF-00032" [ref=e504]:
                        - button "node:UIF-00032" [ref=e505]
                      - 'gridcell "node:UIF-00032 Label: UI benchmark node 00032" [ref=e506]':
                        - 'button "node:UIF-00032 Label: UI benchmark node 00032" [ref=e507]': UI benchmark node 00032
                      - 'gridcell "node:UIF-00032 X: 3.2 m" [ref=e508]':
                        - 'button "node:UIF-00032 X: 3.2 m" [ref=e509]': "3.2"
                      - 'gridcell "node:UIF-00032 Y: 0.005898 m" [ref=e510]':
                        - 'button "node:UIF-00032 Y: 0.005898 m" [ref=e511]': "0.005898"
                      - 'gridcell "node:UIF-00032 Z: 0.006945 m" [ref=e512]':
                        - 'button "node:UIF-00032 Z: 0.006945 m" [ref=e513]': "0.006945"
                      - 'gridcell "node:UIF-00032 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e514]':
                        - 'button "node:UIF-00032 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e515]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00033 node:UIF-00033 Label: UI benchmark node 00033 node:UIF-00033 X: 3.3 m node:UIF-00033 Y: 0.007963 m node:UIF-00033 Z: 0.001275 m node:UIF-00033 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e517]':
                      - rowheader "node:UIF-00033" [ref=e518]:
                        - button "node:UIF-00033" [ref=e519]
                      - 'gridcell "node:UIF-00033 Label: UI benchmark node 00033" [ref=e520]':
                        - 'button "node:UIF-00033 Label: UI benchmark node 00033" [ref=e521]': UI benchmark node 00033
                      - 'gridcell "node:UIF-00033 X: 3.3 m" [ref=e522]':
                        - 'button "node:UIF-00033 X: 3.3 m" [ref=e523]': "3.3"
                      - 'gridcell "node:UIF-00033 Y: 0.007963 m" [ref=e524]':
                        - 'button "node:UIF-00033 Y: 0.007963 m" [ref=e525]': "0.007963"
                      - 'gridcell "node:UIF-00033 Z: 0.001275 m" [ref=e526]':
                        - 'button "node:UIF-00033 Z: 0.001275 m" [ref=e527]': "0.001275"
                      - 'gridcell "node:UIF-00033 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e528]':
                        - 'button "node:UIF-00033 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e529]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00034 node:UIF-00034 Label: UI benchmark node 00034 node:UIF-00034 X: 3.4 m node:UIF-00034 Y: -0.00878 m node:UIF-00034 Z: -0.00457 m node:UIF-00034 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e531]':
                      - rowheader "node:UIF-00034" [ref=e532]:
                        - button "node:UIF-00034" [ref=e533]
                      - 'gridcell "node:UIF-00034 Label: UI benchmark node 00034" [ref=e534]':
                        - 'button "node:UIF-00034 Label: UI benchmark node 00034" [ref=e535]': UI benchmark node 00034
                      - 'gridcell "node:UIF-00034 X: 3.4 m" [ref=e536]':
                        - 'button "node:UIF-00034 X: 3.4 m" [ref=e537]': "3.4"
                      - 'gridcell "node:UIF-00034 Y: -0.00878 m" [ref=e538]':
                        - 'button "node:UIF-00034 Y: -0.00878 m" [ref=e539]': "-0.00878"
                      - 'gridcell "node:UIF-00034 Z: -0.00457 m" [ref=e540]':
                        - 'button "node:UIF-00034 Z: -0.00457 m" [ref=e541]': "-0.00457"
                      - 'gridcell "node:UIF-00034 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e542]':
                        - 'button "node:UIF-00034 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e543]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-00035 node:UIF-00035 Label: UI benchmark node 00035 node:UIF-00035 X: 3.5 m node:UIF-00035 Y: -0.008095 m node:UIF-00035 Z: -0.001466 m node:UIF-00035 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e545]':
                      - rowheader "node:UIF-00035" [ref=e546]:
                        - button "node:UIF-00035" [ref=e547]
                      - 'gridcell "node:UIF-00035 Label: UI benchmark node 00035" [ref=e548]':
                        - 'button "node:UIF-00035 Label: UI benchmark node 00035" [ref=e549]': UI benchmark node 00035
                      - 'gridcell "node:UIF-00035 X: 3.5 m" [ref=e550]':
                        - 'button "node:UIF-00035 X: 3.5 m" [ref=e551]': "3.5"
                      - 'gridcell "node:UIF-00035 Y: -0.008095 m" [ref=e552]':
                        - 'button "node:UIF-00035 Y: -0.008095 m" [ref=e553]': "-0.008095"
                      - 'gridcell "node:UIF-00035 Z: -0.001466 m" [ref=e554]':
                        - 'button "node:UIF-00035 Z: -0.001466 m" [ref=e555]': "-0.001466"
                      - 'gridcell "node:UIF-00035 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e556]':
                        - 'button "node:UIF-00035 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e557]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - group "Node fields footer" [ref=e558]:
                - generic [ref=e559]: 1001 of 1001 rows
            - button "Review multiple changes" [ref=e560] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e561]
      - generic [ref=e563]:
        - generic [ref=e564]:
          - group "Viewport controls" [ref=e565]:
            - generic [ref=e566]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e567]:
              - generic "Deformation · unavailable" [ref=e568] [cursor=pointer]
            - group "Viewport display toggles" [ref=e569]:
              - button "Labels" [pressed] [ref=e570]
              - button "Loads" [pressed] [ref=e571]
              - button "Grid" [pressed] [ref=e572]
            - group "Viewport selection tools" [ref=e573]:
              - button "Box Select" [ref=e574]
              - generic [ref=e575]:
                - generic [ref=e576]: Selection filter
                - combobox "Selection filter" [ref=e577]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e578]
              - button "Isolate" [disabled] [ref=e579]
              - button "Show All" [disabled] [ref=e580]
              - generic [ref=e581]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e582]
              - button "Fit Visible" [ref=e583]
              - button "Fit Selection" [disabled] [ref=e584]
            - group "Viewport geometry" [ref=e585]:
              - button "Schematic" [pressed] [ref=e586]
              - button "Actual OD" [ref=e587]
              - button "Measure" [ref=e588]
          - generic "Viewport status" [ref=e589]:
            - 'generic "Selected project: project:UIF-1000" [ref=e590]': "Selected: project:UIF-1000"
            - status "Schematic centerline geometry" [ref=e591]
            - status "View command status" [ref=e592]: No view command dispatched.
        - generic [ref=e593]:
          - generic "Three.js pipe centerline viewport" [ref=e594]
          - generic "Viewport entity selection":
            - button "Select UI benchmark node 00454 in viewport" [ref=e596] [cursor=pointer]:
              - img [ref=e597]
              - generic [ref=e600]: UIF-00454
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e601]:
            - button "Front" [ref=e602] [cursor=pointer]
            - button "Top" [ref=e603] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e604] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e605]:
          - generic "Object creation tools" [ref=e606]:
            - button "Node" [ref=e607] [cursor=pointer]:
              - img [ref=e608]
              - text: Node
            - button "Pipe" [ref=e610] [cursor=pointer]:
              - img [ref=e611]
              - text: Pipe
            - button "Support" [ref=e615] [cursor=pointer]:
              - img [ref=e616]
              - text: Support
            - button "Component" [ref=e619] [cursor=pointer]:
              - img [ref=e620]
              - text: Component
            - button "Load" [ref=e623] [cursor=pointer]:
              - img [ref=e624]
              - text: Load
          - generic "Model focus" [ref=e626]: Select
          - group [ref=e627]:
            - generic "Selection & navigation" [ref=e628] [cursor=pointer]
      - generic [ref=e629]:
        - button "Close inspector" [ref=e630]:
          - img [ref=e631]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e634]:
          - 'heading "Generated UI foundation 1000-pipe model — project: project:UIF-1000" [level=2] [ref=e635]':
            - text: Generated UI foundation 1000-pipe model
            - generic [ref=e636]: "— project: project:UIF-1000"
          - tablist "Inspector views" [ref=e637]:
            - tab "Properties" [selected] [ref=e638]
            - tab "Task" [ref=e639]
          - tabpanel [ref=e640]:
            - group [ref=e641]:
              - generic "All properties" [ref=e642] [cursor=pointer]
          - generic [ref=e643]:
            - group [ref=e644]:
              - generic "Sources and units" [ref=e645] [cursor=pointer]
            - group [ref=e646]:
              - generic "New support configuration" [ref=e647] [cursor=pointer]
              - text: ▾
            - group [ref=e648]:
              - generic "New section" [ref=e649] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e650]:
              - generic "New material" [ref=e651] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e652]:
              - generic "New support" [ref=e653] [cursor=pointer]
              - text: ▾
            - group [ref=e654]:
              - generic "New component" [ref=e655] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e656]:
      - button "Agent" [disabled] [ref=e658]:
        - img [ref=e659]
        - generic [ref=e662]: Agent
  - generic "Workspace status" [ref=e663]:
    - generic "Analysis statuses" [ref=e664]:
      - button "Solver · Not solved" [ref=e666] [cursor=pointer]
    - button "3 Issues" [ref=e667] [cursor=pointer]:
      - img [ref=e668]
      - text: 3 Issues
    - generic "Selection" [ref=e670]: "project: project:UIF-1000"
    - generic "Display units" [ref=e671]: Entered
    - button "About SWBPIPE…" [ref=e672] [cursor=pointer]:
      - img [ref=e673]
```

# Test source

```ts
  114 |   expect(bodyZ!.x).toBeCloseTo(headerZ!.x, 1); expect(bodyZ!.width).toBeCloseTo(headerZ!.width, 1);
  115 |   await z.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Z [m]" }); await editor.fill("4.6");
  116 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(z).toHaveText("0");
  117 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  118 |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  119 |   await expect(grid).toBeVisible();
  120 |   await page.screenshot({ path: info.outputPath("b4-inspector-pointer-fit.png") });
  121 | });
  122 | 
  123 | async function gridChromeBounds(page: import("@playwright/test").Page, review = false) {
  124 |   return page.evaluate((review) => {
  125 |     const bounds: Record<string, { x: number; y: number; width: number; height: number }> = {};
  126 |     const selectors = { pane: ".shell-table-pane", host: ".shell-tree-host", title: ".model-tree > .panel-title", mode: ".layout-mode-toggle", filter: ".model-tree-controls", families: ".entity-grid-tabs", header: review ? ".entity-grid-summary" : ".engineering-table-header", footer: review ? ".entity-grid-actions" : ".engineering-table-footer", viewportHost: '[data-testid="viewport-canvas"]', drawnCanvas: '[data-testid="viewport-canvas"] canvas' };
  127 |     for (const [key, selector] of Object.entries(selectors)) {
  128 |       const rect = document.querySelector(selector)!.getBoundingClientRect(); bounds[key] = { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  129 |     }
  130 |     return bounds;
  131 |   }, review);
  132 | }
  133 | 
  134 | async function openBoundedGrid(page: import("@playwright/test").Page) {
  135 |   await page.getByTestId("view-switch-both").click();
  136 |   if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  137 |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  138 | }
  139 | 
  140 | // Wheel input returns before browser delivery. Observe that local event and two
  141 | // paints so an unchanged-boundary assertion cannot pass before the input occurs.
  142 | async function tableWheel(page: import("@playwright/test").Page, deltaY: number, info: import("@playwright/test").TestInfo, moves = false, scroller = "engineering-table-rows") {
  143 |   const observed = await page.locator(".model-tree").evaluateHandle((root, { moves, scroller }) => {
  144 |     const rows = root.querySelector<HTMLElement>(`[data-testid="${scroller}"]`)!;
  145 |     const state: { complete: boolean; ended: boolean; supported: boolean; result?: unknown; cleanup?: () => void } = { complete: false, ended: false, supported: "onscrollend" in rows };
  146 |     const sample = () => {
  147 |       const slot = rows.parentElement!;
  148 |       return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
  149 |         filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
  150 |         familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
  151 |         footerY: root.querySelector(scroller === "engineering-table-rows" ? ".direct-coordinate-workarea .engineering-table-footer" : ".entity-grid-actions")!.getBoundingClientRect().y };
  152 |     };
  153 |     const ended = () => { state.ended = true; };
  154 |     if (moves) rows.addEventListener("scrollend", ended, { once: true });
  155 |     state.cleanup = () => rows.removeEventListener("scrollend", ended);
  156 |     root.addEventListener("wheel", (event) => {
  157 |       const target = (event.target as Element).outerHTML.slice(0, 400);
  158 |       requestAnimationFrame(() => { const first = sample(); requestAnimationFrame(() => {
  159 |         state.result = { deltaX: (event as WheelEvent).deltaX, deltaY: (event as WheelEvent).deltaY, target, frames: [first, sample()] }; state.complete = true;
  160 |       }); });
  161 |     }, { once: true, passive: true, capture: true });
  162 |     return state;
  163 |   }, { moves, scroller });
  164 |   try {
  165 |     if (moves) expect(await observed.evaluate((state) => state.supported)).toBe(true);
  166 |     await page.mouse.wheel(0, deltaY);
  167 |     await expect.poll(() => observed.evaluate((state) => state.complete)).toBe(true);
  168 |     if (moves) await expect.poll(() => observed.evaluate((state) => state.ended), { message: "scrolling gesture completed" }).toBe(true);
  169 |     const result = await observed.evaluate((state) => ({ receipt: state.result, scrollEnded: state.ended }));
  170 |     await info.attach(`wheel-${deltaY}-${Date.now()}`, { body: JSON.stringify(result), contentType: "application/json" });
  171 |     return result;
  172 |   } finally { await observed.evaluate((state) => state.cleanup?.()); await observed.dispose(); }
  173 | }
  174 | 
  175 | test("B4 short Grid keeps vertical chrome fixed and retains alternate review drafts", async ({ page, browser }, info) => {
  176 |   await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible(); await openBoundedGrid(page);
  177 |   const table = page.getByTestId("engineering-table"); const rows = page.getByTestId("engineering-table-rows");
  178 |   const wheelEvidence: unknown[] = [];
  179 |   const before = await gridChromeBounds(page);
  180 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  181 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  182 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  183 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  184 |   expect(await rows.evaluate((node) => ({ client: node.clientHeight, scroll: node.scrollHeight, top: node.scrollTop }))).toEqual({ client: 180, scroll: 180, top: 0 });
  185 |   const cell = page.getByTestId("table-cell-node:N-100-x"); await cell.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 X [m]" }); await editor.fill("invalid retained"); await editor.press("Enter");
  186 |   await expect(editor).toHaveAttribute("aria-invalid", "true");
  187 |   expect(await page.getByTestId("engineering-table").locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  188 |   const errorState = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  189 |   const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  190 |   await expect(toggle).toContainText("Return to node fields"); const bulk = page.getByTestId("review-cell-node:N-100-y"); await bulk.dblclick(); await page.getByTestId("engineering-table-review").getByRole("textbox").fill("0.5"); await page.getByRole("button", { name: "Keep draft", exact: true }).click();
  191 |   const reviewState = await gridChromeBounds(page, true); await page.getByTestId("engineering-table-review-rows").hover(); wheelEvidence.push(await tableWheel(page, 600, info, false, "engineering-table-review-rows")); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  192 |   await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  193 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveText("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  194 |   await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  195 |   await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  196 |   const controlledId = await toggle.getAttribute("aria-controls"); expect(controlledId).toBeTruthy();
  197 |   const controlled = page.locator(`[id="${controlledId}"]`); await expect(controlled).toBeHidden();
  198 |   await page.getByTestId("table-cell-node:N-140-provenance").dblclick(); await page.keyboard.press("Tab");
  199 |   await expect(table.getByRole("group", { name: "Node fields footer" })).toBeFocused();
  200 |   await page.keyboard.press("Tab"); await expect(toggle).toBeFocused();
  201 |   await page.keyboard.press("Enter"); await expect(toggle).toHaveAttribute("aria-expanded", "true"); await expect(controlled).toBeVisible(); await expect(table).toBeHidden();
  202 |   await page.keyboard.press("Space"); await expect(toggle).toHaveAttribute("aria-expanded", "false"); await expect(controlled).toBeHidden(); await expect(table).toBeVisible();
  203 |   await expect(toggle).toHaveAttribute("aria-controls", controlledId!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  204 |   await info.attach("vertical-fixed-rectangles", { body: JSON.stringify({ before, errorState, reviewState, wheelEvidence }, null, 2), contentType: "application/json" });
  205 |   await page.screenshot({ path: info.outputPath("b4-short-fixed-chrome.png") });
  206 | });
  207 | 
  208 | test("B4 virtual Grid confines body scrolling and boundary wheel without moving chrome", async ({ page, browser }, info) => {
  209 |   await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json"); await openBoundedGrid(page);
  210 |   const wheelEvidence: unknown[] = [];
  211 |   const before = await gridChromeBounds(page); const rows = page.getByTestId("engineering-table-rows");
  212 |   const scrollState = () => rows.evaluate((node) => ({ top: node.scrollTop, maximum: node.scrollHeight - node.clientHeight, height: node.clientHeight }));
  213 |   expect((await scrollState()).height).toBeGreaterThan(0);
> 214 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
      |                                                                                                                                                                                                                 ^ Error: expect(received).toEqual(expected) // deep equality
  215 |   wheelEvidence.push(await tableWheel(page, 1000000, info, true)); await expect.poll(async () => { const state = await scrollState(); return state.maximum - state.top; }).toBe(0);
  216 |   wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  217 |   wheelEvidence.push(await tableWheel(page, -1000000, info, true));
  218 |   // CDP's large reversal can end a few pixels above the boundary. One further
  219 |   // real wheel input establishes top; the following separate input tests chaining.
  220 |   if ((await scrollState()).top > 0) wheelEvidence.push(await tableWheel(page, -600, info, true));
  221 |   await expect.poll(async () => (await scrollState()).top).toBe(0);
  222 |   wheelEvidence.push(await tableWheel(page, -600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  223 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  224 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id);
  225 |   await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  226 |   await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  227 |   // Swap the mounted virtual body through a small family, changing available
  228 |   // width while it is absent, then verify the new element owns its observation.
  229 |   const review = page.getByTestId("node-grid-review-disclosure"); await review.click();
  230 |   const bulkRows = page.getByTestId("engineering-table-review-rows"); await expect(bulkRows).toBeVisible();
  231 |   await page.getByTestId("entity-grid-type-sections").click(); await expect(bulkRows).toBeHidden();
  232 |   await page.getByTestId("toggle-inspector").click(); await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulkRows).toBeVisible();
  233 |   await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  234 |   const bulkChrome = await gridChromeBounds(page, true); await bulkRows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true, "engineering-table-review-rows"));
  235 |   await expect.poll(() => bulkRows.evaluate((node) => node.scrollTop)).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page, true)).toEqual(bulkChrome);
  236 |   await review.click(); await expect(rows).toBeVisible(); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  237 |   // A page makes the still-laid-out stage inert. Positive resize observations
  238 |   // must remain current even before interaction is restored.
  239 |   const retainedCell = page.getByTestId(`table-cell-${model.nodes[0].id}-x`); await retainedCell.dblclick();
  240 |   const retainedEditor = page.getByTestId("engineering-table").getByRole("textbox", { name: `${model.nodes[0].id} X [${model.project.units.length}]` });
  241 |   await retainedEditor.fill("retained page draft"); await retainedEditor.press("Enter"); await expect(retainedEditor).toHaveAttribute("aria-invalid", "true");
  242 |   const originalViewport = page.viewportSize()!;
  243 |   const selectionBeforePage = await page.getByTestId("command-selection-readout").textContent();
  244 |   await openWorkspaceSection(page, "libraries");
  245 |   await page.setViewportSize({ width: originalViewport.width, height: originalViewport.height + 120 });
  246 |   await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  247 |   await page.getByTestId("workspace-dock-close").click(); await expect(rows).toBeVisible();
  248 |   await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  249 |   expect(await page.evaluate(() => document.activeElement !== document.body)).toBe(true);
  250 |   expect(await page.getByTestId("command-selection-readout").textContent()).toBe(selectionBeforePage);
  251 |   await expect(retainedEditor).toHaveValue("retained page draft"); await expect(retainedEditor).toHaveAttribute("aria-invalid", "true");
  252 |   await page.setViewportSize(originalViewport); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  253 |   await expect(retainedEditor).toHaveValue("retained page draft");
  254 |   await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click(); await expect(retainedCell).toHaveText(String(model.nodes[0].position.x));
  255 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  256 |   await info.attach("vertical-wheel-steps", { body: JSON.stringify({ before, filtered, restored, bulkChrome, wheelEvidence }, null, 2), contentType: "application/json" });
  257 |   await page.screenshot({ path: info.outputPath("b4-virtual-fixed-chrome.png") });
  258 | });
  259 | 
  260 | test("B4 text fields apply, recover filtered focus, preserve text Undo and retain review drafts through pointer Queue and Clear", async ({ page, browser }, info) => {
  261 |   await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  262 |   await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  263 |   const table = page.getByTestId("engineering-table"); const label = table.getByTestId("table-cell-node:N-100-label");
  264 |   const original = await label.textContent(); expect(original).toBeTruthy();
  265 |   await label.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Label", exact: true });
  266 |   await editor.fill(` ${original} `); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  267 |   await label.dblclick(); await editor.fill("   "); await table.getByRole("button", { name: "Apply", exact: true }).click();
  268 |   await expect(editor).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  269 |   await editor.fill("Cancel me"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(label).toHaveText(original!);
  270 |   await label.dblclick(); await editor.fill("Unique filter label"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(label).toHaveText("Unique filter label");
  271 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("Unique filter label");
  272 |   const selection = await page.getByTestId("command-selection-readout").textContent();
  273 |   await label.dblclick(); await editor.fill("Renamed outside filter"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  274 |   await expect(label).toHaveCount(0); await expect(table.getByRole("group", { name: "Node fields footer" })).toBeFocused();
  275 |   expect(await page.getByTestId("command-selection-readout").textContent()).toBe(selection);
  276 |   await filter.fill(""); await expect(label).toHaveText("Renamed outside filter");
  277 |   const provenance = table.getByTestId("table-cell-node:N-100-provenance"); const beforeProvenance = await provenance.textContent();
  278 |   await provenance.dblclick(); const text = table.getByRole("textbox", { name: "node:N-100 Provenance", exact: true });
  279 |   await text.fill("TBD"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(provenance).toHaveText("TBD");
  280 |   await provenance.dblclick(); await text.press("End"); await text.pressSequentially(" transient"); await text.press("Meta+z");
  281 |   await expect(provenance).toHaveCount(0); await expect(text).toHaveValue("TBD");
  282 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(provenance).toHaveText("TBD");
  283 |   await page.getByTestId("workspace-undo").click(); await expect(provenance).toHaveText(beforeProvenance!); await page.getByTestId("workspace-redo").click(); await expect(provenance).toHaveText("TBD");
  284 |   await page.getByTestId("node-grid-review-disclosure").click(); const review = page.getByTestId("engineering-table-review");
  285 |   await review.getByTestId("review-cell-node:N-100-label").focus(); await page.keyboard.press("Q");
  286 |   await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await expect(review.getByRole("status")).toHaveText("Draft retained; model unchanged.");
  287 |   await review.getByTestId("review-cell-node:N-110-label").dblclick(); await review.getByRole("textbox").fill("hidden retained");
  288 |   await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("Renamed outside filter");
  289 |   await page.getByTestId("layout-mode-tree").click(); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("node-grid-review-disclosure").click();
  290 |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  291 |   await expect(review.getByTestId("review-cell-node:N-100-label")).toHaveText("Q"); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  292 |   await filter.fill("node:N-100"); await review.getByTestId("review-cell-node:N-100-label").dblclick(); await review.getByRole("textbox").fill("   ");
  293 |   // Pointer Queue from the active editor must consume the current raw blank once.
  294 |   await page.getByTestId("queue-entity-grid-intents").click(); await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText("TBD");
  295 |   await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText("4 applied");
  296 |   await showModelTree(page); await filter.fill(""); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  297 |   await review.getByTestId("review-cell-node:N-100-provenance").dblclick(); await review.getByRole("textbox").fill("clear active");
  298 |   await page.getByTestId("clear-entity-grid-drafts").click(); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  299 |   await expect(review.getByRole("textbox")).toHaveCount(0); await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("TBD");
  300 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  301 |   await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  302 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(label).toHaveText("TBD"); await expect(provenance).toHaveText("TBD"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  303 | });
  304 | 
```