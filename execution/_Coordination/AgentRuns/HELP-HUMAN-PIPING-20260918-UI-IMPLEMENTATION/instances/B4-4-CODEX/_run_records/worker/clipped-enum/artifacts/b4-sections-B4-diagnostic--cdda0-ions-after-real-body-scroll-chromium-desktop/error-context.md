# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-sections.spec.ts >> B4 diagnostic clipped Section enum dismisses its detached options after real body scroll
- Location: e2e/b4-sections.spec.ts:149:1

# Error details

```
Error: options must not remain hitable when their editor is fully clipped

expect(received).toBe(expected) // Object.is equality

Expected: false
Received: true
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
          - button "Model" [active] [pressed] [ref=e88] [cursor=pointer]:
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
            - generic [ref=e163]: 184 of 184 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - region "Bulk entity grid" [ref=e169]:
            - generic "Grid entity type" [ref=e170]:
              - button "Nodes" [ref=e171]
              - button "Pipes" [ref=e172]
              - button "Supports" [ref=e173]
              - button "Materials" [ref=e174]
              - button "Sections" [pressed] [ref=e175]
              - button "Components" [ref=e176]
              - button "Load Cases" [ref=e177]
              - button "Combinations" [ref=e178]
            - generic [ref=e180]:
              - grid "Section fields" [ref=e181]:
                - row "Section Sort Name Sort Type Sort Outside dia. Sort Wall Sort Provenance" [ref=e182]:
                  - columnheader "Section" [ref=e183]
                  - columnheader "Sort Name" [ref=e184]:
                    - button "Sort Name" [ref=e185]: Name ↕
                  - columnheader "Sort Type" [ref=e186]:
                    - button "Sort Type" [ref=e187]: Type ↕
                  - columnheader "Sort Outside dia." [ref=e188]:
                    - button "Sort Outside dia." [ref=e189]: Outside dia. [per-row entered unit] ↕
                  - columnheader "Sort Wall" [ref=e190]:
                    - button "Sort Wall" [ref=e191]: Wall [per-row entered unit] ↕
                  - columnheader "Sort Provenance" [ref=e192]:
                    - button "Sort Provenance" [ref=e193]: Provenance ↕
                - rowgroup [ref=e195]:
                  - generic [ref=e196]:
                    - 'row "section:B4-0 section:B4-0 Name: Invented 0 section:B4-0 Type: pipe section:B4-0 Outside dia.: 2 m m Quantity readout section:B4-0 Wall: 10 mm mm Quantity readout section:B4-0 Provenance: invented Sections browser fixture" [selected] [ref=e198]':
                      - rowheader "section:B4-0" [ref=e199]:
                        - button "section:B4-0" [ref=e200]
                      - 'gridcell "section:B4-0 Name: Invented 0" [ref=e201]':
                        - 'button "section:B4-0 Name: Invented 0" [ref=e202]': Invented 0
                      - 'gridcell "section:B4-0 Type: pipe" [selected] [ref=e203]':
                        - 'button "section:B4-0 Type: pipe" [ref=e204]': pipe
                      - 'gridcell "section:B4-0 Outside dia.: 2 m m Quantity readout" [ref=e205]':
                        - 'button "section:B4-0 Outside dia.: 2 m" [ref=e206]': "2"
                        - generic "m" [ref=e207]
                        - generic "Quantity readout" [ref=e208]:
                          - generic [ref=e209]: 2 m
                      - 'gridcell "section:B4-0 Wall: 10 mm mm Quantity readout" [ref=e210]':
                        - 'button "section:B4-0 Wall: 10 mm" [ref=e211]': "10"
                        - generic "mm" [ref=e212]
                        - generic "Quantity readout" [ref=e213]:
                          - generic [ref=e214]: 10 mm
                      - 'gridcell "section:B4-0 Provenance: invented Sections browser fixture" [ref=e215]':
                        - 'button "section:B4-0 Provenance: invented Sections browser fixture" [ref=e216]': invented Sections browser fixture
                    - 'row "section:B4-1 section:B4-1 Name: Invented 1 section:B4-1 Type: pipe section:B4-1 Outside dia.: 100 mm mm Quantity readout section:B4-1 Wall: 10 mm mm Quantity readout section:B4-1 Provenance: invented Sections browser fixture" [ref=e218]':
                      - rowheader "section:B4-1" [ref=e219]:
                        - button "section:B4-1" [ref=e220]
                      - 'gridcell "section:B4-1 Name: Invented 1" [ref=e221]':
                        - 'button "section:B4-1 Name: Invented 1" [ref=e222]': Invented 1
                      - 'gridcell "section:B4-1 Type: pipe" [ref=e223]':
                        - 'button "section:B4-1 Type: pipe" [ref=e224]': pipe
                      - 'gridcell "section:B4-1 Outside dia.: 100 mm mm Quantity readout" [ref=e225]':
                        - 'button "section:B4-1 Outside dia.: 100 mm" [ref=e226]': "100"
                        - generic "mm" [ref=e227]
                        - generic "Quantity readout" [ref=e228]:
                          - generic [ref=e229]: 100 mm
                      - 'gridcell "section:B4-1 Wall: 10 mm mm Quantity readout" [ref=e230]':
                        - 'button "section:B4-1 Wall: 10 mm" [ref=e231]': "10"
                        - generic "mm" [ref=e232]
                        - generic "Quantity readout" [ref=e233]:
                          - generic [ref=e234]: 10 mm
                      - 'gridcell "section:B4-1 Provenance: invented Sections browser fixture" [ref=e235]':
                        - 'button "section:B4-1 Provenance: invented Sections browser fixture" [ref=e236]': invented Sections browser fixture
                    - 'row "section:B4-2 section:B4-2 Name: Invented 2 section:B4-2 Type: pipe section:B4-2 Outside dia.: 202 mm mm Quantity readout section:B4-2 Wall: 10 mm mm Quantity readout section:B4-2 Provenance: invented Sections browser fixture" [ref=e238]':
                      - rowheader "section:B4-2" [ref=e239]:
                        - button "section:B4-2" [ref=e240]
                      - 'gridcell "section:B4-2 Name: Invented 2" [ref=e241]':
                        - 'button "section:B4-2 Name: Invented 2" [ref=e242]': Invented 2
                      - 'gridcell "section:B4-2 Type: pipe" [ref=e243]':
                        - 'button "section:B4-2 Type: pipe" [ref=e244]': pipe
                      - 'gridcell "section:B4-2 Outside dia.: 202 mm mm Quantity readout" [ref=e245]':
                        - 'button "section:B4-2 Outside dia.: 202 mm" [ref=e246]': "202"
                        - generic "mm" [ref=e247]
                        - generic "Quantity readout" [ref=e248]:
                          - generic [ref=e249]: 202 mm
                      - 'gridcell "section:B4-2 Wall: 10 mm mm Quantity readout" [ref=e250]':
                        - 'button "section:B4-2 Wall: 10 mm" [ref=e251]': "10"
                        - generic "mm" [ref=e252]
                        - generic "Quantity readout" [ref=e253]:
                          - generic [ref=e254]: 10 mm
                      - 'gridcell "section:B4-2 Provenance: invented Sections browser fixture" [ref=e255]':
                        - 'button "section:B4-2 Provenance: invented Sections browser fixture" [ref=e256]': invented Sections browser fixture
                    - 'row "section:B4-3 section:B4-3 Name: Invented 3 section:B4-3 Type: pipe section:B4-3 Outside dia.: 203 mm mm Quantity readout section:B4-3 Wall: 10 mm mm Quantity readout section:B4-3 Provenance: invented Sections browser fixture" [ref=e258]':
                      - rowheader "section:B4-3" [ref=e259]:
                        - button "section:B4-3" [ref=e260]
                      - 'gridcell "section:B4-3 Name: Invented 3" [ref=e261]':
                        - 'button "section:B4-3 Name: Invented 3" [ref=e262]': Invented 3
                      - 'gridcell "section:B4-3 Type: pipe" [ref=e263]':
                        - 'button "section:B4-3 Type: pipe" [ref=e264]': pipe
                      - 'gridcell "section:B4-3 Outside dia.: 203 mm mm Quantity readout" [ref=e265]':
                        - 'button "section:B4-3 Outside dia.: 203 mm" [ref=e266]': "203"
                        - generic "mm" [ref=e267]
                        - generic "Quantity readout" [ref=e268]:
                          - generic [ref=e269]: 203 mm
                      - 'gridcell "section:B4-3 Wall: 10 mm mm Quantity readout" [ref=e270]':
                        - 'button "section:B4-3 Wall: 10 mm" [ref=e271]': "10"
                        - generic "mm" [ref=e272]
                        - generic "Quantity readout" [ref=e273]:
                          - generic [ref=e274]: 10 mm
                      - 'gridcell "section:B4-3 Provenance: invented Sections browser fixture" [ref=e275]':
                        - 'button "section:B4-3 Provenance: invented Sections browser fixture" [ref=e276]': invented Sections browser fixture
                    - 'row "section:B4-4 section:B4-4 Name: Invented 4 section:B4-4 Type: pipe section:B4-4 Outside dia.: 204 mm mm Quantity readout section:B4-4 Wall: 10 mm mm Quantity readout section:B4-4 Provenance: invented Sections browser fixture" [ref=e278]':
                      - rowheader "section:B4-4" [ref=e279]:
                        - button "section:B4-4" [ref=e280]
                      - 'gridcell "section:B4-4 Name: Invented 4" [ref=e281]':
                        - 'button "section:B4-4 Name: Invented 4" [ref=e282]': Invented 4
                      - 'gridcell "section:B4-4 Type: pipe" [ref=e283]':
                        - 'button "section:B4-4 Type: pipe" [ref=e284]': pipe
                      - 'gridcell "section:B4-4 Outside dia.: 204 mm mm Quantity readout" [ref=e285]':
                        - 'button "section:B4-4 Outside dia.: 204 mm" [ref=e286]': "204"
                        - generic "mm" [ref=e287]
                        - generic "Quantity readout" [ref=e288]:
                          - generic [ref=e289]: 204 mm
                      - 'gridcell "section:B4-4 Wall: 10 mm mm Quantity readout" [ref=e290]':
                        - 'button "section:B4-4 Wall: 10 mm" [ref=e291]': "10"
                        - generic "mm" [ref=e292]
                        - generic "Quantity readout" [ref=e293]:
                          - generic [ref=e294]: 10 mm
                      - 'gridcell "section:B4-4 Provenance: invented Sections browser fixture" [ref=e295]':
                        - 'button "section:B4-4 Provenance: invented Sections browser fixture" [ref=e296]': invented Sections browser fixture
                    - 'row "section:B4-5 section:B4-5 Name: Invented 5 section:B4-5 Type: pipe section:B4-5 Outside dia.: 205 mm mm Quantity readout section:B4-5 Wall: 10 mm mm Quantity readout section:B4-5 Provenance: invented Sections browser fixture" [ref=e298]':
                      - rowheader "section:B4-5" [ref=e299]:
                        - button "section:B4-5" [ref=e300]
                      - 'gridcell "section:B4-5 Name: Invented 5" [ref=e301]':
                        - 'button "section:B4-5 Name: Invented 5" [ref=e302]': Invented 5
                      - 'gridcell "section:B4-5 Type: pipe" [ref=e303]':
                        - 'button "section:B4-5 Type: pipe" [ref=e304]': pipe
                      - 'gridcell "section:B4-5 Outside dia.: 205 mm mm Quantity readout" [ref=e305]':
                        - 'button "section:B4-5 Outside dia.: 205 mm" [ref=e306]': "205"
                        - generic "mm" [ref=e307]
                        - generic "Quantity readout" [ref=e308]:
                          - generic [ref=e309]: 205 mm
                      - 'gridcell "section:B4-5 Wall: 10 mm mm Quantity readout" [ref=e310]':
                        - 'button "section:B4-5 Wall: 10 mm" [ref=e311]': "10"
                        - generic "mm" [ref=e312]
                        - generic "Quantity readout" [ref=e313]:
                          - generic [ref=e314]: 10 mm
                      - 'gridcell "section:B4-5 Provenance: invented Sections browser fixture" [ref=e315]':
                        - 'button "section:B4-5 Provenance: invented Sections browser fixture" [ref=e316]': invented Sections browser fixture
                    - 'row "section:B4-6 section:B4-6 Name: Invented 6 section:B4-6 Type: pipe section:B4-6 Outside dia.: 206 mm mm Quantity readout section:B4-6 Wall: 10 mm mm Quantity readout section:B4-6 Provenance: invented Sections browser fixture" [ref=e318]':
                      - rowheader "section:B4-6" [ref=e319]:
                        - button "section:B4-6" [ref=e320]
                      - 'gridcell "section:B4-6 Name: Invented 6" [ref=e321]':
                        - 'button "section:B4-6 Name: Invented 6" [ref=e322]': Invented 6
                      - 'gridcell "section:B4-6 Type: pipe" [ref=e323]':
                        - 'button "section:B4-6 Type: pipe" [ref=e324]': pipe
                      - 'gridcell "section:B4-6 Outside dia.: 206 mm mm Quantity readout" [ref=e325]':
                        - 'button "section:B4-6 Outside dia.: 206 mm" [ref=e326]': "206"
                        - generic "mm" [ref=e327]
                        - generic "Quantity readout" [ref=e328]:
                          - generic [ref=e329]: 206 mm
                      - 'gridcell "section:B4-6 Wall: 10 mm mm Quantity readout" [ref=e330]':
                        - 'button "section:B4-6 Wall: 10 mm" [ref=e331]': "10"
                        - generic "mm" [ref=e332]
                        - generic "Quantity readout" [ref=e333]:
                          - generic [ref=e334]: 10 mm
                      - 'gridcell "section:B4-6 Provenance: invented Sections browser fixture" [ref=e335]':
                        - 'button "section:B4-6 Provenance: invented Sections browser fixture" [ref=e336]': invented Sections browser fixture
                    - 'row "section:B4-7 section:B4-7 Name: Invented 7 section:B4-7 Type: pipe section:B4-7 Outside dia.: 207 mm mm Quantity readout section:B4-7 Wall: 10 mm mm Quantity readout section:B4-7 Provenance: invented Sections browser fixture" [ref=e338]':
                      - rowheader "section:B4-7" [ref=e339]:
                        - button "section:B4-7" [ref=e340]
                      - 'gridcell "section:B4-7 Name: Invented 7" [ref=e341]':
                        - 'button "section:B4-7 Name: Invented 7" [ref=e342]': Invented 7
                      - 'gridcell "section:B4-7 Type: pipe" [ref=e343]':
                        - 'button "section:B4-7 Type: pipe" [ref=e344]': pipe
                      - 'gridcell "section:B4-7 Outside dia.: 207 mm mm Quantity readout" [ref=e345]':
                        - 'button "section:B4-7 Outside dia.: 207 mm" [ref=e346]': "207"
                        - generic "mm" [ref=e347]
                        - generic "Quantity readout" [ref=e348]:
                          - generic [ref=e349]: 207 mm
                      - 'gridcell "section:B4-7 Wall: 10 mm mm Quantity readout" [ref=e350]':
                        - 'button "section:B4-7 Wall: 10 mm" [ref=e351]': "10"
                        - generic "mm" [ref=e352]
                        - generic "Quantity readout" [ref=e353]:
                          - generic [ref=e354]: 10 mm
                      - 'gridcell "section:B4-7 Provenance: invented Sections browser fixture" [ref=e355]':
                        - 'button "section:B4-7 Provenance: invented Sections browser fixture" [ref=e356]': invented Sections browser fixture
                    - 'row "section:B4-8 section:B4-8 Name: Invented 8 section:B4-8 Type: pipe section:B4-8 Outside dia.: 208 mm mm Quantity readout section:B4-8 Wall: 10 mm mm Quantity readout section:B4-8 Provenance: invented Sections browser fixture" [ref=e358]':
                      - rowheader "section:B4-8" [ref=e359]:
                        - button "section:B4-8" [ref=e360]
                      - 'gridcell "section:B4-8 Name: Invented 8" [ref=e361]':
                        - 'button "section:B4-8 Name: Invented 8" [ref=e362]': Invented 8
                      - 'gridcell "section:B4-8 Type: pipe" [ref=e363]':
                        - 'button "section:B4-8 Type: pipe" [ref=e364]': pipe
                      - 'gridcell "section:B4-8 Outside dia.: 208 mm mm Quantity readout" [ref=e365]':
                        - 'button "section:B4-8 Outside dia.: 208 mm" [ref=e366]': "208"
                        - generic "mm" [ref=e367]
                        - generic "Quantity readout" [ref=e368]:
                          - generic [ref=e369]: 208 mm
                      - 'gridcell "section:B4-8 Wall: 10 mm mm Quantity readout" [ref=e370]':
                        - 'button "section:B4-8 Wall: 10 mm" [ref=e371]': "10"
                        - generic "mm" [ref=e372]
                        - generic "Quantity readout" [ref=e373]:
                          - generic [ref=e374]: 10 mm
                      - 'gridcell "section:B4-8 Provenance: invented Sections browser fixture" [ref=e375]':
                        - 'button "section:B4-8 Provenance: invented Sections browser fixture" [ref=e376]': invented Sections browser fixture
                    - 'row "section:B4-9 section:B4-9 Name: Invented 9 section:B4-9 Type: pipe section:B4-9 Outside dia.: 209 mm mm Quantity readout section:B4-9 Wall: 10 mm mm Quantity readout section:B4-9 Provenance: invented Sections browser fixture" [ref=e378]':
                      - rowheader "section:B4-9" [ref=e379]:
                        - button "section:B4-9" [ref=e380]
                      - 'gridcell "section:B4-9 Name: Invented 9" [ref=e381]':
                        - 'button "section:B4-9 Name: Invented 9" [ref=e382]': Invented 9
                      - 'gridcell "section:B4-9 Type: pipe" [ref=e383]':
                        - 'button "section:B4-9 Type: pipe" [ref=e384]': pipe
                      - 'gridcell "section:B4-9 Outside dia.: 209 mm mm Quantity readout" [ref=e385]':
                        - 'button "section:B4-9 Outside dia.: 209 mm" [ref=e386]': "209"
                        - generic "mm" [ref=e387]
                        - generic "Quantity readout" [ref=e388]:
                          - generic [ref=e389]: 209 mm
                      - 'gridcell "section:B4-9 Wall: 10 mm mm Quantity readout" [ref=e390]':
                        - 'button "section:B4-9 Wall: 10 mm" [ref=e391]': "10"
                        - generic "mm" [ref=e392]
                        - generic "Quantity readout" [ref=e393]:
                          - generic [ref=e394]: 10 mm
                      - 'gridcell "section:B4-9 Provenance: invented Sections browser fixture" [ref=e395]':
                        - 'button "section:B4-9 Provenance: invented Sections browser fixture" [ref=e396]': invented Sections browser fixture
                    - 'row "section:B4-10 section:B4-10 Name: Invented 10 section:B4-10 Type: pipe section:B4-10 Outside dia.: 210 mm mm Quantity readout section:B4-10 Wall: 10 mm mm Quantity readout section:B4-10 Provenance: invented Sections browser fixture" [ref=e398]':
                      - rowheader "section:B4-10" [ref=e399]:
                        - button "section:B4-10" [ref=e400]
                      - 'gridcell "section:B4-10 Name: Invented 10" [ref=e401]':
                        - 'button "section:B4-10 Name: Invented 10" [ref=e402]': Invented 10
                      - 'gridcell "section:B4-10 Type: pipe" [ref=e403]':
                        - 'button "section:B4-10 Type: pipe" [ref=e404]': pipe
                      - 'gridcell "section:B4-10 Outside dia.: 210 mm mm Quantity readout" [ref=e405]':
                        - 'button "section:B4-10 Outside dia.: 210 mm" [ref=e406]': "210"
                        - generic "mm" [ref=e407]
                        - generic "Quantity readout" [ref=e408]:
                          - generic [ref=e409]: 210 mm
                      - 'gridcell "section:B4-10 Wall: 10 mm mm Quantity readout" [ref=e410]':
                        - 'button "section:B4-10 Wall: 10 mm" [ref=e411]': "10"
                        - generic "mm" [ref=e412]
                        - generic "Quantity readout" [ref=e413]:
                          - generic [ref=e414]: 10 mm
                      - 'gridcell "section:B4-10 Provenance: invented Sections browser fixture" [ref=e415]':
                        - 'button "section:B4-10 Provenance: invented Sections browser fixture" [ref=e416]': invented Sections browser fixture
                    - 'row "section:B4-11 section:B4-11 Name: Invented 11 section:B4-11 Type: pipe section:B4-11 Outside dia.: 211 mm mm Quantity readout section:B4-11 Wall: 10 mm mm Quantity readout section:B4-11 Provenance: invented Sections browser fixture" [ref=e418]':
                      - rowheader "section:B4-11" [ref=e419]:
                        - button "section:B4-11" [ref=e420]
                      - 'gridcell "section:B4-11 Name: Invented 11" [ref=e421]':
                        - 'button "section:B4-11 Name: Invented 11" [ref=e422]': Invented 11
                      - 'gridcell "section:B4-11 Type: pipe" [ref=e423]':
                        - 'button "section:B4-11 Type: pipe" [ref=e424]': pipe
                      - 'gridcell "section:B4-11 Outside dia.: 211 mm mm Quantity readout" [ref=e425]':
                        - 'button "section:B4-11 Outside dia.: 211 mm" [ref=e426]': "211"
                        - generic "mm" [ref=e427]
                        - generic "Quantity readout" [ref=e428]:
                          - generic [ref=e429]: 211 mm
                      - 'gridcell "section:B4-11 Wall: 10 mm mm Quantity readout" [ref=e430]':
                        - 'button "section:B4-11 Wall: 10 mm" [ref=e431]': "10"
                        - generic "mm" [ref=e432]
                        - generic "Quantity readout" [ref=e433]:
                          - generic [ref=e434]: 10 mm
                      - 'gridcell "section:B4-11 Provenance: invented Sections browser fixture" [ref=e435]':
                        - 'button "section:B4-11 Provenance: invented Sections browser fixture" [ref=e436]': invented Sections browser fixture
                    - 'row "section:B4-12 section:B4-12 Name: Invented 12 section:B4-12 Type: pipe section:B4-12 Outside dia.: 212 mm mm Quantity readout section:B4-12 Wall: 10 mm mm Quantity readout section:B4-12 Provenance: invented Sections browser fixture" [ref=e438]':
                      - rowheader "section:B4-12" [ref=e439]:
                        - button "section:B4-12" [ref=e440]
                      - 'gridcell "section:B4-12 Name: Invented 12" [ref=e441]':
                        - 'button "section:B4-12 Name: Invented 12" [ref=e442]': Invented 12
                      - 'gridcell "section:B4-12 Type: pipe" [ref=e443]':
                        - 'button "section:B4-12 Type: pipe" [ref=e444]': pipe
                      - 'gridcell "section:B4-12 Outside dia.: 212 mm mm Quantity readout" [ref=e445]':
                        - 'button "section:B4-12 Outside dia.: 212 mm" [ref=e446]': "212"
                        - generic "mm" [ref=e447]
                        - generic "Quantity readout" [ref=e448]:
                          - generic [ref=e449]: 212 mm
                      - 'gridcell "section:B4-12 Wall: 10 mm mm Quantity readout" [ref=e450]':
                        - 'button "section:B4-12 Wall: 10 mm" [ref=e451]': "10"
                        - generic "mm" [ref=e452]
                        - generic "Quantity readout" [ref=e453]:
                          - generic [ref=e454]: 10 mm
                      - 'gridcell "section:B4-12 Provenance: invented Sections browser fixture" [ref=e455]':
                        - 'button "section:B4-12 Provenance: invented Sections browser fixture" [ref=e456]': invented Sections browser fixture
                    - 'row "section:B4-13 section:B4-13 Name: Invented 13 section:B4-13 Type: pipe section:B4-13 Outside dia.: 213 mm mm Quantity readout section:B4-13 Wall: 10 mm mm Quantity readout section:B4-13 Provenance: invented Sections browser fixture" [ref=e458]':
                      - rowheader "section:B4-13" [ref=e459]:
                        - button "section:B4-13" [ref=e460]
                      - 'gridcell "section:B4-13 Name: Invented 13" [ref=e461]':
                        - 'button "section:B4-13 Name: Invented 13" [ref=e462]': Invented 13
                      - 'gridcell "section:B4-13 Type: pipe" [ref=e463]':
                        - 'button "section:B4-13 Type: pipe" [ref=e464]': pipe
                      - 'gridcell "section:B4-13 Outside dia.: 213 mm mm Quantity readout" [ref=e465]':
                        - 'button "section:B4-13 Outside dia.: 213 mm" [ref=e466]': "213"
                        - generic "mm" [ref=e467]
                        - generic "Quantity readout" [ref=e468]:
                          - generic [ref=e469]: 213 mm
                      - 'gridcell "section:B4-13 Wall: 10 mm mm Quantity readout" [ref=e470]':
                        - 'button "section:B4-13 Wall: 10 mm" [ref=e471]': "10"
                        - generic "mm" [ref=e472]
                        - generic "Quantity readout" [ref=e473]:
                          - generic [ref=e474]: 10 mm
                      - 'gridcell "section:B4-13 Provenance: invented Sections browser fixture" [ref=e475]':
                        - 'button "section:B4-13 Provenance: invented Sections browser fixture" [ref=e476]': invented Sections browser fixture
                    - 'row "section:B4-14 section:B4-14 Name: Invented 14 section:B4-14 Type: pipe section:B4-14 Outside dia.: 214 mm mm Quantity readout section:B4-14 Wall: 10 mm mm Quantity readout section:B4-14 Provenance: invented Sections browser fixture" [ref=e478]':
                      - rowheader "section:B4-14" [ref=e479]:
                        - button "section:B4-14" [ref=e480]
                      - 'gridcell "section:B4-14 Name: Invented 14" [ref=e481]':
                        - 'button "section:B4-14 Name: Invented 14" [ref=e482]': Invented 14
                      - 'gridcell "section:B4-14 Type: pipe" [ref=e483]':
                        - 'button "section:B4-14 Type: pipe" [ref=e484]': pipe
                      - 'gridcell "section:B4-14 Outside dia.: 214 mm mm Quantity readout" [ref=e485]':
                        - 'button "section:B4-14 Outside dia.: 214 mm" [ref=e486]': "214"
                        - generic "mm" [ref=e487]
                        - generic "Quantity readout" [ref=e488]:
                          - generic [ref=e489]: 214 mm
                      - 'gridcell "section:B4-14 Wall: 10 mm mm Quantity readout" [ref=e490]':
                        - 'button "section:B4-14 Wall: 10 mm" [ref=e491]': "10"
                        - generic "mm" [ref=e492]
                        - generic "Quantity readout" [ref=e493]:
                          - generic [ref=e494]: 10 mm
                      - 'gridcell "section:B4-14 Provenance: invented Sections browser fixture" [ref=e495]':
                        - 'button "section:B4-14 Provenance: invented Sections browser fixture" [ref=e496]': invented Sections browser fixture
                    - 'row "section:B4-15 section:B4-15 Name: Invented 15 section:B4-15 Type: pipe section:B4-15 Outside dia.: 215 mm mm Quantity readout section:B4-15 Wall: 10 mm mm Quantity readout section:B4-15 Provenance: invented Sections browser fixture" [ref=e498]':
                      - rowheader "section:B4-15" [ref=e499]:
                        - button "section:B4-15" [ref=e500]
                      - 'gridcell "section:B4-15 Name: Invented 15" [ref=e501]':
                        - 'button "section:B4-15 Name: Invented 15" [ref=e502]': Invented 15
                      - 'gridcell "section:B4-15 Type: pipe" [ref=e503]':
                        - 'button "section:B4-15 Type: pipe" [ref=e504]': pipe
                      - 'gridcell "section:B4-15 Outside dia.: 215 mm mm Quantity readout" [ref=e505]':
                        - 'button "section:B4-15 Outside dia.: 215 mm" [ref=e506]': "215"
                        - generic "mm" [ref=e507]
                        - generic "Quantity readout" [ref=e508]:
                          - generic [ref=e509]: 215 mm
                      - 'gridcell "section:B4-15 Wall: 10 mm mm Quantity readout" [ref=e510]':
                        - 'button "section:B4-15 Wall: 10 mm" [ref=e511]': "10"
                        - generic "mm" [ref=e512]
                        - generic "Quantity readout" [ref=e513]:
                          - generic [ref=e514]: 10 mm
                      - 'gridcell "section:B4-15 Provenance: invented Sections browser fixture" [ref=e515]':
                        - 'button "section:B4-15 Provenance: invented Sections browser fixture" [ref=e516]': invented Sections browser fixture
                    - 'row "section:B4-16 section:B4-16 Name: Invented 16 section:B4-16 Type: pipe section:B4-16 Outside dia.: 216 mm mm Quantity readout section:B4-16 Wall: 10 mm mm Quantity readout section:B4-16 Provenance: invented Sections browser fixture" [ref=e518]':
                      - rowheader "section:B4-16" [ref=e519]:
                        - button "section:B4-16" [ref=e520]
                      - 'gridcell "section:B4-16 Name: Invented 16" [ref=e521]':
                        - 'button "section:B4-16 Name: Invented 16" [ref=e522]': Invented 16
                      - 'gridcell "section:B4-16 Type: pipe" [ref=e523]':
                        - 'button "section:B4-16 Type: pipe" [ref=e524]': pipe
                      - 'gridcell "section:B4-16 Outside dia.: 216 mm mm Quantity readout" [ref=e525]':
                        - 'button "section:B4-16 Outside dia.: 216 mm" [ref=e526]': "216"
                        - generic "mm" [ref=e527]
                        - generic "Quantity readout" [ref=e528]:
                          - generic [ref=e529]: 216 mm
                      - 'gridcell "section:B4-16 Wall: 10 mm mm Quantity readout" [ref=e530]':
                        - 'button "section:B4-16 Wall: 10 mm" [ref=e531]': "10"
                        - generic "mm" [ref=e532]
                        - generic "Quantity readout" [ref=e533]:
                          - generic [ref=e534]: 10 mm
                      - 'gridcell "section:B4-16 Provenance: invented Sections browser fixture" [ref=e535]':
                        - 'button "section:B4-16 Provenance: invented Sections browser fixture" [ref=e536]': invented Sections browser fixture
                    - 'row "section:B4-17 section:B4-17 Name: Invented 17 section:B4-17 Type: pipe section:B4-17 Outside dia.: 217 mm mm Quantity readout section:B4-17 Wall: 10 mm mm Quantity readout section:B4-17 Provenance: invented Sections browser fixture" [ref=e538]':
                      - rowheader "section:B4-17" [ref=e539]:
                        - button "section:B4-17" [ref=e540]
                      - 'gridcell "section:B4-17 Name: Invented 17" [ref=e541]':
                        - 'button "section:B4-17 Name: Invented 17" [ref=e542]': Invented 17
                      - 'gridcell "section:B4-17 Type: pipe" [ref=e543]':
                        - 'button "section:B4-17 Type: pipe" [ref=e544]': pipe
                      - 'gridcell "section:B4-17 Outside dia.: 217 mm mm Quantity readout" [ref=e545]':
                        - 'button "section:B4-17 Outside dia.: 217 mm" [ref=e546]': "217"
                        - generic "mm" [ref=e547]
                        - generic "Quantity readout" [ref=e548]:
                          - generic [ref=e549]: 217 mm
                      - 'gridcell "section:B4-17 Wall: 10 mm mm Quantity readout" [ref=e550]':
                        - 'button "section:B4-17 Wall: 10 mm" [ref=e551]': "10"
                        - generic "mm" [ref=e552]
                        - generic "Quantity readout" [ref=e553]:
                          - generic [ref=e554]: 10 mm
                      - 'gridcell "section:B4-17 Provenance: invented Sections browser fixture" [ref=e555]':
                        - 'button "section:B4-17 Provenance: invented Sections browser fixture" [ref=e556]': invented Sections browser fixture
                    - 'row "section:B4-18 section:B4-18 Name: Invented 18 section:B4-18 Type: pipe section:B4-18 Outside dia.: 218 mm mm Quantity readout section:B4-18 Wall: 10 mm mm Quantity readout section:B4-18 Provenance: invented Sections browser fixture" [ref=e558]':
                      - rowheader "section:B4-18" [ref=e559]:
                        - button "section:B4-18" [ref=e560]
                      - 'gridcell "section:B4-18 Name: Invented 18" [ref=e561]':
                        - 'button "section:B4-18 Name: Invented 18" [ref=e562]': Invented 18
                      - 'gridcell "section:B4-18 Type: pipe" [ref=e563]':
                        - 'button "section:B4-18 Type: pipe" [ref=e564]': pipe
                      - 'gridcell "section:B4-18 Outside dia.: 218 mm mm Quantity readout" [ref=e565]':
                        - 'button "section:B4-18 Outside dia.: 218 mm" [ref=e566]': "218"
                        - generic "mm" [ref=e567]
                        - generic "Quantity readout" [ref=e568]:
                          - generic [ref=e569]: 218 mm
                      - 'gridcell "section:B4-18 Wall: 10 mm mm Quantity readout" [ref=e570]':
                        - 'button "section:B4-18 Wall: 10 mm" [ref=e571]': "10"
                        - generic "mm" [ref=e572]
                        - generic "Quantity readout" [ref=e573]:
                          - generic [ref=e574]: 10 mm
                      - 'gridcell "section:B4-18 Provenance: invented Sections browser fixture" [ref=e575]':
                        - 'button "section:B4-18 Provenance: invented Sections browser fixture" [ref=e576]': invented Sections browser fixture
                    - 'row "section:B4-19 section:B4-19 Name: Invented 19 section:B4-19 Type: pipe section:B4-19 Outside dia.: 219 mm mm Quantity readout section:B4-19 Wall: 10 mm mm Quantity readout section:B4-19 Provenance: invented Sections browser fixture" [ref=e578]':
                      - rowheader "section:B4-19" [ref=e579]:
                        - button "section:B4-19" [ref=e580]
                      - 'gridcell "section:B4-19 Name: Invented 19" [ref=e581]':
                        - 'button "section:B4-19 Name: Invented 19" [ref=e582]': Invented 19
                      - 'gridcell "section:B4-19 Type: pipe" [ref=e583]':
                        - 'button "section:B4-19 Type: pipe" [ref=e584]': pipe
                      - 'gridcell "section:B4-19 Outside dia.: 219 mm mm Quantity readout" [ref=e585]':
                        - 'button "section:B4-19 Outside dia.: 219 mm" [ref=e586]': "219"
                        - generic "mm" [ref=e587]
                        - generic "Quantity readout" [ref=e588]:
                          - generic [ref=e589]: 219 mm
                      - 'gridcell "section:B4-19 Wall: 10 mm mm Quantity readout" [ref=e590]':
                        - 'button "section:B4-19 Wall: 10 mm" [ref=e591]': "10"
                        - generic "mm" [ref=e592]
                        - generic "Quantity readout" [ref=e593]:
                          - generic [ref=e594]: 10 mm
                      - 'gridcell "section:B4-19 Provenance: invented Sections browser fixture" [ref=e595]':
                        - 'button "section:B4-19 Provenance: invented Sections browser fixture" [ref=e596]': invented Sections browser fixture
                    - 'row "section:B4-20 section:B4-20 Name: Invented 20 section:B4-20 Type: pipe section:B4-20 Outside dia.: 220 mm mm Quantity readout section:B4-20 Wall: 10 mm mm Quantity readout section:B4-20 Provenance: invented Sections browser fixture" [ref=e598]':
                      - rowheader "section:B4-20" [ref=e599]:
                        - button "section:B4-20" [ref=e600]
                      - 'gridcell "section:B4-20 Name: Invented 20" [ref=e601]':
                        - 'button "section:B4-20 Name: Invented 20" [ref=e602]': Invented 20
                      - 'gridcell "section:B4-20 Type: pipe" [ref=e603]':
                        - 'button "section:B4-20 Type: pipe" [ref=e604]': pipe
                      - 'gridcell "section:B4-20 Outside dia.: 220 mm mm Quantity readout" [ref=e605]':
                        - 'button "section:B4-20 Outside dia.: 220 mm" [ref=e606]': "220"
                        - generic "mm" [ref=e607]
                        - generic "Quantity readout" [ref=e608]:
                          - generic [ref=e609]: 220 mm
                      - 'gridcell "section:B4-20 Wall: 10 mm mm Quantity readout" [ref=e610]':
                        - 'button "section:B4-20 Wall: 10 mm" [ref=e611]': "10"
                        - generic "mm" [ref=e612]
                        - generic "Quantity readout" [ref=e613]:
                          - generic [ref=e614]: 10 mm
                      - 'gridcell "section:B4-20 Provenance: invented Sections browser fixture" [ref=e615]':
                        - 'button "section:B4-20 Provenance: invented Sections browser fixture" [ref=e616]': invented Sections browser fixture
                    - 'row "section:B4-21 section:B4-21 Name: Invented 21 section:B4-21 Type: pipe section:B4-21 Outside dia.: 221 mm mm Quantity readout section:B4-21 Wall: 10 mm mm Quantity readout section:B4-21 Provenance: invented Sections browser fixture" [ref=e618]':
                      - rowheader "section:B4-21" [ref=e619]:
                        - button "section:B4-21" [ref=e620]
                      - 'gridcell "section:B4-21 Name: Invented 21" [ref=e621]':
                        - 'button "section:B4-21 Name: Invented 21" [ref=e622]': Invented 21
                      - 'gridcell "section:B4-21 Type: pipe" [ref=e623]':
                        - 'button "section:B4-21 Type: pipe" [ref=e624]': pipe
                      - 'gridcell "section:B4-21 Outside dia.: 221 mm mm Quantity readout" [ref=e625]':
                        - 'button "section:B4-21 Outside dia.: 221 mm" [ref=e626]': "221"
                        - generic "mm" [ref=e627]
                        - generic "Quantity readout" [ref=e628]:
                          - generic [ref=e629]: 221 mm
                      - 'gridcell "section:B4-21 Wall: 10 mm mm Quantity readout" [ref=e630]':
                        - 'button "section:B4-21 Wall: 10 mm" [ref=e631]': "10"
                        - generic "mm" [ref=e632]
                        - generic "Quantity readout" [ref=e633]:
                          - generic [ref=e634]: 10 mm
                      - 'gridcell "section:B4-21 Provenance: invented Sections browser fixture" [ref=e635]':
                        - 'button "section:B4-21 Provenance: invented Sections browser fixture" [ref=e636]': invented Sections browser fixture
                    - 'row "section:B4-22 section:B4-22 Name: Invented 22 section:B4-22 Type: pipe section:B4-22 Outside dia.: 222 mm mm Quantity readout section:B4-22 Wall: 10 mm mm Quantity readout section:B4-22 Provenance: invented Sections browser fixture" [ref=e638]':
                      - rowheader "section:B4-22" [ref=e639]:
                        - button "section:B4-22" [ref=e640]
                      - 'gridcell "section:B4-22 Name: Invented 22" [ref=e641]':
                        - 'button "section:B4-22 Name: Invented 22" [ref=e642]': Invented 22
                      - 'gridcell "section:B4-22 Type: pipe" [ref=e643]':
                        - 'button "section:B4-22 Type: pipe" [ref=e644]': pipe
                      - 'gridcell "section:B4-22 Outside dia.: 222 mm mm Quantity readout" [ref=e645]':
                        - 'button "section:B4-22 Outside dia.: 222 mm" [ref=e646]': "222"
                        - generic "mm" [ref=e647]
                        - generic "Quantity readout" [ref=e648]:
                          - generic [ref=e649]: 222 mm
                      - 'gridcell "section:B4-22 Wall: 10 mm mm Quantity readout" [ref=e650]':
                        - 'button "section:B4-22 Wall: 10 mm" [ref=e651]': "10"
                        - generic "mm" [ref=e652]
                        - generic "Quantity readout" [ref=e653]:
                          - generic [ref=e654]: 10 mm
                      - 'gridcell "section:B4-22 Provenance: invented Sections browser fixture" [ref=e655]':
                        - 'button "section:B4-22 Provenance: invented Sections browser fixture" [ref=e656]': invented Sections browser fixture
                    - 'row "section:B4-23 section:B4-23 Name: Invented 23 section:B4-23 Type: pipe section:B4-23 Outside dia.: 223 mm mm Quantity readout section:B4-23 Wall: 10 mm mm Quantity readout section:B4-23 Provenance: invented Sections browser fixture" [ref=e658]':
                      - rowheader "section:B4-23" [ref=e659]:
                        - button "section:B4-23" [ref=e660]
                      - 'gridcell "section:B4-23 Name: Invented 23" [ref=e661]':
                        - 'button "section:B4-23 Name: Invented 23" [ref=e662]': Invented 23
                      - 'gridcell "section:B4-23 Type: pipe" [ref=e663]':
                        - 'button "section:B4-23 Type: pipe" [ref=e664]': pipe
                      - 'gridcell "section:B4-23 Outside dia.: 223 mm mm Quantity readout" [ref=e665]':
                        - 'button "section:B4-23 Outside dia.: 223 mm" [ref=e666]': "223"
                        - generic "mm" [ref=e667]
                        - generic "Quantity readout" [ref=e668]:
                          - generic [ref=e669]: 223 mm
                      - 'gridcell "section:B4-23 Wall: 10 mm mm Quantity readout" [ref=e670]':
                        - 'button "section:B4-23 Wall: 10 mm" [ref=e671]': "10"
                        - generic "mm" [ref=e672]
                        - generic "Quantity readout" [ref=e673]:
                          - generic [ref=e674]: 10 mm
                      - 'gridcell "section:B4-23 Provenance: invented Sections browser fixture" [ref=e675]':
                        - 'button "section:B4-23 Provenance: invented Sections browser fixture" [ref=e676]': invented Sections browser fixture
                    - 'row "section:B4-24 section:B4-24 Name: Invented 24 section:B4-24 Type: pipe section:B4-24 Outside dia.: 224 mm mm Quantity readout section:B4-24 Wall: 10 mm mm Quantity readout section:B4-24 Provenance: invented Sections browser fixture" [ref=e678]':
                      - rowheader "section:B4-24" [ref=e679]:
                        - button "section:B4-24" [ref=e680]
                      - 'gridcell "section:B4-24 Name: Invented 24" [ref=e681]':
                        - 'button "section:B4-24 Name: Invented 24" [ref=e682]': Invented 24
                      - 'gridcell "section:B4-24 Type: pipe" [ref=e683]':
                        - 'button "section:B4-24 Type: pipe" [ref=e684]': pipe
                      - 'gridcell "section:B4-24 Outside dia.: 224 mm mm Quantity readout" [ref=e685]':
                        - 'button "section:B4-24 Outside dia.: 224 mm" [ref=e686]': "224"
                        - generic "mm" [ref=e687]
                        - generic "Quantity readout" [ref=e688]:
                          - generic [ref=e689]: 224 mm
                      - 'gridcell "section:B4-24 Wall: 10 mm mm Quantity readout" [ref=e690]':
                        - 'button "section:B4-24 Wall: 10 mm" [ref=e691]': "10"
                        - generic "mm" [ref=e692]
                        - generic "Quantity readout" [ref=e693]:
                          - generic [ref=e694]: 10 mm
                      - 'gridcell "section:B4-24 Provenance: invented Sections browser fixture" [ref=e695]':
                        - 'button "section:B4-24 Provenance: invented Sections browser fixture" [ref=e696]': invented Sections browser fixture
                    - 'row "section:B4-25 section:B4-25 Name: Invented 25 section:B4-25 Type: pipe section:B4-25 Outside dia.: 225 mm mm Quantity readout section:B4-25 Wall: 10 mm mm Quantity readout section:B4-25 Provenance: invented Sections browser fixture" [ref=e698]':
                      - rowheader "section:B4-25" [ref=e699]:
                        - button "section:B4-25" [ref=e700]
                      - 'gridcell "section:B4-25 Name: Invented 25" [ref=e701]':
                        - 'button "section:B4-25 Name: Invented 25" [ref=e702]': Invented 25
                      - 'gridcell "section:B4-25 Type: pipe" [ref=e703]':
                        - 'button "section:B4-25 Type: pipe" [ref=e704]': pipe
                      - 'gridcell "section:B4-25 Outside dia.: 225 mm mm Quantity readout" [ref=e705]':
                        - 'button "section:B4-25 Outside dia.: 225 mm" [ref=e706]': "225"
                        - generic "mm" [ref=e707]
                        - generic "Quantity readout" [ref=e708]:
                          - generic [ref=e709]: 225 mm
                      - 'gridcell "section:B4-25 Wall: 10 mm mm Quantity readout" [ref=e710]':
                        - 'button "section:B4-25 Wall: 10 mm" [ref=e711]': "10"
                        - generic "mm" [ref=e712]
                        - generic "Quantity readout" [ref=e713]:
                          - generic [ref=e714]: 10 mm
                      - 'gridcell "section:B4-25 Provenance: invented Sections browser fixture" [ref=e715]':
                        - 'button "section:B4-25 Provenance: invented Sections browser fixture" [ref=e716]': invented Sections browser fixture
                    - 'row "section:B4-26 section:B4-26 Name: Invented 26 section:B4-26 Type: pipe section:B4-26 Outside dia.: 226 mm mm Quantity readout section:B4-26 Wall: 10 mm mm Quantity readout section:B4-26 Provenance: invented Sections browser fixture" [ref=e718]':
                      - rowheader "section:B4-26" [ref=e719]:
                        - button "section:B4-26" [ref=e720]
                      - 'gridcell "section:B4-26 Name: Invented 26" [ref=e721]':
                        - 'button "section:B4-26 Name: Invented 26" [ref=e722]': Invented 26
                      - 'gridcell "section:B4-26 Type: pipe" [ref=e723]':
                        - 'button "section:B4-26 Type: pipe" [ref=e724]': pipe
                      - 'gridcell "section:B4-26 Outside dia.: 226 mm mm Quantity readout" [ref=e725]':
                        - 'button "section:B4-26 Outside dia.: 226 mm" [ref=e726]': "226"
                        - generic "mm" [ref=e727]
                        - generic "Quantity readout" [ref=e728]:
                          - generic [ref=e729]: 226 mm
                      - 'gridcell "section:B4-26 Wall: 10 mm mm Quantity readout" [ref=e730]':
                        - 'button "section:B4-26 Wall: 10 mm" [ref=e731]': "10"
                        - generic "mm" [ref=e732]
                        - generic "Quantity readout" [ref=e733]:
                          - generic [ref=e734]: 10 mm
                      - 'gridcell "section:B4-26 Provenance: invented Sections browser fixture" [ref=e735]':
                        - 'button "section:B4-26 Provenance: invented Sections browser fixture" [ref=e736]': invented Sections browser fixture
                    - 'row "section:B4-27 section:B4-27 Name: Invented 27 section:B4-27 Type: pipe section:B4-27 Outside dia.: 227 mm mm Quantity readout section:B4-27 Wall: 10 mm mm Quantity readout section:B4-27 Provenance: invented Sections browser fixture" [ref=e738]':
                      - rowheader "section:B4-27" [ref=e739]:
                        - button "section:B4-27" [ref=e740]
                      - 'gridcell "section:B4-27 Name: Invented 27" [ref=e741]':
                        - 'button "section:B4-27 Name: Invented 27" [ref=e742]': Invented 27
                      - 'gridcell "section:B4-27 Type: pipe" [ref=e743]':
                        - 'button "section:B4-27 Type: pipe" [ref=e744]': pipe
                      - 'gridcell "section:B4-27 Outside dia.: 227 mm mm Quantity readout" [ref=e745]':
                        - 'button "section:B4-27 Outside dia.: 227 mm" [ref=e746]': "227"
                        - generic "mm" [ref=e747]
                        - generic "Quantity readout" [ref=e748]:
                          - generic [ref=e749]: 227 mm
                      - 'gridcell "section:B4-27 Wall: 10 mm mm Quantity readout" [ref=e750]':
                        - 'button "section:B4-27 Wall: 10 mm" [ref=e751]': "10"
                        - generic "mm" [ref=e752]
                        - generic "Quantity readout" [ref=e753]:
                          - generic [ref=e754]: 10 mm
                      - 'gridcell "section:B4-27 Provenance: invented Sections browser fixture" [ref=e755]':
                        - 'button "section:B4-27 Provenance: invented Sections browser fixture" [ref=e756]': invented Sections browser fixture
                    - 'row "section:B4-28 section:B4-28 Name: Invented 28 section:B4-28 Type: pipe section:B4-28 Outside dia.: 228 mm mm Quantity readout section:B4-28 Wall: 10 mm mm Quantity readout section:B4-28 Provenance: invented Sections browser fixture" [ref=e758]':
                      - rowheader "section:B4-28" [ref=e759]:
                        - button "section:B4-28" [ref=e760]
                      - 'gridcell "section:B4-28 Name: Invented 28" [ref=e761]':
                        - 'button "section:B4-28 Name: Invented 28" [ref=e762]': Invented 28
                      - 'gridcell "section:B4-28 Type: pipe" [ref=e763]':
                        - 'button "section:B4-28 Type: pipe" [ref=e764]': pipe
                      - 'gridcell "section:B4-28 Outside dia.: 228 mm mm Quantity readout" [ref=e765]':
                        - 'button "section:B4-28 Outside dia.: 228 mm" [ref=e766]': "228"
                        - generic "mm" [ref=e767]
                        - generic "Quantity readout" [ref=e768]:
                          - generic [ref=e769]: 228 mm
                      - 'gridcell "section:B4-28 Wall: 10 mm mm Quantity readout" [ref=e770]':
                        - 'button "section:B4-28 Wall: 10 mm" [ref=e771]': "10"
                        - generic "mm" [ref=e772]
                        - generic "Quantity readout" [ref=e773]:
                          - generic [ref=e774]: 10 mm
                      - 'gridcell "section:B4-28 Provenance: invented Sections browser fixture" [ref=e775]':
                        - 'button "section:B4-28 Provenance: invented Sections browser fixture" [ref=e776]': invented Sections browser fixture
                    - 'row "section:B4-29 section:B4-29 Name: Invented 29 section:B4-29 Type: pipe section:B4-29 Outside dia.: 229 mm mm Quantity readout section:B4-29 Wall: 10 mm mm Quantity readout section:B4-29 Provenance: invented Sections browser fixture" [ref=e778]':
                      - rowheader "section:B4-29" [ref=e779]:
                        - button "section:B4-29" [ref=e780]
                      - 'gridcell "section:B4-29 Name: Invented 29" [ref=e781]':
                        - 'button "section:B4-29 Name: Invented 29" [ref=e782]': Invented 29
                      - 'gridcell "section:B4-29 Type: pipe" [ref=e783]':
                        - 'button "section:B4-29 Type: pipe" [ref=e784]': pipe
                      - 'gridcell "section:B4-29 Outside dia.: 229 mm mm Quantity readout" [ref=e785]':
                        - 'button "section:B4-29 Outside dia.: 229 mm" [ref=e786]': "229"
                        - generic "mm" [ref=e787]
                        - generic "Quantity readout" [ref=e788]:
                          - generic [ref=e789]: 229 mm
                      - 'gridcell "section:B4-29 Wall: 10 mm mm Quantity readout" [ref=e790]':
                        - 'button "section:B4-29 Wall: 10 mm" [ref=e791]': "10"
                        - generic "mm" [ref=e792]
                        - generic "Quantity readout" [ref=e793]:
                          - generic [ref=e794]: 10 mm
                      - 'gridcell "section:B4-29 Provenance: invented Sections browser fixture" [ref=e795]':
                        - 'button "section:B4-29 Provenance: invented Sections browser fixture" [ref=e796]': invented Sections browser fixture
              - group "Section fields footer" [ref=e797]:
                - generic [ref=e798]: 140 of 140 rows
            - button "Review multiple changes" [ref=e799] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e800]:
      - button "Agent" [disabled] [ref=e802]:
        - img [ref=e803]
        - generic [ref=e806]: Agent
  - generic "Workspace status" [ref=e807]:
    - generic "Analysis statuses" [ref=e808]:
      - button "Solver · Not solved" [ref=e810] [cursor=pointer]
    - button "2 Issues" [ref=e811] [cursor=pointer]:
      - img [ref=e812]
      - text: 2 Issues
    - generic "Selection" [ref=e814]: "section: section:B4-0"
    - generic "Display units" [ref=e815]: Entered
    - button "About SWBPIPE…" [ref=e816] [cursor=pointer]:
      - img [ref=e817]
```

