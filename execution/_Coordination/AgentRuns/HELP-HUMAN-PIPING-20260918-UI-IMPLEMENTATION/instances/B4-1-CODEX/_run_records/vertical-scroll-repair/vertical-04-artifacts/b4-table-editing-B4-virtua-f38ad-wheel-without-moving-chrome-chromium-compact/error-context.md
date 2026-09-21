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
              - grid "Node coordinates" [ref=e178]:
                - row "Node Sort X Sort Y Sort Z" [ref=e179]:
                  - columnheader "Node" [ref=e180]
                  - columnheader "Sort X" [ref=e181]:
                    - button "Sort X" [ref=e182]: X [m] ↕
                  - columnheader "Sort Y" [ref=e183]:
                    - button "Sort Y" [ref=e184]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e185]:
                    - button "Sort Z" [ref=e186]: Z [m] ↕
                - rowgroup [ref=e188]:
                  - generic [ref=e189]:
                    - 'row "node:UIF-00000 node:UIF-00000 X: 0 m node:UIF-00000 Y: 0 m node:UIF-00000 Z: 0 m" [ref=e191]':
                      - rowheader "node:UIF-00000" [ref=e192]:
                        - button "node:UIF-00000" [ref=e193]
                      - 'gridcell "node:UIF-00000 X: 0 m" [ref=e194]':
                        - 'button "node:UIF-00000 X: 0 m" [ref=e195]': "0"
                      - 'gridcell "node:UIF-00000 Y: 0 m" [ref=e196]':
                        - 'button "node:UIF-00000 Y: 0 m" [ref=e197]': "0"
                      - 'gridcell "node:UIF-00000 Z: 0 m" [ref=e198]':
                        - 'button "node:UIF-00000 Z: 0 m" [ref=e199]': "0"
                    - 'row "node:UIF-00001 node:UIF-00001 X: 0.1 m node:UIF-00001 Y: -0.008621 m node:UIF-00001 Z: 0.003494 m" [ref=e201]':
                      - rowheader "node:UIF-00001" [ref=e202]:
                        - button "node:UIF-00001" [ref=e203]
                      - 'gridcell "node:UIF-00001 X: 0.1 m" [ref=e204]':
                        - 'button "node:UIF-00001 X: 0.1 m" [ref=e205]': "0.1"
                      - 'gridcell "node:UIF-00001 Y: -0.008621 m" [ref=e206]':
                        - 'button "node:UIF-00001 Y: -0.008621 m" [ref=e207]': "-0.008621"
                      - 'gridcell "node:UIF-00001 Z: 0.003494 m" [ref=e208]':
                        - 'button "node:UIF-00001 Z: 0.003494 m" [ref=e209]': "0.003494"
                    - 'row "node:UIF-00002 node:UIF-00002 X: 0.2 m node:UIF-00002 Y: 0.007379 m node:UIF-00002 Z: 0.007128 m" [ref=e211]':
                      - rowheader "node:UIF-00002" [ref=e212]:
                        - button "node:UIF-00002" [ref=e213]
                      - 'gridcell "node:UIF-00002 X: 0.2 m" [ref=e214]':
                        - 'button "node:UIF-00002 X: 0.2 m" [ref=e215]': "0.2"
                      - 'gridcell "node:UIF-00002 Y: 0.007379 m" [ref=e216]':
                        - 'button "node:UIF-00002 Y: 0.007379 m" [ref=e217]': "0.007379"
                      - 'gridcell "node:UIF-00002 Z: 0.007128 m" [ref=e218]':
                        - 'button "node:UIF-00002 Z: 0.007128 m" [ref=e219]': "0.007128"
                    - 'row "node:UIF-00003 node:UIF-00003 X: 0.3 m node:UIF-00003 Y: 0.007084 m node:UIF-00003 Z: 0.001271 m" [ref=e221]':
                      - rowheader "node:UIF-00003" [ref=e222]:
                        - button "node:UIF-00003" [ref=e223]
                      - 'gridcell "node:UIF-00003 X: 0.3 m" [ref=e224]':
                        - 'button "node:UIF-00003 X: 0.3 m" [ref=e225]': "0.3"
                      - 'gridcell "node:UIF-00003 Y: 0.007084 m" [ref=e226]':
                        - 'button "node:UIF-00003 Y: 0.007084 m" [ref=e227]': "0.007084"
                      - 'gridcell "node:UIF-00003 Z: 0.001271 m" [ref=e228]':
                        - 'button "node:UIF-00003 Z: 0.001271 m" [ref=e229]': "0.001271"
                    - 'row "node:UIF-00004 node:UIF-00004 X: 0.4 m node:UIF-00004 Y: -0.008844 m node:UIF-00004 Z: -0.005178 m" [ref=e231]':
                      - rowheader "node:UIF-00004" [ref=e232]:
                        - button "node:UIF-00004" [ref=e233]
                      - 'gridcell "node:UIF-00004 X: 0.4 m" [ref=e234]':
                        - 'button "node:UIF-00004 X: 0.4 m" [ref=e235]': "0.4"
                      - 'gridcell "node:UIF-00004 Y: -0.008844 m" [ref=e236]':
                        - 'button "node:UIF-00004 Y: -0.008844 m" [ref=e237]': "-0.008844"
                      - 'gridcell "node:UIF-00004 Z: -0.005178 m" [ref=e238]':
                        - 'button "node:UIF-00004 Z: -0.005178 m" [ref=e239]': "-0.005178"
                    - 'row "node:UIF-00005 node:UIF-00005 X: 0.5 m node:UIF-00005 Y: -0.000598 m node:UIF-00005 Z: -0.007318 m" [ref=e241]':
                      - rowheader "node:UIF-00005" [ref=e242]:
                        - button "node:UIF-00005" [ref=e243]
                      - 'gridcell "node:UIF-00005 X: 0.5 m" [ref=e244]':
                        - 'button "node:UIF-00005 X: 0.5 m" [ref=e245]': "0.5"
                      - 'gridcell "node:UIF-00005 Y: -0.000598 m" [ref=e246]':
                        - 'button "node:UIF-00005 Y: -0.000598 m" [ref=e247]': "-0.000598"
                      - 'gridcell "node:UIF-00005 Z: -0.007318 m" [ref=e248]':
                        - 'button "node:UIF-00005 Z: -0.007318 m" [ref=e249]': "-0.007318"
                    - 'row "node:UIF-00006 node:UIF-00006 X: 0.6 m node:UIF-00006 Y: -0.002379 m node:UIF-00006 Z: -0.006069 m" [ref=e251]':
                      - rowheader "node:UIF-00006" [ref=e252]:
                        - button "node:UIF-00006" [ref=e253]
                      - 'gridcell "node:UIF-00006 X: 0.6 m" [ref=e254]':
                        - 'button "node:UIF-00006 X: 0.6 m" [ref=e255]': "0.6"
                      - 'gridcell "node:UIF-00006 Y: -0.002379 m" [ref=e256]':
                        - 'button "node:UIF-00006 Y: -0.002379 m" [ref=e257]': "-0.002379"
                      - 'gridcell "node:UIF-00006 Z: -0.006069 m" [ref=e258]':
                        - 'button "node:UIF-00006 Z: -0.006069 m" [ref=e259]': "-0.006069"
                    - 'row "node:UIF-00007 node:UIF-00007 X: 0.7 m node:UIF-00007 Y: -0.007963 m node:UIF-00007 Z: -0.004501 m" [ref=e261]':
                      - rowheader "node:UIF-00007" [ref=e262]:
                        - button "node:UIF-00007" [ref=e263]
                      - 'gridcell "node:UIF-00007 X: 0.7 m" [ref=e264]':
                        - 'button "node:UIF-00007 X: 0.7 m" [ref=e265]': "0.7"
                      - 'gridcell "node:UIF-00007 Y: -0.007963 m" [ref=e266]':
                        - 'button "node:UIF-00007 Y: -0.007963 m" [ref=e267]': "-0.007963"
                      - 'gridcell "node:UIF-00007 Z: -0.004501 m" [ref=e268]':
                        - 'button "node:UIF-00007 Z: -0.004501 m" [ref=e269]': "-0.004501"
                    - 'row "node:UIF-00008 node:UIF-00008 X: 0.8 m node:UIF-00008 Y: -0.008741 m node:UIF-00008 Z: 0.008924 m" [ref=e271]':
                      - rowheader "node:UIF-00008" [ref=e272]:
                        - button "node:UIF-00008" [ref=e273]
                      - 'gridcell "node:UIF-00008 X: 0.8 m" [ref=e274]':
                        - 'button "node:UIF-00008 X: 0.8 m" [ref=e275]': "0.8"
                      - 'gridcell "node:UIF-00008 Y: -0.008741 m" [ref=e276]':
                        - 'button "node:UIF-00008 Y: -0.008741 m" [ref=e277]': "-0.008741"
                      - 'gridcell "node:UIF-00008 Z: 0.008924 m" [ref=e278]':
                        - 'button "node:UIF-00008 Z: 0.008924 m" [ref=e279]': "0.008924"
                    - 'row "node:UIF-00009 node:UIF-00009 X: 0.9 m node:UIF-00009 Y: 0.005366 m node:UIF-00009 Z: -0.000018 m" [ref=e281]':
                      - rowheader "node:UIF-00009" [ref=e282]:
                        - button "node:UIF-00009" [ref=e283]
                      - 'gridcell "node:UIF-00009 X: 0.9 m" [ref=e284]':
                        - 'button "node:UIF-00009 X: 0.9 m" [ref=e285]': "0.9"
                      - 'gridcell "node:UIF-00009 Y: 0.005366 m" [ref=e286]':
                        - 'button "node:UIF-00009 Y: 0.005366 m" [ref=e287]': "0.005366"
                      - 'gridcell "node:UIF-00009 Z: -0.000018 m" [ref=e288]':
                        - 'button "node:UIF-00009 Z: -0.000018 m" [ref=e289]': "-0.000018"
                    - 'row "node:UIF-00010 node:UIF-00010 X: 1 m node:UIF-00010 Y: -0.007599 m node:UIF-00010 Z: 0.006892 m" [ref=e291]':
                      - rowheader "node:UIF-00010" [ref=e292]:
                        - button "node:UIF-00010" [ref=e293]
                      - 'gridcell "node:UIF-00010 X: 1 m" [ref=e294]':
                        - 'button "node:UIF-00010 X: 1 m" [ref=e295]': "1"
                      - 'gridcell "node:UIF-00010 Y: -0.007599 m" [ref=e296]':
                        - 'button "node:UIF-00010 Y: -0.007599 m" [ref=e297]': "-0.007599"
                      - 'gridcell "node:UIF-00010 Z: 0.006892 m" [ref=e298]':
                        - 'button "node:UIF-00010 Z: 0.006892 m" [ref=e299]': "0.006892"
                    - 'row "node:UIF-00011 node:UIF-00011 X: 1.1 m node:UIF-00011 Y: -0.00614 m node:UIF-00011 Z: -0.000131 m" [ref=e301]':
                      - rowheader "node:UIF-00011" [ref=e302]:
                        - button "node:UIF-00011" [ref=e303]
                      - 'gridcell "node:UIF-00011 X: 1.1 m" [ref=e304]':
                        - 'button "node:UIF-00011 X: 1.1 m" [ref=e305]': "1.1"
                      - 'gridcell "node:UIF-00011 Y: -0.00614 m" [ref=e306]':
                        - 'button "node:UIF-00011 Y: -0.00614 m" [ref=e307]': "-0.00614"
                      - 'gridcell "node:UIF-00011 Z: -0.000131 m" [ref=e308]':
                        - 'button "node:UIF-00011 Z: -0.000131 m" [ref=e309]': "-0.000131"
                    - 'row "node:UIF-00012 node:UIF-00012 X: 1.2 m node:UIF-00012 Y: 0.006592 m node:UIF-00012 Z: -0.003488 m" [ref=e311]':
                      - rowheader "node:UIF-00012" [ref=e312]:
                        - button "node:UIF-00012" [ref=e313]
                      - 'gridcell "node:UIF-00012 X: 1.2 m" [ref=e314]':
                        - 'button "node:UIF-00012 X: 1.2 m" [ref=e315]': "1.2"
                      - 'gridcell "node:UIF-00012 Y: 0.006592 m" [ref=e316]':
                        - 'button "node:UIF-00012 Y: 0.006592 m" [ref=e317]': "0.006592"
                      - 'gridcell "node:UIF-00012 Z: -0.003488 m" [ref=e318]':
                        - 'button "node:UIF-00012 Z: -0.003488 m" [ref=e319]': "-0.003488"
                    - 'row "node:UIF-00013 node:UIF-00013 X: 1.3 m node:UIF-00013 Y: 0.000269 m node:UIF-00013 Z: -0.005106 m" [ref=e321]':
                      - rowheader "node:UIF-00013" [ref=e322]:
                        - button "node:UIF-00013" [ref=e323]
                      - 'gridcell "node:UIF-00013 X: 1.3 m" [ref=e324]':
                        - 'button "node:UIF-00013 X: 1.3 m" [ref=e325]': "1.3"
                      - 'gridcell "node:UIF-00013 Y: 0.000269 m" [ref=e326]':
                        - 'button "node:UIF-00013 Y: 0.000269 m" [ref=e327]': "0.000269"
                      - 'gridcell "node:UIF-00013 Z: -0.005106 m" [ref=e328]':
                        - 'button "node:UIF-00013 Z: -0.005106 m" [ref=e329]': "-0.005106"
                    - 'row "node:UIF-00014 node:UIF-00014 X: 1.4 m node:UIF-00014 Y: 0.003938 m node:UIF-00014 Z: -0.008687 m" [ref=e331]':
                      - rowheader "node:UIF-00014" [ref=e332]:
                        - button "node:UIF-00014" [ref=e333]
                      - 'gridcell "node:UIF-00014 X: 1.4 m" [ref=e334]':
                        - 'button "node:UIF-00014 X: 1.4 m" [ref=e335]': "1.4"
                      - 'gridcell "node:UIF-00014 Y: 0.003938 m" [ref=e336]':
                        - 'button "node:UIF-00014 Y: 0.003938 m" [ref=e337]': "0.003938"
                      - 'gridcell "node:UIF-00014 Z: -0.008687 m" [ref=e338]':
                        - 'button "node:UIF-00014 Z: -0.008687 m" [ref=e339]': "-0.008687"
                    - 'row "node:UIF-00015 node:UIF-00015 X: 1.5 m node:UIF-00015 Y: 0.007479 m node:UIF-00015 Z: -0.003472 m" [ref=e341]':
                      - rowheader "node:UIF-00015" [ref=e342]:
                        - button "node:UIF-00015" [ref=e343]
                      - 'gridcell "node:UIF-00015 X: 1.5 m" [ref=e344]':
                        - 'button "node:UIF-00015 X: 1.5 m" [ref=e345]': "1.5"
                      - 'gridcell "node:UIF-00015 Y: 0.007479 m" [ref=e346]':
                        - 'button "node:UIF-00015 Y: 0.007479 m" [ref=e347]': "0.007479"
                      - 'gridcell "node:UIF-00015 Z: -0.003472 m" [ref=e348]':
                        - 'button "node:UIF-00015 Z: -0.003472 m" [ref=e349]': "-0.003472"
                    - 'row "node:UIF-00016 node:UIF-00016 X: 1.6 m node:UIF-00016 Y: -0.007864 m node:UIF-00016 Z: 0.0085 m" [ref=e351]':
                      - rowheader "node:UIF-00016" [ref=e352]:
                        - button "node:UIF-00016" [ref=e353]
                      - 'gridcell "node:UIF-00016 X: 1.6 m" [ref=e354]':
                        - 'button "node:UIF-00016 X: 1.6 m" [ref=e355]': "1.6"
                      - 'gridcell "node:UIF-00016 Y: -0.007864 m" [ref=e356]':
                        - 'button "node:UIF-00016 Y: -0.007864 m" [ref=e357]': "-0.007864"
                      - 'gridcell "node:UIF-00016 Z: 0.0085 m" [ref=e358]':
                        - 'button "node:UIF-00016 Z: 0.0085 m" [ref=e359]': "0.0085"
                    - 'row "node:UIF-00017 node:UIF-00017 X: 1.7 m node:UIF-00017 Y: -0.007896 m node:UIF-00017 Z: -0.00007 m" [ref=e361]':
                      - rowheader "node:UIF-00017" [ref=e362]:
                        - button "node:UIF-00017" [ref=e363]
                      - 'gridcell "node:UIF-00017 X: 1.7 m" [ref=e364]':
                        - 'button "node:UIF-00017 X: 1.7 m" [ref=e365]': "1.7"
                      - 'gridcell "node:UIF-00017 Y: -0.007896 m" [ref=e366]':
                        - 'button "node:UIF-00017 Y: -0.007896 m" [ref=e367]': "-0.007896"
                      - 'gridcell "node:UIF-00017 Z: -0.00007 m" [ref=e368]':
                        - 'button "node:UIF-00017 Z: -0.00007 m" [ref=e369]': "-0.00007"
                    - 'row "node:UIF-00018 node:UIF-00018 X: 1.8 m node:UIF-00018 Y: 0.004735 m node:UIF-00018 Z: 0.007458 m" [ref=e371]':
                      - rowheader "node:UIF-00018" [ref=e372]:
                        - button "node:UIF-00018" [ref=e373]
                      - 'gridcell "node:UIF-00018 X: 1.8 m" [ref=e374]':
                        - 'button "node:UIF-00018 X: 1.8 m" [ref=e375]': "1.8"
                      - 'gridcell "node:UIF-00018 Y: 0.004735 m" [ref=e376]':
                        - 'button "node:UIF-00018 Y: 0.004735 m" [ref=e377]': "0.004735"
                      - 'gridcell "node:UIF-00018 Z: 0.007458 m" [ref=e378]':
                        - 'button "node:UIF-00018 Z: 0.007458 m" [ref=e379]': "0.007458"
                    - 'row "node:UIF-00019 node:UIF-00019 X: 1.9 m node:UIF-00019 Y: 0.008608 m node:UIF-00019 Z: 0.007815 m" [ref=e381]':
                      - rowheader "node:UIF-00019" [ref=e382]:
                        - button "node:UIF-00019" [ref=e383]
                      - 'gridcell "node:UIF-00019 X: 1.9 m" [ref=e384]':
                        - 'button "node:UIF-00019 X: 1.9 m" [ref=e385]': "1.9"
                      - 'gridcell "node:UIF-00019 Y: 0.008608 m" [ref=e386]':
                        - 'button "node:UIF-00019 Y: 0.008608 m" [ref=e387]': "0.008608"
                      - 'gridcell "node:UIF-00019 Z: 0.007815 m" [ref=e388]':
                        - 'button "node:UIF-00019 Z: 0.007815 m" [ref=e389]': "0.007815"
                    - 'row "node:UIF-00020 node:UIF-00020 X: 2 m node:UIF-00020 Y: -0.002018 m node:UIF-00020 Z: 0.004189 m" [ref=e391]':
                      - rowheader "node:UIF-00020" [ref=e392]:
                        - button "node:UIF-00020" [ref=e393]
                      - 'gridcell "node:UIF-00020 X: 2 m" [ref=e394]':
                        - 'button "node:UIF-00020 X: 2 m" [ref=e395]': "2"
                      - 'gridcell "node:UIF-00020 Y: -0.002018 m" [ref=e396]':
                        - 'button "node:UIF-00020 Y: -0.002018 m" [ref=e397]': "-0.002018"
                      - 'gridcell "node:UIF-00020 Z: 0.004189 m" [ref=e398]':
                        - 'button "node:UIF-00020 Z: 0.004189 m" [ref=e399]': "0.004189"
                    - 'row "node:UIF-00021 node:UIF-00021 X: 2.1 m node:UIF-00021 Y: 0.007456 m node:UIF-00021 Z: 0.00827 m" [ref=e401]':
                      - rowheader "node:UIF-00021" [ref=e402]:
                        - button "node:UIF-00021" [ref=e403]
                      - 'gridcell "node:UIF-00021 X: 2.1 m" [ref=e404]':
                        - 'button "node:UIF-00021 X: 2.1 m" [ref=e405]': "2.1"
                      - 'gridcell "node:UIF-00021 Y: 0.007456 m" [ref=e406]':
                        - 'button "node:UIF-00021 Y: 0.007456 m" [ref=e407]': "0.007456"
                      - 'gridcell "node:UIF-00021 Z: 0.00827 m" [ref=e408]':
                        - 'button "node:UIF-00021 Z: 0.00827 m" [ref=e409]': "0.00827"
                    - 'row "node:UIF-00022 node:UIF-00022 X: 2.2 m node:UIF-00022 Y: 0.005498 m node:UIF-00022 Z: -0.000413 m" [ref=e411]':
                      - rowheader "node:UIF-00022" [ref=e412]:
                        - button "node:UIF-00022" [ref=e413]
                      - 'gridcell "node:UIF-00022 X: 2.2 m" [ref=e414]':
                        - 'button "node:UIF-00022 X: 2.2 m" [ref=e415]': "2.2"
                      - 'gridcell "node:UIF-00022 Y: 0.005498 m" [ref=e416]':
                        - 'button "node:UIF-00022 Y: 0.005498 m" [ref=e417]': "0.005498"
                      - 'gridcell "node:UIF-00022 Z: -0.000413 m" [ref=e418]':
                        - 'button "node:UIF-00022 Z: -0.000413 m" [ref=e419]': "-0.000413"
                    - 'row "node:UIF-00023 node:UIF-00023 X: 2.3 m node:UIF-00023 Y: 0.000262 m node:UIF-00023 Z: 0.005477 m" [ref=e421]':
                      - rowheader "node:UIF-00023" [ref=e422]:
                        - button "node:UIF-00023" [ref=e423]
                      - 'gridcell "node:UIF-00023 X: 2.3 m" [ref=e424]':
                        - 'button "node:UIF-00023 X: 2.3 m" [ref=e425]': "2.3"
                      - 'gridcell "node:UIF-00023 Y: 0.000262 m" [ref=e426]':
                        - 'button "node:UIF-00023 Y: 0.000262 m" [ref=e427]': "0.000262"
                      - 'gridcell "node:UIF-00023 Z: 0.005477 m" [ref=e428]':
                        - 'button "node:UIF-00023 Z: 0.005477 m" [ref=e429]': "0.005477"
                    - 'row "node:UIF-00024 node:UIF-00024 X: 2.4 m node:UIF-00024 Y: -0.002368 m node:UIF-00024 Z: -0.001285 m" [ref=e431]':
                      - rowheader "node:UIF-00024" [ref=e432]:
                        - button "node:UIF-00024" [ref=e433]
                      - 'gridcell "node:UIF-00024 X: 2.4 m" [ref=e434]':
                        - 'button "node:UIF-00024 X: 2.4 m" [ref=e435]': "2.4"
                      - 'gridcell "node:UIF-00024 Y: -0.002368 m" [ref=e436]':
                        - 'button "node:UIF-00024 Y: -0.002368 m" [ref=e437]': "-0.002368"
                      - 'gridcell "node:UIF-00024 Z: -0.001285 m" [ref=e438]':
                        - 'button "node:UIF-00024 Z: -0.001285 m" [ref=e439]': "-0.001285"
              - group "Node coordinates footer" [ref=e440]:
                - generic [ref=e441]: 1001 of 1001 rows
            - group [ref=e442]:
              - generic "Review multiple changes" [ref=e443] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e444]
      - generic [ref=e446]:
        - generic [ref=e447]:
          - group "Viewport controls" [ref=e448]:
            - generic [ref=e449]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e450]:
              - generic "Deformation · unavailable" [ref=e451] [cursor=pointer]
            - group "Viewport display toggles" [ref=e452]:
              - button "Labels" [pressed] [ref=e453]
              - button "Loads" [pressed] [ref=e454]
              - button "Grid" [pressed] [ref=e455]
            - group "Viewport selection tools" [ref=e456]:
              - button "Box Select" [ref=e457]
              - generic [ref=e458]:
                - generic [ref=e459]: Selection filter
                - combobox "Selection filter" [ref=e460]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e461]
              - button "Isolate" [disabled] [ref=e462]
              - button "Show All" [disabled] [ref=e463]
              - generic [ref=e464]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e465]
              - button "Fit Visible" [ref=e466]
              - button "Fit Selection" [disabled] [ref=e467]
            - group "Viewport geometry" [ref=e468]:
              - button "Schematic" [pressed] [ref=e469]
              - button "Actual OD" [ref=e470]
              - button "Measure" [ref=e471]
          - generic "Viewport status" [ref=e472]:
            - 'generic "Selected project: project:UIF-1000" [ref=e473]': "Selected: project:UIF-1000"
            - status "Schematic centerline geometry" [ref=e474]
            - status "View command status" [ref=e475]: No view command dispatched.
        - generic [ref=e476]:
          - generic "Three.js pipe centerline viewport" [ref=e477]
          - generic "Viewport entity selection":
            - button "Select UI benchmark node 00454 in viewport" [ref=e479] [cursor=pointer]:
              - img [ref=e480]
              - generic [ref=e483]: UIF-00454
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e484]:
            - button "Front" [ref=e485] [cursor=pointer]
            - button "Top" [ref=e486] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e487] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e488]:
          - generic "Object creation tools" [ref=e489]:
            - button "Node" [ref=e490] [cursor=pointer]:
              - img [ref=e491]
              - text: Node
            - button "Pipe" [ref=e493] [cursor=pointer]:
              - img [ref=e494]
              - text: Pipe
            - button "Support" [ref=e498] [cursor=pointer]:
              - img [ref=e499]
              - text: Support
            - button "Component" [ref=e502] [cursor=pointer]:
              - img [ref=e503]
              - text: Component
            - button "Load" [ref=e506] [cursor=pointer]:
              - img [ref=e507]
              - text: Load
          - generic "Model focus" [ref=e509]: Select
          - group [ref=e510]:
            - generic "Selection & navigation" [ref=e511] [cursor=pointer]
      - generic [ref=e512]:
        - button "Close inspector" [ref=e513]:
          - img [ref=e514]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e517]:
          - 'heading "Generated UI foundation 1000-pipe model — project: project:UIF-1000" [level=2] [ref=e518]':
            - text: Generated UI foundation 1000-pipe model
            - generic [ref=e519]: "— project: project:UIF-1000"
          - tablist "Inspector views" [ref=e520]:
            - tab "Properties" [selected] [ref=e521]
            - tab "Task" [ref=e522]
          - tabpanel [ref=e523]:
            - group [ref=e524]:
              - generic "All properties" [ref=e525] [cursor=pointer]
          - generic [ref=e526]:
            - group [ref=e527]:
              - generic "Sources and units" [ref=e528] [cursor=pointer]
            - group [ref=e529]:
              - generic "New support configuration" [ref=e530] [cursor=pointer]
              - text: ▾
            - group [ref=e531]:
              - generic "New section" [ref=e532] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e533]:
              - generic "New material" [ref=e534] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e535]:
              - generic "New support" [ref=e536] [cursor=pointer]
              - text: ▾
            - group [ref=e537]:
              - generic "New component" [ref=e538] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e539]:
      - button "Agent" [disabled] [ref=e541]:
        - img [ref=e542]
        - generic [ref=e545]: Agent
  - generic "Workspace status" [ref=e546]:
    - generic "Analysis statuses" [ref=e547]:
      - button "Solver · Not solved" [ref=e549] [cursor=pointer]
    - button "3 Issues" [ref=e550] [cursor=pointer]:
      - img [ref=e551]
      - text: 3 Issues
    - generic "Selection" [ref=e553]: "project: project:UIF-1000"
    - generic "Display units" [ref=e554]: Entered
    - button "About SWBPIPE…" [ref=e555] [cursor=pointer]:
      - img [ref=e556]
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