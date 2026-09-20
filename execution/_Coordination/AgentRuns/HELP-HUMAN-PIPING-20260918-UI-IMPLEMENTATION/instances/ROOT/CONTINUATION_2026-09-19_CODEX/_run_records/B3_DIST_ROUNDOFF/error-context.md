# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation-dist.spec.ts >> decorative viewport overlays pass real canvas gestures while view controls stay interactive
- Location: e2e/ui-foundation-dist.spec.ts:1604:1

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 1

  Array [
    1.2123652737750505,
    -0.12527227124850754,
-   -0.027936806875621,
+   -0.027936806875620994,
  ]
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
      - button "Issues, 2" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "2"
    - group "Panels" [ref=e59]:
      - button "Inspector" [ref=e61] [cursor=pointer]:
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
            - button "Tree" [pressed] [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [ref=e153]:
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
          - tree "Model" [ref=e168]:
            - generic [ref=e169]:
              - treeitem "Generated UI local-render-origin precision probe project:UIF-PRECISION-ORIGIN" [level=1] [ref=e171] [cursor=pointer]:
                - img [ref=e172]
                - generic [ref=e176]:
                  - strong [ref=e177]: Generated UI local-render-origin precision probe
                  - generic [ref=e178]: project:UIF-PRECISION-ORIGIN
              - treeitem "Materials" [expanded] [level=1] [ref=e180] [cursor=pointer]:
                - generic [ref=e181]: ▾
                - strong [ref=e182]: Materials
              - treeitem "Invented UI benchmark elastic material material:UIF-INVENTED-01" [level=2] [ref=e184] [cursor=pointer]:
                - img [ref=e185]
                - generic [ref=e188]:
                  - strong [ref=e189]: Invented UI benchmark elastic material
                  - generic [ref=e190]: material:UIF-INVENTED-01
              - treeitem "Sections" [expanded] [level=1] [ref=e192] [cursor=pointer]:
                - generic [ref=e193]: ▾
                - strong [ref=e194]: Sections
              - treeitem "Invented OD 60 mm section:UIF-OD-060" [level=2] [ref=e196] [cursor=pointer]:
                - img [ref=e197]
                - generic [ref=e200]:
                  - strong [ref=e201]: Invented OD 60 mm
                  - generic [ref=e202]: section:UIF-OD-060
              - treeitem "Invented OD 90 mm section:UIF-OD-090" [level=2] [ref=e204] [cursor=pointer]:
                - img [ref=e205]
                - generic [ref=e208]:
                  - strong [ref=e209]: Invented OD 90 mm
                  - generic [ref=e210]: section:UIF-OD-090
              - treeitem "Invented OD 120 mm section:UIF-OD-120" [level=2] [ref=e212] [cursor=pointer]:
                - img [ref=e213]
                - generic [ref=e216]:
                  - strong [ref=e217]: Invented OD 120 mm
                  - generic [ref=e218]: section:UIF-OD-120
              - treeitem "Invented OD 180 mm section:UIF-OD-180" [level=2] [ref=e220] [cursor=pointer]:
                - img [ref=e221]
                - generic [ref=e224]:
                  - strong [ref=e225]: Invented OD 180 mm
                  - generic [ref=e226]: section:UIF-OD-180
              - treeitem "Nodes" [expanded] [level=1] [ref=e228] [cursor=pointer]:
                - generic [ref=e229]: ▾
                - strong [ref=e230]: Nodes
              - treeitem "UI benchmark node 00000 node:UIF-PRECISION-00000" [level=2] [ref=e232] [cursor=pointer]:
                - img [ref=e233]
                - generic [ref=e236]:
                  - strong [ref=e237]: UI benchmark node 00000
                  - generic [ref=e238]: node:UIF-PRECISION-00000
              - treeitem "UI benchmark node 00001 node:UIF-PRECISION-00001" [level=2] [ref=e240] [cursor=pointer]:
                - img [ref=e241]
                - generic [ref=e244]:
                  - strong [ref=e245]: UI benchmark node 00001
                  - generic [ref=e246]: node:UIF-PRECISION-00001
              - treeitem "UI benchmark node 00002 node:UIF-PRECISION-00002" [level=2] [ref=e248] [cursor=pointer]:
                - img [ref=e249]
                - generic [ref=e252]:
                  - strong [ref=e253]: UI benchmark node 00002
                  - generic [ref=e254]: node:UIF-PRECISION-00002
              - treeitem "UI benchmark node 00003 node:UIF-PRECISION-00003" [level=2] [ref=e256] [cursor=pointer]:
                - img [ref=e257]
                - generic [ref=e260]:
                  - strong [ref=e261]: UI benchmark node 00003
                  - generic [ref=e262]: node:UIF-PRECISION-00003
              - treeitem "UI benchmark node 00004 node:UIF-PRECISION-00004" [level=2] [ref=e264] [cursor=pointer]:
                - img [ref=e265]
                - generic [ref=e268]:
                  - strong [ref=e269]: UI benchmark node 00004
                  - generic [ref=e270]: node:UIF-PRECISION-00004
              - treeitem "UI benchmark node 00005 node:UIF-PRECISION-00005" [level=2] [ref=e272] [cursor=pointer]:
                - img [ref=e273]
                - generic [ref=e276]:
                  - strong [ref=e277]: UI benchmark node 00005
                  - generic [ref=e278]: node:UIF-PRECISION-00005
              - treeitem "UI benchmark node 00006 node:UIF-PRECISION-00006" [level=2] [ref=e280] [cursor=pointer]:
                - img [ref=e281]
                - generic [ref=e284]:
                  - strong [ref=e285]: UI benchmark node 00006
                  - generic [ref=e286]: node:UIF-PRECISION-00006
              - treeitem "UI benchmark node 00007 node:UIF-PRECISION-00007" [level=2] [ref=e288] [cursor=pointer]:
                - img [ref=e289]
                - generic [ref=e292]:
                  - strong [ref=e293]: UI benchmark node 00007
                  - generic [ref=e294]: node:UIF-PRECISION-00007
              - treeitem "UI benchmark node 00008 node:UIF-PRECISION-00008" [level=2] [ref=e296] [cursor=pointer]:
                - img [ref=e297]
                - generic [ref=e300]:
                  - strong [ref=e301]: UI benchmark node 00008
                  - generic [ref=e302]: node:UIF-PRECISION-00008
              - treeitem "UI benchmark node 00009 node:UIF-PRECISION-00009" [level=2] [ref=e304] [cursor=pointer]:
                - img [ref=e305]
                - generic [ref=e308]:
                  - strong [ref=e309]: UI benchmark node 00009
                  - generic [ref=e310]: node:UIF-PRECISION-00009
              - treeitem "UI benchmark node 00010 node:UIF-PRECISION-00010" [level=2] [ref=e312] [cursor=pointer]:
                - img [ref=e313]
                - generic [ref=e316]:
                  - strong [ref=e317]: UI benchmark node 00010
                  - generic [ref=e318]: node:UIF-PRECISION-00010
              - treeitem "UI benchmark node 00011 node:UIF-PRECISION-00011" [level=2] [ref=e320] [cursor=pointer]:
                - img [ref=e321]
                - generic [ref=e324]:
                  - strong [ref=e325]: UI benchmark node 00011
                  - generic [ref=e326]: node:UIF-PRECISION-00011
              - treeitem "UI benchmark node 00012 node:UIF-PRECISION-00012" [level=2] [ref=e328] [cursor=pointer]:
                - img [ref=e329]
                - generic [ref=e332]:
                  - strong [ref=e333]: UI benchmark node 00012
                  - generic [ref=e334]: node:UIF-PRECISION-00012
              - treeitem "UI benchmark node 00013 node:UIF-PRECISION-00013" [level=2] [ref=e336] [cursor=pointer]:
                - img [ref=e337]
                - generic [ref=e340]:
                  - strong [ref=e341]: UI benchmark node 00013
                  - generic [ref=e342]: node:UIF-PRECISION-00013
              - treeitem "UI benchmark node 00014 node:UIF-PRECISION-00014" [level=2] [ref=e344] [cursor=pointer]:
                - img [ref=e345]
                - generic [ref=e348]:
                  - strong [ref=e349]: UI benchmark node 00014
                  - generic [ref=e350]: node:UIF-PRECISION-00014
              - treeitem "UI benchmark node 00015 node:UIF-PRECISION-00015" [level=2] [ref=e352] [cursor=pointer]:
                - img [ref=e353]
                - generic [ref=e356]:
                  - strong [ref=e357]: UI benchmark node 00015
                  - generic [ref=e358]: node:UIF-PRECISION-00015
              - treeitem "UI benchmark node 00016 node:UIF-PRECISION-00016" [level=2] [ref=e360] [cursor=pointer]:
                - img [ref=e361]
                - generic [ref=e364]:
                  - strong [ref=e365]: UI benchmark node 00016
                  - generic [ref=e366]: node:UIF-PRECISION-00016
              - treeitem "UI benchmark node 00017 node:UIF-PRECISION-00017" [level=2] [ref=e368] [cursor=pointer]:
                - img [ref=e369]
                - generic [ref=e372]:
                  - strong [ref=e373]: UI benchmark node 00017
                  - generic [ref=e374]: node:UIF-PRECISION-00017
              - treeitem "UI benchmark node 00018 node:UIF-PRECISION-00018" [level=2] [ref=e376] [cursor=pointer]:
                - img [ref=e377]
                - generic [ref=e380]:
                  - strong [ref=e381]: UI benchmark node 00018
                  - generic [ref=e382]: node:UIF-PRECISION-00018
              - treeitem "UI benchmark node 00019 node:UIF-PRECISION-00019" [level=2] [ref=e384] [cursor=pointer]:
                - img [ref=e385]
                - generic [ref=e388]:
                  - strong [ref=e389]: UI benchmark node 00019
                  - generic [ref=e390]: node:UIF-PRECISION-00019
              - treeitem "UI benchmark node 00020 node:UIF-PRECISION-00020" [level=2] [ref=e392] [cursor=pointer]:
                - img [ref=e393]
                - generic [ref=e396]:
                  - strong [ref=e397]: UI benchmark node 00020
                  - generic [ref=e398]: node:UIF-PRECISION-00020
              - treeitem "Pipes" [expanded] [level=1] [ref=e400] [cursor=pointer]:
                - generic [ref=e401]: ▾
                - strong [ref=e402]: Pipes
              - treeitem "UI benchmark pipe 00001 pipe:UIF-PRECISION-00001" [level=2] [ref=e404] [cursor=pointer]:
                - img [ref=e405]
                - generic [ref=e409]:
                  - strong [ref=e410]: UI benchmark pipe 00001
                  - generic [ref=e411]: pipe:UIF-PRECISION-00001
              - treeitem "UI benchmark pipe 00002 pipe:UIF-PRECISION-00002" [level=2] [ref=e413] [cursor=pointer]:
                - img [ref=e414]
                - generic [ref=e418]:
                  - strong [ref=e419]: UI benchmark pipe 00002
                  - generic [ref=e420]: pipe:UIF-PRECISION-00002
              - treeitem "UI benchmark pipe 00003 pipe:UIF-PRECISION-00003" [level=2] [ref=e422] [cursor=pointer]:
                - img [ref=e423]
                - generic [ref=e427]:
                  - strong [ref=e428]: UI benchmark pipe 00003
                  - generic [ref=e429]: pipe:UIF-PRECISION-00003
              - treeitem "UI benchmark pipe 00004 pipe:UIF-PRECISION-00004" [level=2] [ref=e431] [cursor=pointer]:
                - img [ref=e432]
                - generic [ref=e436]:
                  - strong [ref=e437]: UI benchmark pipe 00004
                  - generic [ref=e438]: pipe:UIF-PRECISION-00004
              - treeitem "UI benchmark pipe 00005 pipe:UIF-PRECISION-00005" [level=2] [ref=e440] [cursor=pointer]:
                - img [ref=e441]
                - generic [ref=e445]:
                  - strong [ref=e446]: UI benchmark pipe 00005
                  - generic [ref=e447]: pipe:UIF-PRECISION-00005
              - treeitem "UI benchmark pipe 00006 pipe:UIF-PRECISION-00006" [level=2] [ref=e449] [cursor=pointer]:
                - img [ref=e450]
                - generic [ref=e454]:
                  - strong [ref=e455]: UI benchmark pipe 00006
                  - generic [ref=e456]: pipe:UIF-PRECISION-00006
              - treeitem "UI benchmark pipe 00007 pipe:UIF-PRECISION-00007" [level=2] [ref=e458] [cursor=pointer]:
                - img [ref=e459]
                - generic [ref=e463]:
                  - strong [ref=e464]: UI benchmark pipe 00007
                  - generic [ref=e465]: pipe:UIF-PRECISION-00007
              - treeitem "UI benchmark pipe 00008 pipe:UIF-PRECISION-00008" [level=2] [ref=e467] [cursor=pointer]:
                - img [ref=e468]
                - generic [ref=e472]:
                  - strong [ref=e473]: UI benchmark pipe 00008
                  - generic [ref=e474]: pipe:UIF-PRECISION-00008
              - treeitem "UI benchmark pipe 00009 pipe:UIF-PRECISION-00009" [level=2] [ref=e476] [cursor=pointer]:
                - img [ref=e477]
                - generic [ref=e481]:
                  - strong [ref=e482]: UI benchmark pipe 00009
                  - generic [ref=e483]: pipe:UIF-PRECISION-00009
              - treeitem "UI benchmark pipe 00010 pipe:UIF-PRECISION-00010" [level=2] [ref=e485] [cursor=pointer]:
                - img [ref=e486]
                - generic [ref=e490]:
                  - strong [ref=e491]: UI benchmark pipe 00010
                  - generic [ref=e492]: pipe:UIF-PRECISION-00010
              - treeitem "UI benchmark pipe 00011 pipe:UIF-PRECISION-00011" [level=2] [ref=e494] [cursor=pointer]:
                - img [ref=e495]
                - generic [ref=e499]:
                  - strong [ref=e500]: UI benchmark pipe 00011
                  - generic [ref=e501]: pipe:UIF-PRECISION-00011
              - treeitem "UI benchmark pipe 00012 pipe:UIF-PRECISION-00012" [level=2] [ref=e503] [cursor=pointer]:
                - img [ref=e504]
                - generic [ref=e508]:
                  - strong [ref=e509]: UI benchmark pipe 00012
                  - generic [ref=e510]: pipe:UIF-PRECISION-00012
              - treeitem "UI benchmark pipe 00013 pipe:UIF-PRECISION-00013" [level=2] [ref=e512] [cursor=pointer]:
                - img [ref=e513]
                - generic [ref=e517]:
                  - strong [ref=e518]: UI benchmark pipe 00013
                  - generic [ref=e519]: pipe:UIF-PRECISION-00013
              - treeitem "UI benchmark pipe 00014 pipe:UIF-PRECISION-00014" [level=2] [ref=e521] [cursor=pointer]:
                - img [ref=e522]
                - generic [ref=e526]:
                  - strong [ref=e527]: UI benchmark pipe 00014
                  - generic [ref=e528]: pipe:UIF-PRECISION-00014
              - treeitem "UI benchmark pipe 00015 pipe:UIF-PRECISION-00015" [level=2] [ref=e530] [cursor=pointer]:
                - img [ref=e531]
                - generic [ref=e535]:
                  - strong [ref=e536]: UI benchmark pipe 00015
                  - generic [ref=e537]: pipe:UIF-PRECISION-00015
              - treeitem "UI benchmark pipe 00016 pipe:UIF-PRECISION-00016" [level=2] [ref=e539] [cursor=pointer]:
                - img [ref=e540]
                - generic [ref=e544]:
                  - strong [ref=e545]: UI benchmark pipe 00016
                  - generic [ref=e546]: pipe:UIF-PRECISION-00016
              - treeitem "UI benchmark pipe 00017 pipe:UIF-PRECISION-00017" [level=2] [ref=e548] [cursor=pointer]:
                - img [ref=e549]
                - generic [ref=e553]:
                  - strong [ref=e554]: UI benchmark pipe 00017
                  - generic [ref=e555]: pipe:UIF-PRECISION-00017
              - treeitem "UI benchmark pipe 00018 pipe:UIF-PRECISION-00018" [level=2] [ref=e557] [cursor=pointer]:
                - img [ref=e558]
                - generic [ref=e562]:
                  - strong [ref=e563]: UI benchmark pipe 00018
                  - generic [ref=e564]: pipe:UIF-PRECISION-00018
              - treeitem "UI benchmark pipe 00019 pipe:UIF-PRECISION-00019" [level=2] [ref=e566] [cursor=pointer]:
                - img [ref=e567]
                - generic [ref=e571]:
                  - strong [ref=e572]: UI benchmark pipe 00019
                  - generic [ref=e573]: pipe:UIF-PRECISION-00019
              - treeitem "UI benchmark pipe 00020 pipe:UIF-PRECISION-00020" [level=2] [ref=e575] [cursor=pointer]:
                - img [ref=e576]
                - generic [ref=e580]:
                  - strong [ref=e581]: UI benchmark pipe 00020
                  - generic [ref=e582]: pipe:UIF-PRECISION-00020
              - treeitem "Supports" [expanded] [level=1] [ref=e584] [cursor=pointer]:
                - generic [ref=e585]: ▾
                - strong [ref=e586]: Supports
              - treeitem "UI benchmark support 00020 support:UIF-PRECISION-00020" [level=2] [ref=e588] [cursor=pointer]:
                - img [ref=e589]
                - generic [ref=e592]:
                  - strong [ref=e593]: UI benchmark support 00020
                  - generic [ref=e594]: support:UIF-PRECISION-00020
      - separator "Resize table and canvas" [ref=e595]
      - generic [ref=e597]:
        - generic [ref=e598]:
          - group "Viewport controls" [ref=e599]:
            - generic [ref=e600]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e601]:
              - generic "Deformation · unavailable" [ref=e602] [cursor=pointer]
            - group "Viewport display toggles" [ref=e603]:
              - button "Labels" [ref=e604]
              - button "Loads" [pressed] [ref=e605]
              - button "Grid" [pressed] [ref=e606]
            - group "Viewport selection tools" [ref=e607]:
              - button "Box Select" [ref=e608]
              - generic [ref=e609]:
                - generic [ref=e610]: Selection filter
                - combobox "Selection filter" [ref=e611]:
                  - option "All"
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components" [selected]
              - button "Hide" [disabled] [ref=e612]
              - button "Isolate" [disabled] [ref=e613]
              - button "Show All" [disabled] [ref=e614]
              - button "Fit Model" [ref=e615]
              - button "Fit Visible" [ref=e616]
              - button "Fit Selection" [disabled] [ref=e617]
            - group "Viewport geometry" [ref=e618]:
              - button "Schematic" [pressed] [ref=e619]
              - button "Actual OD" [ref=e620]
              - button "Measure" [ref=e621]
          - generic "Viewport status" [ref=e622]:
            - 'generic "Selected project: project:UIF-PRECISION-ORIGIN" [ref=e623]': "Selected: project:UIF-PRECISION-ORIGIN"
            - status "Schematic centerline geometry" [ref=e624]
            - status "View command status" [ref=e625]: Box Select disabled.
        - generic [ref=e626]:
          - generic "Three.js pipe centerline viewport" [ref=e627]
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e629]:
            - button "Front" [ref=e630] [cursor=pointer]
            - button "Top" [ref=e631] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e632] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e633]:
          - generic "Object creation tools" [ref=e634]:
            - button "Node" [ref=e635] [cursor=pointer]:
              - img [ref=e636]
              - text: Node
            - button "Pipe" [ref=e638] [cursor=pointer]:
              - img [ref=e639]
              - text: Pipe
            - button "Support" [ref=e643] [cursor=pointer]:
              - img [ref=e644]
              - text: Support
            - button "Component" [ref=e647] [cursor=pointer]:
              - img [ref=e648]
              - text: Component
            - button "Load" [ref=e651] [cursor=pointer]:
              - img [ref=e652]
              - text: Load
          - generic "Model focus" [ref=e654]: Select
          - group [ref=e655]:
            - generic "Selection & navigation" [ref=e656] [cursor=pointer]
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e657]:
      - button "Agent" [disabled] [ref=e659]:
        - img [ref=e660]
        - generic [ref=e663]: Agent
  - generic "Workspace status" [ref=e664]:
    - generic "Analysis statuses"
    - button "2 Issues" [ref=e665] [cursor=pointer]:
      - img [ref=e666]
      - text: 2 Issues
    - generic "Selection" [ref=e668]: "project: project:UIF-PRECISION-ORIGIN"
    - generic "Display units" [ref=e669]: Entered
    - button "About SWBPIPE…" [ref=e670] [cursor=pointer]:
      - img [ref=e671]
