# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 diagnostic Node sorted review preserves text Undo after an active row crosses downward
- Location: e2e/b4-table-editing.spec.ts:552:1

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: "1"
Received: "19"
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
            - generic [ref=e163]: 48 of 48 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - region "Bulk entity grid" [ref=e169]:
            - generic "Grid entity type" [ref=e170]:
              - button "Nodes" [pressed] [ref=e171]
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
                    - 'row "node:UIF-PRECISION-00000 node:UIF-PRECISION-00000 Label: UI benchmark node 00000 node:UIF-PRECISION-00000 X: 2 m node:UIF-PRECISION-00000 Y: 0 m node:UIF-PRECISION-00000 Z: 0 m node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e198]':
                      - rowheader "node:UIF-PRECISION-00000" [ref=e199]:
                        - button "node:UIF-PRECISION-00000" [ref=e200]
                      - 'gridcell "node:UIF-PRECISION-00000 Label: UI benchmark node 00000" [ref=e201]':
                        - 'button "node:UIF-PRECISION-00000 Label: UI benchmark node 00000" [ref=e202]': UI benchmark node 00000
                      - 'gridcell "node:UIF-PRECISION-00000 X: 2 m" [ref=e203]':
                        - 'button "node:UIF-PRECISION-00000 X: 2 m" [ref=e204]': "2"
                      - 'gridcell "node:UIF-PRECISION-00000 Y: 0 m" [ref=e205]':
                        - 'button "node:UIF-PRECISION-00000 Y: 0 m" [ref=e206]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Z: 0 m" [ref=e207]':
                        - 'button "node:UIF-PRECISION-00000 Z: 0 m" [ref=e208]': "0"
                      - 'gridcell "node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e209]':
                        - 'button "node:UIF-PRECISION-00000 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e210]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00001 node:UIF-PRECISION-00001 Label: UI benchmark node 00001 node:UIF-PRECISION-00001 X: 15 m node:UIF-PRECISION-00001 Y: -0.00256 m node:UIF-PRECISION-00001 Z: 0.00648 m node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e212]':
                      - rowheader "node:UIF-PRECISION-00001" [ref=e213]:
                        - button "node:UIF-PRECISION-00001" [ref=e214]
                      - 'gridcell "node:UIF-PRECISION-00001 Label: UI benchmark node 00001" [ref=e215]':
                        - 'button "node:UIF-PRECISION-00001 Label: UI benchmark node 00001" [ref=e216]': UI benchmark node 00001
                      - 'gridcell "node:UIF-PRECISION-00001 X: 15 m" [ref=e217]':
                        - 'button "node:UIF-PRECISION-00001 X: 15 m" [ref=e218]': "15"
                      - 'gridcell "node:UIF-PRECISION-00001 Y: -0.00256 m" [ref=e219]':
                        - 'button "node:UIF-PRECISION-00001 Y: -0.00256 m" [ref=e220]': "-0.00256"
                      - 'gridcell "node:UIF-PRECISION-00001 Z: 0.00648 m" [ref=e221]':
                        - 'button "node:UIF-PRECISION-00001 Z: 0.00648 m" [ref=e222]': "0.00648"
                      - 'gridcell "node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e223]':
                        - 'button "node:UIF-PRECISION-00001 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e224]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00002 node:UIF-PRECISION-00002 Label: UI benchmark node 00002 node:UIF-PRECISION-00002 X: 102 m node:UIF-PRECISION-00002 Y: -0.007015 m node:UIF-PRECISION-00002 Z: 0.006753 m node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e226]':
                      - rowheader "node:UIF-PRECISION-00002" [ref=e227]:
                        - button "node:UIF-PRECISION-00002" [ref=e228]
                      - 'gridcell "node:UIF-PRECISION-00002 Label: UI benchmark node 00002" [ref=e229]':
                        - 'button "node:UIF-PRECISION-00002 Label: UI benchmark node 00002" [ref=e230]': UI benchmark node 00002
                      - 'gridcell "node:UIF-PRECISION-00002 X: 102 m" [ref=e231]':
                        - 'button "node:UIF-PRECISION-00002 X: 102 m" [ref=e232]': "102"
                      - 'gridcell "node:UIF-PRECISION-00002 Y: -0.007015 m" [ref=e233]':
                        - 'button "node:UIF-PRECISION-00002 Y: -0.007015 m" [ref=e234]': "-0.007015"
                      - 'gridcell "node:UIF-PRECISION-00002 Z: 0.006753 m" [ref=e235]':
                        - 'button "node:UIF-PRECISION-00002 Z: 0.006753 m" [ref=e236]': "0.006753"
                      - 'gridcell "node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e237]':
                        - 'button "node:UIF-PRECISION-00002 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e238]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00003 node:UIF-PRECISION-00003 Label: UI benchmark node 00003 node:UIF-PRECISION-00003 X: 103 m node:UIF-PRECISION-00003 Y: -0.007731 m node:UIF-PRECISION-00003 Z: 0.007658 m node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e240]':
                      - rowheader "node:UIF-PRECISION-00003" [ref=e241]:
                        - button "node:UIF-PRECISION-00003" [ref=e242]
                      - 'gridcell "node:UIF-PRECISION-00003 Label: UI benchmark node 00003" [ref=e243]':
                        - 'button "node:UIF-PRECISION-00003 Label: UI benchmark node 00003" [ref=e244]': UI benchmark node 00003
                      - 'gridcell "node:UIF-PRECISION-00003 X: 103 m" [ref=e245]':
                        - 'button "node:UIF-PRECISION-00003 X: 103 m" [ref=e246]': "103"
                      - 'gridcell "node:UIF-PRECISION-00003 Y: -0.007731 m" [ref=e247]':
                        - 'button "node:UIF-PRECISION-00003 Y: -0.007731 m" [ref=e248]': "-0.007731"
                      - 'gridcell "node:UIF-PRECISION-00003 Z: 0.007658 m" [ref=e249]':
                        - 'button "node:UIF-PRECISION-00003 Z: 0.007658 m" [ref=e250]': "0.007658"
                      - 'gridcell "node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e251]':
                        - 'button "node:UIF-PRECISION-00003 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e252]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00004 node:UIF-PRECISION-00004 Label: UI benchmark node 00004 node:UIF-PRECISION-00004 X: 104 m node:UIF-PRECISION-00004 Y: 0.001301 m node:UIF-PRECISION-00004 Z: -0.004954 m node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e254]':
                      - rowheader "node:UIF-PRECISION-00004" [ref=e255]:
                        - button "node:UIF-PRECISION-00004" [ref=e256]
                      - 'gridcell "node:UIF-PRECISION-00004 Label: UI benchmark node 00004" [ref=e257]':
                        - 'button "node:UIF-PRECISION-00004 Label: UI benchmark node 00004" [ref=e258]': UI benchmark node 00004
                      - 'gridcell "node:UIF-PRECISION-00004 X: 104 m" [ref=e259]':
                        - 'button "node:UIF-PRECISION-00004 X: 104 m" [ref=e260]': "104"
                      - 'gridcell "node:UIF-PRECISION-00004 Y: 0.001301 m" [ref=e261]':
                        - 'button "node:UIF-PRECISION-00004 Y: 0.001301 m" [ref=e262]': "0.001301"
                      - 'gridcell "node:UIF-PRECISION-00004 Z: -0.004954 m" [ref=e263]':
                        - 'button "node:UIF-PRECISION-00004 Z: -0.004954 m" [ref=e264]': "-0.004954"
                      - 'gridcell "node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e265]':
                        - 'button "node:UIF-PRECISION-00004 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e266]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00005 node:UIF-PRECISION-00005 Label: UI benchmark node 00005 node:UIF-PRECISION-00005 X: 105 m node:UIF-PRECISION-00005 Y: 0.000066 m node:UIF-PRECISION-00005 Z: -0.006799 m node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e268]':
                      - rowheader "node:UIF-PRECISION-00005" [ref=e269]:
                        - button "node:UIF-PRECISION-00005" [ref=e270]
                      - 'gridcell "node:UIF-PRECISION-00005 Label: UI benchmark node 00005" [ref=e271]':
                        - 'button "node:UIF-PRECISION-00005 Label: UI benchmark node 00005" [ref=e272]': UI benchmark node 00005
                      - 'gridcell "node:UIF-PRECISION-00005 X: 105 m" [ref=e273]':
                        - 'button "node:UIF-PRECISION-00005 X: 105 m" [ref=e274]': "105"
                      - 'gridcell "node:UIF-PRECISION-00005 Y: 0.000066 m" [ref=e275]':
                        - 'button "node:UIF-PRECISION-00005 Y: 0.000066 m" [ref=e276]': "0.000066"
                      - 'gridcell "node:UIF-PRECISION-00005 Z: -0.006799 m" [ref=e277]':
                        - 'button "node:UIF-PRECISION-00005 Z: -0.006799 m" [ref=e278]': "-0.006799"
                      - 'gridcell "node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e279]':
                        - 'button "node:UIF-PRECISION-00005 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e280]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00006 node:UIF-PRECISION-00006 Label: UI benchmark node 00006 node:UIF-PRECISION-00006 X: 106 m node:UIF-PRECISION-00006 Y: 0.007154 m node:UIF-PRECISION-00006 Z: -0.004567 m node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e282]':
                      - rowheader "node:UIF-PRECISION-00006" [ref=e283]:
                        - button "node:UIF-PRECISION-00006" [ref=e284]
                      - 'gridcell "node:UIF-PRECISION-00006 Label: UI benchmark node 00006" [ref=e285]':
                        - 'button "node:UIF-PRECISION-00006 Label: UI benchmark node 00006" [ref=e286]': UI benchmark node 00006
                      - 'gridcell "node:UIF-PRECISION-00006 X: 106 m" [ref=e287]':
                        - 'button "node:UIF-PRECISION-00006 X: 106 m" [ref=e288]': "106"
                      - 'gridcell "node:UIF-PRECISION-00006 Y: 0.007154 m" [ref=e289]':
                        - 'button "node:UIF-PRECISION-00006 Y: 0.007154 m" [ref=e290]': "0.007154"
                      - 'gridcell "node:UIF-PRECISION-00006 Z: -0.004567 m" [ref=e291]':
                        - 'button "node:UIF-PRECISION-00006 Z: -0.004567 m" [ref=e292]': "-0.004567"
                      - 'gridcell "node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e293]':
                        - 'button "node:UIF-PRECISION-00006 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e294]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00007 node:UIF-PRECISION-00007 Label: UI benchmark node 00007 node:UIF-PRECISION-00007 X: 107 m node:UIF-PRECISION-00007 Y: -0.000754 m node:UIF-PRECISION-00007 Z: -0.001598 m node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e296]':
                      - rowheader "node:UIF-PRECISION-00007" [ref=e297]:
                        - button "node:UIF-PRECISION-00007" [ref=e298]
                      - 'gridcell "node:UIF-PRECISION-00007 Label: UI benchmark node 00007" [ref=e299]':
                        - 'button "node:UIF-PRECISION-00007 Label: UI benchmark node 00007" [ref=e300]': UI benchmark node 00007
                      - 'gridcell "node:UIF-PRECISION-00007 X: 107 m" [ref=e301]':
                        - 'button "node:UIF-PRECISION-00007 X: 107 m" [ref=e302]': "107"
                      - 'gridcell "node:UIF-PRECISION-00007 Y: -0.000754 m" [ref=e303]':
                        - 'button "node:UIF-PRECISION-00007 Y: -0.000754 m" [ref=e304]': "-0.000754"
                      - 'gridcell "node:UIF-PRECISION-00007 Z: -0.001598 m" [ref=e305]':
                        - 'button "node:UIF-PRECISION-00007 Z: -0.001598 m" [ref=e306]': "-0.001598"
                      - 'gridcell "node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e307]':
                        - 'button "node:UIF-PRECISION-00007 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e308]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00008 node:UIF-PRECISION-00008 Label: UI benchmark node 00008 node:UIF-PRECISION-00008 X: 108 m node:UIF-PRECISION-00008 Y: 0.001591 m node:UIF-PRECISION-00008 Z: -0.004104 m node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e310]':
                      - rowheader "node:UIF-PRECISION-00008" [ref=e311]:
                        - button "node:UIF-PRECISION-00008" [ref=e312]
                      - 'gridcell "node:UIF-PRECISION-00008 Label: UI benchmark node 00008" [ref=e313]':
                        - 'button "node:UIF-PRECISION-00008 Label: UI benchmark node 00008" [ref=e314]': UI benchmark node 00008
                      - 'gridcell "node:UIF-PRECISION-00008 X: 108 m" [ref=e315]':
                        - 'button "node:UIF-PRECISION-00008 X: 108 m" [ref=e316]': "108"
                      - 'gridcell "node:UIF-PRECISION-00008 Y: 0.001591 m" [ref=e317]':
                        - 'button "node:UIF-PRECISION-00008 Y: 0.001591 m" [ref=e318]': "0.001591"
                      - 'gridcell "node:UIF-PRECISION-00008 Z: -0.004104 m" [ref=e319]':
                        - 'button "node:UIF-PRECISION-00008 Z: -0.004104 m" [ref=e320]': "-0.004104"
                      - 'gridcell "node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e321]':
                        - 'button "node:UIF-PRECISION-00008 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e322]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00009 node:UIF-PRECISION-00009 Label: UI benchmark node 00009 node:UIF-PRECISION-00009 X: 109 m node:UIF-PRECISION-00009 Y: -0.001553 m node:UIF-PRECISION-00009 Z: -0.000201 m node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e324]':
                      - rowheader "node:UIF-PRECISION-00009" [ref=e325]:
                        - button "node:UIF-PRECISION-00009" [ref=e326]
                      - 'gridcell "node:UIF-PRECISION-00009 Label: UI benchmark node 00009" [ref=e327]':
                        - 'button "node:UIF-PRECISION-00009 Label: UI benchmark node 00009" [ref=e328]': UI benchmark node 00009
                      - 'gridcell "node:UIF-PRECISION-00009 X: 109 m" [ref=e329]':
                        - 'button "node:UIF-PRECISION-00009 X: 109 m" [ref=e330]': "109"
                      - 'gridcell "node:UIF-PRECISION-00009 Y: -0.001553 m" [ref=e331]':
                        - 'button "node:UIF-PRECISION-00009 Y: -0.001553 m" [ref=e332]': "-0.001553"
                      - 'gridcell "node:UIF-PRECISION-00009 Z: -0.000201 m" [ref=e333]':
                        - 'button "node:UIF-PRECISION-00009 Z: -0.000201 m" [ref=e334]': "-0.000201"
                      - 'gridcell "node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e335]':
                        - 'button "node:UIF-PRECISION-00009 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e336]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00010 node:UIF-PRECISION-00010 Label: UI benchmark node 00010 node:UIF-PRECISION-00010 X: 110 m node:UIF-PRECISION-00010 Y: -0.002755 m node:UIF-PRECISION-00010 Z: -0.004038 m node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e338]':
                      - rowheader "node:UIF-PRECISION-00010" [ref=e339]:
                        - button "node:UIF-PRECISION-00010" [ref=e340]
                      - 'gridcell "node:UIF-PRECISION-00010 Label: UI benchmark node 00010" [ref=e341]':
                        - 'button "node:UIF-PRECISION-00010 Label: UI benchmark node 00010" [ref=e342]': UI benchmark node 00010
                      - 'gridcell "node:UIF-PRECISION-00010 X: 110 m" [ref=e343]':
                        - 'button "node:UIF-PRECISION-00010 X: 110 m" [ref=e344]': "110"
                      - 'gridcell "node:UIF-PRECISION-00010 Y: -0.002755 m" [ref=e345]':
                        - 'button "node:UIF-PRECISION-00010 Y: -0.002755 m" [ref=e346]': "-0.002755"
                      - 'gridcell "node:UIF-PRECISION-00010 Z: -0.004038 m" [ref=e347]':
                        - 'button "node:UIF-PRECISION-00010 Z: -0.004038 m" [ref=e348]': "-0.004038"
                      - 'gridcell "node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e349]':
                        - 'button "node:UIF-PRECISION-00010 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e350]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00011 node:UIF-PRECISION-00011 Label: UI benchmark node 00011 node:UIF-PRECISION-00011 X: 111 m node:UIF-PRECISION-00011 Y: -0.002077 m node:UIF-PRECISION-00011 Z: 0.003109 m node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e352]':
                      - rowheader "node:UIF-PRECISION-00011" [ref=e353]:
                        - button "node:UIF-PRECISION-00011" [ref=e354]
                      - 'gridcell "node:UIF-PRECISION-00011 Label: UI benchmark node 00011" [ref=e355]':
                        - 'button "node:UIF-PRECISION-00011 Label: UI benchmark node 00011" [ref=e356]': UI benchmark node 00011
                      - 'gridcell "node:UIF-PRECISION-00011 X: 111 m" [ref=e357]':
                        - 'button "node:UIF-PRECISION-00011 X: 111 m" [ref=e358]': "111"
                      - 'gridcell "node:UIF-PRECISION-00011 Y: -0.002077 m" [ref=e359]':
                        - 'button "node:UIF-PRECISION-00011 Y: -0.002077 m" [ref=e360]': "-0.002077"
                      - 'gridcell "node:UIF-PRECISION-00011 Z: 0.003109 m" [ref=e361]':
                        - 'button "node:UIF-PRECISION-00011 Z: 0.003109 m" [ref=e362]': "0.003109"
                      - 'gridcell "node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e363]':
                        - 'button "node:UIF-PRECISION-00011 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e364]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00012 node:UIF-PRECISION-00012 Label: UI benchmark node 00012 node:UIF-PRECISION-00012 X: 112 m node:UIF-PRECISION-00012 Y: 0.006273 m node:UIF-PRECISION-00012 Z: -0.006632 m node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e366]':
                      - rowheader "node:UIF-PRECISION-00012" [ref=e367]:
                        - button "node:UIF-PRECISION-00012" [ref=e368]
                      - 'gridcell "node:UIF-PRECISION-00012 Label: UI benchmark node 00012" [ref=e369]':
                        - 'button "node:UIF-PRECISION-00012 Label: UI benchmark node 00012" [ref=e370]': UI benchmark node 00012
                      - 'gridcell "node:UIF-PRECISION-00012 X: 112 m" [ref=e371]':
                        - 'button "node:UIF-PRECISION-00012 X: 112 m" [ref=e372]': "112"
                      - 'gridcell "node:UIF-PRECISION-00012 Y: 0.006273 m" [ref=e373]':
                        - 'button "node:UIF-PRECISION-00012 Y: 0.006273 m" [ref=e374]': "0.006273"
                      - 'gridcell "node:UIF-PRECISION-00012 Z: -0.006632 m" [ref=e375]':
                        - 'button "node:UIF-PRECISION-00012 Z: -0.006632 m" [ref=e376]': "-0.006632"
                      - 'gridcell "node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e377]':
                        - 'button "node:UIF-PRECISION-00012 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e378]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00013 node:UIF-PRECISION-00013 Label: UI benchmark node 00013 node:UIF-PRECISION-00013 X: 113 m node:UIF-PRECISION-00013 Y: 0.008498 m node:UIF-PRECISION-00013 Z: 0.005846 m node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e380]':
                      - rowheader "node:UIF-PRECISION-00013" [ref=e381]:
                        - button "node:UIF-PRECISION-00013" [ref=e382]
                      - 'gridcell "node:UIF-PRECISION-00013 Label: UI benchmark node 00013" [ref=e383]':
                        - 'button "node:UIF-PRECISION-00013 Label: UI benchmark node 00013" [ref=e384]': UI benchmark node 00013
                      - 'gridcell "node:UIF-PRECISION-00013 X: 113 m" [ref=e385]':
                        - 'button "node:UIF-PRECISION-00013 X: 113 m" [ref=e386]': "113"
                      - 'gridcell "node:UIF-PRECISION-00013 Y: 0.008498 m" [ref=e387]':
                        - 'button "node:UIF-PRECISION-00013 Y: 0.008498 m" [ref=e388]': "0.008498"
                      - 'gridcell "node:UIF-PRECISION-00013 Z: 0.005846 m" [ref=e389]':
                        - 'button "node:UIF-PRECISION-00013 Z: 0.005846 m" [ref=e390]': "0.005846"
                      - 'gridcell "node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e391]':
                        - 'button "node:UIF-PRECISION-00013 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e392]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00014 node:UIF-PRECISION-00014 Label: UI benchmark node 00014 node:UIF-PRECISION-00014 X: 114 m node:UIF-PRECISION-00014 Y: 0.001425 m node:UIF-PRECISION-00014 Z: -0.003282 m node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e394]':
                      - rowheader "node:UIF-PRECISION-00014" [ref=e395]:
                        - button "node:UIF-PRECISION-00014" [ref=e396]
                      - 'gridcell "node:UIF-PRECISION-00014 Label: UI benchmark node 00014" [ref=e397]':
                        - 'button "node:UIF-PRECISION-00014 Label: UI benchmark node 00014" [ref=e398]': UI benchmark node 00014
                      - 'gridcell "node:UIF-PRECISION-00014 X: 114 m" [ref=e399]':
                        - 'button "node:UIF-PRECISION-00014 X: 114 m" [ref=e400]': "114"
                      - 'gridcell "node:UIF-PRECISION-00014 Y: 0.001425 m" [ref=e401]':
                        - 'button "node:UIF-PRECISION-00014 Y: 0.001425 m" [ref=e402]': "0.001425"
                      - 'gridcell "node:UIF-PRECISION-00014 Z: -0.003282 m" [ref=e403]':
                        - 'button "node:UIF-PRECISION-00014 Z: -0.003282 m" [ref=e404]': "-0.003282"
                      - 'gridcell "node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e405]':
                        - 'button "node:UIF-PRECISION-00014 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e406]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00015 node:UIF-PRECISION-00015 Label: UI benchmark node 00015 node:UIF-PRECISION-00015 X: 115 m node:UIF-PRECISION-00015 Y: 0.005946 m node:UIF-PRECISION-00015 Z: -0.000737 m node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e408]':
                      - rowheader "node:UIF-PRECISION-00015" [ref=e409]:
                        - button "node:UIF-PRECISION-00015" [ref=e410]
                      - 'gridcell "node:UIF-PRECISION-00015 Label: UI benchmark node 00015" [ref=e411]':
                        - 'button "node:UIF-PRECISION-00015 Label: UI benchmark node 00015" [ref=e412]': UI benchmark node 00015
                      - 'gridcell "node:UIF-PRECISION-00015 X: 115 m" [ref=e413]':
                        - 'button "node:UIF-PRECISION-00015 X: 115 m" [ref=e414]': "115"
                      - 'gridcell "node:UIF-PRECISION-00015 Y: 0.005946 m" [ref=e415]':
                        - 'button "node:UIF-PRECISION-00015 Y: 0.005946 m" [ref=e416]': "0.005946"
                      - 'gridcell "node:UIF-PRECISION-00015 Z: -0.000737 m" [ref=e417]':
                        - 'button "node:UIF-PRECISION-00015 Z: -0.000737 m" [ref=e418]': "-0.000737"
                      - 'gridcell "node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e419]':
                        - 'button "node:UIF-PRECISION-00015 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e420]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00016 node:UIF-PRECISION-00016 Label: UI benchmark node 00016 node:UIF-PRECISION-00016 X: 116 m node:UIF-PRECISION-00016 Y: 0.005719 m node:UIF-PRECISION-00016 Z: -0.003647 m node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e422]':
                      - rowheader "node:UIF-PRECISION-00016" [ref=e423]:
                        - button "node:UIF-PRECISION-00016" [ref=e424]
                      - 'gridcell "node:UIF-PRECISION-00016 Label: UI benchmark node 00016" [ref=e425]':
                        - 'button "node:UIF-PRECISION-00016 Label: UI benchmark node 00016" [ref=e426]': UI benchmark node 00016
                      - 'gridcell "node:UIF-PRECISION-00016 X: 116 m" [ref=e427]':
                        - 'button "node:UIF-PRECISION-00016 X: 116 m" [ref=e428]': "116"
                      - 'gridcell "node:UIF-PRECISION-00016 Y: 0.005719 m" [ref=e429]':
                        - 'button "node:UIF-PRECISION-00016 Y: 0.005719 m" [ref=e430]': "0.005719"
                      - 'gridcell "node:UIF-PRECISION-00016 Z: -0.003647 m" [ref=e431]':
                        - 'button "node:UIF-PRECISION-00016 Z: -0.003647 m" [ref=e432]': "-0.003647"
                      - 'gridcell "node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e433]':
                        - 'button "node:UIF-PRECISION-00016 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e434]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00017 node:UIF-PRECISION-00017 Label: UI benchmark node 00017 node:UIF-PRECISION-00017 X: 117 m node:UIF-PRECISION-00017 Y: 0.000353 m node:UIF-PRECISION-00017 Z: 0.001092 m node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e436]':
                      - rowheader "node:UIF-PRECISION-00017" [ref=e437]:
                        - button "node:UIF-PRECISION-00017" [ref=e438]
                      - 'gridcell "node:UIF-PRECISION-00017 Label: UI benchmark node 00017" [ref=e439]':
                        - 'button "node:UIF-PRECISION-00017 Label: UI benchmark node 00017" [ref=e440]': UI benchmark node 00017
                      - 'gridcell "node:UIF-PRECISION-00017 X: 117 m" [ref=e441]':
                        - 'button "node:UIF-PRECISION-00017 X: 117 m" [ref=e442]': "117"
                      - 'gridcell "node:UIF-PRECISION-00017 Y: 0.000353 m" [ref=e443]':
                        - 'button "node:UIF-PRECISION-00017 Y: 0.000353 m" [ref=e444]': "0.000353"
                      - 'gridcell "node:UIF-PRECISION-00017 Z: 0.001092 m" [ref=e445]':
                        - 'button "node:UIF-PRECISION-00017 Z: 0.001092 m" [ref=e446]': "0.001092"
                      - 'gridcell "node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e447]':
                        - 'button "node:UIF-PRECISION-00017 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e448]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00018 node:UIF-PRECISION-00018 Label: UI benchmark node 00018 node:UIF-PRECISION-00018 X: 118 m node:UIF-PRECISION-00018 Y: 0.003395 m node:UIF-PRECISION-00018 Z: 0.002323 m node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e450]':
                      - rowheader "node:UIF-PRECISION-00018" [ref=e451]:
                        - button "node:UIF-PRECISION-00018" [ref=e452]
                      - 'gridcell "node:UIF-PRECISION-00018 Label: UI benchmark node 00018" [ref=e453]':
                        - 'button "node:UIF-PRECISION-00018 Label: UI benchmark node 00018" [ref=e454]': UI benchmark node 00018
                      - 'gridcell "node:UIF-PRECISION-00018 X: 118 m" [ref=e455]':
                        - 'button "node:UIF-PRECISION-00018 X: 118 m" [ref=e456]': "118"
                      - 'gridcell "node:UIF-PRECISION-00018 Y: 0.003395 m" [ref=e457]':
                        - 'button "node:UIF-PRECISION-00018 Y: 0.003395 m" [ref=e458]': "0.003395"
                      - 'gridcell "node:UIF-PRECISION-00018 Z: 0.002323 m" [ref=e459]':
                        - 'button "node:UIF-PRECISION-00018 Z: 0.002323 m" [ref=e460]': "0.002323"
                      - 'gridcell "node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e461]':
                        - 'button "node:UIF-PRECISION-00018 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e462]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00019 node:UIF-PRECISION-00019 Label: UI benchmark node 00019 node:UIF-PRECISION-00019 X: 119 m node:UIF-PRECISION-00019 Y: 0.006887 m node:UIF-PRECISION-00019 Z: 0.00658 m node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e464]':
                      - rowheader "node:UIF-PRECISION-00019" [ref=e465]:
                        - button "node:UIF-PRECISION-00019" [ref=e466]
                      - 'gridcell "node:UIF-PRECISION-00019 Label: UI benchmark node 00019" [ref=e467]':
                        - 'button "node:UIF-PRECISION-00019 Label: UI benchmark node 00019" [ref=e468]': UI benchmark node 00019
                      - 'gridcell "node:UIF-PRECISION-00019 X: 119 m" [ref=e469]':
                        - 'button "node:UIF-PRECISION-00019 X: 119 m" [ref=e470]': "119"
                      - 'gridcell "node:UIF-PRECISION-00019 Y: 0.006887 m" [ref=e471]':
                        - 'button "node:UIF-PRECISION-00019 Y: 0.006887 m" [ref=e472]': "0.006887"
                      - 'gridcell "node:UIF-PRECISION-00019 Z: 0.00658 m" [ref=e473]':
                        - 'button "node:UIF-PRECISION-00019 Z: 0.00658 m" [ref=e474]': "0.00658"
                      - 'gridcell "node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e475]':
                        - 'button "node:UIF-PRECISION-00019 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e476]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "node:UIF-PRECISION-00020 node:UIF-PRECISION-00020 Label: UI benchmark node 00020 node:UIF-PRECISION-00020 X: 120 m node:UIF-PRECISION-00020 Y: -0.004467 m node:UIF-PRECISION-00020 Z: -0.007252 m node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e478]':
                      - rowheader "node:UIF-PRECISION-00020" [ref=e479]:
                        - button "node:UIF-PRECISION-00020" [ref=e480]
                      - 'gridcell "node:UIF-PRECISION-00020 Label: UI benchmark node 00020" [ref=e481]':
                        - 'button "node:UIF-PRECISION-00020 Label: UI benchmark node 00020" [ref=e482]': UI benchmark node 00020
                      - 'gridcell "node:UIF-PRECISION-00020 X: 120 m" [ref=e483]':
                        - 'button "node:UIF-PRECISION-00020 X: 120 m" [ref=e484]': "120"
                      - 'gridcell "node:UIF-PRECISION-00020 Y: -0.004467 m" [ref=e485]':
                        - 'button "node:UIF-PRECISION-00020 Y: -0.004467 m" [ref=e486]': "-0.004467"
                      - 'gridcell "node:UIF-PRECISION-00020 Z: -0.007252 m" [ref=e487]':
                        - 'button "node:UIF-PRECISION-00020 Z: -0.007252 m" [ref=e488]': "-0.007252"
                      - 'gridcell "node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e489]':
                        - 'button "node:UIF-PRECISION-00020 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e490]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - group "Node fields footer" [ref=e491]:
                - generic [ref=e492]: 21 of 21 rows
            - button "Review multiple changes" [active] [ref=e493] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
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
    - generic "Selection" [ref=e508]: "project: project:UIF-PRECISION-ORIGIN"
    - generic "Display units" [ref=e509]: Entered
    - button "About SWBPIPE…" [ref=e510] [cursor=pointer]:
      - img [ref=e511]