# Test source

```ts
  83  |   await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2); await page.mouse.down(); await expect(input).toHaveValue("pi");
  84  |   await page.mouse.move(box!.x + box!.width + 30, box!.y); await page.mouse.up(); await expect(input).toHaveValue("pi");
  85  |   await option.click(); await expect(input).toHaveValue("pipe"); await expect(input).toBeFocused(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  86  |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(type).toHaveText("p");
  87  |   await type.dblclick(); await input.fill("pi"); await page.keyboard.press("Tab"); await expect(type).toHaveText("pipe"); await expect(table.getByTestId("table-cell-section:B4-0-outside")).toBeFocused();
  88  |   await page.getByTestId("workspace-undo").click(); await expect(type).toHaveText("p"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  89  |   await page.getByTestId("section-grid-review-disclosure").click(); const review = page.getByTestId("section-engineering-table-review");
  90  |   await review.getByTestId("review-cell-section:B4-0-type").dblclick(); await review.getByRole("combobox").fill("TBD"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
  91  |   await expect(page.getByTestId("queue-entity-grid-intents")).toBeDisabled(); await page.getByTestId("section-grid-review-disclosure").click(); await expect(page.getByTestId("section-grid-review-disclosure")).toContainText("1 retained draft");
  92  |   await page.getByTestId("section-grid-review-disclosure").click(); await page.getByTestId("clear-entity-grid-drafts").click(); await expect(review.getByTestId("review-cell-section:B4-0-type")).toHaveText("p");
  93  | });
  94  | 
  95  | test("B4 Sections Apply Tab accepts the next actual key during unrelated quantity reconversion", async ({ page, browser }, info) => {
  96  |   await attachBrowserIdentity(browser, info); const model = await inventedSections();
  97  |   await page.route("**/src/services/displayQuantityService.ts", async (route) => {
  98  |     const response = await route.fetch(); const source = await response.text(); expect(source).toContain("export async function convertDisplayQuantities(");
  99  |     await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
  100 | export async function convertDisplayQuantities(items) {
  101 |  const result = await originalConvertDisplayQuantities(items);
  102 |  const gate = window.__b4SectionGate;
  103 |  if (gate.hold && items.some(item => item.id.includes('section') && item.id.includes('outside'))) await new Promise(resolve => gate.pending.push(resolve));
  104 |  return result;
  105 | }
  106 | ` });
  107 |   });
  108 |   await page.addInitScript(() => { (window as any).__b4SectionGate = { hold: false, pending: [] }; });
  109 |   await openSections(page, model); const table = page.getByTestId("section-engineering-table");
  110 |   const od = table.getByTestId("table-cell-section:B4-0-outside"), wall = table.getByTestId("table-cell-section:B4-0-wall");
  111 |   await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1");
  112 |   await od.dblclick(); await table.getByRole("textbox").fill("2.1"); await page.evaluate(() => { (window as any).__b4SectionGate.hold = true; }); await page.keyboard.press("Tab");
  113 |   await expect(od).toHaveText("2.1"); await expect(wall).toBeFocused(); await expect.poll(() => page.evaluate(() => (window as any).__b4SectionGate.pending.length)).toBeGreaterThan(0);
  114 |   await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toBeVisible(); await page.keyboard.press("8");
  115 |   const input = table.getByRole("textbox", { name: "section:B4-0 Wall [mm]" }); await expect(input).toHaveValue("8"); await expect(input).toBeFocused(); await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  116 |   await table.getByRole("button", { name: "Cancel", exact: true }).click();
  117 |   await page.evaluate(() => { const gate = (window as any).__b4SectionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  118 |   await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1"); await page.getByTestId("workspace-undo").click(); await expect(od).toHaveText("2"); await expect(wall).toHaveText("10"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  119 | });
  120 | 
  121 | test("B4 Sections moved review row preserves text Undo, input ownership and virtual hidden lifetimes", async ({ page, browser }, info) => {
  122 |   await attachBrowserIdentity(browser, info); const model = await inventedSections(140);
  123 |   model.sections[0].properties.outside_diameter = { value: 2, unit: "mm" }; model.sections[1].properties.outside_diameter = { value: 15, unit: "mm" };
  124 |   await openSections(page, model); const hashBefore = await currentModelHashThroughVisibleExport(page);
  125 |   await page.getByTestId("section-grid-review-disclosure").click(); const table = page.getByTestId("section-engineering-table-review"); const cell = table.getByTestId("review-cell-section:B4-0-outside");
  126 |   await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await cell.focus(); await page.keyboard.press("1"); await table.getByRole("button", { name: "Keep draft", exact: true }).click();
  127 |   await cell.dblclick(); const input = table.getByRole("textbox"); await page.keyboard.press("ArrowRight"); const original = await input.elementHandle();
  128 |   const movement = await table.evaluateHandle((root) => {
  129 |     const input = root.querySelector("input")!, row = root.querySelector('[data-testid="review-cell-section:B4-0-wall"]')!.closest("[data-virtual-index]")!;
  130 |     const counts = { rowRemoved: 0, inputRemoved: 0 }; const observer = new MutationObserver((records) => records.forEach((r) => {
  131 |       counts.rowRemoved += [...r.removedNodes].filter((n) => n === row).length; counts.inputRemoved += [...r.removedNodes].filter((n) => n === input || n.contains(input)).length;
  132 |     })); observer.observe(root, { childList: true, subtree: true }); return { counts, observer };
  133 |   });
  134 |   await page.keyboard.press("9"); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1"); await expect(input).toHaveValue("19");
  135 |   expect(await input.evaluate((node, prior) => node === prior, original)).toBe(true); await expect(input).toBeFocused();
  136 |   expect(await table.locator('[aria-owns]').count()).toBe(1); expect(await table.locator('[aria-owns]').getAttribute("aria-owns")).toBe(await input.getAttribute("id"));
  137 |   await page.keyboard.press("ControlOrMeta+z"); await expect(input).toHaveValue("1"); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-0");
  138 |   const moved = await movement.evaluate((state) => { state.observer.disconnect(); return state.counts; }); expect(moved.rowRemoved).toBeGreaterThan(0); expect(moved.inputRemoved).toBe(0);
  139 |   await input.fill("invalid"); const rows = page.getByTestId("section-engineering-table-review-rows"); await rows.hover(); await page.mouse.wheel(0, 2500); await expect(input).toHaveValue("invalid");
  140 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("section:B4-139");
  141 |   // Natural review blur Keeps the raw draft and closes the live editor. The
  142 |   // filter owns focus; clearing it reveals the retained cell without reopening.
  143 |   await expect(input).toHaveCount(0); await expect(filter).toBeFocused(); await filter.fill(""); await expect(cell).toHaveText("invalid");
  144 |   await page.getByTestId("entity-grid-type-nodes").click(); await page.getByTestId("entity-grid-type-sections").click(); await expect(cell).toHaveText("invalid");
  145 |   await page.getByTestId("clear-entity-grid-drafts").click(); await filter.fill(""); await expect(cell).toHaveText("2"); expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  146 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await movement.dispose();
  147 | });
  148 | 
  149 | test("B4 diagnostic clipped Section enum dismisses its detached options after real body scroll", async ({ page, browser }, info) => {
  150 |   await attachBrowserIdentity(browser, info); const model = await inventedSections(140);
  151 |   await openSections(page, model); const hashBefore = await currentModelHashThroughVisibleExport(page);
  152 |   const table = page.getByTestId("section-engineering-table"), rows = page.getByTestId("section-engineering-table-rows");
  153 |   await table.getByTestId("table-cell-section:B4-0-type").dblclick(); const input = table.getByRole("combobox");
  154 |   await input.fill("pi"); await input.click(); await expect(page.getByRole("listbox", { name: "Supported values" })).toBeVisible();
  155 |   const capture = () => table.evaluate((root) => {
  156 |     const editor = root.querySelector<HTMLInputElement>('input[role="combobox"]')!, body = root.querySelector('[data-testid="section-engineering-table-rows"]')!;
  157 |     const row = root.querySelector('[data-editor-anchor]')!.closest('[role="row"]')!, popup = document.querySelector('.engineering-table-enum-popup'), option = popup?.querySelector('[role="option"]');
  158 |     const rect = (node: Element | null | undefined) => node ? (() => { const r = node.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height, top: r.top, bottom: r.bottom }; })() : null;
  159 |     const e = editor.getBoundingClientRect(), b = body.getBoundingClientRect(), o = option?.getBoundingClientRect();
  160 |     return { scrollTop: body.scrollTop, editor: rect(editor), anchor: rect(root.querySelector("[data-editor-anchor]")), bodySlot: rect(root.querySelector(".engineering-table-body-slot")), grid: rect(root.querySelector('[role="grid"]')), row: rect(row), body: rect(body), popup: rect(popup), option: rect(option),
  161 |       clipPath: getComputedStyle(editor.closest('.engineering-table-editor-layer')!).clipPath,
  162 |       editorBodyIntersection: Math.max(0, Math.min(e.bottom, b.bottom) - Math.max(e.top, b.top)),
  163 |       popupVisibility: popup ? getComputedStyle(popup).visibility : null,
  164 |       optionHit: Boolean(option && o && option.contains(document.elementFromPoint(o.x + o.width / 2, o.y + o.height / 2))),
  165 |       draft: editor.value, focusedEditor: document.activeElement === editor, activeTag: document.activeElement?.tagName,
  166 |       undoDisabled: (document.querySelector('[data-testid="workspace-undo"]') as HTMLButtonElement).disabled,
  167 |       editedMarkers: document.querySelectorAll('[data-testid="project-edited"]').length };
  168 |   });
  169 |   const before = await capture(); const bodyBox = await rows.boundingBox(); expect(bodyBox).not.toBeNull();
  170 |   // Pointer motion preserves input focus; use a body location away from its popup.
  171 |   await page.mouse.move(bodyBox!.x + bodyBox!.width - 30, bodyBox!.y + bodyBox!.height / 2); await page.mouse.wheel(0, 90);
  172 |   await expect.poll(() => rows.evaluate((node) => node.scrollTop)).toBeGreaterThan(60);
  173 |   const after = await capture(); await info.attach("clipped-enum-observation", { body: JSON.stringify({ before, after }, null, 2), contentType: "application/json" });
  174 |   await page.screenshot({ path: info.outputPath("clipped-enum-after-wheel.png") });
  175 |   // External focus is intentional, after the decisive scroll snapshot. Invalid
  176 |   // direct text stays retained; no operation or prefix completion is permitted.
  177 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.click(); await expect(filter).toBeFocused(); await expect(input).toHaveValue("pi");
  178 |   await expect(page.getByRole("listbox", { name: "Supported values" })).toHaveCount(0);
  179 |   await table.getByRole("button", { name: "Cancel", exact: true }).click();
  180 |   const hashAfter = await currentModelHashThroughVisibleExport(page); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  181 |   expect(hashAfter).toBe(hashBefore); expect(before.draft).toBe("pi"); expect(after.draft).toBe("pi");
  182 |   expect(after.focusedEditor).toBe(true); expect(after.undoDisabled).toBe(true); expect(after.editedMarkers).toBe(0);
> 183 |   expect(after.editorBodyIntersection).toBe(0); expect(after.optionHit, "options must not remain hitable when their editor is fully clipped").toBe(false);
      |                                                                                                                                               ^ Error: options must not remain hitable when their editor is fully clipped
  184 |   expect(after.popup, "fully clipped editor dismisses the detached options").toBeNull();
  185 | });
  186 | 
```