```

# Test source

```ts
  1596 |   await identity([node], node, node);
  1597 |   await selectTreeRow(page, node.type, node.id, { toggle: true });
  1598 |   await identity([], null, project);
  1599 |   await testInfo.attach("actual-empty-selection-publications", { body: JSON.stringify({ model: model.project.id, independentEmptyBasis: "fixture has zero components", records }, null, 2), contentType: "application/json" });
  1600 |   await page.screenshot({ path: testInfo.outputPath("project-after-empty.png") });
  1601 | });
  1602 | 
  1603 | 
  1604 | test("decorative viewport overlays pass real canvas gestures while view controls stay interactive", async ({ page }, testInfo) => {
  1605 |   // The frozen Box16 endpoints below were characterized against the canvas the
  1606 |   // shell gave a 1440 x 920 window while it still carried its footer (about 21 px).
  1607 |   // DEC-105 removed that footer, so the window is 21 px shorter here to keep
  1608 |   // the canvas, the gizmo and the frozen endpoints in the same geometry.
  1609 |   // 2026-09-19, slice B3 (the shell): the same move again. Before the shell a
  1610 |   // 1440 x 899 window gave this test a drawn canvas of 794 x 559 CSS px
  1611 |   // (measured on main at d20eb1294). The shell's Both view gives that same box
  1612 |   // to a 1688 x 787 window with the stored split at 50 % (surfaces 1588 wide,
  1613 |   // canvas pane 794; 787 less the 228 px of bars above and below the drawn canvas is 559).
  1614 |   // The box is asserted below, so the geometry the frozen endpoints depend on is
  1615 |   // pinned and not implied. The endpoints and every other assertion are untouched.
  1616 |   await page.setViewportSize({ width: 1688, height: 787 });
  1617 |   await page.addInitScript(() => {
  1618 |     window.localStorage.setItem("chirality.desktop.ui-preferences.v1", JSON.stringify({ version: 1, bothSplitPct: 50 }));
  1619 |   });
  1620 |   const model = await gotoRoutedFixture(page);
  1621 |   expect(model.components ?? []).toHaveLength(0);
  1622 |   await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  1623 |   await expect(page.getByTestId("toggle-viewport-labels")).toHaveAttribute("aria-pressed", "false");
  1624 |   const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  1625 |   const axis = page.getByRole("img", { name: "Orientation gizmo showing X, Y, Z axes", exact: true });
  1626 |   const scale = page.getByTestId("viewport-scale-bar");
  1627 |   await expect(axis).toBeVisible(); await expect(scale).toHaveText(/1\s*m/);
  1628 |   const rect = (await canvas.boundingBox())!;
  1629 |   expect({ width: rect.width, height: rect.height }).toEqual({ width: 794, height: 559 });
  1630 |   const axisRect = (await axis.boundingBox())!;
  1631 |   const scaleRect = (await scale.boundingBox())!;
  1632 |   // Preserve both frozen Box16 endpoints. This component-free control fixture
  1633 |   // supplies an independent empty expectation; it is not a cohort replay.
  1634 |   const start = { x: rect.x + rect.width * 0.29, y: rect.y + rect.height * 0.58 };
  1635 |   const end = { x: rect.x + rect.width * 0.08, y: rect.y + rect.height * 0.81 };
  1636 |   const scalePoint = { x: scaleRect.x + scaleRect.width / 2, y: scaleRect.y + scaleRect.height / 2 };
  1637 |   expect(end.x).toBeGreaterThan(axisRect.x); expect(end.x).toBeLessThan(axisRect.x + axisRect.width);
  1638 |   expect(end.y).toBeGreaterThan(axisRect.y); expect(end.y).toBeLessThan(axisRect.y + axisRect.height);
  1639 |   await page.evaluate(() => {
  1640 |     const records: any[] = []; (globalThis as any).__decorativePointerWitness = records;
  1641 |     for (const type of ["pointerdown", "pointerup", "wheel"]) document.addEventListener(type, (event) => {
  1642 |       const target = event.target as HTMLElement;
  1643 |       const pointer = event as MouseEvent;
  1644 |       records.push({ type, trusted: event.isTrusted, x: pointer.clientX, y: pointer.clientY,
  1645 |         mainCanvas: target === document.querySelector('[data-testid="viewport-canvas"] canvas'),
  1646 |         targetTag: target.tagName, targetTestId: target.dataset.testid ?? null });
  1647 |     }, { capture: true, passive: true });
  1648 |   });
  1649 |   const hits = await page.evaluate((points) => points.map((point) => {
  1650 |     const target = document.elementFromPoint(point.x, point.y) as HTMLElement | null;
  1651 |     return { point, mainCanvas: target === document.querySelector('[data-testid="viewport-canvas"] canvas'),
  1652 |       tag: target?.tagName, testId: target?.dataset.testid ?? null };
  1653 |   }), [start, end, scalePoint]);
  1654 |   // Record trusted events at both decorative surfaces before asserting passthrough,
  1655 |   // so the unpatched failure includes actual pointer ownership, not CSS inspection.
  1656 |   for (const point of [end, scalePoint]) { await page.mouse.move(point.x, point.y); await page.mouse.down(); await page.mouse.up(); }
  1657 |   const pointerPreconditions = await page.evaluate(() => (globalThis as any).__decorativePointerWitness);
  1658 |   await testInfo.attach("decorative-pointer-preconditions", { body: JSON.stringify({ rect, axisRect, scaleRect, hits, pointerPreconditions }, null, 2), contentType: "application/json" });
  1659 |   expect(hits.every((hit) => hit.mainCanvas), JSON.stringify(hits)).toBe(true);
  1660 |   expect(pointerPreconditions.every((event: any) => event.trusted && event.mainCanvas)).toBe(true);
  1661 |   const read = async () => page.evaluate(() => {
  1662 |     const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
  1663 |     if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
  1664 |     return snapshot.viewport;
  1665 |   });
  1666 |   const settled = async () => {
  1667 |     await expect.poll(async () => (await read()).resources.ownedPendingRafCount).toBe(0);
  1668 |     return read();
  1669 |   };
  1670 |   await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  1671 |   await page.getByTestId("viewport-selection-filter").selectOption("components");
  1672 |   const beforeBox = await settled();
  1673 |   await page.mouse.move(start.x, start.y); await page.mouse.down();
  1674 |   await page.mouse.move(end.x, end.y, { steps: 6 }); await page.mouse.up();
  1675 |   await expect.poll(async () => { const box = (await read()).box; return "status" in box ? null : box.orderedRefs; }).toEqual([]);
  1676 |   const afterBox = await settled();
  1677 |   expect(afterBox.box).toMatchObject({ orderedRefs: [], primaryRef: null, direction: "right-to-left", filter: "components" });
  1678 |   expect(afterBox.selection.orderedRefs).toEqual([]); expect(afterBox.selection.primaryRef).toBeNull();
  1679 |   expect(afterBox.camera).toEqual(beforeBox.camera); expect(afterBox.canvas).toEqual(beforeBox.canvas);
  1680 |   await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  1681 |   const orbitBefore = await settled();
  1682 |   await page.mouse.move(end.x, end.y); await page.mouse.down();
  1683 |   await page.mouse.move(end.x + 42, end.y - 28, { steps: 8 }); await page.mouse.up();
  1684 |   const orbitAfter = await settled();
  1685 |   expect(orbitAfter.camera.position).not.toEqual(orbitBefore.camera.position);
  1686 |   expect(orbitAfter.camera.target).toEqual(orbitBefore.camera.target);
  1687 |   await page.mouse.move(scalePoint.x, scalePoint.y); await page.mouse.down({ button: "right" });
  1688 |   await page.mouse.move(scalePoint.x - 40, scalePoint.y - 25, { steps: 8 }); await page.mouse.up({ button: "right" });
  1689 |   const panAfter = await settled();
  1690 |   expect(panAfter.camera.target).not.toEqual(orbitAfter.camera.target);
  1691 |   panAfter.camera.position.forEach((value, i) => expect(value - orbitAfter.camera.position[i]).toBeCloseTo(panAfter.camera.target[i] - orbitAfter.camera.target[i], 8));
  1692 |   const distance = (state: typeof panAfter) => Math.hypot(...state.camera.position.map((value, i) => value - state.camera.target[i]));
  1693 |   await page.mouse.move(scalePoint.x, scalePoint.y); await page.mouse.wheel(0, -120);
  1694 |   await expect.poll(async () => distance(await read())).toBeLessThan(distance(panAfter));
  1695 |   const wheelAfter = await settled();
> 1696 |   expect(wheelAfter.camera.target).toEqual(panAfter.camera.target);
       |                                    ^ Error: expect(received).toEqual(expected) // deep equality
  1697 |   const front = page.getByRole("button", { name: "Front", exact: true });
  1698 |   await front.click(); await expect(front).toHaveAttribute("aria-pressed", "true");
  1699 |   const frontAfter = await settled(); expect(frontAfter.camera.position).not.toEqual(wheelAfter.camera.position);
  1700 |   await page.getByTestId("viewport-view-isometric").click(); await settled();
  1701 |   await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  1702 |   const node = model.nodes[10]; await selectTreeRow(page, "node", node.id);
  1703 |   const label = page.getByRole("button", { name: `Select ${node.label} in viewport`, exact: true });
  1704 |   await expect(label).toBeVisible(); await label.click();
  1705 |   await expect(page.getByTestId("command-selection-readout")).toContainText(`node: ${node.id}`);
  1706 |   await expect(axis).toBeVisible(); await expect(scale).toHaveText(/1\s*m/);
  1707 |   const events = await page.evaluate(() => (globalThis as any).__decorativePointerWitness);
  1708 |   expect(events.filter((event: any) => event.type === "wheel")).toEqual(expect.arrayContaining([expect.objectContaining({ trusted: true, mainCanvas: true })]));
  1709 |   await testInfo.attach("decorative-pointer-gesture-results", { body: JSON.stringify({ beforeBox, afterBox, orbitBefore, orbitAfter, panAfter, wheelAfter, frontAfter, events }, null, 2), contentType: "application/json" });
  1710 |   await page.screenshot({ path: testInfo.outputPath("decorative-overlays-and-view-controls.png") });
  1711 | });
  1712 | 
  1713 | // V30: genuine captured-pointer lifetime, including independent normal Open control.
  1714 | test.describe("captured Box lifetime", () => {
  1715 | async function read(page: Page) {
  1716 |   return page.evaluate(() => {
  1717 |     const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
  1718 |     if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
  1719 |     return { snapshot, rectangleCount: document.querySelectorAll(".viewport-box-rect").length,
  1720 |       readout: document.querySelector('[data-testid="command-selection-readout"]')?.textContent,
  1721 |       inspector: document.querySelector('[data-testid="property-inspector"] h2')?.textContent,
  1722 |       publicPreparationControls: [...document.querySelectorAll<HTMLElement>('[data-testid*="draft-review"], [data-testid*="direct-review"]')].map(e => ({ id: e.dataset.testid, text: e.textContent })),
  1723 |       events: (globalThis as any).__boxGestureEvents };
  1724 |   });
  1725 | }
  1726 | async function settle(page: Page) {
  1727 |   await expect.poll(async () => (await read(page)).snapshot.viewport.resources.ownedPendingRafCount).toBe(0);
  1728 |   return read(page);
  1729 | }
  1730 | async function setup(page: Page) {
  1731 |   const model = await gotoRoutedFixture(page); expect(model.components ?? []).toHaveLength(0);
  1732 |   await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  1733 |   await selectTreeRow(page, "node", model.nodes[10].id);
  1734 |   await expect.poll(async () => (await read(page)).snapshot.viewport.selection.orderedRefs).toEqual([{ type: "node", id: model.nodes[10].id }]);
  1735 |   await page.evaluate(() => {
  1736 |     const events: any[] = []; (globalThis as any).__boxGestureEvents = events;
  1737 |     for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel", "lostpointercapture", "keydown"]) document.addEventListener(type, event => {
  1738 |       const e = event as PointerEvent & KeyboardEvent;
  1739 |       events.push({ type, trusted: event.isTrusted, key: e.key ?? null, pointerId: e.pointerId ?? null, buttons: e.buttons ?? null,
  1740 |         targetTag: (event.target as Element).tagName, mainCanvas: event.target === document.querySelector('[data-testid="viewport-canvas"] canvas') });
  1741 |     }, true);
  1742 |   });
  1743 |   return model;
  1744 | }
  1745 | async function begin(page: Page) {
  1746 |   await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  1747 |   await page.getByTestId("viewport-selection-filter").selectOption("components");
  1748 |   const rect = (await page.getByTestId("viewport-canvas").locator("canvas").boundingBox())!;
  1749 |   const start = { x: rect.x + .29 * rect.width, y: rect.y + .58 * rect.height };
  1750 |   const end = { x: rect.x + .08 * rect.width, y: rect.y + .81 * rect.height };
  1751 |   for (const point of [start, end]) expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y) === document.querySelector('[data-testid="viewport-canvas"] canvas'), point)).toBe(true);
  1752 |   const before = await settle(page);
  1753 |   await page.mouse.move(start.x, start.y); await page.mouse.down();
  1754 |   await page.mouse.move((start.x + end.x) / 2, (start.y + end.y) / 2, { steps: 5 });
  1755 |   expect(await page.locator(".viewport-box-rect").count()).toBe(1);
  1756 |   return { before, start, end, rect };
  1757 | }
  1758 | async function finish(page: Page, end: { x: number; y: number }) {
  1759 |   await page.mouse.move(end.x, end.y, { steps: 5 }); await page.mouse.up();
  1760 |   return settle(page);
  1761 | }
  1762 | 
  1763 | test("ordinary real Box applies independent empty Components membership", async ({ page }, info) => {
  1764 |   await setup(page); const input = await begin(page); const after = await finish(page, input.end);
  1765 |   await info.attach("ordinary-box", { body: JSON.stringify({ input, after }, null, 2), contentType: "application/json" });
  1766 |   expect(after.snapshot.viewport.selection.orderedRefs).toEqual([]); expect(after.snapshot.viewport.selection.primaryRef).toBeNull();
  1767 |   expect(after.snapshot.viewport.box).toMatchObject({ orderedRefs: [], primaryRef: null, filter: "components" });
  1768 |   expect(after.snapshot.viewport.camera).toEqual(input.before.snapshot.viewport.camera);
  1769 | });
  1770 | 
  1771 | test("Escape retires captured Box before delayed pointer up", async ({ page }, info) => {
  1772 |   await setup(page); const input = await begin(page);
  1773 |   await page.keyboard.press("Escape"); const afterEscape = await read(page);
  1774 |   await page.screenshot({ path: info.outputPath("after-escape-before-up.png") });
  1775 |   const after = await finish(page, input.end);
  1776 |   await info.attach("escape-reproduction", { body: JSON.stringify({ input, afterEscape, after }, null, 2), contentType: "application/json" });
  1777 |   await page.screenshot({ path: info.outputPath("after-delayed-up.png") });
  1778 |   expect.soft(afterEscape.rectangleCount).toBe(0);
  1779 |   expect.soft(after.snapshot.viewport.selection.orderedRefs).toEqual(input.before.snapshot.viewport.selection.orderedRefs);
  1780 |   expect.soft(after.snapshot.viewport.selection.primaryRef).toEqual(input.before.snapshot.viewport.selection.primaryRef);
  1781 |   expect.soft(after.snapshot.viewport.selection.actionSequence).toBe(input.before.snapshot.viewport.selection.actionSequence);
  1782 |   expect.soft(after.snapshot.viewport.box).toEqual(input.before.snapshot.viewport.box);
  1783 |   expect.soft(after.publicPreparationControls).toEqual(input.before.publicPreparationControls);
  1784 |   expect(after.snapshot.viewport.camera).toEqual(input.before.snapshot.viewport.camera);
  1785 | });
  1786 | 
  1787 | test("same-ID Open retires old captured Box before delayed pointer up", async ({ page }, info) => {
  1788 |   const model = await setup(page); const project = { type: "project", id: model.project.id };
  1789 |   await projectCommand(page, "save-local");
  1790 |   await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  1791 |   const beforeOrdinaryOpen = await settle(page);
  1792 |   await projectCommand(page, "open-local", true);
  1793 |   await expect.poll(async () => (await read(page)).snapshot.model.projectSessionGeneration).toBe(beforeOrdinaryOpen.snapshot.model.projectSessionGeneration + 1);
  1794 |   await expect.poll(async () => (await read(page)).snapshot.viewport.selection.orderedRefs).toEqual([project]);
  1795 |   const ordinaryOpen = await settle(page);
  1796 |   expect(ordinaryOpen.snapshot.viewport.selection.primaryRef).toEqual(project);
```