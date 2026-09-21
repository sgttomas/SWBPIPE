# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 virtual Grid confines body scrolling and boundary wheel without moving chrome
- Location: e2e/b4-table-editing.spec.ts:200:1

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 0
Received: 3

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
      - button "Select" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [pressed] [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Both
    - generic [ref=e49]:
      - button "Run" [ref=e50] [cursor=pointer]:
        - img [ref=e51]
        - generic [ref=e53]: Run
      - button "Issues, 3" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "3"
    - group "Panels" [ref=e59]:
      - button "Inspector" [expanded] [ref=e61] [cursor=pointer]:
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
          - button "Issues, 3" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "3"
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
            - button "Grid" [active] [pressed] [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]
            - generic [ref=e163]: 2069 of 2069 model entities visible
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
              - grid "Node coordinates" [ref=e181]:
                - row "Node Sort X Sort Y Sort Z" [ref=e182]:
                  - columnheader "Node" [ref=e183]
                  - columnheader "Sort X" [ref=e184]:
                    - button "Sort X" [ref=e185]: X [m] ↕
                  - columnheader "Sort Y" [ref=e186]:
                    - button "Sort Y" [ref=e187]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e188]:
                    - button "Sort Z" [ref=e189]: Z [m] ↕
                - rowgroup [ref=e191]:
                  - generic [ref=e192]:
                    - 'row "node:UIF-00000 node:UIF-00000 X: 0 m node:UIF-00000 Y: 0 m node:UIF-00000 Z: 0 m" [ref=e194]':
                      - rowheader "node:UIF-00000" [ref=e195]:
                        - button "node:UIF-00000" [ref=e196]
                      - 'gridcell "node:UIF-00000 X: 0 m" [ref=e197]':
                        - 'button "node:UIF-00000 X: 0 m" [ref=e198]': "0"
                      - 'gridcell "node:UIF-00000 Y: 0 m" [ref=e199]':
                        - 'button "node:UIF-00000 Y: 0 m" [ref=e200]': "0"
                      - 'gridcell "node:UIF-00000 Z: 0 m" [ref=e201]':
                        - 'button "node:UIF-00000 Z: 0 m" [ref=e202]': "0"
                    - 'row "node:UIF-00001 node:UIF-00001 X: 0.1 m node:UIF-00001 Y: -0.008621 m node:UIF-00001 Z: 0.003494 m" [ref=e204]':
                      - rowheader "node:UIF-00001" [ref=e205]:
                        - button "node:UIF-00001" [ref=e206]
                      - 'gridcell "node:UIF-00001 X: 0.1 m" [ref=e207]':
                        - 'button "node:UIF-00001 X: 0.1 m" [ref=e208]': "0.1"
                      - 'gridcell "node:UIF-00001 Y: -0.008621 m" [ref=e209]':
                        - 'button "node:UIF-00001 Y: -0.008621 m" [ref=e210]': "-0.008621"
                      - 'gridcell "node:UIF-00001 Z: 0.003494 m" [ref=e211]':
                        - 'button "node:UIF-00001 Z: 0.003494 m" [ref=e212]': "0.003494"
                    - 'row "node:UIF-00002 node:UIF-00002 X: 0.2 m node:UIF-00002 Y: 0.007379 m node:UIF-00002 Z: 0.007128 m" [ref=e214]':
                      - rowheader "node:UIF-00002" [ref=e215]:
                        - button "node:UIF-00002" [ref=e216]
                      - 'gridcell "node:UIF-00002 X: 0.2 m" [ref=e217]':
                        - 'button "node:UIF-00002 X: 0.2 m" [ref=e218]': "0.2"
                      - 'gridcell "node:UIF-00002 Y: 0.007379 m" [ref=e219]':
                        - 'button "node:UIF-00002 Y: 0.007379 m" [ref=e220]': "0.007379"
                      - 'gridcell "node:UIF-00002 Z: 0.007128 m" [ref=e221]':
                        - 'button "node:UIF-00002 Z: 0.007128 m" [ref=e222]': "0.007128"
                    - 'row "node:UIF-00003 node:UIF-00003 X: 0.3 m node:UIF-00003 Y: 0.007084 m node:UIF-00003 Z: 0.001271 m" [ref=e224]':
                      - rowheader "node:UIF-00003" [ref=e225]:
                        - button "node:UIF-00003" [ref=e226]
                      - 'gridcell "node:UIF-00003 X: 0.3 m" [ref=e227]':
                        - 'button "node:UIF-00003 X: 0.3 m" [ref=e228]': "0.3"
                      - 'gridcell "node:UIF-00003 Y: 0.007084 m" [ref=e229]':
                        - 'button "node:UIF-00003 Y: 0.007084 m" [ref=e230]': "0.007084"
                      - 'gridcell "node:UIF-00003 Z: 0.001271 m" [ref=e231]':
                        - 'button "node:UIF-00003 Z: 0.001271 m" [ref=e232]': "0.001271"
                    - 'row "node:UIF-00004 node:UIF-00004 X: 0.4 m node:UIF-00004 Y: -0.008844 m node:UIF-00004 Z: -0.005178 m" [ref=e234]':
                      - rowheader "node:UIF-00004" [ref=e235]:
                        - button "node:UIF-00004" [ref=e236]
                      - 'gridcell "node:UIF-00004 X: 0.4 m" [ref=e237]':
                        - 'button "node:UIF-00004 X: 0.4 m" [ref=e238]': "0.4"
                      - 'gridcell "node:UIF-00004 Y: -0.008844 m" [ref=e239]':
                        - 'button "node:UIF-00004 Y: -0.008844 m" [ref=e240]': "-0.008844"
                      - 'gridcell "node:UIF-00004 Z: -0.005178 m" [ref=e241]':
                        - 'button "node:UIF-00004 Z: -0.005178 m" [ref=e242]': "-0.005178"
                    - 'row "node:UIF-00005 node:UIF-00005 X: 0.5 m node:UIF-00005 Y: -0.000598 m node:UIF-00005 Z: -0.007318 m" [ref=e244]':
                      - rowheader "node:UIF-00005" [ref=e245]:
                        - button "node:UIF-00005" [ref=e246]
                      - 'gridcell "node:UIF-00005 X: 0.5 m" [ref=e247]':
                        - 'button "node:UIF-00005 X: 0.5 m" [ref=e248]': "0.5"
                      - 'gridcell "node:UIF-00005 Y: -0.000598 m" [ref=e249]':
                        - 'button "node:UIF-00005 Y: -0.000598 m" [ref=e250]': "-0.000598"
                      - 'gridcell "node:UIF-00005 Z: -0.007318 m" [ref=e251]':
                        - 'button "node:UIF-00005 Z: -0.007318 m" [ref=e252]': "-0.007318"
                    - 'row "node:UIF-00006 node:UIF-00006 X: 0.6 m node:UIF-00006 Y: -0.002379 m node:UIF-00006 Z: -0.006069 m" [ref=e254]':
                      - rowheader "node:UIF-00006" [ref=e255]:
                        - button "node:UIF-00006" [ref=e256]
                      - 'gridcell "node:UIF-00006 X: 0.6 m" [ref=e257]':
                        - 'button "node:UIF-00006 X: 0.6 m" [ref=e258]': "0.6"
                      - 'gridcell "node:UIF-00006 Y: -0.002379 m" [ref=e259]':
                        - 'button "node:UIF-00006 Y: -0.002379 m" [ref=e260]': "-0.002379"
                      - 'gridcell "node:UIF-00006 Z: -0.006069 m" [ref=e261]':
                        - 'button "node:UIF-00006 Z: -0.006069 m" [ref=e262]': "-0.006069"
                    - 'row "node:UIF-00007 node:UIF-00007 X: 0.7 m node:UIF-00007 Y: -0.007963 m node:UIF-00007 Z: -0.004501 m" [ref=e264]':
                      - rowheader "node:UIF-00007" [ref=e265]:
                        - button "node:UIF-00007" [ref=e266]
                      - 'gridcell "node:UIF-00007 X: 0.7 m" [ref=e267]':
                        - 'button "node:UIF-00007 X: 0.7 m" [ref=e268]': "0.7"
                      - 'gridcell "node:UIF-00007 Y: -0.007963 m" [ref=e269]':
                        - 'button "node:UIF-00007 Y: -0.007963 m" [ref=e270]': "-0.007963"
                      - 'gridcell "node:UIF-00007 Z: -0.004501 m" [ref=e271]':
                        - 'button "node:UIF-00007 Z: -0.004501 m" [ref=e272]': "-0.004501"
                    - 'row "node:UIF-00008 node:UIF-00008 X: 0.8 m node:UIF-00008 Y: -0.008741 m node:UIF-00008 Z: 0.008924 m" [ref=e274]':
                      - rowheader "node:UIF-00008" [ref=e275]:
                        - button "node:UIF-00008" [ref=e276]
                      - 'gridcell "node:UIF-00008 X: 0.8 m" [ref=e277]':
                        - 'button "node:UIF-00008 X: 0.8 m" [ref=e278]': "0.8"
                      - 'gridcell "node:UIF-00008 Y: -0.008741 m" [ref=e279]':
                        - 'button "node:UIF-00008 Y: -0.008741 m" [ref=e280]': "-0.008741"
                      - 'gridcell "node:UIF-00008 Z: 0.008924 m" [ref=e281]':
                        - 'button "node:UIF-00008 Z: 0.008924 m" [ref=e282]': "0.008924"
                    - 'row "node:UIF-00009 node:UIF-00009 X: 0.9 m node:UIF-00009 Y: 0.005366 m node:UIF-00009 Z: -0.000018 m" [ref=e284]':
                      - rowheader "node:UIF-00009" [ref=e285]:
                        - button "node:UIF-00009" [ref=e286]
                      - 'gridcell "node:UIF-00009 X: 0.9 m" [ref=e287]':
                        - 'button "node:UIF-00009 X: 0.9 m" [ref=e288]': "0.9"
                      - 'gridcell "node:UIF-00009 Y: 0.005366 m" [ref=e289]':
                        - 'button "node:UIF-00009 Y: 0.005366 m" [ref=e290]': "0.005366"
                      - 'gridcell "node:UIF-00009 Z: -0.000018 m" [ref=e291]':
                        - 'button "node:UIF-00009 Z: -0.000018 m" [ref=e292]': "-0.000018"
                    - 'row "node:UIF-00010 node:UIF-00010 X: 1 m node:UIF-00010 Y: -0.007599 m node:UIF-00010 Z: 0.006892 m" [ref=e294]':
                      - rowheader "node:UIF-00010" [ref=e295]:
                        - button "node:UIF-00010" [ref=e296]
                      - 'gridcell "node:UIF-00010 X: 1 m" [ref=e297]':
                        - 'button "node:UIF-00010 X: 1 m" [ref=e298]': "1"
                      - 'gridcell "node:UIF-00010 Y: -0.007599 m" [ref=e299]':
                        - 'button "node:UIF-00010 Y: -0.007599 m" [ref=e300]': "-0.007599"
                      - 'gridcell "node:UIF-00010 Z: 0.006892 m" [ref=e301]':
                        - 'button "node:UIF-00010 Z: 0.006892 m" [ref=e302]': "0.006892"
                    - 'row "node:UIF-00011 node:UIF-00011 X: 1.1 m node:UIF-00011 Y: -0.00614 m node:UIF-00011 Z: -0.000131 m" [ref=e304]':
                      - rowheader "node:UIF-00011" [ref=e305]:
                        - button "node:UIF-00011" [ref=e306]
                      - 'gridcell "node:UIF-00011 X: 1.1 m" [ref=e307]':
                        - 'button "node:UIF-00011 X: 1.1 m" [ref=e308]': "1.1"
                      - 'gridcell "node:UIF-00011 Y: -0.00614 m" [ref=e309]':
                        - 'button "node:UIF-00011 Y: -0.00614 m" [ref=e310]': "-0.00614"
                      - 'gridcell "node:UIF-00011 Z: -0.000131 m" [ref=e311]':
                        - 'button "node:UIF-00011 Z: -0.000131 m" [ref=e312]': "-0.000131"
                    - 'row "node:UIF-00012 node:UIF-00012 X: 1.2 m node:UIF-00012 Y: 0.006592 m node:UIF-00012 Z: -0.003488 m" [ref=e314]':
                      - rowheader "node:UIF-00012" [ref=e315]:
                        - button "node:UIF-00012" [ref=e316]
                      - 'gridcell "node:UIF-00012 X: 1.2 m" [ref=e317]':
                        - 'button "node:UIF-00012 X: 1.2 m" [ref=e318]': "1.2"
                      - 'gridcell "node:UIF-00012 Y: 0.006592 m" [ref=e319]':
                        - 'button "node:UIF-00012 Y: 0.006592 m" [ref=e320]': "0.006592"
                      - 'gridcell "node:UIF-00012 Z: -0.003488 m" [ref=e321]':
                        - 'button "node:UIF-00012 Z: -0.003488 m" [ref=e322]': "-0.003488"
                    - 'row "node:UIF-00013 node:UIF-00013 X: 1.3 m node:UIF-00013 Y: 0.000269 m node:UIF-00013 Z: -0.005106 m" [ref=e324]':
                      - rowheader "node:UIF-00013" [ref=e325]:
                        - button "node:UIF-00013" [ref=e326]
                      - 'gridcell "node:UIF-00013 X: 1.3 m" [ref=e327]':
                        - 'button "node:UIF-00013 X: 1.3 m" [ref=e328]': "1.3"
                      - 'gridcell "node:UIF-00013 Y: 0.000269 m" [ref=e329]':
                        - 'button "node:UIF-00013 Y: 0.000269 m" [ref=e330]': "0.000269"
                      - 'gridcell "node:UIF-00013 Z: -0.005106 m" [ref=e331]':
                        - 'button "node:UIF-00013 Z: -0.005106 m" [ref=e332]': "-0.005106"
                    - 'row "node:UIF-00014 node:UIF-00014 X: 1.4 m node:UIF-00014 Y: 0.003938 m node:UIF-00014 Z: -0.008687 m" [ref=e334]':
                      - rowheader "node:UIF-00014" [ref=e335]:
                        - button "node:UIF-00014" [ref=e336]
                      - 'gridcell "node:UIF-00014 X: 1.4 m" [ref=e337]':
                        - 'button "node:UIF-00014 X: 1.4 m" [ref=e338]': "1.4"
                      - 'gridcell "node:UIF-00014 Y: 0.003938 m" [ref=e339]':
                        - 'button "node:UIF-00014 Y: 0.003938 m" [ref=e340]': "0.003938"
                      - 'gridcell "node:UIF-00014 Z: -0.008687 m" [ref=e341]':
                        - 'button "node:UIF-00014 Z: -0.008687 m" [ref=e342]': "-0.008687"
                    - 'row "node:UIF-00015 node:UIF-00015 X: 1.5 m node:UIF-00015 Y: 0.007479 m node:UIF-00015 Z: -0.003472 m" [ref=e344]':
                      - rowheader "node:UIF-00015" [ref=e345]:
                        - button "node:UIF-00015" [ref=e346]
                      - 'gridcell "node:UIF-00015 X: 1.5 m" [ref=e347]':
                        - 'button "node:UIF-00015 X: 1.5 m" [ref=e348]': "1.5"
                      - 'gridcell "node:UIF-00015 Y: 0.007479 m" [ref=e349]':
                        - 'button "node:UIF-00015 Y: 0.007479 m" [ref=e350]': "0.007479"
                      - 'gridcell "node:UIF-00015 Z: -0.003472 m" [ref=e351]':
                        - 'button "node:UIF-00015 Z: -0.003472 m" [ref=e352]': "-0.003472"
                    - 'row "node:UIF-00016 node:UIF-00016 X: 1.6 m node:UIF-00016 Y: -0.007864 m node:UIF-00016 Z: 0.0085 m" [ref=e354]':
                      - rowheader "node:UIF-00016" [ref=e355]:
                        - button "node:UIF-00016" [ref=e356]
                      - 'gridcell "node:UIF-00016 X: 1.6 m" [ref=e357]':
                        - 'button "node:UIF-00016 X: 1.6 m" [ref=e358]': "1.6"
                      - 'gridcell "node:UIF-00016 Y: -0.007864 m" [ref=e359]':
                        - 'button "node:UIF-00016 Y: -0.007864 m" [ref=e360]': "-0.007864"
                      - 'gridcell "node:UIF-00016 Z: 0.0085 m" [ref=e361]':
                        - 'button "node:UIF-00016 Z: 0.0085 m" [ref=e362]': "0.0085"
                    - 'row "node:UIF-00017 node:UIF-00017 X: 1.7 m node:UIF-00017 Y: -0.007896 m node:UIF-00017 Z: -0.00007 m" [ref=e364]':
                      - rowheader "node:UIF-00017" [ref=e365]:
                        - button "node:UIF-00017" [ref=e366]
                      - 'gridcell "node:UIF-00017 X: 1.7 m" [ref=e367]':
                        - 'button "node:UIF-00017 X: 1.7 m" [ref=e368]': "1.7"
                      - 'gridcell "node:UIF-00017 Y: -0.007896 m" [ref=e369]':
                        - 'button "node:UIF-00017 Y: -0.007896 m" [ref=e370]': "-0.007896"
                      - 'gridcell "node:UIF-00017 Z: -0.00007 m" [ref=e371]':
                        - 'button "node:UIF-00017 Z: -0.00007 m" [ref=e372]': "-0.00007"
                    - 'row "node:UIF-00018 node:UIF-00018 X: 1.8 m node:UIF-00018 Y: 0.004735 m node:UIF-00018 Z: 0.007458 m" [ref=e374]':
                      - rowheader "node:UIF-00018" [ref=e375]:
                        - button "node:UIF-00018" [ref=e376]
                      - 'gridcell "node:UIF-00018 X: 1.8 m" [ref=e377]':
                        - 'button "node:UIF-00018 X: 1.8 m" [ref=e378]': "1.8"
                      - 'gridcell "node:UIF-00018 Y: 0.004735 m" [ref=e379]':
                        - 'button "node:UIF-00018 Y: 0.004735 m" [ref=e380]': "0.004735"
                      - 'gridcell "node:UIF-00018 Z: 0.007458 m" [ref=e381]':
                        - 'button "node:UIF-00018 Z: 0.007458 m" [ref=e382]': "0.007458"
                    - 'row "node:UIF-00019 node:UIF-00019 X: 1.9 m node:UIF-00019 Y: 0.008608 m node:UIF-00019 Z: 0.007815 m" [ref=e384]':
                      - rowheader "node:UIF-00019" [ref=e385]:
                        - button "node:UIF-00019" [ref=e386]
                      - 'gridcell "node:UIF-00019 X: 1.9 m" [ref=e387]':
                        - 'button "node:UIF-00019 X: 1.9 m" [ref=e388]': "1.9"
                      - 'gridcell "node:UIF-00019 Y: 0.008608 m" [ref=e389]':
                        - 'button "node:UIF-00019 Y: 0.008608 m" [ref=e390]': "0.008608"
                      - 'gridcell "node:UIF-00019 Z: 0.007815 m" [ref=e391]':
                        - 'button "node:UIF-00019 Z: 0.007815 m" [ref=e392]': "0.007815"
                    - 'row "node:UIF-00020 node:UIF-00020 X: 2 m node:UIF-00020 Y: -0.002018 m node:UIF-00020 Z: 0.004189 m" [ref=e394]':
                      - rowheader "node:UIF-00020" [ref=e395]:
                        - button "node:UIF-00020" [ref=e396]
                      - 'gridcell "node:UIF-00020 X: 2 m" [ref=e397]':
                        - 'button "node:UIF-00020 X: 2 m" [ref=e398]': "2"
                      - 'gridcell "node:UIF-00020 Y: -0.002018 m" [ref=e399]':
                        - 'button "node:UIF-00020 Y: -0.002018 m" [ref=e400]': "-0.002018"
                      - 'gridcell "node:UIF-00020 Z: 0.004189 m" [ref=e401]':
                        - 'button "node:UIF-00020 Z: 0.004189 m" [ref=e402]': "0.004189"
                    - 'row "node:UIF-00021 node:UIF-00021 X: 2.1 m node:UIF-00021 Y: 0.007456 m node:UIF-00021 Z: 0.00827 m" [ref=e404]':
                      - rowheader "node:UIF-00021" [ref=e405]:
                        - button "node:UIF-00021" [ref=e406]
                      - 'gridcell "node:UIF-00021 X: 2.1 m" [ref=e407]':
                        - 'button "node:UIF-00021 X: 2.1 m" [ref=e408]': "2.1"
                      - 'gridcell "node:UIF-00021 Y: 0.007456 m" [ref=e409]':
                        - 'button "node:UIF-00021 Y: 0.007456 m" [ref=e410]': "0.007456"
                      - 'gridcell "node:UIF-00021 Z: 0.00827 m" [ref=e411]':
                        - 'button "node:UIF-00021 Z: 0.00827 m" [ref=e412]': "0.00827"
                    - 'row "node:UIF-00022 node:UIF-00022 X: 2.2 m node:UIF-00022 Y: 0.005498 m node:UIF-00022 Z: -0.000413 m" [ref=e414]':
                      - rowheader "node:UIF-00022" [ref=e415]:
                        - button "node:UIF-00022" [ref=e416]
                      - 'gridcell "node:UIF-00022 X: 2.2 m" [ref=e417]':
                        - 'button "node:UIF-00022 X: 2.2 m" [ref=e418]': "2.2"
                      - 'gridcell "node:UIF-00022 Y: 0.005498 m" [ref=e419]':
                        - 'button "node:UIF-00022 Y: 0.005498 m" [ref=e420]': "0.005498"
                      - 'gridcell "node:UIF-00022 Z: -0.000413 m" [ref=e421]':
                        - 'button "node:UIF-00022 Z: -0.000413 m" [ref=e422]': "-0.000413"
                    - 'row "node:UIF-00023 node:UIF-00023 X: 2.3 m node:UIF-00023 Y: 0.000262 m node:UIF-00023 Z: 0.005477 m" [ref=e424]':
                      - rowheader "node:UIF-00023" [ref=e425]:
                        - button "node:UIF-00023" [ref=e426]
                      - 'gridcell "node:UIF-00023 X: 2.3 m" [ref=e427]':
                        - 'button "node:UIF-00023 X: 2.3 m" [ref=e428]': "2.3"
                      - 'gridcell "node:UIF-00023 Y: 0.000262 m" [ref=e429]':
                        - 'button "node:UIF-00023 Y: 0.000262 m" [ref=e430]': "0.000262"
                      - 'gridcell "node:UIF-00023 Z: 0.005477 m" [ref=e431]':
                        - 'button "node:UIF-00023 Z: 0.005477 m" [ref=e432]': "0.005477"
                    - 'row "node:UIF-00024 node:UIF-00024 X: 2.4 m node:UIF-00024 Y: -0.002368 m node:UIF-00024 Z: -0.001285 m" [ref=e434]':
                      - rowheader "node:UIF-00024" [ref=e435]:
                        - button "node:UIF-00024" [ref=e436]
                      - 'gridcell "node:UIF-00024 X: 2.4 m" [ref=e437]':
                        - 'button "node:UIF-00024 X: 2.4 m" [ref=e438]': "2.4"
                      - 'gridcell "node:UIF-00024 Y: -0.002368 m" [ref=e439]':
                        - 'button "node:UIF-00024 Y: -0.002368 m" [ref=e440]': "-0.002368"
                      - 'gridcell "node:UIF-00024 Z: -0.001285 m" [ref=e441]':
                        - 'button "node:UIF-00024 Z: -0.001285 m" [ref=e442]': "-0.001285"
                    - 'row "node:UIF-00025 node:UIF-00025 X: 2.5 m node:UIF-00025 Y: -0.004322 m node:UIF-00025 Z: 0.005539 m" [ref=e444]':
                      - rowheader "node:UIF-00025" [ref=e445]:
                        - button "node:UIF-00025" [ref=e446]
                      - 'gridcell "node:UIF-00025 X: 2.5 m" [ref=e447]':
                        - 'button "node:UIF-00025 X: 2.5 m" [ref=e448]': "2.5"
                      - 'gridcell "node:UIF-00025 Y: -0.004322 m" [ref=e449]':
                        - 'button "node:UIF-00025 Y: -0.004322 m" [ref=e450]': "-0.004322"
                      - 'gridcell "node:UIF-00025 Z: 0.005539 m" [ref=e451]':
                        - 'button "node:UIF-00025 Z: 0.005539 m" [ref=e452]': "0.005539"
                    - 'row "node:UIF-00026 node:UIF-00026 X: 2.6 m node:UIF-00026 Y: -0.002013 m node:UIF-00026 Z: 0.005263 m" [ref=e454]':
                      - rowheader "node:UIF-00026" [ref=e455]:
                        - button "node:UIF-00026" [ref=e456]
                      - 'gridcell "node:UIF-00026 X: 2.6 m" [ref=e457]':
                        - 'button "node:UIF-00026 X: 2.6 m" [ref=e458]': "2.6"
                      - 'gridcell "node:UIF-00026 Y: -0.002013 m" [ref=e459]':
                        - 'button "node:UIF-00026 Y: -0.002013 m" [ref=e460]': "-0.002013"
                      - 'gridcell "node:UIF-00026 Z: 0.005263 m" [ref=e461]':
                        - 'button "node:UIF-00026 Z: 0.005263 m" [ref=e462]': "0.005263"
                    - 'row "node:UIF-00027 node:UIF-00027 X: 2.7 m node:UIF-00027 Y: 0.003404 m node:UIF-00027 Z: 0.002876 m" [ref=e464]':
                      - rowheader "node:UIF-00027" [ref=e465]:
                        - button "node:UIF-00027" [ref=e466]
                      - 'gridcell "node:UIF-00027 X: 2.7 m" [ref=e467]':
                        - 'button "node:UIF-00027 X: 2.7 m" [ref=e468]': "2.7"
                      - 'gridcell "node:UIF-00027 Y: 0.003404 m" [ref=e469]':
                        - 'button "node:UIF-00027 Y: 0.003404 m" [ref=e470]': "0.003404"
                      - 'gridcell "node:UIF-00027 Z: 0.002876 m" [ref=e471]':
                        - 'button "node:UIF-00027 Z: 0.002876 m" [ref=e472]': "0.002876"
                    - 'row "node:UIF-00028 node:UIF-00028 X: 2.8 m node:UIF-00028 Y: -0.000109 m node:UIF-00028 Z: 0.004767 m" [ref=e474]':
                      - rowheader "node:UIF-00028" [ref=e475]:
                        - button "node:UIF-00028" [ref=e476]
                      - 'gridcell "node:UIF-00028 X: 2.8 m" [ref=e477]':
                        - 'button "node:UIF-00028 X: 2.8 m" [ref=e478]': "2.8"
                      - 'gridcell "node:UIF-00028 Y: -0.000109 m" [ref=e479]':
                        - 'button "node:UIF-00028 Y: -0.000109 m" [ref=e480]': "-0.000109"
                      - 'gridcell "node:UIF-00028 Z: 0.004767 m" [ref=e481]':
                        - 'button "node:UIF-00028 Z: 0.004767 m" [ref=e482]': "0.004767"
              - group "Node coordinates footer" [ref=e483]:
                - generic [ref=e484]: 1001 of 1001 rows
            - group [ref=e485]:
              - generic "Review multiple changes" [ref=e486] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e487]
      - generic [ref=e489]:
        - generic [ref=e490]:
          - group "Viewport controls" [ref=e491]:
            - generic [ref=e492]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e493]:
              - generic "Deformation · unavailable" [ref=e494] [cursor=pointer]
            - group "Viewport display toggles" [ref=e495]:
              - button "Labels" [pressed] [ref=e496]
              - button "Loads" [pressed] [ref=e497]
              - button "Grid" [pressed] [ref=e498]
            - group "Viewport selection tools" [ref=e499]:
              - button "Box Select" [ref=e500]
              - generic [ref=e501]:
                - generic [ref=e502]: Selection filter
                - combobox "Selection filter" [ref=e503]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e504]
              - button "Isolate" [disabled] [ref=e505]
              - button "Show All" [disabled] [ref=e506]
              - generic [ref=e507]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e508]
              - button "Fit Visible" [ref=e509]
              - button "Fit Selection" [disabled] [ref=e510]
            - group "Viewport geometry" [ref=e511]:
              - button "Schematic" [pressed] [ref=e512]
              - button "Actual OD" [ref=e513]
              - button "Measure" [ref=e514]
          - generic "Viewport status" [ref=e515]:
            - 'generic "Selected project: project:UIF-1000" [ref=e516]': "Selected: project:UIF-1000"
            - status "Schematic centerline geometry" [ref=e517]
            - status "View command status" [ref=e518]: No view command dispatched.
        - generic [ref=e519]:
          - generic "Three.js pipe centerline viewport" [ref=e520]
          - generic "Viewport entity selection":
            - button "Select UI benchmark node 00454 in viewport" [ref=e522] [cursor=pointer]:
              - img [ref=e523]
              - generic [ref=e526]: UIF-00454
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e527]:
            - button "Front" [ref=e528] [cursor=pointer]
            - button "Top" [ref=e529] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e530] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e531]:
          - generic "Object creation tools" [ref=e532]:
            - button "Node" [ref=e533] [cursor=pointer]:
              - img [ref=e534]
              - text: Node
            - button "Pipe" [ref=e536] [cursor=pointer]:
              - img [ref=e537]
              - text: Pipe
            - button "Support" [ref=e541] [cursor=pointer]:
              - img [ref=e542]
              - text: Support
            - button "Component" [ref=e545] [cursor=pointer]:
              - img [ref=e546]
              - text: Component
            - button "Load" [ref=e549] [cursor=pointer]:
              - img [ref=e550]
              - text: Load
          - generic "Model focus" [ref=e552]: Select
          - group [ref=e553]:
            - generic "Selection & navigation" [ref=e554] [cursor=pointer]
      - generic [ref=e555]:
        - button "Close inspector" [ref=e556]:
          - img [ref=e557]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e560]:
          - 'heading "Generated UI foundation 1000-pipe model — project: project:UIF-1000" [level=2] [ref=e561]':
            - text: Generated UI foundation 1000-pipe model
            - generic [ref=e562]: "— project: project:UIF-1000"
          - tablist "Inspector views" [ref=e563]:
            - tab "Properties" [selected] [ref=e564]
            - tab "Task" [ref=e565]
          - tabpanel [ref=e566]:
            - group [ref=e567]:
              - generic "All properties" [ref=e568] [cursor=pointer]
          - generic [ref=e569]:
            - group [ref=e570]:
              - generic "Sources and units" [ref=e571] [cursor=pointer]
            - group [ref=e572]:
              - generic "New support configuration" [ref=e573] [cursor=pointer]
              - text: ▾
            - group [ref=e574]:
              - generic "New section" [ref=e575] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e576]:
              - generic "New material" [ref=e577] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e578]:
              - generic "New support" [ref=e579] [cursor=pointer]
              - text: ▾
            - group [ref=e580]:
              - generic "New component" [ref=e581] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e582]:
      - button "Agent" [disabled] [ref=e584]:
        - img [ref=e585]
        - generic [ref=e588]: Agent
  - generic "Workspace status" [ref=e589]:
    - generic "Analysis statuses" [ref=e590]:
      - button "Solver · Not solved" [ref=e592] [cursor=pointer]
    - button "3 Issues" [ref=e593] [cursor=pointer]:
      - img [ref=e594]
      - text: 3 Issues
    - generic "Selection" [ref=e596]: "project: project:UIF-1000"
    - generic "Display units" [ref=e597]: Entered
    - button "About SWBPIPE…" [ref=e598] [cursor=pointer]:
      - img [ref=e599]