```

# Test source

```ts
  508 |   const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0]; const firstId = material.id; const secondId = "material:B4-P2-second";
  509 |   model.materials = [{ ...material, elastic_modulus: { value: 200000, unit: "MPa" } },
  510 |     { ...material, id: secondId, elastic_modulus: { value: 100000000000, unit: "Pa" } }];
  511 |   await page.route("**/src/services/displayQuantityService.ts", async (route) => {
  512 |     const response = await route.fetch(); const source = await response.text(); expect(source).toContain("export async function convertDisplayQuantities(");
  513 |     await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
  514 | export async function convertDisplayQuantities(items) {
  515 |   const result = await originalConvertDisplayQuantities(items);
  516 |   const gate = window.__b4MaterialP2Gate;
  517 |   if (gate.hold && items.some(item => item.id.includes('material') && item.id.includes('elastic'))) await new Promise(resolve => gate.pending.push(resolve));
  518 |   return result;
  519 | }
  520 | ` });
  521 |   });
  522 |   await page.addInitScript(() => { (window as any).__b4MaterialP2Gate = { hold: false, pending: [] }; });
  523 |   await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  524 |   const table = page.getByTestId("material-engineering-table"); const elastic = table.getByTestId(`table-cell-${firstId}-elastic`); const shear = table.getByTestId(`table-cell-${firstId}-shear`);
  525 |   await expect(elastic.locator("..")).toHaveAttribute("aria-readonly", "false"); await expect(shear.locator("..")).toHaveAttribute("aria-readonly", "false");
  526 |   await table.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  527 |   await elastic.dblclick(); await table.getByRole("textbox").fill("210000");
  528 |   await page.evaluate(() => { (window as any).__b4MaterialP2Gate.hold = true; });
  529 |   await page.keyboard.press("Tab"); await expect(elastic).toHaveText("210000"); await expect(shear).toBeFocused();
  530 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialP2Gate.pending.length)).toBeGreaterThan(0);
  531 |   await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toContainText("sort unavailable"); await expect(table.getByRole("rowheader").first()).toHaveText(firstId);
  532 |   await expect(page.getByTestId("workspace-undo")).toBeEnabled();
  533 |   // No locator focus/fill repair: this is the first actual key after Apply+Tab.
  534 |   await page.keyboard.press("8");
  535 |   const firstKey = await table.evaluate((root) => ({ editorValue: root.querySelector<HTMLInputElement>("input")?.value ?? null,
  536 |     activeTag: document.activeElement?.tagName, activeColumn: (document.activeElement as HTMLElement)?.dataset.columnKey,
  537 |     shearReadonly: root.querySelector('[data-column-key="shear"]')?.parentElement?.getAttribute("aria-readonly") }));
  538 |   await info.attach("material-p2-first-key", { body: JSON.stringify(firstKey), contentType: "application/json" });
  539 |   const input = table.getByRole("textbox", { name: `${firstId} Shear [${material.shear_modulus.unit}]` });
  540 |   await expect(input).toHaveValue("8"); await expect(input).toBeFocused();
  541 |   await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  542 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([2, 2]);
  543 |   await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toContainText("sort unavailable");
  544 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(shear).toHaveText(String(material.shear_modulus.value));
  545 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialP2Gate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  546 |   await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  547 |   // Exactly one accepted model operation; cancelled Shear entry adds no history.
  548 |   await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000"); await expect(shear).toHaveText(String(material.shear_modulus.value)); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  549 | });
  550 | 
  551 | // Diagnosis only: no Node production change accompanies this regression probe.
  552 | test("B4 diagnostic Node sorted review preserves text Undo after an active row crosses downward", async ({ page, browser }, info) => {
  553 |   await attachBrowserIdentity(browser, info);
  554 |   const { model } = await readFixture("precision-origin-base.model.json");
  555 |   model.nodes.forEach((node: { position: { x: number } }, index: number) => { node.position.x = index === 0 ? 2 : index === 1 ? 15 : 100 + index; });
  556 |   const firstId = model.nodes[0].id; const secondId = model.nodes[1].id;
  557 |   await gotoModel(page, model); const hashBefore = await currentModelHashThroughVisibleExport(page);
  558 |   await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("node-grid-review-disclosure").click();
  559 |   const table = page.getByTestId("engineering-table-review"); const cell = table.getByTestId(`review-cell-${firstId}-x`);
  560 |   await table.getByRole("button", { name: "Sort X", exact: true }).click();
  561 |   await cell.focus(); await page.keyboard.press("1"); await table.getByRole("button", { name: "Keep draft", exact: true }).click();
  562 |   await expect(cell).toHaveText("1"); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("1 changed cells");
  563 |   await cell.dblclick(); const input = table.getByRole("textbox", { name: `${firstId} X [${model.project.units.length}]` }); await page.keyboard.press("ArrowRight");
  564 |   const originalInput = await input.elementHandle();
  565 |   // Read-only DOM observation distinguishes actual active-host movement from
  566 |   // reorderings that move only its neighbours; no production behavior is hooked.
  567 |   const movement = await input.evaluateHandle((input) => {
  568 |     const wrapper = input.closest("[data-virtual-index]")!; const counts = { removed: 0, added: 0 };
  569 |     const observer = new MutationObserver((records) => records.forEach((record) => {
  570 |       counts.removed += [...record.removedNodes].filter((node) => node === wrapper).length;
  571 |       counts.added += [...record.addedNodes].filter((node) => node === wrapper).length;
  572 |     }));
  573 |     observer.observe(wrapper.parentElement!, { childList: true });
  574 |     return { counts, observer };
  575 |   });
  576 |   const snapshot = () => table.evaluate((root, original) => {
  577 |     const current = root.querySelector<HTMLInputElement>("input");
  578 |     return { value: current?.value, sameInput: current === original, focused: document.activeElement === current,
  579 |       caret: current ? [current.selectionStart, current.selectionEnd] : null,
  580 |       rowOrder: [...root.querySelectorAll('[role="rowheader"] button')].map((button) => button.textContent),
  581 |       inputY: current?.getBoundingClientRect().y };
  582 |   }, originalInput);
  583 |   const before = await snapshot();
  584 |   await page.keyboard.press("9"); const afterCrossing = await snapshot();
  585 |   await page.keyboard.press("ControlOrMeta+z"); const afterUndo = await snapshot();
  586 |   const hostMoves = await movement.evaluate((state) => { state.observer.disconnect(); return state.counts; }); await movement.dispose();
  587 |   const canonicalX = await page.getByTestId(`table-cell-${firstId}-x`).textContent();
  588 |   const history = { undoDisabled: await page.getByTestId("workspace-undo").isDisabled(), redoDisabled: await page.getByTestId("workspace-redo").isDisabled(), editedMarkers: await page.getByTestId("project-edited").count() };
  589 |   await page.screenshot({ path: info.outputPath("node-sorted-review-after-undo.png") });
  590 |   // Export is passive with respect to canonical model state; it occurs only
  591 |   // after the decisive focus/caret/Undo snapshots and may close the review editor.
  592 |   const hashAfterStaging = await currentModelHashThroughVisibleExport(page);
  593 |   await page.getByTestId("clear-entity-grid-drafts").click(); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  594 |   await table.getByRole("button", { name: /Sorted by X/ }).click();
  595 |   await page.getByTestId("node-grid-review-disclosure").click();
  596 |   const canonicalAfterClear = await page.getByTestId(`table-cell-${firstId}-x`).textContent();
  597 |   const observation = { firstId, secondId, before, afterCrossing, afterUndo, hostMoves, canonicalX, canonicalAfterClear, hashBefore, hashAfterStaging, history };
  598 |   await info.attach("node-sorted-review-undo-observation", { body: JSON.stringify(observation, null, 2), contentType: "application/json" });
  599 |   // Preserve full observation and test-owned cleanup before the expected Undo
  600 |   // oracle, so a failure still proves movement and canonical non-mutation.
  601 |   expect(before.value).toBe("1"); expect(before.caret).toEqual([1, 1]); expect(before.rowOrder.slice(0, 2)).toEqual([firstId, secondId]);
  602 |   expect(afterCrossing.value).toBe("19"); expect(afterCrossing.rowOrder.slice(0, 2)).toEqual([secondId, firstId]);
  603 |   expect(hostMoves.removed).toBeGreaterThan(0); expect(hostMoves.added).toBeGreaterThan(0);
  604 |   expect(afterCrossing.sameInput).toBe(true); expect(afterCrossing.focused).toBe(true); expect(afterCrossing.caret).toEqual([2, 2]);
  605 |   expect(afterUndo.sameInput).toBe(true); expect(afterUndo.focused).toBe(true);
  606 |   expect(canonicalX).toBe("2"); expect(canonicalAfterClear).toBe("2"); expect(hashAfterStaging).toBe(hashBefore);
  607 |   expect(history).toEqual({ undoDisabled: true, redoDisabled: true, editedMarkers: 0 });
> 608 |   expect(afterUndo.value).toBe("1"); expect(afterUndo.caret).toEqual([1, 1]); expect(afterUndo.rowOrder.slice(0, 2)).toEqual([firstId, secondId]);
      |                           ^ Error: expect(received).toBe(expected) // Object.is equality
  609 | });
  610 | 
```