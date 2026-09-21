# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Materials preserve mixed-unit editing, delayed review sort, history and saved ownership
- Location: e2e/b4-table-editing.spec.ts:362:1

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.dblclick: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByTestId('table-cell-node:N-100-label')

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
      - button "Undo model edit" [ref=e20] [cursor=pointer]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select (⎋)" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
    - group "View" [ref=e31]:
      - button "Table" [pressed] [ref=e33] [cursor=pointer]:
        - img [ref=e34]
        - generic [ref=e36]: Table
      - button "Model" [ref=e38] [cursor=pointer]:
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
      - button "Inspector" [disabled] [ref=e60]:
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
          - button "Collapse table drawer" [disabled] [expanded] [ref=e138]:
            - img [ref=e139]
        - generic "Model tree" [ref=e143]:
          - generic [ref=e144]: Model
          - region "Layout grid mode" [ref=e145]:
            - button "Tree" [ref=e146]:
              - img [ref=e147]
              - text: Tree
            - button "Grid" [pressed] [ref=e150]:
              - img [ref=e151]
              - text: Grid
          - region "Model tree filtering" [ref=e153]:
            - generic [ref=e154]:
              - img [ref=e155]
              - generic [ref=e158]: Filter model
              - searchbox "Filter model tree" [ref=e159]
            - generic [ref=e160]: 49 of 49 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e161]:
              - img [ref=e162]
          - region "Bulk entity grid" [ref=e166]:
            - generic "Grid entity type" [ref=e167]:
              - button "Nodes" [active] [pressed] [ref=e168]
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
                    - 'row "node:UIF-PRECISION-00000 node:UIF-PRECISION-00000 Label: UI benchmark node 00000 node:UIF-PRECISION-00000 X: 0 m node:UIF-PRECISION-00000 Y: 0 m node:UIF-PRECISION-00000 Z: 0 m node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e195]':
                      - rowheader "node:UIF-PRECISION-00000" [ref=e196]:
                        - button "node:UIF-PRECISION-00000" [ref=e197]
                      - 'gridcell "node:UIF-PRECISION-00000 Label: UI benchmark node 00000" [ref=e198]':
                        - 'button "node:UIF-PRECISION-00000 Label: UI benchmark node 00000" [ref=e199]': UI benchmark node 00000
                      - 'gridcell "node:UIF-PRECISION-00000 X: 0 m" [ref=e200]':
                        - 'button "node:UIF-PRECISION-00000 X: 0 m" [ref=e201]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Y: 0 m" [ref=e202]':
                        - 'button "node:UIF-PRECISION-00000 Y: 0 m" [ref=e203]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Z: 0 m" [ref=e204]':
                        - 'button "node:UIF-PRECISION-00000 Z: 0 m" [ref=e205]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e206]':
                        - 'button "node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e207]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00001 node:UIF-PRECISION-00001 Label: UI benchmark node 00001 node:UIF-PRECISION-00001 X: 0.1 m node:UIF-PRECISION-00001 Y: -0.00256 m node:UIF-PRECISION-00001 Z: 0.00648 m node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e209]':
                      - rowheader "node:UIF-PRECISION-00001" [ref=e210]:
                        - button "node:UIF-PRECISION-00001" [ref=e211]
                      - 'gridcell "node:UIF-PRECISION-00001 Label: UI benchmark node 00001" [ref=e212]':
                        - 'button "node:UIF-PRECISION-00001 Label: UI benchmark node 00001" [ref=e213]': UI benchmark node 00001
                      - 'gridcell "node:UIF-PRECISION-00001 X: 0.1 m" [ref=e214]':
                        - 'button "node:UIF-PRECISION-00001 X: 0.1 m" [ref=e215]': "0.1"
                      - 'gridcell "node:UIF-PRECISION-00001 Y: -0.00256 m" [ref=e216]':
                        - 'button "node:UIF-PRECISION-00001 Y: -0.00256 m" [ref=e217]': "-0.00256"
                      - 'gridcell "node:UIF-PRECISION-00001 Z: 0.00648 m" [ref=e218]':
                        - 'button "node:UIF-PRECISION-00001 Z: 0.00648 m" [ref=e219]': "0.00648"
                      - 'gridcell "node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e220]':
                        - 'button "node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e221]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00002 node:UIF-PRECISION-00002 Label: UI benchmark node 00002 node:UIF-PRECISION-00002 X: 0.2 m node:UIF-PRECISION-00002 Y: -0.007015 m node:UIF-PRECISION-00002 Z: 0.006753 m node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e223]':
                      - rowheader "node:UIF-PRECISION-00002" [ref=e224]:
                        - button "node:UIF-PRECISION-00002" [ref=e225]
                      - 'gridcell "node:UIF-PRECISION-00002 Label: UI benchmark node 00002" [ref=e226]':
                        - 'button "node:UIF-PRECISION-00002 Label: UI benchmark node 00002" [ref=e227]': UI benchmark node 00002
                      - 'gridcell "node:UIF-PRECISION-00002 X: 0.2 m" [ref=e228]':
                        - 'button "node:UIF-PRECISION-00002 X: 0.2 m" [ref=e229]': "0.2"
                      - 'gridcell "node:UIF-PRECISION-00002 Y: -0.007015 m" [ref=e230]':
                        - 'button "node:UIF-PRECISION-00002 Y: -0.007015 m" [ref=e231]': "-0.007015"
                      - 'gridcell "node:UIF-PRECISION-00002 Z: 0.006753 m" [ref=e232]':
                        - 'button "node:UIF-PRECISION-00002 Z: 0.006753 m" [ref=e233]': "0.006753"
                      - 'gridcell "node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e234]':
                        - 'button "node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e235]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00003 node:UIF-PRECISION-00003 Label: UI benchmark node 00003 node:UIF-PRECISION-00003 X: 0.3 m node:UIF-PRECISION-00003 Y: -0.007731 m node:UIF-PRECISION-00003 Z: 0.007658 m node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e237]':
                      - rowheader "node:UIF-PRECISION-00003" [ref=e238]:
                        - button "node:UIF-PRECISION-00003" [ref=e239]
                      - 'gridcell "node:UIF-PRECISION-00003 Label: UI benchmark node 00003" [ref=e240]':
                        - 'button "node:UIF-PRECISION-00003 Label: UI benchmark node 00003" [ref=e241]': UI benchmark node 00003
                      - 'gridcell "node:UIF-PRECISION-00003 X: 0.3 m" [ref=e242]':
                        - 'button "node:UIF-PRECISION-00003 X: 0.3 m" [ref=e243]': "0.3"
                      - 'gridcell "node:UIF-PRECISION-00003 Y: -0.007731 m" [ref=e244]':
                        - 'button "node:UIF-PRECISION-00003 Y: -0.007731 m" [ref=e245]': "-0.007731"
                      - 'gridcell "node:UIF-PRECISION-00003 Z: 0.007658 m" [ref=e246]':
                        - 'button "node:UIF-PRECISION-00003 Z: 0.007658 m" [ref=e247]': "0.007658"
                      - 'gridcell "node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e248]':
                        - 'button "node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e249]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00004 node:UIF-PRECISION-00004 Label: UI benchmark node 00004 node:UIF-PRECISION-00004 X: 0.4 m node:UIF-PRECISION-00004 Y: 0.001301 m node:UIF-PRECISION-00004 Z: -0.004954 m node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e251]':
                      - rowheader "node:UIF-PRECISION-00004" [ref=e252]:
                        - button "node:UIF-PRECISION-00004" [ref=e253]
                      - 'gridcell "node:UIF-PRECISION-00004 Label: UI benchmark node 00004" [ref=e254]':
                        - 'button "node:UIF-PRECISION-00004 Label: UI benchmark node 00004" [ref=e255]': UI benchmark node 00004
                      - 'gridcell "node:UIF-PRECISION-00004 X: 0.4 m" [ref=e256]':
                        - 'button "node:UIF-PRECISION-00004 X: 0.4 m" [ref=e257]': "0.4"
                      - 'gridcell "node:UIF-PRECISION-00004 Y: 0.001301 m" [ref=e258]':
                        - 'button "node:UIF-PRECISION-00004 Y: 0.001301 m" [ref=e259]': "0.001301"
                      - 'gridcell "node:UIF-PRECISION-00004 Z: -0.004954 m" [ref=e260]':
                        - 'button "node:UIF-PRECISION-00004 Z: -0.004954 m" [ref=e261]': "-0.004954"
                      - 'gridcell "node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e262]':
                        - 'button "node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e263]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00005 node:UIF-PRECISION-00005 Label: UI benchmark node 00005 node:UIF-PRECISION-00005 X: 0.5 m node:UIF-PRECISION-00005 Y: 0.000066 m node:UIF-PRECISION-00005 Z: -0.006799 m node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e265]':
                      - rowheader "node:UIF-PRECISION-00005" [ref=e266]:
                        - button "node:UIF-PRECISION-00005" [ref=e267]
                      - 'gridcell "node:UIF-PRECISION-00005 Label: UI benchmark node 00005" [ref=e268]':
                        - 'button "node:UIF-PRECISION-00005 Label: UI benchmark node 00005" [ref=e269]': UI benchmark node 00005
                      - 'gridcell "node:UIF-PRECISION-00005 X: 0.5 m" [ref=e270]':
                        - 'button "node:UIF-PRECISION-00005 X: 0.5 m" [ref=e271]': "0.5"
                      - 'gridcell "node:UIF-PRECISION-00005 Y: 0.000066 m" [ref=e272]':
                        - 'button "node:UIF-PRECISION-00005 Y: 0.000066 m" [ref=e273]': "0.000066"
                      - 'gridcell "node:UIF-PRECISION-00005 Z: -0.006799 m" [ref=e274]':
                        - 'button "node:UIF-PRECISION-00005 Z: -0.006799 m" [ref=e275]': "-0.006799"
                      - 'gridcell "node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e276]':
                        - 'button "node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e277]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00006 node:UIF-PRECISION-00006 Label: UI benchmark node 00006 node:UIF-PRECISION-00006 X: 0.6 m node:UIF-PRECISION-00006 Y: 0.007154 m node:UIF-PRECISION-00006 Z: -0.004567 m node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e279]':
                      - rowheader "node:UIF-PRECISION-00006" [ref=e280]:
                        - button "node:UIF-PRECISION-00006" [ref=e281]
                      - 'gridcell "node:UIF-PRECISION-00006 Label: UI benchmark node 00006" [ref=e282]':
                        - 'button "node:UIF-PRECISION-00006 Label: UI benchmark node 00006" [ref=e283]': UI benchmark node 00006
                      - 'gridcell "node:UIF-PRECISION-00006 X: 0.6 m" [ref=e284]':
                        - 'button "node:UIF-PRECISION-00006 X: 0.6 m" [ref=e285]': "0.6"
                      - 'gridcell "node:UIF-PRECISION-00006 Y: 0.007154 m" [ref=e286]':
                        - 'button "node:UIF-PRECISION-00006 Y: 0.007154 m" [ref=e287]': "0.007154"
                      - 'gridcell "node:UIF-PRECISION-00006 Z: -0.004567 m" [ref=e288]':
                        - 'button "node:UIF-PRECISION-00006 Z: -0.004567 m" [ref=e289]': "-0.004567"
                      - 'gridcell "node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e290]':
                        - 'button "node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e291]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00007 node:UIF-PRECISION-00007 Label: UI benchmark node 00007 node:UIF-PRECISION-00007 X: 0.7 m node:UIF-PRECISION-00007 Y: -0.000754 m node:UIF-PRECISION-00007 Z: -0.001598 m node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e293]':
                      - rowheader "node:UIF-PRECISION-00007" [ref=e294]:
                        - button "node:UIF-PRECISION-00007" [ref=e295]
                      - 'gridcell "node:UIF-PRECISION-00007 Label: UI benchmark node 00007" [ref=e296]':
                        - 'button "node:UIF-PRECISION-00007 Label: UI benchmark node 00007" [ref=e297]': UI benchmark node 00007
                      - 'gridcell "node:UIF-PRECISION-00007 X: 0.7 m" [ref=e298]':
                        - 'button "node:UIF-PRECISION-00007 X: 0.7 m" [ref=e299]': "0.7"
                      - 'gridcell "node:UIF-PRECISION-00007 Y: -0.000754 m" [ref=e300]':
                        - 'button "node:UIF-PRECISION-00007 Y: -0.000754 m" [ref=e301]': "-0.000754"
                      - 'gridcell "node:UIF-PRECISION-00007 Z: -0.001598 m" [ref=e302]':
                        - 'button "node:UIF-PRECISION-00007 Z: -0.001598 m" [ref=e303]': "-0.001598"
                      - 'gridcell "node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e304]':
                        - 'button "node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e305]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00008 node:UIF-PRECISION-00008 Label: UI benchmark node 00008 node:UIF-PRECISION-00008 X: 0.8 m node:UIF-PRECISION-00008 Y: 0.001591 m node:UIF-PRECISION-00008 Z: -0.004104 m node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e307]':
                      - rowheader "node:UIF-PRECISION-00008" [ref=e308]:
                        - button "node:UIF-PRECISION-00008" [ref=e309]
                      - 'gridcell "node:UIF-PRECISION-00008 Label: UI benchmark node 00008" [ref=e310]':
                        - 'button "node:UIF-PRECISION-00008 Label: UI benchmark node 00008" [ref=e311]': UI benchmark node 00008
                      - 'gridcell "node:UIF-PRECISION-00008 X: 0.8 m" [ref=e312]':
                        - 'button "node:UIF-PRECISION-00008 X: 0.8 m" [ref=e313]': "0.8"
                      - 'gridcell "node:UIF-PRECISION-00008 Y: 0.001591 m" [ref=e314]':
                        - 'button "node:UIF-PRECISION-00008 Y: 0.001591 m" [ref=e315]': "0.001591"
                      - 'gridcell "node:UIF-PRECISION-00008 Z: -0.004104 m" [ref=e316]':
                        - 'button "node:UIF-PRECISION-00008 Z: -0.004104 m" [ref=e317]': "-0.004104"
                      - 'gridcell "node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e318]':
                        - 'button "node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e319]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00009 node:UIF-PRECISION-00009 Label: UI benchmark node 00009 node:UIF-PRECISION-00009 X: 0.9 m node:UIF-PRECISION-00009 Y: -0.001553 m node:UIF-PRECISION-00009 Z: -0.000201 m node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e321]':
                      - rowheader "node:UIF-PRECISION-00009" [ref=e322]:
                        - button "node:UIF-PRECISION-00009" [ref=e323]
                      - 'gridcell "node:UIF-PRECISION-00009 Label: UI benchmark node 00009" [ref=e324]':
                        - 'button "node:UIF-PRECISION-00009 Label: UI benchmark node 00009" [ref=e325]': UI benchmark node 00009
                      - 'gridcell "node:UIF-PRECISION-00009 X: 0.9 m" [ref=e326]':
                        - 'button "node:UIF-PRECISION-00009 X: 0.9 m" [ref=e327]': "0.9"
                      - 'gridcell "node:UIF-PRECISION-00009 Y: -0.001553 m" [ref=e328]':
                        - 'button "node:UIF-PRECISION-00009 Y: -0.001553 m" [ref=e329]': "-0.001553"
                      - 'gridcell "node:UIF-PRECISION-00009 Z: -0.000201 m" [ref=e330]':
                        - 'button "node:UIF-PRECISION-00009 Z: -0.000201 m" [ref=e331]': "-0.000201"
                      - 'gridcell "node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e332]':
                        - 'button "node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e333]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00010 node:UIF-PRECISION-00010 Label: UI benchmark node 00010 node:UIF-PRECISION-00010 X: 1 m node:UIF-PRECISION-00010 Y: -0.002755 m node:UIF-PRECISION-00010 Z: -0.004038 m node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e335]':
                      - rowheader "node:UIF-PRECISION-00010" [ref=e336]:
                        - button "node:UIF-PRECISION-00010" [ref=e337]
                      - 'gridcell "node:UIF-PRECISION-00010 Label: UI benchmark node 00010" [ref=e338]':
                        - 'button "node:UIF-PRECISION-00010 Label: UI benchmark node 00010" [ref=e339]': UI benchmark node 00010
                      - 'gridcell "node:UIF-PRECISION-00010 X: 1 m" [ref=e340]':
                        - 'button "node:UIF-PRECISION-00010 X: 1 m" [ref=e341]': "1"
                      - 'gridcell "node:UIF-PRECISION-00010 Y: -0.002755 m" [ref=e342]':
                        - 'button "node:UIF-PRECISION-00010 Y: -0.002755 m" [ref=e343]': "-0.002755"
                      - 'gridcell "node:UIF-PRECISION-00010 Z: -0.004038 m" [ref=e344]':
                        - 'button "node:UIF-PRECISION-00010 Z: -0.004038 m" [ref=e345]': "-0.004038"
                      - 'gridcell "node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e346]':
                        - 'button "node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e347]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00011 node:UIF-PRECISION-00011 Label: UI benchmark node 00011 node:UIF-PRECISION-00011 X: 1.1 m node:UIF-PRECISION-00011 Y: -0.002077 m node:UIF-PRECISION-00011 Z: 0.003109 m node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e349]':
                      - rowheader "node:UIF-PRECISION-00011" [ref=e350]:
                        - button "node:UIF-PRECISION-00011" [ref=e351]
                      - 'gridcell "node:UIF-PRECISION-00011 Label: UI benchmark node 00011" [ref=e352]':
                        - 'button "node:UIF-PRECISION-00011 Label: UI benchmark node 00011" [ref=e353]': UI benchmark node 00011
                      - 'gridcell "node:UIF-PRECISION-00011 X: 1.1 m" [ref=e354]':
                        - 'button "node:UIF-PRECISION-00011 X: 1.1 m" [ref=e355]': "1.1"
                      - 'gridcell "node:UIF-PRECISION-00011 Y: -0.002077 m" [ref=e356]':
                        - 'button "node:UIF-PRECISION-00011 Y: -0.002077 m" [ref=e357]': "-0.002077"
                      - 'gridcell "node:UIF-PRECISION-00011 Z: 0.003109 m" [ref=e358]':
                        - 'button "node:UIF-PRECISION-00011 Z: 0.003109 m" [ref=e359]': "0.003109"
                      - 'gridcell "node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e360]':
                        - 'button "node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e361]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00012 node:UIF-PRECISION-00012 Label: UI benchmark node 00012 node:UIF-PRECISION-00012 X: 1.2 m node:UIF-PRECISION-00012 Y: 0.006273 m node:UIF-PRECISION-00012 Z: -0.006632 m node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e363]':
                      - rowheader "node:UIF-PRECISION-00012" [ref=e364]:
                        - button "node:UIF-PRECISION-00012" [ref=e365]
                      - 'gridcell "node:UIF-PRECISION-00012 Label: UI benchmark node 00012" [ref=e366]':
                        - 'button "node:UIF-PRECISION-00012 Label: UI benchmark node 00012" [ref=e367]': UI benchmark node 00012
                      - 'gridcell "node:UIF-PRECISION-00012 X: 1.2 m" [ref=e368]':
                        - 'button "node:UIF-PRECISION-00012 X: 1.2 m" [ref=e369]': "1.2"
                      - 'gridcell "node:UIF-PRECISION-00012 Y: 0.006273 m" [ref=e370]':
                        - 'button "node:UIF-PRECISION-00012 Y: 0.006273 m" [ref=e371]': "0.006273"
                      - 'gridcell "node:UIF-PRECISION-00012 Z: -0.006632 m" [ref=e372]':
                        - 'button "node:UIF-PRECISION-00012 Z: -0.006632 m" [ref=e373]': "-0.006632"
                      - 'gridcell "node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e374]':
                        - 'button "node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e375]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00013 node:UIF-PRECISION-00013 Label: UI benchmark node 00013 node:UIF-PRECISION-00013 X: 1.3 m node:UIF-PRECISION-00013 Y: 0.008498 m node:UIF-PRECISION-00013 Z: 0.005846 m node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e377]':
                      - rowheader "node:UIF-PRECISION-00013" [ref=e378]:
                        - button "node:UIF-PRECISION-00013" [ref=e379]
                      - 'gridcell "node:UIF-PRECISION-00013 Label: UI benchmark node 00013" [ref=e380]':
                        - 'button "node:UIF-PRECISION-00013 Label: UI benchmark node 00013" [ref=e381]': UI benchmark node 00013
                      - 'gridcell "node:UIF-PRECISION-00013 X: 1.3 m" [ref=e382]':
                        - 'button "node:UIF-PRECISION-00013 X: 1.3 m" [ref=e383]': "1.3"
                      - 'gridcell "node:UIF-PRECISION-00013 Y: 0.008498 m" [ref=e384]':
                        - 'button "node:UIF-PRECISION-00013 Y: 0.008498 m" [ref=e385]': "0.008498"
                      - 'gridcell "node:UIF-PRECISION-00013 Z: 0.005846 m" [ref=e386]':
                        - 'button "node:UIF-PRECISION-00013 Z: 0.005846 m" [ref=e387]': "0.005846"
                      - 'gridcell "node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e388]':
                        - 'button "node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e389]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00014 node:UIF-PRECISION-00014 Label: UI benchmark node 00014 node:UIF-PRECISION-00014 X: 1.4 m node:UIF-PRECISION-00014 Y: 0.001425 m node:UIF-PRECISION-00014 Z: -0.003282 m node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e391]':
                      - rowheader "node:UIF-PRECISION-00014" [ref=e392]:
                        - button "node:UIF-PRECISION-00014" [ref=e393]
                      - 'gridcell "node:UIF-PRECISION-00014 Label: UI benchmark node 00014" [ref=e394]':
                        - 'button "node:UIF-PRECISION-00014 Label: UI benchmark node 00014" [ref=e395]': UI benchmark node 00014
                      - 'gridcell "node:UIF-PRECISION-00014 X: 1.4 m" [ref=e396]':
                        - 'button "node:UIF-PRECISION-00014 X: 1.4 m" [ref=e397]': "1.4"
                      - 'gridcell "node:UIF-PRECISION-00014 Y: 0.001425 m" [ref=e398]':
                        - 'button "node:UIF-PRECISION-00014 Y: 0.001425 m" [ref=e399]': "0.001425"
                      - 'gridcell "node:UIF-PRECISION-00014 Z: -0.003282 m" [ref=e400]':
                        - 'button "node:UIF-PRECISION-00014 Z: -0.003282 m" [ref=e401]': "-0.003282"
                      - 'gridcell "node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e402]':
                        - 'button "node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e403]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00015 node:UIF-PRECISION-00015 Label: UI benchmark node 00015 node:UIF-PRECISION-00015 X: 1.5 m node:UIF-PRECISION-00015 Y: 0.005946 m node:UIF-PRECISION-00015 Z: -0.000737 m node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e405]':
                      - rowheader "node:UIF-PRECISION-00015" [ref=e406]:
                        - button "node:UIF-PRECISION-00015" [ref=e407]
                      - 'gridcell "node:UIF-PRECISION-00015 Label: UI benchmark node 00015" [ref=e408]':
                        - 'button "node:UIF-PRECISION-00015 Label: UI benchmark node 00015" [ref=e409]': UI benchmark node 00015
                      - 'gridcell "node:UIF-PRECISION-00015 X: 1.5 m" [ref=e410]':
                        - 'button "node:UIF-PRECISION-00015 X: 1.5 m" [ref=e411]': "1.5"
                      - 'gridcell "node:UIF-PRECISION-00015 Y: 0.005946 m" [ref=e412]':
                        - 'button "node:UIF-PRECISION-00015 Y: 0.005946 m" [ref=e413]': "0.005946"
                      - 'gridcell "node:UIF-PRECISION-00015 Z: -0.000737 m" [ref=e414]':
                        - 'button "node:UIF-PRECISION-00015 Z: -0.000737 m" [ref=e415]': "-0.000737"
                      - 'gridcell "node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e416]':
                        - 'button "node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e417]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00016 node:UIF-PRECISION-00016 Label: UI benchmark node 00016 node:UIF-PRECISION-00016 X: 1.6 m node:UIF-PRECISION-00016 Y: 0.005719 m node:UIF-PRECISION-00016 Z: -0.003647 m node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e419]':
                      - rowheader "node:UIF-PRECISION-00016" [ref=e420]:
                        - button "node:UIF-PRECISION-00016" [ref=e421]
                      - 'gridcell "node:UIF-PRECISION-00016 Label: UI benchmark node 00016" [ref=e422]':
                        - 'button "node:UIF-PRECISION-00016 Label: UI benchmark node 00016" [ref=e423]': UI benchmark node 00016
                      - 'gridcell "node:UIF-PRECISION-00016 X: 1.6 m" [ref=e424]':
                        - 'button "node:UIF-PRECISION-00016 X: 1.6 m" [ref=e425]': "1.6"
                      - 'gridcell "node:UIF-PRECISION-00016 Y: 0.005719 m" [ref=e426]':
                        - 'button "node:UIF-PRECISION-00016 Y: 0.005719 m" [ref=e427]': "0.005719"
                      - 'gridcell "node:UIF-PRECISION-00016 Z: -0.003647 m" [ref=e428]':
                        - 'button "node:UIF-PRECISION-00016 Z: -0.003647 m" [ref=e429]': "-0.003647"
                      - 'gridcell "node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e430]':
                        - 'button "node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e431]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00017 node:UIF-PRECISION-00017 Label: UI benchmark node 00017 node:UIF-PRECISION-00017 X: 1.7 m node:UIF-PRECISION-00017 Y: 0.000353 m node:UIF-PRECISION-00017 Z: 0.001092 m node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e433]':
                      - rowheader "node:UIF-PRECISION-00017" [ref=e434]:
                        - button "node:UIF-PRECISION-00017" [ref=e435]
                      - 'gridcell "node:UIF-PRECISION-00017 Label: UI benchmark node 00017" [ref=e436]':
                        - 'button "node:UIF-PRECISION-00017 Label: UI benchmark node 00017" [ref=e437]': UI benchmark node 00017
                      - 'gridcell "node:UIF-PRECISION-00017 X: 1.7 m" [ref=e438]':
                        - 'button "node:UIF-PRECISION-00017 X: 1.7 m" [ref=e439]': "1.7"
                      - 'gridcell "node:UIF-PRECISION-00017 Y: 0.000353 m" [ref=e440]':
                        - 'button "node:UIF-PRECISION-00017 Y: 0.000353 m" [ref=e441]': "0.000353"
                      - 'gridcell "node:UIF-PRECISION-00017 Z: 0.001092 m" [ref=e442]':
                        - 'button "node:UIF-PRECISION-00017 Z: 0.001092 m" [ref=e443]': "0.001092"
                      - 'gridcell "node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e444]':
                        - 'button "node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e445]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00018 node:UIF-PRECISION-00018 Label: UI benchmark node 00018 node:UIF-PRECISION-00018 X: 1.8 m node:UIF-PRECISION-00018 Y: 0.003395 m node:UIF-PRECISION-00018 Z: 0.002323 m node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e447]':
                      - rowheader "node:UIF-PRECISION-00018" [ref=e448]:
                        - button "node:UIF-PRECISION-00018" [ref=e449]
                      - 'gridcell "node:UIF-PRECISION-00018 Label: UI benchmark node 00018" [ref=e450]':
                        - 'button "node:UIF-PRECISION-00018 Label: UI benchmark node 00018" [ref=e451]': UI benchmark node 00018
                      - 'gridcell "node:UIF-PRECISION-00018 X: 1.8 m" [ref=e452]':
                        - 'button "node:UIF-PRECISION-00018 X: 1.8 m" [ref=e453]': "1.8"
                      - 'gridcell "node:UIF-PRECISION-00018 Y: 0.003395 m" [ref=e454]':
                        - 'button "node:UIF-PRECISION-00018 Y: 0.003395 m" [ref=e455]': "0.003395"
                      - 'gridcell "node:UIF-PRECISION-00018 Z: 0.002323 m" [ref=e456]':
                        - 'button "node:UIF-PRECISION-00018 Z: 0.002323 m" [ref=e457]': "0.002323"
                      - 'gridcell "node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e458]':
                        - 'button "node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e459]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00019 node:UIF-PRECISION-00019 Label: UI benchmark node 00019 node:UIF-PRECISION-00019 X: 1.9 m node:UIF-PRECISION-00019 Y: 0.006887 m node:UIF-PRECISION-00019 Z: 0.00658 m node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e461]':
                      - rowheader "node:UIF-PRECISION-00019" [ref=e462]:
                        - button "node:UIF-PRECISION-00019" [ref=e463]
                      - 'gridcell "node:UIF-PRECISION-00019 Label: UI benchmark node 00019" [ref=e464]':
                        - 'button "node:UIF-PRECISION-00019 Label: UI benchmark node 00019" [ref=e465]': UI benchmark node 00019
                      - 'gridcell "node:UIF-PRECISION-00019 X: 1.9 m" [ref=e466]':
                        - 'button "node:UIF-PRECISION-00019 X: 1.9 m" [ref=e467]': "1.9"
                      - 'gridcell "node:UIF-PRECISION-00019 Y: 0.006887 m" [ref=e468]':
                        - 'button "node:UIF-PRECISION-00019 Y: 0.006887 m" [ref=e469]': "0.006887"
                      - 'gridcell "node:UIF-PRECISION-00019 Z: 0.00658 m" [ref=e470]':
                        - 'button "node:UIF-PRECISION-00019 Z: 0.00658 m" [ref=e471]': "0.00658"
                      - 'gridcell "node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e472]':
                        - 'button "node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e473]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00020 node:UIF-PRECISION-00020 Label: UI benchmark node 00020 node:UIF-PRECISION-00020 X: 2 m node:UIF-PRECISION-00020 Y: -0.004467 m node:UIF-PRECISION-00020 Z: -0.007252 m node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e475]':
                      - rowheader "node:UIF-PRECISION-00020" [ref=e476]:
                        - button "node:UIF-PRECISION-00020" [ref=e477]
                      - 'gridcell "node:UIF-PRECISION-00020 Label: UI benchmark node 00020" [ref=e478]':
                        - 'button "node:UIF-PRECISION-00020 Label: UI benchmark node 00020" [ref=e479]': UI benchmark node 00020
                      - 'gridcell "node:UIF-PRECISION-00020 X: 2 m" [ref=e480]':
                        - 'button "node:UIF-PRECISION-00020 X: 2 m" [ref=e481]': "2"
                      - 'gridcell "node:UIF-PRECISION-00020 Y: -0.004467 m" [ref=e482]':
                        - 'button "node:UIF-PRECISION-00020 Y: -0.004467 m" [ref=e483]': "-0.004467"
                      - 'gridcell "node:UIF-PRECISION-00020 Z: -0.007252 m" [ref=e484]':
                        - 'button "node:UIF-PRECISION-00020 Z: -0.007252 m" [ref=e485]': "-0.007252"
                      - 'gridcell "node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e486]':
                        - 'button "node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e487]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - group "Node fields footer" [ref=e488]:
                - generic [ref=e489]: 21 of 21 rows
            - button "Review multiple changes" [ref=e490] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e491]:
      - button "Agent" [disabled] [ref=e493]:
        - img [ref=e494]
        - generic [ref=e497]: Agent
  - generic "Workspace status" [ref=e498]:
    - generic "Analysis statuses" [ref=e499]:
      - button "Solver · Not solved" [ref=e501] [cursor=pointer]
    - button "2 Issues" [ref=e502] [cursor=pointer]:
      - img [ref=e503]
      - text: 2 Issues
    - generic "Selection" [ref=e505]: "material: material:UIF-INVENTED-01"
    - generic "Display units" [ref=e506]: Entered
    - button "About SWBPIPE…" [ref=e507] [cursor=pointer]:
      - img [ref=e508]