```

# Test source

```ts
  109 |     expect(after[name].x).toBeGreaterThanOrEqual(after.pane.x); expect(after[name].right).toBeLessThanOrEqual(after.pane.right);
  110 |   }
  111 |   expect(after.header.width).toBe(after.body.width);
  112 |   const z = page.getByTestId("table-cell-node:N-100-z");
  113 |   const headerZ = await grid.getByRole("columnheader").last().boundingBox(); const bodyZ = await z.locator("..").boundingBox();
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
  147 |       const slot = root.querySelector<HTMLElement>(scroller === "engineering-table-rows" ? ".engineering-table-body-slot" : ".bulk-grid-body-slot")!;
  148 |       return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
  149 |         filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
  150 |         familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
  151 |         footerY: root.querySelector(scroller === "engineering-table-rows" ? ".engineering-table-footer" : ".entity-grid-actions")!.getBoundingClientRect().y };
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
  187 |   expect(await page.locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  188 |   const errorState = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  189 |   const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  190 |   await expect(toggle).toContainText("Return to node coordinates"); const bulk = page.getByTestId("entity-grid-input-node:N-100-y"); await bulk.fill("0.5");
  191 |   const reviewState = await gridChromeBounds(page, true); await page.locator(".entity-grid-scroll").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  192 |   await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  193 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveValue("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  194 |   await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  195 |   await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  196 |   await info.attach("vertical-fixed-rectangles", { body: JSON.stringify({ before, errorState, reviewState, wheelEvidence }, null, 2), contentType: "application/json" });
  197 |   await page.screenshot({ path: info.outputPath("b4-short-fixed-chrome.png") });
  198 | });
  199 | 
  200 | test("B4 virtual Grid confines body scrolling and boundary wheel without moving chrome", async ({ page, browser }, info) => {
  201 |   await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json"); await openBoundedGrid(page);
  202 |   const wheelEvidence: unknown[] = [];
  203 |   const before = await gridChromeBounds(page); const rows = page.getByTestId("engineering-table-rows");
  204 |   const scrollState = () => rows.evaluate((node) => ({ top: node.scrollTop, maximum: node.scrollHeight - node.clientHeight, height: node.clientHeight }));
  205 |   expect((await scrollState()).height).toBeGreaterThan(0);
  206 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  207 |   wheelEvidence.push(await tableWheel(page, 1000000, info, true)); await expect.poll(async () => { const state = await scrollState(); return state.maximum - state.top; }).toBe(0);
  208 |   wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
> 209 |   wheelEvidence.push(await tableWheel(page, -1000000, info, true)); await expect.poll(async () => (await scrollState()).top).toBe(0); wheelEvidence.push(await tableWheel(page, -600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
      |                                                                                                                              ^ Error: expect(received).toBe(expected) // Object.is equality
  210 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  211 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id);
  212 |   await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  213 |   await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  214 |   await info.attach("vertical-wheel-steps", { body: JSON.stringify({ before, filtered, restored, wheelEvidence }, null, 2), contentType: "application/json" });
  215 |   await page.screenshot({ path: info.outputPath("b4-virtual-fixed-chrome.png") });
  216 | });
  217 | 
```