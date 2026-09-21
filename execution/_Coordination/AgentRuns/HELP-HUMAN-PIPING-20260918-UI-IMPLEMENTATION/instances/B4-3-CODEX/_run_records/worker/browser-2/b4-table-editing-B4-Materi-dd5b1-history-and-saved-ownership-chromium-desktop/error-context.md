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
      - button "Select" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [pressed] [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Both
    - generic [ref=e49]:
      - button "Run" [ref=e50] [cursor=pointer]:
        - img [ref=e51]
        - generic [ref=e53]: Run
      - button "Issues, 2" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "2"
    - group "Panels" [ref=e59]:
      - button "Inspector" [disabled] [ref=e61]:
        - img [ref=e62]
        - generic [ref=e64]: Inspector
      - button "Agent" [disabled] [ref=e66]:
        - img [ref=e67]
        - generic [ref=e70]: Agent
    - generic "Display units" [ref=e71]:
      - combobox "Display units" [ref=e72]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e73]:
      - generic "Appearance" [ref=e74] [cursor=pointer]:
        - img [ref=e75]
      - option "System" [selected]
      - option "Light"
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e77]:
      - button "Find modeling commands" [ref=e78] [cursor=pointer]:
        - img [ref=e79]
        - generic [ref=e82]: Search or command…
        - generic "Command K" [ref=e83]: ⌘K
  - generic [ref=e84]:
    - navigation "Stages" [ref=e85]:
      - list [ref=e86]:
        - listitem [ref=e87]:
          - button "Model" [pressed] [ref=e88] [cursor=pointer]:
            - img [ref=e89]
            - generic [ref=e92]: Model
        - listitem [ref=e93]:
          - button "Loads" [ref=e94] [cursor=pointer]:
            - img [ref=e95]
            - generic [ref=e99]: Loads
        - listitem [ref=e100]:
          - button "Results" [disabled] [ref=e101]:
            - img [ref=e102]
            - generic [ref=e105]: Results
        - listitem [ref=e106]:
          - button "Review" [disabled] [ref=e107]:
            - img [ref=e108]
            - generic [ref=e112]: Review
      - separator [ref=e113]
      - list [ref=e114]:
        - listitem [ref=e115]:
          - button "Libraries" [ref=e116] [cursor=pointer]:
            - img [ref=e117]
            - generic [ref=e119]: Libraries
        - listitem [ref=e120]:
          - button "Rules" [ref=e121] [cursor=pointer]:
            - img [ref=e122]
            - generic [ref=e126]: Rules
        - listitem [ref=e127]:
          - button "Issues, 2" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "2"
    - region "Modeling workspace" [ref=e134]:
      - generic [ref=e135]:
        - group "Tables" [ref=e136]:
          - button "Model" [pressed] [ref=e137] [cursor=pointer]
          - button "Review changes" [ref=e138] [cursor=pointer]
          - button "Collapse table drawer" [disabled] [expanded] [ref=e141]:
            - img [ref=e142]
        - generic "Model tree" [ref=e146]:
          - generic [ref=e147]: Model
          - region "Layout grid mode" [ref=e148]:
            - button "Tree" [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [pressed] [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]
            - generic [ref=e163]: 49 of 49 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - region "Bulk entity grid" [ref=e169]:
            - generic "Grid entity type" [ref=e170]:
              - button "Nodes" [active] [pressed] [ref=e171]
              - button "Pipes" [ref=e172]
              - button "Supports" [ref=e173]
              - button "Materials" [ref=e174]
              - button "Sections" [ref=e175]
              - button "Components" [ref=e176]
              - button "Load Cases" [ref=e177]
              - button "Combinations" [ref=e178]
            - generic [ref=e180]:
              - grid "Node fields" [ref=e181]:
                - row "Node Sort Label Sort X Sort Y Sort Z Sort Provenance" [ref=e182]:
                  - columnheader "Node" [ref=e183]
                  - columnheader "Sort Label" [ref=e184]:
                    - button "Sort Label" [ref=e185]: Label ↕
                  - columnheader "Sort X" [ref=e186]:
                    - button "Sort X" [ref=e187]: X [m] ↕
                  - columnheader "Sort Y" [ref=e188]:
                    - button "Sort Y" [ref=e189]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e190]:
                    - button "Sort Z" [ref=e191]: Z [m] ↕
                  - columnheader "Sort Provenance" [ref=e192]:
                    - button "Sort Provenance" [ref=e193]: Provenance ↕
                - rowgroup [ref=e195]:
                  - generic [ref=e196]:
                    - 'row "node:UIF-PRECISION-00000 node:UIF-PRECISION-00000 Label: UI benchmark node 00000 node:UIF-PRECISION-00000 X: 0 m node:UIF-PRECISION-00000 Y: 0 m node:UIF-PRECISION-00000 Z: 0 m node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e198]':
                      - rowheader "node:UIF-PRECISION-00000" [ref=e199]:
                        - button "node:UIF-PRECISION-00000" [ref=e200]
                      - 'gridcell "node:UIF-PRECISION-00000 Label: UI benchmark node 00000" [ref=e201]':
                        - 'button "node:UIF-PRECISION-00000 Label: UI benchmark node 00000" [ref=e202]': UI benchmark node 00000
                      - 'gridcell "node:UIF-PRECISION-00000 X: 0 m" [ref=e203]':
                        - 'button "node:UIF-PRECISION-00000 X: 0 m" [ref=e204]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Y: 0 m" [ref=e205]':
                        - 'button "node:UIF-PRECISION-00000 Y: 0 m" [ref=e206]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Z: 0 m" [ref=e207]':
                        - 'button "node:UIF-PRECISION-00000 Z: 0 m" [ref=e208]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e209]':
                        - 'button "node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e210]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00001 node:UIF-PRECISION-00001 Label: UI benchmark node 00001 node:UIF-PRECISION-00001 X: 0.1 m node:UIF-PRECISION-00001 Y: -0.00256 m node:UIF-PRECISION-00001 Z: 0.00648 m node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e212]':
                      - rowheader "node:UIF-PRECISION-00001" [ref=e213]:
                        - button "node:UIF-PRECISION-00001" [ref=e214]
                      - 'gridcell "node:UIF-PRECISION-00001 Label: UI benchmark node 00001" [ref=e215]':
                        - 'button "node:UIF-PRECISION-00001 Label: UI benchmark node 00001" [ref=e216]': UI benchmark node 00001
                      - 'gridcell "node:UIF-PRECISION-00001 X: 0.1 m" [ref=e217]':
                        - 'button "node:UIF-PRECISION-00001 X: 0.1 m" [ref=e218]': "0.1"
                      - 'gridcell "node:UIF-PRECISION-00001 Y: -0.00256 m" [ref=e219]':
                        - 'button "node:UIF-PRECISION-00001 Y: -0.00256 m" [ref=e220]': "-0.00256"
                      - 'gridcell "node:UIF-PRECISION-00001 Z: 0.00648 m" [ref=e221]':
                        - 'button "node:UIF-PRECISION-00001 Z: 0.00648 m" [ref=e222]': "0.00648"
                      - 'gridcell "node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e223]':
                        - 'button "node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e224]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00002 node:UIF-PRECISION-00002 Label: UI benchmark node 00002 node:UIF-PRECISION-00002 X: 0.2 m node:UIF-PRECISION-00002 Y: -0.007015 m node:UIF-PRECISION-00002 Z: 0.006753 m node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e226]':
                      - rowheader "node:UIF-PRECISION-00002" [ref=e227]:
                        - button "node:UIF-PRECISION-00002" [ref=e228]
                      - 'gridcell "node:UIF-PRECISION-00002 Label: UI benchmark node 00002" [ref=e229]':
                        - 'button "node:UIF-PRECISION-00002 Label: UI benchmark node 00002" [ref=e230]': UI benchmark node 00002
                      - 'gridcell "node:UIF-PRECISION-00002 X: 0.2 m" [ref=e231]':
                        - 'button "node:UIF-PRECISION-00002 X: 0.2 m" [ref=e232]': "0.2"
                      - 'gridcell "node:UIF-PRECISION-00002 Y: -0.007015 m" [ref=e233]':
                        - 'button "node:UIF-PRECISION-00002 Y: -0.007015 m" [ref=e234]': "-0.007015"
                      - 'gridcell "node:UIF-PRECISION-00002 Z: 0.006753 m" [ref=e235]':
                        - 'button "node:UIF-PRECISION-00002 Z: 0.006753 m" [ref=e236]': "0.006753"
                      - 'gridcell "node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e237]':
                        - 'button "node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e238]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00003 node:UIF-PRECISION-00003 Label: UI benchmark node 00003 node:UIF-PRECISION-00003 X: 0.3 m node:UIF-PRECISION-00003 Y: -0.007731 m node:UIF-PRECISION-00003 Z: 0.007658 m node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e240]':
                      - rowheader "node:UIF-PRECISION-00003" [ref=e241]:
                        - button "node:UIF-PRECISION-00003" [ref=e242]
                      - 'gridcell "node:UIF-PRECISION-00003 Label: UI benchmark node 00003" [ref=e243]':
                        - 'button "node:UIF-PRECISION-00003 Label: UI benchmark node 00003" [ref=e244]': UI benchmark node 00003
                      - 'gridcell "node:UIF-PRECISION-00003 X: 0.3 m" [ref=e245]':
                        - 'button "node:UIF-PRECISION-00003 X: 0.3 m" [ref=e246]': "0.3"
                      - 'gridcell "node:UIF-PRECISION-00003 Y: -0.007731 m" [ref=e247]':
                        - 'button "node:UIF-PRECISION-00003 Y: -0.007731 m" [ref=e248]': "-0.007731"
                      - 'gridcell "node:UIF-PRECISION-00003 Z: 0.007658 m" [ref=e249]':
                        - 'button "node:UIF-PRECISION-00003 Z: 0.007658 m" [ref=e250]': "0.007658"
                      - 'gridcell "node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e251]':
                        - 'button "node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e252]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00004 node:UIF-PRECISION-00004 Label: UI benchmark node 00004 node:UIF-PRECISION-00004 X: 0.4 m node:UIF-PRECISION-00004 Y: 0.001301 m node:UIF-PRECISION-00004 Z: -0.004954 m node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e254]':
                      - rowheader "node:UIF-PRECISION-00004" [ref=e255]:
                        - button "node:UIF-PRECISION-00004" [ref=e256]
                      - 'gridcell "node:UIF-PRECISION-00004 Label: UI benchmark node 00004" [ref=e257]':
                        - 'button "node:UIF-PRECISION-00004 Label: UI benchmark node 00004" [ref=e258]': UI benchmark node 00004
                      - 'gridcell "node:UIF-PRECISION-00004 X: 0.4 m" [ref=e259]':
                        - 'button "node:UIF-PRECISION-00004 X: 0.4 m" [ref=e260]': "0.4"
                      - 'gridcell "node:UIF-PRECISION-00004 Y: 0.001301 m" [ref=e261]':
                        - 'button "node:UIF-PRECISION-00004 Y: 0.001301 m" [ref=e262]': "0.001301"
                      - 'gridcell "node:UIF-PRECISION-00004 Z: -0.004954 m" [ref=e263]':
                        - 'button "node:UIF-PRECISION-00004 Z: -0.004954 m" [ref=e264]': "-0.004954"
                      - 'gridcell "node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e265]':
                        - 'button "node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e266]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00005 node:UIF-PRECISION-00005 Label: UI benchmark node 00005 node:UIF-PRECISION-00005 X: 0.5 m node:UIF-PRECISION-00005 Y: 0.000066 m node:UIF-PRECISION-00005 Z: -0.006799 m node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e268]':
                      - rowheader "node:UIF-PRECISION-00005" [ref=e269]:
                        - button "node:UIF-PRECISION-00005" [ref=e270]
                      - 'gridcell "node:UIF-PRECISION-00005 Label: UI benchmark node 00005" [ref=e271]':
                        - 'button "node:UIF-PRECISION-00005 Label: UI benchmark node 00005" [ref=e272]': UI benchmark node 00005
                      - 'gridcell "node:UIF-PRECISION-00005 X: 0.5 m" [ref=e273]':
                        - 'button "node:UIF-PRECISION-00005 X: 0.5 m" [ref=e274]': "0.5"
                      - 'gridcell "node:UIF-PRECISION-00005 Y: 0.000066 m" [ref=e275]':
                        - 'button "node:UIF-PRECISION-00005 Y: 0.000066 m" [ref=e276]': "0.000066"
                      - 'gridcell "node:UIF-PRECISION-00005 Z: -0.006799 m" [ref=e277]':
                        - 'button "node:UIF-PRECISION-00005 Z: -0.006799 m" [ref=e278]': "-0.006799"
                      - 'gridcell "node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e279]':
                        - 'button "node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e280]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00006 node:UIF-PRECISION-00006 Label: UI benchmark node 00006 node:UIF-PRECISION-00006 X: 0.6 m node:UIF-PRECISION-00006 Y: 0.007154 m node:UIF-PRECISION-00006 Z: -0.004567 m node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e282]':
                      - rowheader "node:UIF-PRECISION-00006" [ref=e283]:
                        - button "node:UIF-PRECISION-00006" [ref=e284]
                      - 'gridcell "node:UIF-PRECISION-00006 Label: UI benchmark node 00006" [ref=e285]':
                        - 'button "node:UIF-PRECISION-00006 Label: UI benchmark node 00006" [ref=e286]': UI benchmark node 00006
                      - 'gridcell "node:UIF-PRECISION-00006 X: 0.6 m" [ref=e287]':
                        - 'button "node:UIF-PRECISION-00006 X: 0.6 m" [ref=e288]': "0.6"
                      - 'gridcell "node:UIF-PRECISION-00006 Y: 0.007154 m" [ref=e289]':
                        - 'button "node:UIF-PRECISION-00006 Y: 0.007154 m" [ref=e290]': "0.007154"
                      - 'gridcell "node:UIF-PRECISION-00006 Z: -0.004567 m" [ref=e291]':
                        - 'button "node:UIF-PRECISION-00006 Z: -0.004567 m" [ref=e292]': "-0.004567"
                      - 'gridcell "node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e293]':
                        - 'button "node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e294]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00007 node:UIF-PRECISION-00007 Label: UI benchmark node 00007 node:UIF-PRECISION-00007 X: 0.7 m node:UIF-PRECISION-00007 Y: -0.000754 m node:UIF-PRECISION-00007 Z: -0.001598 m node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e296]':
                      - rowheader "node:UIF-PRECISION-00007" [ref=e297]:
                        - button "node:UIF-PRECISION-00007" [ref=e298]
                      - 'gridcell "node:UIF-PRECISION-00007 Label: UI benchmark node 00007" [ref=e299]':
                        - 'button "node:UIF-PRECISION-00007 Label: UI benchmark node 00007" [ref=e300]': UI benchmark node 00007
                      - 'gridcell "node:UIF-PRECISION-00007 X: 0.7 m" [ref=e301]':
                        - 'button "node:UIF-PRECISION-00007 X: 0.7 m" [ref=e302]': "0.7"
                      - 'gridcell "node:UIF-PRECISION-00007 Y: -0.000754 m" [ref=e303]':
                        - 'button "node:UIF-PRECISION-00007 Y: -0.000754 m" [ref=e304]': "-0.000754"
                      - 'gridcell "node:UIF-PRECISION-00007 Z: -0.001598 m" [ref=e305]':
                        - 'button "node:UIF-PRECISION-00007 Z: -0.001598 m" [ref=e306]': "-0.001598"
                      - 'gridcell "node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e307]':
                        - 'button "node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e308]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00008 node:UIF-PRECISION-00008 Label: UI benchmark node 00008 node:UIF-PRECISION-00008 X: 0.8 m node:UIF-PRECISION-00008 Y: 0.001591 m node:UIF-PRECISION-00008 Z: -0.004104 m node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e310]':
                      - rowheader "node:UIF-PRECISION-00008" [ref=e311]:
                        - button "node:UIF-PRECISION-00008" [ref=e312]
                      - 'gridcell "node:UIF-PRECISION-00008 Label: UI benchmark node 00008" [ref=e313]':
                        - 'button "node:UIF-PRECISION-00008 Label: UI benchmark node 00008" [ref=e314]': UI benchmark node 00008
                      - 'gridcell "node:UIF-PRECISION-00008 X: 0.8 m" [ref=e315]':
                        - 'button "node:UIF-PRECISION-00008 X: 0.8 m" [ref=e316]': "0.8"
                      - 'gridcell "node:UIF-PRECISION-00008 Y: 0.001591 m" [ref=e317]':
                        - 'button "node:UIF-PRECISION-00008 Y: 0.001591 m" [ref=e318]': "0.001591"
                      - 'gridcell "node:UIF-PRECISION-00008 Z: -0.004104 m" [ref=e319]':
                        - 'button "node:UIF-PRECISION-00008 Z: -0.004104 m" [ref=e320]': "-0.004104"
                      - 'gridcell "node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e321]':
                        - 'button "node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e322]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00009 node:UIF-PRECISION-00009 Label: UI benchmark node 00009 node:UIF-PRECISION-00009 X: 0.9 m node:UIF-PRECISION-00009 Y: -0.001553 m node:UIF-PRECISION-00009 Z: -0.000201 m node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e324]':
                      - rowheader "node:UIF-PRECISION-00009" [ref=e325]:
                        - button "node:UIF-PRECISION-00009" [ref=e326]
                      - 'gridcell "node:UIF-PRECISION-00009 Label: UI benchmark node 00009" [ref=e327]':
                        - 'button "node:UIF-PRECISION-00009 Label: UI benchmark node 00009" [ref=e328]': UI benchmark node 00009
                      - 'gridcell "node:UIF-PRECISION-00009 X: 0.9 m" [ref=e329]':
                        - 'button "node:UIF-PRECISION-00009 X: 0.9 m" [ref=e330]': "0.9"
                      - 'gridcell "node:UIF-PRECISION-00009 Y: -0.001553 m" [ref=e331]':
                        - 'button "node:UIF-PRECISION-00009 Y: -0.001553 m" [ref=e332]': "-0.001553"
                      - 'gridcell "node:UIF-PRECISION-00009 Z: -0.000201 m" [ref=e333]':
                        - 'button "node:UIF-PRECISION-00009 Z: -0.000201 m" [ref=e334]': "-0.000201"
                      - 'gridcell "node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e335]':
                        - 'button "node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e336]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00010 node:UIF-PRECISION-00010 Label: UI benchmark node 00010 node:UIF-PRECISION-00010 X: 1 m node:UIF-PRECISION-00010 Y: -0.002755 m node:UIF-PRECISION-00010 Z: -0.004038 m node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e338]':
                      - rowheader "node:UIF-PRECISION-00010" [ref=e339]:
                        - button "node:UIF-PRECISION-00010" [ref=e340]
                      - 'gridcell "node:UIF-PRECISION-00010 Label: UI benchmark node 00010" [ref=e341]':
                        - 'button "node:UIF-PRECISION-00010 Label: UI benchmark node 00010" [ref=e342]': UI benchmark node 00010
                      - 'gridcell "node:UIF-PRECISION-00010 X: 1 m" [ref=e343]':
                        - 'button "node:UIF-PRECISION-00010 X: 1 m" [ref=e344]': "1"
                      - 'gridcell "node:UIF-PRECISION-00010 Y: -0.002755 m" [ref=e345]':
                        - 'button "node:UIF-PRECISION-00010 Y: -0.002755 m" [ref=e346]': "-0.002755"
                      - 'gridcell "node:UIF-PRECISION-00010 Z: -0.004038 m" [ref=e347]':
                        - 'button "node:UIF-PRECISION-00010 Z: -0.004038 m" [ref=e348]': "-0.004038"
                      - 'gridcell "node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e349]':
                        - 'button "node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e350]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00011 node:UIF-PRECISION-00011 Label: UI benchmark node 00011 node:UIF-PRECISION-00011 X: 1.1 m node:UIF-PRECISION-00011 Y: -0.002077 m node:UIF-PRECISION-00011 Z: 0.003109 m node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e352]':
                      - rowheader "node:UIF-PRECISION-00011" [ref=e353]:
                        - button "node:UIF-PRECISION-00011" [ref=e354]
                      - 'gridcell "node:UIF-PRECISION-00011 Label: UI benchmark node 00011" [ref=e355]':
                        - 'button "node:UIF-PRECISION-00011 Label: UI benchmark node 00011" [ref=e356]': UI benchmark node 00011
                      - 'gridcell "node:UIF-PRECISION-00011 X: 1.1 m" [ref=e357]':
                        - 'button "node:UIF-PRECISION-00011 X: 1.1 m" [ref=e358]': "1.1"
                      - 'gridcell "node:UIF-PRECISION-00011 Y: -0.002077 m" [ref=e359]':
                        - 'button "node:UIF-PRECISION-00011 Y: -0.002077 m" [ref=e360]': "-0.002077"
                      - 'gridcell "node:UIF-PRECISION-00011 Z: 0.003109 m" [ref=e361]':
                        - 'button "node:UIF-PRECISION-00011 Z: 0.003109 m" [ref=e362]': "0.003109"
                      - 'gridcell "node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e363]':
                        - 'button "node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e364]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00012 node:UIF-PRECISION-00012 Label: UI benchmark node 00012 node:UIF-PRECISION-00012 X: 1.2 m node:UIF-PRECISION-00012 Y: 0.006273 m node:UIF-PRECISION-00012 Z: -0.006632 m node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e366]':
                      - rowheader "node:UIF-PRECISION-00012" [ref=e367]:
                        - button "node:UIF-PRECISION-00012" [ref=e368]
                      - 'gridcell "node:UIF-PRECISION-00012 Label: UI benchmark node 00012" [ref=e369]':
                        - 'button "node:UIF-PRECISION-00012 Label: UI benchmark node 00012" [ref=e370]': UI benchmark node 00012
                      - 'gridcell "node:UIF-PRECISION-00012 X: 1.2 m" [ref=e371]':
                        - 'button "node:UIF-PRECISION-00012 X: 1.2 m" [ref=e372]': "1.2"
                      - 'gridcell "node:UIF-PRECISION-00012 Y: 0.006273 m" [ref=e373]':
                        - 'button "node:UIF-PRECISION-00012 Y: 0.006273 m" [ref=e374]': "0.006273"
                      - 'gridcell "node:UIF-PRECISION-00012 Z: -0.006632 m" [ref=e375]':
                        - 'button "node:UIF-PRECISION-00012 Z: -0.006632 m" [ref=e376]': "-0.006632"
                      - 'gridcell "node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e377]':
                        - 'button "node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e378]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00013 node:UIF-PRECISION-00013 Label: UI benchmark node 00013 node:UIF-PRECISION-00013 X: 1.3 m node:UIF-PRECISION-00013 Y: 0.008498 m node:UIF-PRECISION-00013 Z: 0.005846 m node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e380]':
                      - rowheader "node:UIF-PRECISION-00013" [ref=e381]:
                        - button "node:UIF-PRECISION-00013" [ref=e382]
                      - 'gridcell "node:UIF-PRECISION-00013 Label: UI benchmark node 00013" [ref=e383]':
                        - 'button "node:UIF-PRECISION-00013 Label: UI benchmark node 00013" [ref=e384]': UI benchmark node 00013
                      - 'gridcell "node:UIF-PRECISION-00013 X: 1.3 m" [ref=e385]':
                        - 'button "node:UIF-PRECISION-00013 X: 1.3 m" [ref=e386]': "1.3"
                      - 'gridcell "node:UIF-PRECISION-00013 Y: 0.008498 m" [ref=e387]':
                        - 'button "node:UIF-PRECISION-00013 Y: 0.008498 m" [ref=e388]': "0.008498"
                      - 'gridcell "node:UIF-PRECISION-00013 Z: 0.005846 m" [ref=e389]':
                        - 'button "node:UIF-PRECISION-00013 Z: 0.005846 m" [ref=e390]': "0.005846"
                      - 'gridcell "node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e391]':
                        - 'button "node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e392]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00014 node:UIF-PRECISION-00014 Label: UI benchmark node 00014 node:UIF-PRECISION-00014 X: 1.4 m node:UIF-PRECISION-00014 Y: 0.001425 m node:UIF-PRECISION-00014 Z: -0.003282 m node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e394]':
                      - rowheader "node:UIF-PRECISION-00014" [ref=e395]:
                        - button "node:UIF-PRECISION-00014" [ref=e396]
                      - 'gridcell "node:UIF-PRECISION-00014 Label: UI benchmark node 00014" [ref=e397]':
                        - 'button "node:UIF-PRECISION-00014 Label: UI benchmark node 00014" [ref=e398]': UI benchmark node 00014
                      - 'gridcell "node:UIF-PRECISION-00014 X: 1.4 m" [ref=e399]':
                        - 'button "node:UIF-PRECISION-00014 X: 1.4 m" [ref=e400]': "1.4"
                      - 'gridcell "node:UIF-PRECISION-00014 Y: 0.001425 m" [ref=e401]':
                        - 'button "node:UIF-PRECISION-00014 Y: 0.001425 m" [ref=e402]': "0.001425"
                      - 'gridcell "node:UIF-PRECISION-00014 Z: -0.003282 m" [ref=e403]':
                        - 'button "node:UIF-PRECISION-00014 Z: -0.003282 m" [ref=e404]': "-0.003282"
                      - 'gridcell "node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e405]':
                        - 'button "node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e406]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00015 node:UIF-PRECISION-00015 Label: UI benchmark node 00015 node:UIF-PRECISION-00015 X: 1.5 m node:UIF-PRECISION-00015 Y: 0.005946 m node:UIF-PRECISION-00015 Z: -0.000737 m node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e408]':
                      - rowheader "node:UIF-PRECISION-00015" [ref=e409]:
                        - button "node:UIF-PRECISION-00015" [ref=e410]
                      - 'gridcell "node:UIF-PRECISION-00015 Label: UI benchmark node 00015" [ref=e411]':
                        - 'button "node:UIF-PRECISION-00015 Label: UI benchmark node 00015" [ref=e412]': UI benchmark node 00015
                      - 'gridcell "node:UIF-PRECISION-00015 X: 1.5 m" [ref=e413]':
                        - 'button "node:UIF-PRECISION-00015 X: 1.5 m" [ref=e414]': "1.5"
                      - 'gridcell "node:UIF-PRECISION-00015 Y: 0.005946 m" [ref=e415]':
                        - 'button "node:UIF-PRECISION-00015 Y: 0.005946 m" [ref=e416]': "0.005946"
                      - 'gridcell "node:UIF-PRECISION-00015 Z: -0.000737 m" [ref=e417]':
                        - 'button "node:UIF-PRECISION-00015 Z: -0.000737 m" [ref=e418]': "-0.000737"
                      - 'gridcell "node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e419]':
                        - 'button "node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e420]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00016 node:UIF-PRECISION-00016 Label: UI benchmark node 00016 node:UIF-PRECISION-00016 X: 1.6 m node:UIF-PRECISION-00016 Y: 0.005719 m node:UIF-PRECISION-00016 Z: -0.003647 m node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e422]':
                      - rowheader "node:UIF-PRECISION-00016" [ref=e423]:
                        - button "node:UIF-PRECISION-00016" [ref=e424]
                      - 'gridcell "node:UIF-PRECISION-00016 Label: UI benchmark node 00016" [ref=e425]':
                        - 'button "node:UIF-PRECISION-00016 Label: UI benchmark node 00016" [ref=e426]': UI benchmark node 00016
                      - 'gridcell "node:UIF-PRECISION-00016 X: 1.6 m" [ref=e427]':
                        - 'button "node:UIF-PRECISION-00016 X: 1.6 m" [ref=e428]': "1.6"
                      - 'gridcell "node:UIF-PRECISION-00016 Y: 0.005719 m" [ref=e429]':
                        - 'button "node:UIF-PRECISION-00016 Y: 0.005719 m" [ref=e430]': "0.005719"
                      - 'gridcell "node:UIF-PRECISION-00016 Z: -0.003647 m" [ref=e431]':
                        - 'button "node:UIF-PRECISION-00016 Z: -0.003647 m" [ref=e432]': "-0.003647"
                      - 'gridcell "node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e433]':
                        - 'button "node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e434]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00017 node:UIF-PRECISION-00017 Label: UI benchmark node 00017 node:UIF-PRECISION-00017 X: 1.7 m node:UIF-PRECISION-00017 Y: 0.000353 m node:UIF-PRECISION-00017 Z: 0.001092 m node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e436]':
                      - rowheader "node:UIF-PRECISION-00017" [ref=e437]:
                        - button "node:UIF-PRECISION-00017" [ref=e438]
                      - 'gridcell "node:UIF-PRECISION-00017 Label: UI benchmark node 00017" [ref=e439]':
                        - 'button "node:UIF-PRECISION-00017 Label: UI benchmark node 00017" [ref=e440]': UI benchmark node 00017
                      - 'gridcell "node:UIF-PRECISION-00017 X: 1.7 m" [ref=e441]':
                        - 'button "node:UIF-PRECISION-00017 X: 1.7 m" [ref=e442]': "1.7"
                      - 'gridcell "node:UIF-PRECISION-00017 Y: 0.000353 m" [ref=e443]':
                        - 'button "node:UIF-PRECISION-00017 Y: 0.000353 m" [ref=e444]': "0.000353"
                      - 'gridcell "node:UIF-PRECISION-00017 Z: 0.001092 m" [ref=e445]':
                        - 'button "node:UIF-PRECISION-00017 Z: 0.001092 m" [ref=e446]': "0.001092"
                      - 'gridcell "node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e447]':
                        - 'button "node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e448]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00018 node:UIF-PRECISION-00018 Label: UI benchmark node 00018 node:UIF-PRECISION-00018 X: 1.8 m node:UIF-PRECISION-00018 Y: 0.003395 m node:UIF-PRECISION-00018 Z: 0.002323 m node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e450]':
                      - rowheader "node:UIF-PRECISION-00018" [ref=e451]:
                        - button "node:UIF-PRECISION-00018" [ref=e452]
                      - 'gridcell "node:UIF-PRECISION-00018 Label: UI benchmark node 00018" [ref=e453]':
                        - 'button "node:UIF-PRECISION-00018 Label: UI benchmark node 00018" [ref=e454]': UI benchmark node 00018
                      - 'gridcell "node:UIF-PRECISION-00018 X: 1.8 m" [ref=e455]':
                        - 'button "node:UIF-PRECISION-00018 X: 1.8 m" [ref=e456]': "1.8"
                      - 'gridcell "node:UIF-PRECISION-00018 Y: 0.003395 m" [ref=e457]':
                        - 'button "node:UIF-PRECISION-00018 Y: 0.003395 m" [ref=e458]': "0.003395"
                      - 'gridcell "node:UIF-PRECISION-00018 Z: 0.002323 m" [ref=e459]':
                        - 'button "node:UIF-PRECISION-00018 Z: 0.002323 m" [ref=e460]': "0.002323"
                      - 'gridcell "node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e461]':
                        - 'button "node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e462]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00019 node:UIF-PRECISION-00019 Label: UI benchmark node 00019 node:UIF-PRECISION-00019 X: 1.9 m node:UIF-PRECISION-00019 Y: 0.006887 m node:UIF-PRECISION-00019 Z: 0.00658 m node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e464]':
                      - rowheader "node:UIF-PRECISION-00019" [ref=e465]:
                        - button "node:UIF-PRECISION-00019" [ref=e466]
                      - 'gridcell "node:UIF-PRECISION-00019 Label: UI benchmark node 00019" [ref=e467]':
                        - 'button "node:UIF-PRECISION-00019 Label: UI benchmark node 00019" [ref=e468]': UI benchmark node 00019
                      - 'gridcell "node:UIF-PRECISION-00019 X: 1.9 m" [ref=e469]':
                        - 'button "node:UIF-PRECISION-00019 X: 1.9 m" [ref=e470]': "1.9"
                      - 'gridcell "node:UIF-PRECISION-00019 Y: 0.006887 m" [ref=e471]':
                        - 'button "node:UIF-PRECISION-00019 Y: 0.006887 m" [ref=e472]': "0.006887"
                      - 'gridcell "node:UIF-PRECISION-00019 Z: 0.00658 m" [ref=e473]':
                        - 'button "node:UIF-PRECISION-00019 Z: 0.00658 m" [ref=e474]': "0.00658"
                      - 'gridcell "node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e475]':
                        - 'button "node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e476]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00020 node:UIF-PRECISION-00020 Label: UI benchmark node 00020 node:UIF-PRECISION-00020 X: 2 m node:UIF-PRECISION-00020 Y: -0.004467 m node:UIF-PRECISION-00020 Z: -0.007252 m node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e478]':
                      - rowheader "node:UIF-PRECISION-00020" [ref=e479]:
                        - button "node:UIF-PRECISION-00020" [ref=e480]
                      - 'gridcell "node:UIF-PRECISION-00020 Label: UI benchmark node 00020" [ref=e481]':
                        - 'button "node:UIF-PRECISION-00020 Label: UI benchmark node 00020" [ref=e482]': UI benchmark node 00020
                      - 'gridcell "node:UIF-PRECISION-00020 X: 2 m" [ref=e483]':
                        - 'button "node:UIF-PRECISION-00020 X: 2 m" [ref=e484]': "2"
                      - 'gridcell "node:UIF-PRECISION-00020 Y: -0.004467 m" [ref=e485]':
                        - 'button "node:UIF-PRECISION-00020 Y: -0.004467 m" [ref=e486]': "-0.004467"
                      - 'gridcell "node:UIF-PRECISION-00020 Z: -0.007252 m" [ref=e487]':
                        - 'button "node:UIF-PRECISION-00020 Z: -0.007252 m" [ref=e488]': "-0.007252"
                      - 'gridcell "node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e489]':
                        - 'button "node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e490]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - group "Node fields footer" [ref=e491]:
                - generic [ref=e492]: 21 of 21 rows
            - button "Review multiple changes" [ref=e493] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e494]:
      - button "Agent" [disabled] [ref=e496]:
        - img [ref=e497]
        - generic [ref=e500]: Agent
  - generic "Workspace status" [ref=e501]:
    - generic "Analysis statuses" [ref=e502]:
      - button "Solver · Not solved" [ref=e504] [cursor=pointer]
    - button "2 Issues" [ref=e505] [cursor=pointer]:
      - img [ref=e506]
      - text: 2 Issues
    - generic "Selection" [ref=e508]: "material: material:UIF-INVENTED-01"
    - generic "Display units" [ref=e509]: Entered
    - button "About SWBPIPE…" [ref=e510] [cursor=pointer]:
      - img [ref=e511]
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