```

# Test source

```ts
  339 |       await editor.press(field === "label" ? "s" : "3");
  340 |       await expect(editor).toHaveValue(first + (field === "label" ? "s" : "3") + second);
  341 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(original!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  342 |       await cell.focus(); await page.keyboard.press(first); await page.keyboard.press(second); await expect(editor).toHaveValue(first + second);
  343 |       await table.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply", exact: true }).click(); await expect(cell).toHaveText(first + second);
  344 |       if (policy === "review") {
  345 |         await expect(table.getByRole("status")).toHaveText("Draft retained; model unchanged."); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  346 |       } else {
  347 |         await expect(page.getByTestId("workspace-undo")).toBeEnabled(); await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText(original!);
  348 |         await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText(first + second);
  349 |       }
  350 |       await cell.focus(); await page.keyboard.press("Enter");
  351 |       expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
  352 |       await editor.press(field === "label" ? "t" : "4"); await expect(editor).toHaveValue(field === "label" ? "t" : "4");
  353 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
  354 |       await cell.dblclick(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
  355 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
  356 |     });
  357 |   }
  358 | }
  359 | 
  360 | // Real operation/conversion services; only the public synthetic starting model
  361 | // and delivery timing of conversion results are controlled by this scenario.
  362 | test("B4 Materials preserve mixed-unit editing, delayed review sort, history and saved ownership", async ({ page, browser }, info) => {
  363 |   await attachBrowserIdentity(browser, info);
  364 |   const { model } = await readFixture("precision-origin-base.model.json");
  365 |   const material = model.materials[0]; const firstId = material.id; const secondId = "material:B4-second";
  366 |   model.materials = [{ ...material, elastic_modulus: { value: 200000, unit: "MPa" } },
  367 |     { ...material, id: secondId, label: "Synthetic second material", elastic_modulus: { value: 100000000000, unit: "Pa" } }];
  368 |   // Delay delivery after the existing Rust conversion has finished. No factors,
  369 |   // replacement conversion answers, or model mutation are injected.
  370 |   await page.route("**/src/services/displayQuantityService.ts", async (route) => {
  371 |     const response = await route.fetch(); const source = await response.text();
  372 |     expect(source).toContain("export async function convertDisplayQuantities(");
  373 |     await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
  374 | export async function convertDisplayQuantities(items) {
  375 |   const result = await originalConvertDisplayQuantities(items);
  376 |   const gate = window.__b4MaterialConversionGate;
  377 |   if (gate && gate.hold && items.some(item => item.id.includes('material') && item.id.includes('elastic'))) {
  378 |     await new Promise(resolve => gate.pending.push(resolve));
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
> 439 |   const nodeLabel = page.getByTestId("table-cell-node:N-100-label"); await nodeLabel.dblclick(); const nodeEditor = page.getByTestId("engineering-table").getByRole("textbox"); await nodeEditor.fill(" ");
      |                                                                                      ^ Error: locator.dblclick: Test timeout of 120000ms exceeded.
  440 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  441 |   await expect(nodeEditor).toBeFocused(); await expect(nodeEditor).toHaveValue(" "); await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click();
  442 |   await page.getByTestId("entity-grid-type-materials").click(); await reviewCell.dblclick(); await review.getByRole("textbox").fill("80");
  443 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  444 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  445 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(elastic).toHaveText("50"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  446 |   await direct.getByTestId(`table-cell-${firstId}-label`).dblclick(); const newEditor = direct.getByRole("textbox"); await newEditor.press("R"); await newEditor.press("S");
  447 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  448 |   await expect(newEditor).toHaveValue("RS"); await expect(newEditor).toBeFocused(); await direct.getByRole("button", { name: "Cancel", exact: true }).click();
  449 |   await page.screenshot({ path: info.outputPath("b4-materials-reopened.png") });
  450 | });
  451 | 
```